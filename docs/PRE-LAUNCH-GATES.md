# 4DA Pre-Launch Gates

Every gate must pass before launch. No exceptions.

---

## Gate 1: Billing Infrastructure (Stripe)

- [x] **1.1** Create Stripe account and complete business verification
- [x] **1.2** Create Stripe products:
  - Community: $29/mo (`prod_U1ZztSzoZQZf6p` / `price_1T3Wpx1U2RWjTPKyrw8czFVh`)
  - Community Annual: $249/yr (`price_1T3Wpx1U2RWjTPKypLNXM9JQ`)
  - Cohort: $499 one-time (`prod_U1ZzGVms5ZI1I1` / `price_1T3Wpy1U2RWjTPKyymVFL6Ss`)
- [x] **1.3** Configure Stripe Checkout sessions with `streets_tier` metadata — `site/api/streets/checkout.js`
- [x] **1.4** Set up Stripe webhook endpoint (`we_1T3Wpy1U2RWjTPKysz62mFF6`) pointing to `4da.ai/api/streets/activate`
- [x] **1.5** Wire real Checkout URLs into `site/streets.html` (3 buttons wired to `streetsCheckout()`)
- [x] **1.6** Wire real Checkout URLs into `site/streets/index.html` (3 buttons wired to `streetsCheckout()`)
- [ ] **1.7** Test full flow: Stripe test mode → webhook fires → license generated → retrieved via email → activates in 4DA

## Gate 2: License Persistence (Stripe Customer Metadata)

- [x] **2.1** ~~Vercel KV~~ — Using Stripe customer metadata instead (no extra infrastructure)
- [x] **2.2** `site/api/streets/activate.js` stores licenses in Stripe customer metadata on webhook
- [x] **2.3** License retrieval via Stripe customer search by email — persists across cold starts
- [x] **2.4** License expiration embedded in signed payload (`expires_at` = 1 year from purchase)

## Gate 3: Environment Variables (Vercel)

- [x] **3.1** Generate Ed25519 keypair for license signing
- [x] **3.2** Set `LICENSE_PRIVATE_KEY_HEX` in Vercel project env vars
- [x] **3.3** Set `STRIPE_WEBHOOK_SECRET` in Vercel project env vars
- [x] **3.4** Set Ed25519 public key in Rust backend (`src-tauri/src/settings/license.rs:222`)
- [x] **3.5** Deploy site and verify API endpoints respond (GET and POST) — all 3 tiers tested

## Gate 4: Landing Pages

- [x] **4.1** `site/streets.html` — all `href="#"` replaced with `streetsCheckout()` calls
- [x] **4.2** `site/streets/index.html` — same replacements
- [x] **4.3** `site/streets/activate.html` — license retrieval flow works (email + session_id + deep link)
- [x] **4.4** `site/merch.html` — no placeholder links found, all navigation functional
- [ ] **4.5** Verify all pages render correctly on mobile
- [x] **4.6** Vercel rewrites verified: `/streets`, `/streets/activate`, `/merch`, `/api/streets/*`, `/api/notify`

## Gate 5: Build & Test Integrity

- [x] **5.1** `cargo check` — zero errors (verified: 2046 tests compile)
- [x] **5.2** `cargo test --lib` — all tests pass (2046 passed, 0 failed, 4 ignored)
- [x] **5.3** `pnpm run test` — all tests pass (1101 passed, 78 test files)
- [x] **5.4** `pnpm run validate:sizes` — no errors (31 warnings, 0 errors)
- [x] **5.5** `pnpm run tauri build` — production binary builds (`4DA Home_1.0.0_x64-setup.exe`)
- [x] **5.6** `tests/stack_simulation.rs` — 84 tests pass (Gate verified)
- [x] **5.7** `npx tsc --noEmit` — zero TypeScript errors

## Gate 6: Product Readiness

- [x] **6.1** Coach tab renders correctly with StreetsGate for playbook-tier users
- [x] **6.2** License activation flow works in Settings modal (Ed25519 verification tested)
- [x] **6.3** Coach chat works end-to-end with Claude API key configured (error now surfaced to user)
- [x] **6.4** Engine Recommender returns structured recommendations
- [x] **6.5** Strategy Generator produces markdown document
- [x] **6.6** Launch Reviewer returns structured scores (0-100 scale explicit in prompt)
- [x] **6.7** Progress Dashboard shows playbook progress and nudges (self-loading data)
- [x] **6.8** Video Curriculum shows lock/unlock states based on drip schedule
- [x] **6.9** Template Library loads and displays all 5 templates
- [x] **6.10** Monitoring scheduler runs daily nudge check without errors

## Gate 7: Main Site (4da.ai)

- [x] **7.1** Landing page has real screenshots (8 screenshots + 8 demo videos + posters)
- [x] **7.2** Pricing section displays correctly (Free + Pro tiers, structured data)
- [x] **7.3** Download links work for all platforms (GitHub releases, platform detection)
- [x] **7.4** Email capture form functional (POST to `/api/notify`, stores in Stripe customers)
- [x] **7.5** All external links valid (no broken `href="#"`, STREETS buttons wired to JS)

## Gate 8: Content Assets

- [x] **8.1** Product screenshots (8 screenshots in `site/screenshots/`, 8 demo videos in `site/media/`)
- [ ] **8.2** 90-second demo video recorded
- [x] **8.3** Show HN post draft finalized (`docs/marketing/show-hn-draft.md`)
- [x] **8.4** Product Hunt listing copy ready (`docs/marketing/product-hunt-draft.md`)
- [x] **8.5** Blog post drafts ready (3 drafts in `docs/marketing/`)
- [x] **8.6** GitHub README has screenshots and clear value proposition (hero + 4 demo GIFs)

## Gate 9: Security & Privacy

- [x] **9.1** No API keys or secrets in committed code
- [x] **9.2** Ed25519 private key only exists in Vercel env vars
- [x] **9.3** Stripe webhook signature verification works correctly
- [x] **9.4** License verification works offline (Ed25519 public key baked into binary)
- [x] **9.5** CORS headers on API endpoints are properly scoped (not `*` in production)
- [x] **9.6** `data/settings.json` is in `.gitignore`

## Gate 10: Deployment

- [x] **10.1** Vercel site deploys cleanly (`vercel.json` configured, Eleventy build, serverless functions)
- [ ] **10.2** DNS configured for `streets.4da.ai` (or subdomain) — add custom domain in Vercel dashboard
- [x] **10.3** SSL certificate active (auto-provisioned by Vercel on domain setup)
- [x] **10.4** Stripe webhook handler verified (signature validation, raw body, test mode compatible)
- [x] **10.5** GitHub release workflow configured (`.github/workflows/release.yml` — Windows, macOS x2, Linux)
- [x] **10.6** All platform builds configured (`nsis`, `dmg`, `appimage`, `deb` in `tauri.conf.json`)

---

## Summary

| Gate | Status | Remaining |
|------|--------|-----------|
| 1. Billing | 6/7 | 1.7: Live Stripe test transaction |
| 2. License | 4/4 | COMPLETE |
| 3. Env Vars | 5/5 | COMPLETE |
| 4. Landing | 5/6 | 4.5: Mobile rendering check |
| 5. Build | 7/7 | COMPLETE |
| 6. Product | 10/10 | COMPLETE |
| 7. Site | 5/5 | COMPLETE |
| 8. Content | 5/6 | 8.2: 90-second demo video |
| 9. Security | 6/6 | COMPLETE |
| 10. Deploy | 5/6 | 10.2: DNS custom domain setup |

**48 of 51 gates PASS. 3 remaining require user action (external systems).**
