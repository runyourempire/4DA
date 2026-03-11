# 4DA Enterprise Tier — Implementation Plan

**Tier:** Enterprise (custom pricing)
**Philosophy:** Enterprise customers don't buy features — they buy trust. Trust that the tool meets compliance requirements, integrates into their existing toolchain, scales across divisions, and provides the visibility leadership needs without compromising the developer experience that made them want 4DA in the first place.

---

## What Makes Enterprise Worth Custom Pricing

Enterprise includes everything in Team, plus the three things large organizations actually require: **governance**, **integration**, and **scale**.

The pitch: "4DA Enterprise gives your engineering organization collective developer intelligence with the compliance, integration, and administrative controls your security team requires. Every seat gets smarter. Every team gets coordinated. Leadership gets visibility without micromanagement."

---

## Phase 1: Audit & Compliance Infrastructure (Week 1-2)

### 1.1 Comprehensive Audit Log
**Files:** `src-tauri/src/db/migrations.rs` (Phase 30), new `src-tauri/src/audit.rs`

The audit log table already exists conceptually — now it needs to be production-grade.

```sql
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id TEXT NOT NULL UNIQUE,         -- UUID for deduplication
    team_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,                -- team_member.id who performed action
    actor_email TEXT NOT NULL,             -- Denormalized for query efficiency
    action TEXT NOT NULL,                  -- Structured: 'resource.verb' format
    resource_type TEXT NOT NULL,           -- 'license' | 'member' | 'signal' | 'decision' | 'briefing' | 'settings' | 'export'
    resource_id TEXT,
    details TEXT,                          -- JSON: action-specific metadata
    ip_context TEXT,                       -- Local machine identifier (not IP — desktop app)
    created_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);

CREATE INDEX idx_audit_team_time ON audit_log(team_id, created_at DESC);
CREATE INDEX idx_audit_actor ON audit_log(actor_id, created_at DESC);
CREATE INDEX idx_audit_action ON audit_log(action);
CREATE INDEX idx_audit_resource ON audit_log(resource_type, resource_id);
```

**Action taxonomy (structured, not freeform):**

| Category | Actions |
|----------|---------|
| `license.*` | `license.activated`, `license.deactivated`, `license.validated`, `license.seat_claimed`, `license.seat_released` |
| `member.*` | `member.invited`, `member.joined`, `member.removed`, `member.role_changed`, `member.sharing_updated` |
| `signal.*` | `signal.detected`, `signal.resolved`, `signal.escalated`, `signal.snoozed` |
| `decision.*` | `decision.proposed`, `decision.voted`, `decision.accepted`, `decision.rejected` |
| `briefing.*` | `briefing.generated`, `briefing.shared`, `briefing.exported` |
| `export.*` | `export.dna_markdown`, `export.dna_svg`, `export.dna_png`, `export.dna_json`, `export.audit_csv` |
| `settings.*` | `settings.monitoring_changed`, `settings.alert_policy_changed`, `settings.sources_modified` |
| `admin.*` | `admin.team_created`, `admin.team_renamed`, `admin.webhook_registered`, `admin.retention_changed` |

### 1.2 Audit Write Paths
**Integrate `audit::log()` into every write command.**

```rust
// src-tauri/src/audit.rs
pub async fn log(
    team_id: &str,
    actor_id: &str,
    actor_email: &str,
    action: &str,
    resource_type: &str,
    resource_id: Option<&str>,
    details: Option<serde_json::Value>,
) -> Result<()> {
    // Write to audit_log table
    // Non-blocking — audit failure must never break the primary operation
    // Log warning if audit write fails, but don't propagate error
}
```

**Integration points (every pro/team command gets an audit call):**
- `activate_license` → `license.activated`
- `join_team` → `member.joined`
- `generate_ai_briefing` → `briefing.generated`
- `export_developer_dna_*` → `export.dna_*`
- `resolve_team_signal` → `signal.resolved`
- `propose_team_decision` → `decision.proposed`
- Every settings mutation → `settings.*`

### 1.3 Audit Log Viewer
**Frontend:** New `src/components/enterprise/AuditLogViewer.tsx`

- Filterable by: actor, action category, resource type, date range
- Searchable by keyword
- Paginated (50 per page)
- CSV export for compliance reporting
- Retention period display (configurable, see 1.5)

### 1.4 Audit Log Export
**Commands:**
- `export_audit_log(team_id, from, to, format)` → CSV or JSON
- `get_audit_summary(team_id, period)` → aggregated stats (actions/day, active users, top actions)

### 1.5 Data Retention Policies
**Files:** Extend `src-tauri/src/settings/mod.rs`, new migration

```sql
CREATE TABLE retention_policies (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    resource_type TEXT NOT NULL,      -- 'audit_log' | 'shared_resources' | 'signals' | 'briefings'
    retention_days INTEGER NOT NULL,  -- 30, 90, 365, -1 (forever)
    created_at TEXT,
    updated_at TEXT,
    UNIQUE(team_id, resource_type),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);
```

**Defaults:**
- Audit log: 365 days
- Shared resources: 90 days
- Team signals: 90 days
- Briefings: 180 days

**Enforcement:** Daily cleanup job checks retention policies, purges expired records.

**Commands:**
- `set_retention_policy(team_id, resource_type, days)` — Enterprise admin only
- `get_retention_policies(team_id)` → current policies

---

## Phase 2: Integration Layer — Webhooks & External Systems (Weeks 2-3)

### 2.1 Webhook Infrastructure
**Files:** `src-tauri/src/db/migrations.rs` (Phase 31), new `src-tauri/src/webhooks.rs`

```sql
CREATE TABLE webhooks (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    name TEXT NOT NULL,                    -- Human-readable label
    url TEXT NOT NULL,
    events TEXT NOT NULL,                  -- JSON array of event patterns
    secret TEXT NOT NULL,                  -- For HMAC-SHA256 signing
    active INTEGER DEFAULT 1,
    failure_count INTEGER DEFAULT 0,
    last_fired_at TEXT,
    last_status_code INTEGER,
    created_at TEXT,
    created_by TEXT,
    FOREIGN KEY (team_id) REFERENCES teams(id),
    FOREIGN KEY (created_by) REFERENCES team_members(id)
);

CREATE TABLE webhook_deliveries (
    id TEXT PRIMARY KEY,
    webhook_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL,                 -- JSON
    status TEXT DEFAULT 'pending',         -- 'pending' | 'delivered' | 'failed' | 'exhausted'
    http_status INTEGER,
    response_body TEXT,                    -- First 1KB of response (for debugging)
    attempt_count INTEGER DEFAULT 0,
    next_retry_at TEXT,
    created_at TEXT,
    delivered_at TEXT,
    FOREIGN KEY (webhook_id) REFERENCES webhooks(id)
);

CREATE INDEX idx_deliveries_pending ON webhook_deliveries(status, next_retry_at)
    WHERE status IN ('pending', 'failed');
```

### 2.2 Webhook Dispatch Engine
**File:** `src-tauri/src/webhooks.rs`

```rust
pub struct WebhookPayload {
    pub event: String,              // "signal.detected"
    pub timestamp: String,          // ISO 8601
    pub team_id: String,
    pub data: serde_json::Value,    // Event-specific payload
}

// Delivery with HMAC signing
pub async fn dispatch_webhook(webhook: &Webhook, payload: &WebhookPayload) -> Result<()> {
    let body = serde_json::to_string(payload)?;
    let signature = hmac_sha256(&webhook.secret, body.as_bytes());

    let response = reqwest::Client::new()
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("X-4DA-Signature", format!("sha256={}", hex::encode(signature)))
        .header("X-4DA-Event", &payload.event)
        .header("X-4DA-Delivery", uuid::Uuid::new_v4().to_string())
        .body(body)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    // Record delivery result
    // On failure: schedule retry with exponential backoff
    // Max 5 retries: 1min, 5min, 30min, 2hr, 12hr
    // After 5 failures: mark 'exhausted', disable webhook, notify admin
}
```

### 2.3 Webhook Retry & Circuit Breaker

**Retry schedule:** 1min → 5min → 30min → 2hr → 12hr → exhausted

**Circuit breaker:** If a webhook accumulates 10 consecutive failures (across any events), auto-disable and notify admin via in-app notification + email.

**Background job:** Runs every 60 seconds, processes pending/failed deliveries with `next_retry_at <= now()`.

### 2.4 Webhook Event Catalog

| Event | Trigger | Payload |
|-------|---------|---------|
| `signal.detected` | New high-severity signal detected by any seat | `{signal_id, severity, title, detected_by, source}` |
| `signal.corroborated` | 2+ seats detect same signal | `{signal_id, seat_count, confidence, title}` |
| `signal.resolved` | Team resolves a signal | `{signal_id, resolved_by, resolution_notes}` |
| `chain.detected` | New signal chain formed | `{chain_id, priority, link_count, suggested_action}` |
| `decision.proposed` | New team decision proposed | `{decision_id, subject, proposed_by}` |
| `decision.accepted` | Team accepts a decision | `{decision_id, subject, vote_summary}` |
| `briefing.generated` | Team briefing ready | `{briefing_id, item_count, critical_count}` |
| `member.joined` | New team member activated | `{member_email, role, invited_by}` |
| `member.removed` | Team member removed | `{member_email, removed_by}` |
| `alert.critical` | Critical severity signal affecting multiple seats | `{signal_id, affected_seats, recommended_action}` |
| `dna.updated` | Team member's Developer DNA changed significantly | `{member_name, changes_summary}` |
| `blind_spot.detected` | New team blind spot identified | `{topic, relevance, recommendation}` |

### 2.5 Webhook Management UI
**Frontend:** New `src/components/enterprise/WebhookManager.tsx`

- Create/edit/delete webhooks
- Event selector (checkboxes for each event type)
- Secret display (show once on creation, then masked)
- Delivery history with status indicators
- Test button (sends test payload to verify endpoint)
- Circuit breaker status + manual re-enable

### 2.6 Integration Templates
Pre-configured webhook setups for common tools:

**Slack integration:**
- Events: `signal.corroborated`, `alert.critical`, `briefing.generated`
- URL format: Slack incoming webhook URL
- Payload adapter: transforms 4DA payload → Slack Block Kit format

**Microsoft Teams:**
- Same events, Teams Incoming Webhook format (Adaptive Cards)

**PagerDuty:**
- Events: `alert.critical`
- PagerDuty Events API v2 format

**Custom (raw JSON):**
- Any event selection
- Raw 4DA payload format
- HMAC-SHA256 signature verification docs

**Commands:**
- `register_webhook(name, url, events, secret)` → creates webhook
- `list_webhooks(team_id)` → all webhooks with status
- `test_webhook(webhook_id)` → sends test event
- `delete_webhook(webhook_id)` → removes + cleans delivery history
- `get_webhook_deliveries(webhook_id, status_filter)` → delivery log

---

## Phase 3: Multi-Team & Organizational Scale (Weeks 3-4)

### 3.1 Organization Entity
**Files:** `src-tauri/src/db/migrations.rs` (Phase 32), new `src-tauri/src/organization.rs`

Enterprise customers have multiple teams. Divisions, squads, departments.

```sql
CREATE TABLE organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    keygen_license_key_hash TEXT NOT NULL,
    settings TEXT,                          -- JSON: org-level defaults
    created_at TEXT,
    updated_at TEXT
);

CREATE TABLE org_teams (
    org_id TEXT NOT NULL,
    team_id TEXT NOT NULL,
    PRIMARY KEY (org_id, team_id),
    FOREIGN KEY (org_id) REFERENCES organizations(id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
);

CREATE TABLE org_admins (
    org_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'org_admin',  -- 'org_admin' | 'org_owner'
    PRIMARY KEY (org_id, member_id),
    FOREIGN KEY (org_id) REFERENCES organizations(id),
    FOREIGN KEY (member_id) REFERENCES team_members(id)
);
```

### 3.2 Org-Level Dashboard
**Frontend:** New `src/components/enterprise/OrgDashboard.tsx`

Org admins see across all teams:
- **Team health overview** — which teams are active, which are quiet
- **Cross-team signal correlation** — "Backend team and Frontend team both detected the same React vulnerability"
- **Organization-wide blind spots** — gaps that no team in the org covers
- **Aggregate tech stack** — what the entire org depends on, weighted by team size
- **License utilization** — seats used vs available, per team

### 3.3 Cross-Team Signal Correlation
**File:** New `src-tauri/src/org_intelligence.rs`

The highest-value Enterprise feature. When separate teams independently detect related signals, that's organizational intelligence.

```rust
pub struct CrossTeamCorrelation {
    pub correlation_id: String,
    pub signal_type: String,
    pub teams_affected: Vec<TeamSignalSummary>,
    pub org_severity: String,              // Escalated if multi-team
    pub first_detected: String,
    pub recommended_response: String,      // "Coordinate across Backend and Platform teams"
}
```

**Detection logic:**
1. Each team's signal chains are already tracked
2. Cross-team correlator runs hourly (or on critical signal)
3. Matches signals by: topic overlap > 0.7, temporal proximity < 48 hours
4. If 2+ teams match → `CrossTeamCorrelation` created
5. Webhook: `org.cross_team_signal` fired
6. Org admin notified

### 3.4 Org-Level Policies
Org admins can set defaults that cascade to all teams:

```rust
pub struct OrgPolicies {
    pub default_retention_days: HashMap<String, i32>,
    pub required_sharing: Vec<String>,        // Resources that MUST be shared within team
    pub webhook_allowlist: Vec<String>,        // Allowed webhook domains
    pub min_monitoring_interval: u64,          // Org-wide floor
    pub require_decision_tracking: bool,       // Teams must log decisions
}
```

**Commands:**
- `set_org_policy(org_id, policy)` → org owner only
- `get_org_policies(org_id)` → current policies
- `get_org_compliance_report(org_id)` → which teams comply with policies

---

## Phase 4: Enterprise Administration (Week 5)

### 4.1 Admin Console
**Frontend:** New `src/components/enterprise/AdminConsole.tsx`

Single pane of glass for org admins:

**Sections:**
1. **License Management** — Seat allocation across teams, utilization metrics
2. **Team Management** — Create/archive teams, assign org admins
3. **Policy Configuration** — Retention, sharing requirements, webhook allowlist
4. **Audit Log** — Org-wide audit viewer with cross-team filtering
5. **Integration Status** — Webhook health across all teams
6. **Compliance Dashboard** — Policy adherence per team

### 4.2 Usage Analytics
**File:** New `src-tauri/src/enterprise_analytics.rs`

```rust
pub struct OrgAnalytics {
    pub period: String,                        // "last_30_days"
    pub active_seats: usize,
    pub total_seats: usize,
    pub signals_detected: usize,
    pub signals_resolved: usize,
    pub avg_resolution_time_hours: f32,
    pub decisions_tracked: usize,
    pub briefings_generated: usize,
    pub top_signal_categories: Vec<(String, usize)>,
    pub team_activity_breakdown: Vec<TeamActivity>,
}

pub struct TeamActivity {
    pub team_name: String,
    pub active_members: usize,
    pub signals_this_period: usize,
    pub decisions_this_period: usize,
    pub engagement_score: f32,                 // 0-1, based on daily active usage
}
```

**Commands:**
- `get_org_analytics(org_id, period)` → usage summary
- `export_org_analytics(org_id, period, format)` → CSV/JSON for leadership reporting

### 4.3 Role Hierarchy

| Role | Scope | Can Do |
|------|-------|--------|
| **Org Owner** | Organization | Everything. Transfer ownership. Delete org. |
| **Org Admin** | Organization | Manage teams, set policies, view all audit logs, manage licenses |
| **Team Admin** | Single team | Invite/remove members, configure team alerts, resolve signals |
| **Team Member** | Single team | Use all features, share resources, vote on decisions |

**Enforcement:** `require_org_role(org_id, "org_admin")` guard on org-level commands.

### 4.4 Enterprise Onboarding Flow
**Frontend:** New `src/components/enterprise/EnterpriseSetup.tsx`

Guided setup for new Enterprise customers:
1. Enter Enterprise license key → validates against Keygen
2. Create organization → name, org admin email
3. Create first team → name, import members (CSV or manual)
4. Configure policies → retention, sharing, monitoring defaults
5. Set up integrations → webhook for Slack/Teams (optional)
6. Review & activate → summary of configuration

---

## Phase 5: Security Hardening & Production Polish (Week 6)

### 5.1 License Enforcement
- Enterprise license validates against Keygen with `policy: "enterprise"`
- Seat count enforced at org level (distributed across teams)
- License expiry checked daily — 30-day grace period with warnings
- Expired license → read-only mode (can view, can't generate/share/export)

### 5.2 Data Isolation
- Each team's data is team_id-scoped — no cross-team leakage
- Org-level queries only aggregate metadata, never raw content
- Shared resources between teams require explicit org admin approval
- Database queries always include team_id in WHERE clause (enforced by helper functions)

### 5.3 Export Controls
- All exports (DNA, audit, analytics) logged in audit trail
- Org admin can disable specific export types via policy
- Exported files include metadata: `exported_by`, `exported_at`, `team_id`

### 5.4 Graceful Degradation
- If Keygen is unreachable, cached license honored for 7 days (Enterprise gets longer grace)
- If webhook endpoint is down, events queue locally (up to 1000 per webhook)
- If a team member's machine is offline, they work normally — sync on reconnect

### 5.5 Tests
- Audit log completeness (every write command produces audit entry)
- Audit log export (CSV format validation, date filtering)
- Retention policy enforcement (records purged after expiry)
- Webhook HMAC signature verification
- Webhook retry + circuit breaker behavior
- Cross-team signal correlation accuracy
- Org policy cascade (set at org → applied to team)
- Role hierarchy enforcement at every level
- Data isolation (team A cannot see team B's resources)
- License grace period behavior
- Enterprise onboarding flow (end-to-end)

---

## Acceptance Criteria

Enterprise tier ships when ALL of these are true:

1. **Audit:** Every write operation produces an audit log entry with structured action taxonomy
2. **Audit Viewer:** Filterable, searchable, exportable (CSV + JSON) audit log UI
3. **Retention:** Configurable per-resource-type retention policies, enforced by daily cleanup
4. **Webhooks:** Create, test, monitor webhooks with HMAC signing and exponential backoff retry
5. **Webhook Templates:** One-click setup for Slack, Teams, PagerDuty
6. **Multi-Team:** Organizations contain multiple teams with independent data isolation
7. **Cross-Team Intelligence:** Automatic correlation of signals detected by separate teams
8. **Org Dashboard:** Single-pane view of all teams, signals, compliance, utilization
9. **Admin Console:** License management, team management, policy configuration
10. **Usage Analytics:** Org-wide metrics exportable for leadership reporting
11. **Role Hierarchy:** Four-level RBAC (org owner → org admin → team admin → member) enforced on all commands
12. **Enterprise Onboarding:** Guided setup flow for new customers
13. **Data Isolation:** Zero cross-team data leakage, verified by test suite
14. **Graceful Degradation:** Offline-tolerant with extended license grace period
15. **Full test coverage** on all Enterprise commands, RBAC, and integration points
16. **All Team tier acceptance criteria also met** (Enterprise includes Team)
