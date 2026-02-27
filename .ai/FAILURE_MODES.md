# Failure Modes
## Known Bugs, Fragile Areas, and Previous Regressions

**Version:** 1.0.0
**Source:** ARCHITECTURE.md Risk Register (127 items audited)
**Purpose:** Claude should read this before touching risky code

---

## Critical Failures (Must Never Happen)

### FM-CRIT-001: API Keys Exposed
- **What:** API keys appear in logs, errors, or transmitted to unauthorized endpoints
- **Impact:** Critical - Security breach, credential theft
- **Detection:** Log audit, network traffic analysis
- **Code Areas:** `settings.rs`, `llm.rs`, any serialization code
- **Mitigation:** OS keychain storage, redaction in all log/error paths
- **If This Happens:** Immediate rotation of affected keys, incident response

### FM-CRIT-002: Data Exfiltration
- **What:** User data sent to unauthorized remote services
- **Impact:** Critical - Privacy violation, trust destruction
- **Detection:** Network audit, code review of all HTTP calls
- **Code Areas:** Any code with network access
- **Mitigation:** Strict allow-list for network destinations
- **If This Happens:** Full audit, user notification, potential legal exposure

### FM-CRIT-003: Update Mechanism Compromised
- **What:** Malicious code delivered via auto-update
- **Impact:** Critical - Complete system compromise
- **Detection:** Signature verification
- **Code Areas:** Update/installer code
- **Mitigation:** Code signing, HTTPS only, certificate pinning
- **If This Happens:** Emergency rollback, security disclosure

---

## High-Impact Failures

### FM-HIGH-001: Index Corruption on Crash
- **Risk ID:** 1.11
- **What:** SQLite database corruption if app crashes during write
- **Impact:** High - Data loss, requires re-index
- **Code Areas:** `db.rs`, any database write paths
- **Mitigation:** WAL mode, atomic transactions
- **Verification:** Kill -9 during write test
- **Fragile Pattern:**
```rust
// DANGEROUS: Non-atomic multi-step write
db.execute("INSERT INTO table1 ...")?;
// Crash here = inconsistent state
db.execute("INSERT INTO table2 ...")?;

// SAFE: Transaction-wrapped
let tx = db.transaction()?;
tx.execute("INSERT INTO table1 ...")?;
tx.execute("INSERT INTO table2 ...")?;
tx.commit()?;
```

### FM-HIGH-002: File Watcher Handle Exhaustion
- **Risk ID:** 1.5
- **What:** System runs out of file handles watching too many directories
- **Impact:** High - App becomes unresponsive, OS-level issues
- **Code Areas:** `watcher.rs`, file system monitoring
- **Mitigation:** Debounce (500ms), limit watched directories
- **Verification:** Stress test with 100k files

### FM-HIGH-003: Large File Memory Exhaustion
- **Risk ID:** 1.6
- **What:** Loading a large file into memory causes OOM
- **Impact:** High - App crash
- **Code Areas:** File reading, indexing
- **Mitigation:** Stream processing, 100MB size limit
- **Verification:** Test with 1GB file
- **Fragile Pattern:**
```rust
// DANGEROUS: Loads entire file into memory
let content = fs::read_to_string(path)?;

// SAFE: Streaming with size check
let metadata = fs::metadata(path)?;
if metadata.len() > 100_000_000 {
    return Err(Error::FileTooLarge);
}
```

### FM-HIGH-004: Symlink Infinite Loops
- **Risk ID:** 1.3
- **What:** Circular symlinks cause infinite loop during directory traversal
- **Impact:** High - Hang, resource exhaustion
- **Code Areas:** `scanner.rs`, directory walking
- **Mitigation:** Track visited inodes, max depth limit
- **Verification:** Test with circular symlinks

### FM-HIGH-005: LLM Cost Explosion
- **Risk ID:** 4.1, 3.12
- **What:** Uncontrolled API calls exceed budget
- **Impact:** High - Financial loss
- **Code Areas:** `llm.rs`, any API call code
- **Mitigation:** Hard daily limit, cost alerts, circuit breaker
- **Verification:** Test limit enforcement

### FM-HIGH-006: Sensitive Window Titles Logged
- **Risk ID:** 2.1
- **What:** Password dialogs, banking sites captured in activity log
- **Impact:** High - Privacy violation
- **Code Areas:** Activity tracker (if enabled)
- **Mitigation:** Regex filter for passwords, banking keywords
- **Blocked Keywords:** password, banking, paypal, login, 1password, credit, ssn

### FM-HIGH-007: Notification Fatigue
- **Risk ID:** 5.4
- **What:** Too many notifications annoy user, leading to disable/uninstall
- **Impact:** High - Product failure
- **Code Areas:** Notification delivery
- **Mitigation:** Smart batching, quiet hours, relevance threshold
- **Verification:** User testing

---

## Medium-Impact Failures

### FM-MED-001: Binary Files Crash Parser
- **Risk ID:** 1.1
- **What:** Attempting to parse binary file as text causes panic
- **Impact:** Medium - Task failure
- **Code Areas:** Content extraction
- **Mitigation:** Magic byte detection, skip non-text
- **Verification:** Test with 50+ file types

### FM-MED-002: Encoding Detection Fails
- **Risk ID:** 1.2
- **What:** Non-UTF8 files misread or fail
- **Impact:** Low - Incorrect content
- **Code Areas:** File reading
- **Mitigation:** chardet library, UTF-8 fallback
- **Verification:** Test CJK, Cyrillic, Arabic files

### FM-MED-003: API Rate Limit Exceeded
- **Risk ID:** 3.1
- **What:** Source API blocks requests due to rate limiting
- **Impact:** Medium - Temporary loss of source
- **Code Areas:** Source adapters (HN, arXiv, Reddit)
- **Mitigation:** Token bucket, exponential backoff
- **Fragile Pattern:**
```rust
// DANGEROUS: No rate limiting
for item in items {
    api.fetch(item)?;
}

// SAFE: With rate limiting
for item in items {
    rate_limiter.acquire().await;
    api.fetch(item)?;
}
```

### FM-MED-004: Cold Start Poor Relevance
- **Risk ID:** 4.5
- **What:** New user gets irrelevant results before learning kicks in
- **Impact:** Medium - Poor first impression
- **Code Areas:** Interest model, onboarding
- **Mitigation:** Onboarding topic selection, manual seeds

### FM-MED-005: LLM Hallucination in Scoring
- **Risk ID:** 4.2
- **What:** LLM gives confident but wrong relevance scores
- **Impact:** Medium - Incorrect filtering
- **Code Areas:** `llm.rs`, relevance scoring
- **Mitigation:** Binary scoring, confidence thresholds, spot-checks

### FM-MED-006: Content Extraction Fails
- **Risk ID:** 3.3
- **What:** Unable to extract text from URL/document
- **Impact:** Medium - Content not indexed
- **Code Areas:** Content extractors
- **Mitigation:** Multiple extractors, graceful fallback

### FM-MED-007: Cloud Sync Conflicts
- **Risk ID:** 1.7
- **What:** Dropbox/OneDrive conflicts during file access
- **Impact:** Medium - Read failures
- **Code Areas:** File reading
- **Mitigation:** Lock files, retry logic

---

## Low-Impact Failures (Logged for Completeness)

| ID | Risk | Area | Mitigation |
|----|------|------|------------|
| FM-LOW-001 | Permission denied | File access | Graceful skip |
| FM-LOW-002 | Temp files indexed | Indexer | Pattern exclusion |
| FM-LOW-003 | Duplicate content | Scanner | Hash deduplication |
| FM-LOW-004 | Timezone errors | Digest delivery | Store UTC |
| FM-LOW-005 | Hidden files ignored | Indexer | Configurable |
| FM-LOW-006 | Unicode normalization | Indexer | NFC normalization |
| FM-LOW-007 | gitignore edge cases | Scanner | Use ignore crate |

---

## Known Fragile Areas

### Area: File System Operations (`scanner.rs`, `watcher.rs`)
- Highly OS-dependent behavior
- Edge cases with symlinks, permissions, path lengths
- Always test on Windows, macOS, Linux

### Area: External API Integration (`sources/*.rs`)
- APIs change without notice
- Rate limits vary by account tier
- Network conditions unpredictable

### Area: LLM Integration (`llm.rs`)
- Model output format may change
- Token limits vary by model
- Cost can escalate unexpectedly

### Area: SQLite Vector Search (`db.rs`)
- sqlite-vss extension compatibility
- Performance degrades with large datasets
- Index rebuilding can be slow

---

## Regression History

*Document any bugs that were fixed but could recur*

| Date | ID | Bug | Root Cause | Fix | Test Added |
|------|----|-----|------------|-----|------------|
| 2026-02 | FM-LOW-008 | CI minutes exhausted during development sprint | Excessive parallel CI runs without caching, plus large test suite | Added cargo test caching, reduced CI trigger frequency | Monitoring CI usage dashboard |
| 2026-02 | FM-MED-008 | `cargo test` OOM on CI runners | SQLite in-memory test databases accumulating across 800+ tests in single process | Split test execution with `--test-threads=1` for DB-heavy tests, added cleanup | OOM watchdog in CI config |

---

## How to Use This File

1. **Before modifying risky code:** Check if the area has known failure modes
2. **When debugging:** Look for similar past failures
3. **After fixing bugs:** Add to regression history
4. **During code review:** Verify mitigations are preserved
5. **After incidents:** Update this file with new failure patterns

---

## Adding New Failure Modes

When documenting a new failure mode:

```markdown
### FM-[SEVERITY]-NNN: [Short Title]
- **Risk ID:** (from risk register if applicable)
- **What:** One-sentence description of what fails
- **Impact:** [Critical|High|Medium|Low] - Why it matters
- **Code Areas:** Files/modules affected
- **Mitigation:** How to prevent/recover
- **Verification:** How to test the mitigation
- **Fragile Pattern:** (optional) Code example of wrong vs right way
```

---

*This file is living documentation. Update it when new failures are discovered.*
