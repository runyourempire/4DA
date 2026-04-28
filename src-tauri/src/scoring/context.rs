// SPDX-License-Identifier: FSL-1.1-Apache-2.0
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;
use tracing::info;

use crate::db::Database;
use crate::error::{Result, ResultExt};
use crate::taste_test::continuous;

use super::{compute_taste_embedding, get_ace_context, get_topic_embeddings, ScoringContext};

static SCORING_CONTEXT_CACHE: LazyLock<Mutex<Option<(ScoringContext, Instant)>>> =
    LazyLock::new(|| Mutex::new(None));
const SCORING_CONTEXT_TTL_SECS: u64 = 300;

/// Build a ScoringContext by loading all needed state. Call once per analysis run.
/// Results are cached with a 5-minute TTL to avoid redundant DB queries.
pub(crate) async fn build_scoring_context(db: &Database) -> Result<ScoringContext> {
    // Check cache first (block scope ensures MutexGuard is dropped before any .await)
    {
        let cache = SCORING_CONTEXT_CACHE.lock().unwrap_or_else(|e| {
            tracing::warn!("SCORING_CONTEXT_CACHE mutex poisoned, recovering");
            e.into_inner()
        });
        if let Some((ref ctx, ref instant)) = *cache {
            if instant.elapsed().as_secs() < SCORING_CONTEXT_TTL_SECS {
                return Ok(ctx.clone());
            }
        }
    }
    let cached_context_count = db.context_count()?;
    let feedback_interaction_count: i64 = db.query_feedback_count().unwrap_or(0);

    let context_engine = crate::get_context_engine()?;
    let static_identity = context_engine
        .get_static_identity()
        .context("Failed to load context")?;

    // User's explicit tech stack from onboarding (small, curated list)
    let declared_tech: Vec<String> = static_identity
        .tech_stack
        .iter()
        .map(|t| t.to_lowercase())
        .collect();

    // User's professional role from onboarding
    let user_role = static_identity.role.clone();

    // User's experience level (safe query — column may not exist yet)
    let experience_level: Option<String> = {
        let conn = crate::open_db_connection()?;
        conn.query_row(
            "SELECT experience_level FROM user_identity WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .ok()
        .flatten()
    };

    let mut ace_ctx = get_ace_context();

    // ── Build Negative Stack from direct runtime deps + competing tech ──
    {
        let direct_dep_names: std::collections::HashSet<String> = ace_ctx
            .dependency_info
            .iter()
            .filter(|(_, info)| info.is_direct && !info.is_dev)
            .map(|(name, _)| name.to_lowercase())
            .collect();

        let anti_topic_pairs: Vec<(String, f32)> = ace_ctx
            .anti_topics
            .iter()
            .filter_map(|t| {
                ace_ctx
                    .anti_topic_confidence
                    .get(t)
                    .map(|&conf| (t.clone(), conf))
            })
            .collect();

        ace_ctx.negative_stack = crate::stacks::negative_stack::build_negative_stack(
            &direct_dep_names,
            crate::competing_tech::COMPETING_TECH,
            &anti_topic_pairs,
        );

        if !ace_ctx.negative_stack.priors.is_empty() {
            tracing::debug!(target: "4da::scoring",
                suppressed_count = ace_ctx.negative_stack.priors.len(),
                "Negative stack built: {} technologies suppressed",
                ace_ctx.negative_stack.priors.len()
            );
        }
    }

    // Load session-aware work topics for intent scoring.
    // Uses gap-based session detection instead of a fixed 2h window:
    // current session = 1.0, previous same-day = 0.5, yesterday = 0.2
    let work_topics: Vec<String> = match crate::get_ace_engine() {
        Ok(ace) => ace
            .get_session_aware_work_topics()
            .unwrap_or_default()
            .into_iter()
            .map(|(topic, _weight)| topic)
            .collect(),
        Err(_) => vec![],
    };
    let has_active_work = !work_topics.is_empty();

    let topic_embeddings = get_topic_embeddings(&ace_ctx).await;

    // Load feedback-derived topic boosts (Phase 9: feedback learning loop)
    let mut feedback_boosts: HashMap<String, f64> = db
        .get_feedback_topic_summary()
        .unwrap_or_default()
        .into_iter()
        .map(|f| (f.topic, f.net_score))
        .collect();

    // Load source quality preferences from ACE behavior learning
    let source_quality: HashMap<String, f32> = match crate::get_ace_engine() {
        Ok(ace) => ace
            .get_source_preferences()
            .unwrap_or_default()
            .into_iter()
            .collect(),
        Err(_) => HashMap::new(),
    };

    // Open a single shared connection for all DB queries in context building
    let shared_conn = crate::open_db_connection()?;

    // Build domain profile for graduated domain relevance scoring
    let (
        domain_profile,
        composed_stack,
        open_windows,
        calibration_deltas,
        topic_half_lives,
        source_autopsies,
        anti_pattern_penalties,
        archetype_penalties,
        sovereign_profile,
    ) = {
        let dp = crate::domain_profile::build_domain_profile(&shared_conn);
        let cs = crate::stacks::load_composed_stack(&shared_conn);
        // Intelligence metabolism: load decision windows and autophagy calibrations
        let ow = crate::decision_advantage::get_open_windows(&shared_conn);
        let cd = crate::autophagy::load_calibration_deltas(&shared_conn);
        let thl = crate::autophagy::load_topic_decay_profiles(&shared_conn);
        // Autophagy intelligence: per-source engagement rates and anti-pattern penalties
        let sa = crate::autophagy::load_source_autopsies(&shared_conn);
        let ap = crate::autophagy::load_anti_patterns(&shared_conn);
        let arch = crate::autophagy::load_archetype_penalties(&shared_conn);
        // Unified profile (non-fatal if assembly fails)
        let sp = Some(crate::sovereign_developer_profile::assemble_profile(
            &shared_conn,
        ));
        (dp, cs, ow, cd, thl, sa, ap, arch, sp)
    };

    // ── ACE Auto-Enrichment: promote high-confidence detected tech into domain profile ──
    let mut domain_profile = domain_profile;
    {
        // Access raw ACE detected tech (with confidence/category) before it's flattened to strings
        let raw_detected_tech = match crate::get_ace_engine() {
            Ok(ace) => ace.get_detected_tech().unwrap_or_default(),
            Err(_) => vec![],
        };
        let mut promoted = 0usize;
        for tech in &raw_detected_tech {
            if tech.confidence >= 0.75
                && matches!(
                    tech.category,
                    crate::ace::TechCategory::Language | crate::ace::TechCategory::Framework
                )
            {
                let name_lower = tech.name.to_lowercase();
                if !domain_profile.primary_stack.contains(&name_lower)
                    && !domain_profile.all_tech.contains(&name_lower)
                {
                    domain_profile.all_tech.insert(name_lower.clone());
                    domain_profile.ace_promoted_tech.insert(name_lower);
                    promoted += 1;
                }
            }
        }
        if promoted > 0 {
            tracing::info!(target: "4da::scoring", promoted, "ACE auto-enrichment: promoted detected tech into domain profile");
        }
    }

    // ── Synthesize implicit interests from ACE-discovered context ──
    let mut interests = static_identity.interests;
    synthesize_ace_interests(&mut interests, &ace_ctx, &topic_embeddings);

    // ── Count implicit interactions for faster bootstrap exit ──
    let implicit_interaction_count: i64 = {
        let conn = crate::open_db_connection();
        match conn {
            Ok(c) => c
                .query_row(
                    "SELECT COUNT(*) FROM interactions WHERE ABS(signal_strength) >= 0.3",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0),
            Err(_) => 0,
        }
    };
    // 3 implicit signals = 1 effective explicit signal
    let effective_feedback_count = feedback_interaction_count + implicit_interaction_count / 3;

    // Warm-start source preferences from stack profiles (only fills gaps)
    let mut source_quality = source_quality;
    if composed_stack.active {
        for (&source, &pref) in &composed_stack.source_preferences {
            source_quality.entry(source.to_string()).or_insert(pref);
        }
    }

    // Compute taste embedding from topic affinities + topic embeddings
    // In bootstrap mode, use lower exposure threshold for faster learning
    let affinity_min_exposures = if effective_feedback_count < 10 { 2 } else { 5 };
    let taste_embedding = {
        let affinities: Vec<(String, f32, f32)> = match crate::get_ace_engine() {
            Ok(ace) => ace
                .get_topic_affinities_min(affinity_min_exposures)
                .unwrap_or_default()
                .into_iter()
                .filter(|a| a.confidence > 0.05)
                .map(|a| (a.topic, a.affinity_score, a.confidence))
                .collect(),
            Err(_) => vec![],
        };
        compute_taste_embedding(&affinities, &topic_embeddings)
    };

    // Load persona posterior and inject persona-derived topic boosts
    let dominant_persona = {
        let (weights, update_count) = continuous::load_posterior(&shared_conn).unwrap_or_default();
        if update_count > 0 {
            let (idx, &max_w) = weights
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, &0.0));
            // Above uniform threshold (1/9 ~ 0.11) with margin
            if max_w > 0.2 {
                let persona_boosts = continuous::get_persona_topic_boosts(idx, max_w as f32);
                for (topic, boost) in persona_boosts {
                    feedback_boosts
                        .entry(topic)
                        .and_modify(|v| *v += boost as f64)
                        .or_insert(boost as f64);
                }
                Some((idx, max_w as f32))
            } else {
                None
            }
        } else {
            None
        }
    };

    // Detect contradicted topics (both high affinity AND anti-topic).
    // Lightweight query — just topic names for necessity scoring.
    let contradicted_topics = {
        let conn = db.conn.lock();
        crate::anomaly::get_contradicted_topics(&conn).unwrap_or_default()
    };
    if !contradicted_topics.is_empty() {
        info!(target: "4da::scoring",
            count = contradicted_topics.len(),
            "Contradicted topics detected for necessity boosting"
        );
    }

    info!(target: "4da::ace",
        topics = ace_ctx.active_topics.len(),
        tech = ace_ctx.detected_tech.len(),
        embeddings = topic_embeddings.len(),
        feedback_topics = feedback_boosts.len(),
        source_prefs = source_quality.len(),
        domain_primary = domain_profile.primary_stack.len(),
        domain_all = domain_profile.all_tech.len(),
        stack_active = composed_stack.active,
        has_active_work,
        has_taste_embedding = taste_embedding.is_some(),
        has_dominant_persona = dominant_persona.is_some(),
        "ACE context loaded for scoring"
    );

    // ── Apply learned topic affinities to keyword interest weights ──
    {
        let affinities: HashMap<String, f32> = match crate::get_ace_engine() {
            Ok(ace) => ace
                .get_topic_affinities_min(affinity_min_exposures)
                .unwrap_or_default()
                .into_iter()
                .filter(|a| a.confidence > 0.3)
                .map(|a| (a.topic.to_lowercase(), a.affinity_score))
                .collect(),
            Err(_) => HashMap::new(),
        };
        apply_affinity_adjustments(&mut interests, &affinities);
    }

    if effective_feedback_count < 10 {
        info!(target: "4da::scoring",
            explicit = feedback_interaction_count,
            implicit = implicit_interaction_count,
            effective = effective_feedback_count,
            "Bootstrap mode: relaxed 1-signal gate for new user"
        );
    }

    let context = ScoringContext {
        cached_context_count,
        interest_count: interests.len(),
        interests,
        exclusions: static_identity.exclusions,
        ace_ctx,
        topic_embeddings,
        feedback_boosts,
        source_quality,
        declared_tech,
        domain_profile,
        work_topics,
        feedback_interaction_count: effective_feedback_count,
        composed_stack,
        open_windows,
        calibration_deltas,
        taste_embedding,
        topic_half_lives,
        source_autopsies,
        anti_pattern_penalties,
        archetype_penalties,
        contradicted_topics,
        sovereign_profile,
        dominant_persona,
        user_role,
        experience_level,
    };

    // Store in cache for subsequent calls within TTL
    {
        let mut cache = SCORING_CONTEXT_CACHE.lock().unwrap_or_else(|e| {
            tracing::warn!("SCORING_CONTEXT_CACHE mutex poisoned, recovering");
            e.into_inner()
        });
        *cache = Some((context.clone(), Instant::now()));
    }

    Ok(context)
}

/// Synthesize ACE-discovered context into keyword interests.
///
/// Bridges ACE intelligence into keyword scoring: detected tech, active topics,
/// and key dependencies become synthetic interests so the keyword engine matches
/// content against the user's actual stack — not just onboarding declarations.
pub(crate) fn synthesize_ace_interests(
    interests: &mut Vec<crate::context_engine::Interest>,
    ace_ctx: &super::ace_context::ACEContext,
    topic_embeddings: &HashMap<String, Vec<f32>>,
) {
    let mut existing: std::collections::HashSet<String> =
        interests.iter().map(|i| i.topic.to_lowercase()).collect();

    // Phase 1: Detected tech → keyword interests (weight from tech_weights)
    let mut tech_synth = 0usize;
    for tech_name in &ace_ctx.detected_tech {
        let lower = tech_name.to_lowercase();
        if existing.contains(&lower) {
            continue;
        }
        let weight = ace_ctx.tech_weights.get(tech_name).copied().unwrap_or(0.4);
        let embedding = topic_embeddings.get(tech_name).cloned();
        interests.push(crate::context_engine::Interest {
            id: None,
            topic: tech_name.clone(),
            weight,
            embedding,
            source: crate::context_engine::InterestSource::Inferred,
        });
        existing.insert(lower);
        tech_synth += 1;
    }
    if tech_synth > 0 {
        tracing::info!(target: "4da::scoring", tech_synth, "ACE auto-enrichment: synthesized interests from detected tech");
    }

    // Phase 2: Active topics → keyword interests (confidence-weighted)
    let mut topic_synth = 0usize;
    for topic_name in &ace_ctx.active_topics {
        if topic_synth >= 10 {
            break;
        }
        let conf = ace_ctx
            .topic_confidence
            .get(topic_name)
            .copied()
            .unwrap_or(0.0);
        if conf >= 0.65 && !existing.contains(&topic_name.to_lowercase()) {
            let embedding = topic_embeddings.get(topic_name).cloned();
            interests.push(crate::context_engine::Interest {
                id: None,
                topic: topic_name.clone(),
                weight: 0.5 + (conf - 0.65) * 0.7, // 0.65→0.50, 1.0→0.75
                embedding,
                source: crate::context_engine::InterestSource::Inferred,
            });
            existing.insert(topic_name.to_lowercase());
            topic_synth += 1;
        }
    }
    if topic_synth > 0 {
        tracing::info!(target: "4da::scoring", topic_synth, "ACE auto-enrichment: synthesized interests from active topics");
    }

    // Phase 3: Direct dependencies → low-weight keyword interests
    let mut dep_synth = 0usize;
    for (dep_name, dep_info) in &ace_ctx.dependency_info {
        if dep_synth >= 15 {
            break;
        }
        if !dep_info.is_direct || dep_info.is_dev {
            continue;
        }
        let lower = dep_name.to_lowercase();
        if existing.contains(&lower) || lower.len() < 3 {
            continue;
        }
        interests.push(crate::context_engine::Interest {
            id: None,
            topic: dep_name.clone(),
            weight: 0.3,
            embedding: None,
            source: crate::context_engine::InterestSource::Inferred,
        });
        existing.insert(lower);
        dep_synth += 1;
    }
    if dep_synth > 0 {
        tracing::info!(target: "4da::scoring", dep_synth, "ACE auto-enrichment: synthesized interests from direct dependencies");
    }
}

/// Apply learned topic affinities to keyword interest weights.
/// Positive affinity (+1.0) boosts weight by up to +0.2, negative (-1.0) reduces by up to -0.2.
pub(crate) fn apply_affinity_adjustments(
    interests: &mut [crate::context_engine::Interest],
    affinities: &HashMap<String, f32>,
) {
    if affinities.is_empty() {
        return;
    }
    let mut adjusted = 0usize;
    for interest in interests.iter_mut() {
        if let Some(&affinity) = affinities.get(&interest.topic.to_lowercase()) {
            let adjustment = affinity * 0.2;
            let new_weight = (interest.weight + adjustment).clamp(0.1, 1.0);
            if (new_weight - interest.weight).abs() > 0.01 {
                interest.weight = new_weight;
                adjusted += 1;
            }
        }
    }
    if adjusted > 0 {
        tracing::debug!(target: "4da::scoring", adjusted, "Applied topic affinity adjustments to keyword interest weights");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context_engine::{Interest, InterestSource};
    use crate::scoring::ace_context::ACEContext;
    use crate::scoring::dependencies::DepInfo;

    fn make_interest(topic: &str, weight: f32) -> Interest {
        Interest {
            id: None,
            topic: topic.to_string(),
            weight,
            embedding: None,
            source: InterestSource::Explicit,
        }
    }

    fn make_dep(name: &str, direct: bool, dev: bool) -> DepInfo {
        DepInfo {
            package_name: name.to_string(),
            version: None,
            is_dev: dev,
            is_direct: direct,
            search_terms: vec![],
            ecosystem: "rust".to_string(),
        }
    }

    // ── Phase 1: Detected tech synthesis ──

    #[test]
    fn test_detected_tech_becomes_interests() {
        let mut interests = vec![make_interest("Rust", 1.0)];
        let mut ace = ACEContext::default();
        ace.detected_tech = vec!["typescript".into(), "react".into()];
        ace.tech_weights.insert("typescript".into(), 0.85);
        ace.tech_weights.insert("react".into(), 0.40);

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 3);
        let ts = interests.iter().find(|i| i.topic == "typescript").unwrap();
        assert!(
            (ts.weight - 0.85).abs() < 0.01,
            "primary tech should get weight 0.85"
        );
        assert_eq!(ts.source, InterestSource::Inferred);

        let react = interests.iter().find(|i| i.topic == "react").unwrap();
        assert!(
            (react.weight - 0.40).abs() < 0.01,
            "secondary tech should get weight 0.40"
        );
    }

    #[test]
    fn test_detected_tech_no_duplicates() {
        let mut interests = vec![make_interest("Rust", 1.0)];
        let mut ace = ACEContext::default();
        ace.detected_tech = vec!["rust".into()]; // same as existing (case-insensitive)

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 1, "should not duplicate existing interest");
    }

    #[test]
    fn test_detected_tech_default_weight() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.detected_tech = vec!["python".into()];
        // No tech_weights entry → should default to 0.4

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 1);
        assert!(
            (interests[0].weight - 0.4).abs() < 0.01,
            "no tech_weight → default 0.4"
        );
    }

    // ── Phase 2: Active topic synthesis ──

    #[test]
    fn test_active_topics_high_confidence() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.active_topics = vec!["docker".into(), "kubernetes".into()];
        ace.topic_confidence.insert("docker".into(), 0.90);
        ace.topic_confidence.insert("kubernetes".into(), 0.65);

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 2);
        let docker = interests.iter().find(|i| i.topic == "docker").unwrap();
        // weight = 0.5 + (0.90 - 0.65) * 0.7 = 0.5 + 0.175 = 0.675
        assert!(
            (docker.weight - 0.675).abs() < 0.01,
            "docker weight should be ~0.675, got {}",
            docker.weight
        );

        let k8s = interests.iter().find(|i| i.topic == "kubernetes").unwrap();
        // weight = 0.5 + (0.65 - 0.65) * 0.7 = 0.5
        assert!(
            (k8s.weight - 0.5).abs() < 0.01,
            "kubernetes weight should be 0.5, got {}",
            k8s.weight
        );
    }

    #[test]
    fn test_active_topics_below_threshold_ignored() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.active_topics = vec!["lowconf".into()];
        ace.topic_confidence.insert("lowconf".into(), 0.50); // below 0.65

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(
            interests.len(),
            0,
            "low-confidence topics should not be synthesized"
        );
    }

    #[test]
    fn test_active_topics_cap_at_10() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        for i in 0..15 {
            let name = format!("topic_{i}");
            ace.active_topics.push(name.clone());
            ace.topic_confidence.insert(name, 0.80);
        }

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 10, "should cap at 10 topic interests");
    }

    // ── Phase 3: Dependency synthesis ──

    #[test]
    fn test_direct_deps_become_interests() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.dependency_info
            .insert("tokio".into(), make_dep("tokio", true, false));
        ace.dependency_info
            .insert("serde".into(), make_dep("serde", true, false));

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 2);
        for i in &interests {
            assert!((i.weight - 0.3).abs() < 0.01, "deps should have weight 0.3");
            assert_eq!(i.source, InterestSource::Inferred);
        }
    }

    #[test]
    fn test_dev_deps_excluded() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.dependency_info
            .insert("insta".into(), make_dep("insta", true, true)); // dev dep

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 0, "dev deps should not be synthesized");
    }

    #[test]
    fn test_transitive_deps_excluded() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.dependency_info
            .insert("syn".into(), make_dep("syn", false, false)); // transitive

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(
            interests.len(),
            0,
            "transitive deps should not be synthesized"
        );
    }

    #[test]
    fn test_short_dep_names_excluded() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        ace.dependency_info
            .insert("cc".into(), make_dep("cc", true, false)); // too short

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(
            interests.len(),
            0,
            "deps with name < 3 chars should be excluded"
        );
    }

    #[test]
    fn test_dep_cap_at_15() {
        let mut interests = vec![];
        let mut ace = ACEContext::default();
        for i in 0..20 {
            let name = format!("package-{i:02}");
            ace.dependency_info
                .insert(name.clone(), make_dep(&name, true, false));
        }

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert!(
            interests.len() <= 15,
            "should cap at 15 dep interests, got {}",
            interests.len()
        );
    }

    // ── Cross-phase deduplication ──

    #[test]
    fn test_no_cross_phase_duplicates() {
        let mut interests = vec![make_interest("tokio", 1.0)];
        let mut ace = ACEContext::default();
        // tokio appears in all three phases
        ace.detected_tech = vec!["tokio".into()];
        ace.tech_weights.insert("tokio".into(), 0.85);
        ace.active_topics = vec!["tokio".into()];
        ace.topic_confidence.insert("tokio".into(), 0.90);
        ace.dependency_info
            .insert("tokio".into(), make_dep("tokio", true, false));

        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        assert_eq!(interests.len(), 1, "tokio should appear exactly once");
        assert!(
            (interests[0].weight - 1.0).abs() < 0.01,
            "original explicit weight preserved"
        );
    }

    // ── Affinity adjustments ──

    #[test]
    fn test_positive_affinity_boosts_weight() {
        let mut interests = vec![make_interest("Rust", 0.6)];
        let mut affinities = HashMap::new();
        affinities.insert("rust".to_string(), 0.8); // strong positive

        apply_affinity_adjustments(&mut interests, &affinities);

        // 0.6 + 0.8 * 0.2 = 0.6 + 0.16 = 0.76
        assert!(
            (interests[0].weight - 0.76).abs() < 0.01,
            "weight should be ~0.76, got {}",
            interests[0].weight
        );
    }

    #[test]
    fn test_negative_affinity_reduces_weight() {
        let mut interests = vec![make_interest("Java", 0.6)];
        let mut affinities = HashMap::new();
        affinities.insert("java".to_string(), -1.0); // strong negative

        apply_affinity_adjustments(&mut interests, &affinities);

        // 0.6 + (-1.0) * 0.2 = 0.6 - 0.2 = 0.4
        assert!(
            (interests[0].weight - 0.4).abs() < 0.01,
            "weight should be ~0.4, got {}",
            interests[0].weight
        );
    }

    #[test]
    fn test_affinity_clamps_to_minimum() {
        let mut interests = vec![make_interest("Cobol", 0.15)];
        let mut affinities = HashMap::new();
        affinities.insert("cobol".to_string(), -1.0);

        apply_affinity_adjustments(&mut interests, &affinities);

        // 0.15 - 0.2 = -0.05 → clamped to 0.1
        assert!(
            (interests[0].weight - 0.1).abs() < 0.01,
            "weight should clamp to 0.1, got {}",
            interests[0].weight
        );
    }

    #[test]
    fn test_affinity_clamps_to_maximum() {
        let mut interests = vec![make_interest("Rust", 0.95)];
        let mut affinities = HashMap::new();
        affinities.insert("rust".to_string(), 1.0);

        apply_affinity_adjustments(&mut interests, &affinities);

        // 0.95 + 0.2 = 1.15 → clamped to 1.0
        assert!(
            (interests[0].weight - 1.0).abs() < 0.01,
            "weight should clamp to 1.0, got {}",
            interests[0].weight
        );
    }

    #[test]
    fn test_no_affinity_leaves_weight_unchanged() {
        let mut interests = vec![make_interest("Rust", 0.7)];
        let affinities = HashMap::new(); // empty

        apply_affinity_adjustments(&mut interests, &affinities);

        assert!(
            (interests[0].weight - 0.7).abs() < 0.001,
            "weight should be unchanged"
        );
    }

    // ── End-to-end: ACE synthesis → keyword scoring ──

    #[test]
    fn test_e2e_synthesized_interest_improves_keyword_score() {
        use crate::scoring::keywords::compute_keyword_interest_score;

        // Without ACE: only explicit "Rust" interest
        let explicit_only = vec![make_interest("Rust", 1.0)];
        let score_without = compute_keyword_interest_score(
            "New TypeScript 5.5 features for React developers",
            "TypeScript introduces new type guards and React Server Components support",
            &explicit_only,
        );

        // With ACE: "Rust" + synthesized "typescript" and "react"
        let mut with_ace = vec![make_interest("Rust", 1.0)];
        let mut ace = ACEContext::default();
        ace.detected_tech = vec!["typescript".into(), "react".into()];
        ace.tech_weights.insert("typescript".into(), 0.85);
        ace.tech_weights.insert("react".into(), 0.40);
        synthesize_ace_interests(&mut with_ace, &ace, &HashMap::new());

        let score_with = compute_keyword_interest_score(
            "New TypeScript 5.5 features for React developers",
            "TypeScript introduces new type guards and React Server Components support",
            &with_ace,
        );

        assert!(
            score_with > score_without + 0.1,
            "ACE synthesis should significantly improve score for stack-relevant content: \
             without={score_without:.3}, with={score_with:.3}"
        );
        assert_eq!(
            score_without, 0.0,
            "explicit Rust interest should not match TypeScript content"
        );
        assert!(
            score_with > 0.5,
            "synthesized interests should produce a strong match, got {score_with:.3}"
        );
    }

    #[test]
    fn test_e2e_dep_synthesis_catches_dependency_content() {
        use crate::scoring::keywords::compute_keyword_interest_score;

        let mut interests = vec![make_interest("Rust", 1.0)];
        let mut ace = ACEContext::default();
        ace.dependency_info
            .insert("tokio".into(), make_dep("tokio", true, false));
        ace.dependency_info
            .insert("serde".into(), make_dep("serde", true, false));
        synthesize_ace_interests(&mut interests, &ace, &HashMap::new());

        let score = compute_keyword_interest_score(
            "Tokio 2.0 release brings major async runtime improvements",
            "The tokio async runtime for Rust gets a major update with new scheduler",
            &interests,
        );

        assert!(
            score > 0.2,
            "content about a direct dependency should score, got {score:.3}"
        );
    }

    #[test]
    fn test_e2e_affinity_changes_keyword_ranking() {
        use crate::scoring::keywords::compute_keyword_interest_score;

        let mut interests = vec![make_interest("Rust", 0.5), make_interest("Python", 0.5)];

        // User engages heavily with Rust, dismisses Python
        let mut affinities = HashMap::new();
        affinities.insert("rust".to_string(), 0.9);
        affinities.insert("python".to_string(), -0.8);
        apply_affinity_adjustments(&mut interests, &affinities);

        let rust_score = compute_keyword_interest_score(
            "Advanced Rust async patterns",
            "Exploring async Rust with tokio and futures",
            &interests,
        );
        let python_score = compute_keyword_interest_score(
            "Advanced Python async patterns",
            "Exploring async Python with asyncio and aiohttp",
            &interests,
        );

        assert!(
            rust_score > python_score,
            "Rust (positive affinity) should rank higher than Python (negative): \
             rust={rust_score:.3}, python={python_score:.3}"
        );
    }
}
