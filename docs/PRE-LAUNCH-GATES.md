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
- [ ] **4.3** `site/streets/activate.html` — verify license retrieval flow works end-to-end
- [ ] **4.4** `site/merch.html` — replace placeholder social links (`href="#"`) or remove
- [ ] **4.5** Verify all pages render correctly on mobile
- [ ] **4.6** Verify Vercel rewrites work: `/streets`, `/streets/activate`, `/api/streets/*`

## Gate 5: Build & Test Integrity

- [ ] **5.1** `cargo check` — zero errors
- [ ] **5.2** `cargo test --lib` — all tests pass
- [ ] **5.3** `pnpm run test` — all tests pass
- [ ] **5.4** `pnpm run validate:sizes` — no errors (warnings acceptable)
- [ ] **5.5** `pnpm run tauri build` — production binary builds on Windows
- [ ] **5.6** Fix `tests/stack_simulation.rs` compilation errors (type inference issues)
- [ ] **5.7** `npx tsc --noEmit` — zero TypeScript errors

## Gate 6: Product Readiness

- [ ] **6.1** Coach tab renders correctly with StreetsGate for playbook-tier users
- [ ] **6.2** License activation flow works in Settings modal
- [ ] **6.3** Coach chat works end-to-end with Claude API key configured
- [ ] **6.4** Engine Recommender returns structured recommendations
- [ ] **6.5** Strategy Generator produces markdown document
- [ ] **6.6** Launch Reviewer returns structured scores
- [ ] **6.7** Progress Dashboard shows playbook progress and nudges
- [ ] **6.8** Video Curriculum shows lock/unlock states based on drip schedule
- [ ] **6.9** Template Library loads and displays all 5 templates
- [ ] **6.10** Monitoring scheduler runs daily nudge check without errors

## Gate 7: Main Site (4da.ai)

- [ ] **7.1** Landing page has real screenshots (not placeholders)
- [ ] **7.2** Pricing section displays correctly
- [ ] **7.3** Download links work for all platforms
- [ ] **7.4** Email capture form functional
- [ ] **7.5** All external links valid (no broken links)

## Gate 8: Content Assets

- [ ] **8.1** Product screenshots (5 minimum)
- [ ] **8.2** 90-second demo video recorded
- [ ] **8.3** Show HN post draft finalized (`docs/marketing/show-hn-draft.md`)
- [ ] **8.4** Product Hunt listing copy ready (`docs/marketing/product-hunt-draft.md`)
- [ ] **8.5** Blog post draft ready
- [ ] **8.6** GitHub README has screenshots and clear value proposition

## Gate 9: Security & Privacy

- [ ] **9.1** No API keys or secrets in committed code
- [ ] **9.2** Ed25519 private key only exists in Vercel env vars
- [ ] **9.3** Stripe webhook signature verification works correctly
- [ ] **9.4** License verification works offline (Ed25519 public key baked into binary)
- [ ] **9.5** CORS headers on API endpoints are properly scoped (not `*` in production)
- [ ] **9.6** `data/settings.json` is in `.gitignore`

## Gate 10: Deployment

- [ ] **10.1** Vercel site deploys cleanly from `site/` directory
- [ ] **10.2** DNS configured for `streets.4da.ai` (or subdomain)
- [ ] **10.3** SSL certificate active
- [ ] **10.4** Stripe webhook test event succeeds
- [ ] **10.5** GitHub release created with Windows binary
- [ ] **10.6** macOS and Linux builds tested (if applicable)

---

## Execution Order

```
Gate 1 (Stripe) → Gate 3 (Env Vars) → Gate 2 (Vercel KV) → Gate 4 (Landing Pages)
                                                              ↓
Gate 5 (Build) ← parallel with → Gate 6 (Product) ← parallel with → Gate 9 (Security)
                                                              ↓
                                    Gate 7 (Main Site) → Gate 8 (Content) → Gate 10 (Deploy)
```

Start with Gate 1. Each gate must be fully green before launch.
