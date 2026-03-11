# 4DA Team Tier — Implementation Plan

**Tier:** Team ($29/seat/mo)
**Philosophy:** A team's collective intelligence is exponentially more powerful than any individual's. Team tier doesn't just give everyone Signal features — it creates a shared nervous system where the whole team detects, decides, and acts faster than any member alone.

---

## What Makes Team Worth $29/seat/mo

Signal gives one developer superpowers. Team makes those superpowers **compound across every seat**.

The pitch: "Your team already tracks the same ecosystems. 4DA Team turns that overlap into a multiplier — shared signal detection, collective blind spot elimination, coordinated response to critical events, and an organizational memory that never forgets a decision."

---

## Phase 1: Foundation — Identity & Licensing (Week 1)

### 1.1 Keygen Seat Management
**Files:** `src-tauri/src/settings/license.rs`, `src-tauri/src/settings_commands_license.rs`

- Parse `meta.seats` and `data.attributes.metadata.entitlements` from Keygen response
- Extend `KeygenValidationResult` with `seats_total`, `seats_used`, `entitlements`
- Add `validate_seat_claim()` — checks if team has available seats before activation
- Cache seat metadata alongside tier in `license_cache.json`
- Seat enforcement: activation fails with clear error if seats exhausted

### 1.2 Team & Member Schema
**Files:** `src-tauri/src/db/migrations.rs` (Phase 27)

```sql
CREATE TABLE teams (
    id TEXT PRIMARY KEY,                    -- UUID
    name TEXT NOT NULL,
    keygen_license_key_hash TEXT NOT NULL,  -- SHA-256 of team license key
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE team_members (
    id TEXT PRIMARY KEY,                    -- UUID
    team_id TEXT NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',    -- 'admin' | 'member'
    invited_by TEXT,
    joined_at TEXT DEFAULT (datetime('now')),
    last_active_at TEXT,
    sharing_preferences TEXT,              -- JSON: {dna, decisions, chains, context}
    UNIQUE(team_id, email),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (invited_by) REFERENCES team_members(id)
);

CREATE INDEX idx_team_members_team ON team_members(team_id);
```

### 1.3 Team Settings Module
**Files:** New `src-tauri/src/team.rs`, extend `src-tauri/src/settings/mod.rs`

```rust
pub struct TeamConfig {
    pub team_id: Option<String>,
    pub display_name: Option<String>,
    pub seat_email: Option<String>,
    pub seat_role: Option<String>,
    pub sharing: TeamSharingPreferences,
}

pub struct TeamSharingPreferences {
    pub share_developer_dna: bool,        // Default: true
    pub share_decisions: bool,            // Default: true
    pub share_signal_chains: bool,        // Default: true
    pub share_context_summary: bool,      // Default: true (topics, not code)
    pub share_blind_spots: bool,          // Default: true
}
```

### 1.4 Joining Flow
**Commands:**
- `create_team(name, license_key)` — Admin creates team, becomes first member
- `generate_invite_code(team_id, email, role)` — Admin generates time-limited invite
- `join_team(invite_code)` — Member activates seat via invite
- `get_team_members(team_id)` — List all seats
- `remove_team_member(team_id, member_id)` — Admin removes seat (frees license)

**Invite codes:** `4DA-TEAM-{base64(team_id + email + expiry + hmac)}` — 72-hour expiry, single-use, cryptographically signed.

### 1.5 Frontend: Team Settings Panel
**Files:** New `src/components/settings/TeamSection.tsx`

- Team name + member count display
- Member list with roles (admin badge)
- Invite member form (email + role selector)
- Sharing preference toggles per member
- Leave team / remove member actions

**Tests:** Team creation flow, invite generation, join flow, role enforcement, seat limit errors.

---

## Phase 2: Shared Intelligence — The Core Value (Weeks 2-3)

### 2.1 Shared Resource Infrastructure
**Files:** `src-tauri/src/db/migrations.rs` (Phase 28)

```sql
CREATE TABLE shared_resources (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    resource_type TEXT NOT NULL,       -- 'dna' | 'decision' | 'signal_chain' | 'briefing' | 'context'
    resource_data TEXT NOT NULL,       -- JSON payload (sanitized, no raw content)
    shared_by TEXT NOT NULL,
    visibility TEXT DEFAULT 'team',    -- 'team' | 'specific_members'
    visible_to TEXT,                   -- JSON array of member_ids (if specific)
    created_at TEXT DEFAULT (datetime('now')),
    expires_at TEXT,                   -- Optional TTL
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (shared_by) REFERENCES team_members(id)
);

CREATE INDEX idx_shared_team_type ON shared_resources(team_id, resource_type);
CREATE INDEX idx_shared_expires ON shared_resources(expires_at) WHERE expires_at IS NOT NULL;
```

### 2.2 Team Developer DNA Dashboard
**The killer feature.** Every seat's Developer DNA aggregated into a team profile.

**New file:** `src-tauri/src/team_intelligence.rs`

```rust
pub struct TeamProfile {
    pub team_id: String,
    pub member_count: usize,
    pub collective_stack: Vec<TeamTechEntry>,    // Tech + who uses it + confidence
    pub stack_coverage: f32,                     // % of team's ecosystem covered
    pub blind_spots: Vec<TeamBlindSpot>,         // Topics NO ONE on the team tracks
    pub overlap_zones: Vec<OverlapZone>,         // Where 3+ members watch same thing
    pub unique_strengths: Vec<UniqueStrength>,   // Tech only 1 member knows (bus factor)
    pub generated_at: String,
}

pub struct TeamBlindSpot {
    pub topic: String,
    pub relevance_score: f32,          // How relevant to team's stack
    pub nearest_member: Option<String>, // Who's closest to covering this
    pub recommendation: String,         // "Consider assigning X to monitor this"
}

pub struct UniqueStrength {
    pub tech: String,
    pub sole_expert: String,            // Member display name
    pub bus_factor_risk: String,        // "high" | "medium" | "low"
    pub recommendation: String,
}
```

**Commands:**
- `get_team_profile(team_id)` → aggregated team intelligence
- `get_team_blind_spots(team_id)` → gaps no one covers
- `get_bus_factor_report(team_id)` → single-expert risks

**Frontend:** New `src/components/TeamDashboard.tsx`
- Visual tech stack heatmap (who knows what, at what depth)
- Blind spot alerts with assignment suggestions
- Bus factor warnings (red indicators for sole-expert tech)

### 2.3 Shared Signal Chains
**Files:** Extend `src-tauri/src/signal_chains.rs`

When multiple seats detect the same signal chain independently, that's **high-confidence intelligence**.

```rust
pub struct TeamSignalChain {
    pub chain: SignalChain,
    pub detected_by: Vec<MemberDetection>,   // Who detected, when
    pub team_confidence: f32,                // Higher when multiple seats confirm
    pub consensus_action: Option<String>,    // Team-agreed resolution
    pub resolution_history: Vec<ResolutionEntry>,
}

pub struct MemberDetection {
    pub member_name: String,
    pub detected_at: String,
    pub local_priority: String,
}
```

**Commands:**
- `get_team_signal_chains(team_id)` → merged chains across seats
- `resolve_team_signal(chain_id, action, notes)` → team consensus resolution
- `subscribe_team_pattern(pattern)` → watch for specific patterns across team

**Multi-seat confirmation logic:**
When seat A detects "CVE-2026-XXXX affects lodash" and seat B detects the same CVE within 24 hours, the team chain gets `team_confidence: 0.9` and triggers team-wide alert.

### 2.4 Shared Decision Tracking
**Files:** Extend `src-tauri/src/developer_decisions.rs`

Teams make technical decisions together. 4DA should track them.

```rust
pub struct TeamDecision {
    pub decision: DeveloperDecision,
    pub proposed_by: String,
    pub affected_members: Vec<String>,
    pub status: TeamDecisionStatus,    // Proposed | Accepted | Rejected | Superseded
    pub votes: Vec<DecisionVote>,      // Member + stance + rationale
    pub evidence: Vec<DecisionEvidence>, // 4DA signals that informed the decision
}
```

**Commands:**
- `propose_team_decision(decision)` → creates shared decision for team review
- `vote_on_decision(decision_id, stance, rationale)` → member weighs in
- `get_team_decisions(team_id, status_filter)` → decision log

### 2.5 Team Briefings
**Files:** Extend `src-tauri/src/digest_commands.rs`

Aggregate individual briefings into a team digest.

- Each member's briefing highlights get pooled
- Deduplicated and ranked by team-wide relevance
- "3 of 5 team members flagged this as critical" carries more weight
- Weekly team digest email with collective insights

**Command:** `generate_team_briefing(team_id)` → pulls from all members' recent signals

---

## Phase 3: Team Monitoring & Alerts (Week 4)

### 3.1 Team Signal Aggregation
**Files:** `src-tauri/src/db/migrations.rs` (Phase 29), new `src-tauri/src/team_monitoring.rs`

```sql
CREATE TABLE team_signals (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    signal_type TEXT NOT NULL,
    title TEXT NOT NULL,
    severity TEXT NOT NULL,
    detected_by_count INTEGER DEFAULT 1,
    first_detected TEXT,
    last_detected TEXT,
    source_items TEXT,                   -- JSON: item IDs that triggered
    resolved INTEGER DEFAULT 0,
    resolved_by TEXT,
    resolved_at TEXT,
    resolution_notes TEXT,
    FOREIGN KEY (team_id) REFERENCES teams(id)
);
```

**Alert policies (admin-configurable):**
- `min_seats_to_alert: 2` — only team-alert when 2+ seats detect same signal
- `aggregation_window_minutes: 60` — how long to wait for corroboration
- `notification_channels: ["in_app", "email", "webhook"]`

### 3.2 Team Monitoring Dashboard
**Frontend:** New `src/components/TeamMonitoring.tsx`

- Real-time team signal feed (who detected what, when)
- Severity heatmap across time
- "Quiet zones" — periods where no team member detected anything (concerning if expected activity)
- Resolution tracking — which signals were acted on, by whom

---

## Phase 4: Polish & Production Readiness (Week 5)

### 4.1 RBAC Enforcement
**Principle:** Admins manage, members participate.

| Action | Admin | Member |
|--------|-------|--------|
| Invite/remove members | Yes | No |
| Configure alert policies | Yes | No |
| View team dashboard | Yes | Yes |
| Share own DNA/decisions | Yes | Yes |
| Resolve team signals | Yes | Yes |
| Change team name | Yes | No |
| Generate team briefing | Yes | No |
| View shared resources | Yes | Yes |

Enforcement: `require_team_role(team_id, "admin")` guard on admin-only commands.

### 4.2 Privacy Controls
- Members control exactly what they share (per-resource toggles)
- Shared data is **metadata only** — topics, tech names, scores — never raw content or code
- Leaving a team deletes all shared resources from that member
- Admin cannot override member sharing preferences

### 4.3 Data Cleanup
- Expired shared resources auto-purged (weekly cleanup job)
- Removed members' shared data deleted within 24 hours
- Team deletion cascades to all shared resources + signals

### 4.4 Tests
- Team creation + invitation + joining (happy path + edge cases)
- Seat limit enforcement (attempt to exceed)
- RBAC enforcement (member attempts admin action)
- Signal aggregation (multi-seat detection → team alert)
- Blind spot detection accuracy
- Bus factor calculation
- Sharing preference enforcement
- Data cleanup on member removal
- Keygen seat validation

---

## Acceptance Criteria

Team tier ships when ALL of these are true:

1. Admin can create team, invite members, manage seats
2. Members can join via invite code, configure sharing preferences
3. Team Dashboard shows aggregated tech profile, blind spots, bus factor
4. Signal chains merge across seats with confidence scoring
5. Team briefings aggregate individual insights
6. Alert policies configurable by admin, enforced across team
7. RBAC enforced on all team commands
8. Privacy controls respected — no data shared without member consent
9. Keygen seat limits enforced — no over-provisioning
10. All team features gated behind `is_team_tier()` check
11. Full test coverage on all team commands and aggregation logic
12. Frontend components accessible (WCAG 2.1 AA)
