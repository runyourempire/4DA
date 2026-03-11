# Security Hardening & Key Management Plan

**Origin:** Cybersecurity audit (2026-03-12) — 2 CRITICAL, 6 HIGH, 14 MEDIUM findings
**Scope:** Encrypted key storage, env var auto-detection, key validation, audit remediation
**Approach:** 4 phases, ordered by impact. Each phase is independently shippable.

---

## Phase 1: Encrypted Key Storage (HIGH impact, MEDIUM effort)

**Problem:** API keys stored in plaintext `settings.json`. Anyone with filesystem access reads everything.
**Solution:** Platform-native credential storage via the `keyring` crate.

### 1.1 Add `keyring` crate dependency
- **File:** `src-tauri/Cargo.toml`
- Add `keyring = "3"` to `[dependencies]`
- The `keyring` crate wraps: Windows Credential Manager, macOS Keychain, Linux Secret Service (freedesktop)
- Service name: `"com.4da.app"`, username per key: `"llm_api_key"`, `"openai_api_key"`, `"x_api_key"`, `"license_key"`

### 1.2 Create `src-tauri/src/settings/keystore.rs` module
- `store_secret(key_name: &str, value: &str) -> Result<()>` — stores in platform keychain
- `get_secret(key_name: &str) -> Result<Option<String>>` — retrieves from keychain
- `delete_secret(key_name: &str) -> Result<()>` — removes from keychain
- `has_secret(key_name: &str) -> bool` — check existence without loading value
- `migrate_from_plaintext(settings: &Settings) -> Result<MigrationReport>` — one-time migration
- Fallback: If keychain unavailable (headless Linux, WSL), fall back to existing plaintext with a logged warning. Never crash if keychain fails.

### 1.3 Modify `settings/mod.rs` — remove keys from JSON
- On `save()`: store keys in keychain via `keystore::store_secret()`, write `settings.json` with `api_key: ""` (empty)
- On `load()`: read `settings.json`, then hydrate keys from keychain via `keystore::get_secret()`
- Add `has_api_key: bool` fields to the serialized JSON (so the frontend knows a key exists without loading it)
- First load after upgrade: detect non-empty keys in JSON, call `migrate_from_plaintext()`, clear them from JSON

### 1.4 Update `set_llm_provider` Tauri command
- **File:** `src-tauri/src/settings_commands.rs`
- When receiving a new API key from frontend, store directly in keychain
- Never write the key to `settings.json`
- Return `has_api_key: true` to frontend

### 1.5 Update `get_settings` Tauri command
- Continue returning `has_api_key: bool` (already does this)
- Never return the actual key value over IPC

### 1.6 Update `mcp-4da-server/src/llm.ts`
- Read keys from a new Tauri command `get_llm_key_for_mcp()` instead of reading `settings.json` directly
- Or: accept keys via environment variable set by the Tauri app when launching the MCP server

### 1.7 Windows-specific: Set ACL on settings.json
- **File:** `src-tauri/src/settings/mod.rs` line 772-777
- The `#[cfg(unix)]` chmod block has no Windows equivalent
- Use `windows-acl` crate or raw Win32 API to set owner-only ACL
- Or accept that keychain storage makes this less critical (keys no longer in the file)

### 1.8 Tests
- Unit test: store/retrieve/delete/has_secret round-trip
- Unit test: migration from plaintext to keychain
- Unit test: fallback when keychain unavailable
- Integration test: save settings, reload, verify key accessible but not in JSON
- Privacy test: verify `settings.json` on disk contains no API key strings after save

### Verification gate
- [ ] `cargo test --lib` passes
- [ ] `pnpm run test` passes
- [ ] `settings.json` contains no API key values after save
- [ ] Keys retrievable after app restart
- [ ] Fallback works when keychain unavailable (test with mock)

---

## Phase 2: Environment Variable Auto-Detection (HIGH impact, LOW effort)

**Problem:** Developers who already have `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` set must manually copy-paste them.
**Solution:** Detect env vars on startup and during onboarding, offer one-click import.

### 2.1 Create `src-tauri/src/settings/env_detection.rs`
- `detect_api_keys() -> DetectedKeys` struct:
  ```
  struct DetectedKeys {
      anthropic_key: Option<String>,    // from ANTHROPIC_API_KEY
      openai_key: Option<String>,       // from OPENAI_API_KEY
      ollama_running: bool,             // from localhost:11434 health check
      ollama_url: Option<String>,       // from OLLAMA_HOST env var
  }
  ```
- Check env vars: `ANTHROPIC_API_KEY`, `CLAUDE_API_KEY`, `OPENAI_API_KEY`, `OPENAI_KEY`
- Check `OLLAMA_HOST` for custom Ollama URL
- Return detected info (keys masked to first 8 + last 4 chars for display)

### 2.2 Add Tauri command `detect_environment`
- **File:** `src-tauri/src/settings_commands.rs`
- Returns `DetectedKeysResponse` with:
  - `has_anthropic_env: bool`
  - `anthropic_env_preview: String` (e.g., "sk-ant-ap...3f2d")
  - `has_openai_env: bool`
  - `openai_env_preview: String`
  - `ollama_running: bool`
  - `ollama_url: Option<String>`
- NEVER returns the full key over IPC — only the preview mask

### 2.3 Add Tauri command `import_env_key`
- Takes `provider: String` ("anthropic" | "openai")
- Reads the full key from env var server-side
- Stores it directly in keychain (Phase 1) or settings
- Returns success/failure
- The key never crosses the IPC boundary

### 2.4 Update onboarding UI — `setup-ai-provider.tsx`
- On mount, call `detect_environment()`
- If env keys detected, show a banner: "We detected your Anthropic API key (sk-ant-ap...3f2d). Use it?"
- One-click "Use This Key" button calls `import_env_key("anthropic")`
- If Ollama detected with models, auto-select Ollama (already exists, enhance)
- Priority display: Ollama (free, local) > env var keys > manual entry

### 2.5 Update settings UI — `AIProviderSection.tsx`
- Add "Import from environment" button next to the API key input
- Only shown when `detect_environment()` reports available keys
- Calls `import_env_key()` on click

### 2.6 Tests
- Rust unit test: `detect_api_keys()` with mocked env vars
- Rust unit test: `import_env_key()` stores correctly
- Frontend test: onboarding shows env detection banner when keys available
- Frontend test: settings shows import button when keys available

### Verification gate
- [ ] Env var detection works on Windows, macOS, Linux
- [ ] Keys never cross IPC (only preview + import action)
- [ ] Onboarding auto-detects and offers import
- [ ] Settings panel shows import option
- [ ] Works when no env vars set (graceful degradation)

---

## Phase 3: Real-Time Key Validation (MEDIUM impact, LOW effort)

**Problem:** Users paste invalid keys and don't know until they try to use a feature. Current validation is "length > 20 chars."
**Solution:** Validate key format on input, test connection before saving.

### 3.1 Add key format validation — `src-tauri/src/settings/validation.rs`
- `validate_key_format(provider: &str, key: &str) -> ValidationResult`
- Anthropic: must start with `sk-ant-` and be 90-120 chars
- OpenAI: must start with `sk-` and be 40-60 chars (or `sk-proj-` for project keys)
- Returns: `Valid`, `InvalidFormat(reason)`, `PossiblyValid` (unknown format but non-empty)

### 3.2 Add Tauri command `validate_api_key`
- **File:** `src-tauri/src/settings_commands.rs`
- Takes `provider: String`, `key: String`
- Step 1: Format validation (instant, no network)
- Step 2: Lightweight API call to verify the key works:
  - Anthropic: `POST /v1/messages` with `max_tokens: 1`, model: cheapest available
  - OpenAI: `GET /v1/models` (list models, minimal cost)
- Returns: `{ valid: bool, format_ok: bool, connection_ok: bool, error: Option<String>, model_access: Vec<String> }`
- Timeout: 10 seconds
- The key is consumed server-side and stored if valid — never returned to frontend

### 3.3 Update settings UI — real-time feedback
- **File:** `src/components/settings/AIProviderSection.tsx`
- Debounce key input (500ms)
- On each debounced change, call `validate_api_key()`
- Show inline status:
  - Typing: neutral (no indicator)
  - Format invalid: red "Invalid key format — Anthropic keys start with sk-ant-"
  - Format OK, testing: spinner "Verifying..."
  - Connection OK: green checkmark "Key verified — access to claude-sonnet-4-20250514, claude-opus-4-20250514"
  - Connection failed: amber "Key format is correct but connection failed: {error}"

### 3.4 Update onboarding — same validation
- Apply the same real-time validation in `setup-ai-provider.tsx`
- Green checkmark = "Continue" button enabled
- No checkmark = "Continue" still enabled but with warning

### 3.5 Tests
- Unit test: format validation for known key patterns
- Unit test: validation returns correct status for empty/short/malformed keys
- Frontend test: debounced validation triggers on input
- Frontend test: correct status indicators shown

### Verification gate
- [ ] Instant format feedback (no network delay)
- [ ] Connection test completes in < 5 seconds
- [ ] Anthropic, OpenAI, and Ollama all validated
- [ ] Invalid keys show clear, helpful error messages
- [ ] Valid keys show available models

---

## Phase 4: Remaining Audit Remediation (MEDIUM impact, LOW effort)

Cleanup items from the security audit that weren't addressed in the initial fix pass.

### 4.1 CSP: Tighten `img-src`
- **File:** `src-tauri/tauri.conf.json`
- Change `img-src 'self' data: https:` to `img-src 'self' data:` (block external images)
- If external images are needed (content thumbnails), enumerate specific CDN domains
- Verify no components break with the tighter policy

### 4.2 Webhook signing upgrade
- **File:** `src-tauri/src/webhooks.rs`
- Replace `SHA256(secret + "." + body)` with proper `HMAC-SHA256` using the `hmac` crate
- The `hmac` crate is likely already transitively available
- Update signature header format to `sha256=<hex>`
- Add `X-4DA-Signature-256` header (match GitHub webhook convention)

### 4.3 MCP agent_memory size limits
- **File:** `mcp-4da-server/src/tools/agent-memory.ts`
- Add content size limit: max 10KB per entry
- Add storage quota: max 1000 entries per agent
- Return error if limits exceeded

### 4.4 Certificate pinning for critical endpoints
- **File:** `src-tauri/src/http_client.rs`
- Pin certificates for `api.anthropic.com`, `api.openai.com`, `api.keygen.sh`
- Use reqwest's `add_root_certificate()` with the known CA certificates
- Fallback: if pinned cert fails, warn but allow (don't break for cert rotations)

### 4.5 Privacy claim documentation
- Update any user-facing text that says "data never leaves your machine"
- Qualified version: "Your data stays on your machine. When you configure a cloud AI provider, content analysis queries are sent to that provider using your API key."
- Apply in: onboarding, settings, about page, website

### Verification gate
- [ ] `pnpm run validate:all` passes
- [ ] `cargo test --lib` passes
- [ ] Webhook signature matches HMAC-SHA256 standard
- [ ] MCP memory limits enforced
- [ ] No external images loaded in CSP (or only from enumerated domains)

---

## Execution Order & Dependencies

```
Phase 1 (Encrypted Storage)          Phase 2 (Env Detection)
        |                                    |
        +---- 1.1-1.3: keystore module ------+
        |     1.4-1.5: IPC updates           | 2.1-2.3: detection + import
        |     1.6: MCP server update          | 2.4-2.5: UI integration
        |     1.7-1.8: ACL + tests           | 2.6: tests
        |                                    |
        +--------------+---------+-----------+
                       |
              Phase 3 (Key Validation)
                       |
              3.1-3.2: validation module
              3.3-3.4: UI real-time feedback
              3.5: tests
                       |
              Phase 4 (Audit Cleanup)
                       |
              4.1-4.5: independent items
```

- **Phase 1 and 2 can run in parallel** — env detection can store keys in plaintext initially, then keychain when Phase 1 lands
- **Phase 3 depends on Phase 1** — validation should store keys in keychain, not plaintext
- **Phase 4 is independent** — can run anytime

## File Change Summary

| Phase | New Files | Modified Files |
|-------|-----------|----------------|
| 1 | `settings/keystore.rs` | `Cargo.toml`, `settings/mod.rs`, `settings_commands.rs`, `lib.rs`, `mcp-4da-server/src/llm.ts`, privacy tests |
| 2 | `settings/env_detection.rs` | `settings_commands.rs`, `lib.rs`, `commands.ts`, `setup-ai-provider.tsx`, `AIProviderSection.tsx` |
| 3 | `settings/validation.rs` | `settings_commands.rs`, `lib.rs`, `commands.ts`, `AIProviderSection.tsx`, `setup-ai-provider.tsx` |
| 4 | None | `tauri.conf.json`, `webhooks.rs`, `agent-memory.ts`, `http_client.rs` |

## Test Impact

| Phase | New Tests | Risk |
|-------|-----------|------|
| 1 | ~8 Rust + ~4 frontend | Medium — touches core settings load/save |
| 2 | ~4 Rust + ~4 frontend | Low — additive feature, no existing code changes |
| 3 | ~6 Rust + ~4 frontend | Low — validation is pure functions + one API call |
| 4 | ~4 Rust | Low — isolated fixes |

## Success Criteria

After all 4 phases:
- Zero API keys in any file on disk (all in platform keychain)
- One-click key import from environment variables
- Real-time key validation with format + connection testing
- All security audit HIGH/MEDIUM findings resolved
- 3000+ tests passing, zero regressions
- Privacy claim accurately documented everywhere
