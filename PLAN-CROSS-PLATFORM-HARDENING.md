# Cross-Platform Hardening Plan

> Owner: Claude (Lead Senior Dev) + Antony (Product Owner)
> Created: 15 March 2026
> Scope: macOS + Linux seamless experience, deprecated code cleanup, remaining fixes

---

## Phase 1: macOS Build & Distribution [BLOCKER]

### 1.1 Apple Developer Account (Antony — manual)
- [ ] Enroll in Apple Developer Program ($99 USD/year)
  - https://developer.apple.com/programs/enroll/
  - Requires Apple ID, can take 24-48 hours for approval
- [ ] Create Developer ID Application certificate
  - Keychain Access → Certificate Assistant → Request from CA
  - Then download from developer.apple.com
- [ ] Export certificate as base64 for CI
  - `base64 -i certificate.p12 -o cert-base64.txt`

### 1.2 macOS Code Signing in CI (Claude)
- [ ] Add secrets to GitHub repo:
  - `APPLE_CERTIFICATE` — base64 of .p12 cert
  - `APPLE_CERTIFICATE_PASSWORD` — cert password
  - `APPLE_SIGNING_IDENTITY` — "Developer ID Application: 4DA Systems Pty Ltd (XXXXXXXXXX)"
  - `APPLE_ID` — Apple ID email
  - `APPLE_PASSWORD` — app-specific password (not main password)
  - `APPLE_TEAM_ID` — 10-char team ID
- [ ] Update release.yml build step with signing env vars:
  ```yaml
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
    APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
  ```
- [ ] Tauri 2.0 handles notarization automatically when these are set

### 1.3 macOS Intel Cross-Compilation Fix (Claude)
- [ ] Change macOS Intel runner from `macos-latest` (now ARM) to `macos-13` (Intel)
  - `macos-latest` is Apple Silicon — cross-compiling x86_64 is fragile
  - `macos-13` is the last Intel runner available on GitHub Actions

### 1.4 macOS Memory Reporting (Claude)
- [ ] Add macOS-specific memory reporting in diagnostics.rs
  - Current: reads `/proc/self/status` which doesn't exist on macOS
  - Fix: use `mach_task_self()` + `task_info()` for macOS
  - Or use `sysinfo` crate for cross-platform (already may be a dep)

---

## Phase 2: Linux Verification [HIGH]

### 2.1 Linux Build Dependencies
- [x] webkit2gtk 4.1 (correct for Tauri 2.0)
- [x] libappindicator3 (system tray)
- [x] librsvg2 (icon rendering)
- [x] patchelf (binary patching)
- [x] libssl-dev (TLS)

### 2.2 Linux Runtime Dependencies
- [ ] Verify keyring works without gnome-keyring/kwallet
  - Current: graceful fallback to in-memory if no Secret Service
  - Test: run on minimal Ubuntu Server (no desktop environment)
- [ ] Verify AppImage runs on non-Ubuntu distros (Fedora, Arch)
  - AppImage should be self-contained but webkit2gtk version mismatches possible

### 2.3 Linux-Specific Code Audit
- [x] WSL path conversion — fixed (guarded with #[cfg(target_os = "linux")])
- [x] /proc/self/status — works on Linux (not macOS)
- [x] File permissions — #[cfg(unix)] for chmod 600 on settings

---

## Phase 3: Windows Hardening [MEDIUM]

### 3.1 WMIC Deprecation
- [ ] Replace `wmic` usage in streets_commands.rs with PowerShell
  - `wmic` deprecated in Windows 11+
  - Replace with: `powershell -Command "Get-CimInstance Win32_Processor | ..."`
  - Current usage: CPU/GPU info for STREETS electricity cost calculator

### 3.2 WebView2 Install Mode
- [ ] Add explicit WebView2 install mode to tauri.conf.json
  - Default prompts user for download if WebView2 missing
  - Better: embed bootstrapper for automatic silent install
  ```json
  "windows": {
    "webviewInstallMode": {
      "type": "embedBootstrapper"
    }
  }
  ```

---

## Phase 4: Release Workflow Fixes [HIGH]

### 4.1 Fix Release Body
- [ ] Change `.msi or .nsis` to `.exe (NSIS installer)` in release.yml
  - NSIS produces .exe, not .msi — current text is misleading

### 4.2 Add macOS minimumSystemVersion
- [ ] Add to tauri.conf.json:
  ```json
  "macOS": {
    "minimumSystemVersion": "10.15"
  }
  ```

---

## Phase 5: Deprecated Code Cleanup [MEDIUM]

### 5.1 STREETS Stripe Integration
The Stripe checkout flow was built for STREETS paid tiers (Community $29/mo,
Cohort $499) which are now deprecated (AD-022). STREETS playbook is free.

Decision needed (Antony):
- [ ] **Option A:** Remove Stripe entirely — delete checkout.js, simplify activate.js
  - Pro: Less code, less infrastructure, less confusion
  - Con: Need to rebuild if Signal tier needs Stripe checkout later
- [ ] **Option B:** Repurpose for Signal tier — update tiers to Signal ($12/mo, $99/yr)
  - Pro: Payment infra already built
  - Con: Keygen already handles Signal licensing
- [ ] **Option C:** Keep as-is, dormant — Stripe products exist but no UI links to them
  - Pro: Zero work
  - Con: Dead code accumulates

### 5.2 Update Pre-Launch Gates
- [ ] Gate 1 (Billing) — update to reflect actual payment stack
  - If Shopify + Keygen: remove Stripe gates entirely
  - If keeping Stripe for Signal: update tier names/prices
- [ ] Gate 4.5 (Mobile rendering) — can now be marked PASS
  - Landing page audit confirmed responsive design with proper viewports

---

## Phase 6: Remaining Audit Fixes [LOW]

### 6.1 Accessibility
- [ ] Add focus-trap to modals (KeyboardShortcutsModal, SettingsModal)
  - Install `focus-trap-react` or implement manual focus containment
- [ ] Add keyboard support to GhostPreview.tsx clickable div
  - Add `tabIndex={0}` + `onKeyDown` handler for Enter/Space

### 6.2 SEO Gaps
- [ ] merch.html — add og:image, twitter cards, canonical URL
- [ ] streets/activate.html — add meta description, canonical URL

### 6.3 Uncommitted Changes
- [ ] Review and commit the 10 remaining modified files:
  - docs/legal/PRIVACY-POLICY.md (updated)
  - docs/legal/TERMS-OF-SERVICE.md (updated)
  - src-tauri/src/model_registry.rs (updates)
  - src-tauri/src/ace/db.rs, analysis_deep_scan.rs, analysis_status.rs
  - src-tauri/src/context_engine.rs, db/migrations.rs

---

## Execution Order

1. **Phase 1.3 + 1.4 + Phase 4** — CI fixes (Claude, now)
2. **Phase 3** — Windows hardening (Claude, now)
3. **Phase 5.1** — Stripe decision (Antony decides, Claude implements)
4. **Phase 1.1 + 1.2** — Apple certs (Antony enrolls, Claude wires CI)
5. **Phase 2** — Linux verification (on next test cycle)
6. **Phase 6** — Polish (post-launch is fine)
