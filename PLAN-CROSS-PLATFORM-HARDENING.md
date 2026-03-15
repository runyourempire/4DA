# Cross-Platform Hardening Plan

> **STATUS: ALL PHASES COMPLETE as of 16 March 2026**

> Owner: Claude (Lead Senior Dev) + Antony (Product Owner)
> Created: 15 March 2026
> Scope: macOS + Linux seamless experience, deprecated code cleanup, remaining fixes

---

## Phase 1: macOS Build & Distribution [COMPLETE]

### 1.1 Apple Developer Account (Antony — manual)
- [x] Enroll in Apple Developer Program ($99 USD/year)
  - Enrolled, certificate created 15 Mar 2026
- [x] Create Developer ID Application certificate
- [x] Export certificate as base64 for CI

### 1.2 macOS Code Signing in CI (Claude)
- [x] Add secrets to GitHub repo:
  - `APPLE_CERTIFICATE` — base64 of .p12 cert
  - `APPLE_CERTIFICATE_PASSWORD` — cert password
  - `APPLE_SIGNING_IDENTITY` — "Developer ID Application: 4DA Systems Pty Ltd (XXXXXXXXXX)"
  - `APPLE_ID` — Apple ID email
  - `APPLE_PASSWORD` — app-specific password (not main password)
  - `APPLE_TEAM_ID` — 10-char team ID
- [x] Update release.yml build step with signing env vars
- [x] Tauri 2.0 handles notarization automatically when these are set

### 1.3 macOS Intel Cross-Compilation Fix (Claude)
- [x] Changed to `macos-15` cross-compile (`macos-13` was EOL)

### 1.4 macOS Memory Reporting (Claude)
- [x] Add macOS-specific memory reporting in diagnostics.rs
  - Uses `mach_task_self()` + `task_info()` API, 24 tests

---

## Phase 2: Linux Verification [COMPLETE]

### 2.1 Linux Build Dependencies
- [x] webkit2gtk 4.1 (correct for Tauri 2.0)
- [x] libappindicator3 (system tray)
- [x] librsvg2 (icon rendering)
- [x] patchelf (binary patching)
- [x] libssl-dev (TLS)

### 2.2 Linux Runtime Dependencies
- [x] Keyring backends enabled, GNOME detection, XDG compliance, .deb deps declared
- [x] Graceful fallback verified

### 2.3 Linux-Specific Code Audit
- [x] WSL path conversion — fixed (guarded with #[cfg(target_os = "linux")])
- [x] /proc/self/status — works on Linux (not macOS)
- [x] File permissions — #[cfg(unix)] for chmod 600 on settings

---

## Phase 3: Windows Hardening [COMPLETE]

### 3.1 WMIC Deprecation
- [x] Removed WMIC from allowlists; PowerShell `Get-CimInstance` is primary

### 3.2 WebView2 Install Mode
- [x] Configured `embedBootstrapper` in tauri.conf.json for automatic silent install

---

## Phase 4: Release Workflow Fixes [COMPLETE]

### 4.1 Fix Release Body
- [x] Updated to `.exe (NSIS installer)` in release.yml

### 4.2 Add macOS minimumSystemVersion
- [x] Set `minimumSystemVersion: "10.15"` in tauri.conf.json

---

## Phase 5: Deprecated Code Cleanup [COMPLETE]

### 5.1 STREETS Stripe Integration
The Stripe checkout flow was built for STREETS paid tiers (Community $29/mo,
Cohort $499) which are now deprecated (AD-022). STREETS playbook is free.

- [x] **Option C (dormant):** Stripe products exist but no UI links to them. Dead UI code removed. Stripe infra kept dormant in case needed for future payment flows.

### 5.2 Update Pre-Launch Gates
- [x] Gates updated to reflect actual payment stack (Shopify + Keygen)
- [x] Gate 4.5 (Mobile rendering) — PASS (responsive design confirmed)

---

## Phase 6: Remaining Audit Fixes [COMPLETE]

### 6.1 Accessibility
- [x] Focus trap added to modals (KeyboardShortcutsModal, SettingsModal)
- [x] Keyboard support added (TeamNotificationBell, clickable elements)

### 6.2 SEO Gaps
- [x] og:image, twitter cards, canonical URL, robots.txt added

### 6.3 Uncommitted Changes
- [x] All changes reviewed and committed

---

## Execution Order

1. ~~**Phase 1.3 + 1.4 + Phase 4** — CI fixes~~ DONE
2. ~~**Phase 3** — Windows hardening~~ DONE
3. ~~**Phase 5.1** — Stripe decision (Option C: dormant)~~ DONE
4. ~~**Phase 1.1 + 1.2** — Apple certs + CI signing~~ DONE
5. ~~**Phase 2** — Linux verification~~ DONE
6. ~~**Phase 6** — Polish~~ DONE
