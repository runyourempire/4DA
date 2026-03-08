# 4DA Launch Checklist

## Section 1: What's Already Done (by Claude)

- [x] Fortification plan: all 10 phases complete
- [x] 2,435 tests passing (1,618 Rust + 817 frontend)
- [x] Release build: 11MB NSIS installer
- [x] CHANGELOG.md written
- [x] GitHub release notes drafted
- [x] Updater latest.json template created
- [x] Landing page exists (site/ directory, Vercel)
- [x] Legal docs: Privacy Policy, Terms of Service, SECURITY.md, CODE_OF_CONDUCT.md, CONTRIBUTING.md, NOTICE
- [x] License: FSL-1.1-Apache-2.0
- [x] Keygen license validation working (both key formats)
- [x] All contact info verified (security@4da.ai, support@4da.ai)

---

## Section 2: Your Moves (ordered)

### Phase A — Legal Foundation (do first, everything else depends on this)

- [ ] Register 4DA Systems Pty Ltd (Director ID application → ASIC → ABN)
- [ ] Open business bank account
- [ ] Update LICENSE copyright from personal name to "4DA Systems Pty Ltd"
- [ ] Assign trademarks to company entity

### Phase B — Payment & Distribution (do once company exists)

- [ ] Set up payment processor (Paddle recommended — handles AU GST/international taxes)
- [ ] Connect Keygen to payment processor for Pro license generation
- [ ] Test purchase flow end-to-end: buy → receive key → activate in app

### Phase C — Pre-Launch Verification

- [ ] Run NSIS installer on a clean Windows machine (VM or fresh install)
- [ ] Walk through full first-run flow as a new user
- [ ] Test with no API keys (free tier should work fully)
- [ ] Test with Ollama running (local AI features)
- [ ] Test license activation with a real Keygen key
- [ ] Verify auto-updater checks for updates on startup

### Phase D — Ship It

- [ ] Run: `git tag v1.0.0 && git push origin v1.0.0`
- [ ] Create GitHub release: attach installer + latest.json (signed)
- [ ] Sign the installer with Minisign: `minisign -Sm 4DA-Home_1.0.0_x64-setup.nsis.zip`
- [ ] Upload signed latest.json to the release
- [ ] Verify: install from GitHub release link works
- [ ] Deploy site/ to Vercel (if not already live): `cd site && vercel --prod`

### Phase E — Post-Launch (first week)

- [ ] Monitor GitHub issues
- [ ] Check auto-updater works (create v1.0.1 test release)
- [ ] File US/EU trademarks under Paris Convention (deadline: ~Aug 2026)
- [ ] Set up support@4da.ai email forwarding

### Phase F — Distribution (when ready for volume)

- [ ] Show HN post (draft below)
- [ ] Submit to developer tool directories
- [ ] Record 2-minute demo video

---

## Section 3: Show HN Draft

```
Show HN: 4DA – Privacy-first developer intelligence that scores content against your codebase

Hi HN,

I built 4DA because I was drowning in developer content. HN, Reddit, arXiv, RSS — hundreds of articles daily, but only 2-3 actually relevant to my current work.

4DA runs locally on your machine and scores content from 10 sources using 5-axis relevance scoring. It auto-discovers your projects, detects your tech stack, and surfaces only what matters.

Key decisions:
- Privacy-first: zero telemetry, runs 100% locally
- BYOK: bring your own API key (or use Ollama for fully offline)
- Free tier includes everything except AI briefings
- Built with Tauri 2.0 (Rust backend, React frontend, SQLite)

The scoring engine cross-references articles against your dependency graph, so if you're using React 19 and an article about a React 19 breaking change drops, it scores high. If it's about Angular, it doesn't.

Free download: https://4da.ai
Source: https://github.com/runyourempire/4DA (FSL-1.1-Apache-2.0)

Would love feedback on the scoring accuracy and first-run experience.
```
