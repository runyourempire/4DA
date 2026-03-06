# 4DA Merch — Master Plan: Store to Revenue

---

## The Situation

- 8 products ready (6 tees, 1 crewneck, 1 hoodie)
- Store is password-locked
- People are already emailing asking to buy (organic demand, zero ads)
- Prices: $59/$79/$89 AUD | Margins: 62-72%
- Fulfillment: Printful (print on demand, zero inventory risk)
- All content, CSS, pages, legal, emails, SEO, structured data — created and ready to paste

---

## Phase 0: Password Page (NOW — Before Anything Else)

**Time: 5 minutes**

Since the store has a password and people are visiting, make the password page work for you.

- [ ] Customize password page using `D:\4DA\merch-print-ready\shopify\pages\password-page.html`
- [ ] Theme Customize > switch to "Password" template > Custom Liquid > paste content

This turns a dead page into a branded "opening soon" page with your email for notifications. Every person who hits the password page should know: this is 4DA, it's opening soon, and here's how to get notified.

---

## Phase 1: Fix & Verify (Day 1 — 30 min)

**Goal: Every element on every page is visually correct.**

| # | Task | File | Done |
|---|------|------|------|
| 1 | Deploy CSS v2 | `D:\4DA\merch-print-ready\shopify\theme-custom-v2.css` | [ ] |
| 2 | Verify all 8 products exist in Printful + Shopify | Check Printful dashboard | [ ] |
| 3 | Verify Crewneck + Hoodie are synced (most likely missing) | Create in Printful if needed | [ ] |
| 4 | Verify every product has mockup images | Printful mockup generator | [ ] |
| 5 | Test checkout with Bogus Gateway | Shopify Settings > Payments | [ ] |
| 6 | Verify prices visible, sizes selectable, cart works | Browse store | [ ] |

**GATE: Do not proceed until prices/sizes/checkout are confirmed working.**

---

## Phase 2: Build Trust Layer (Day 1-2 — 40 min)

**Goal: A stranger landing on any product page has zero reason to doubt you.**

| # | Task | File | Done |
|---|------|------|------|
| 7 | Create Size Chart page | `D:\4DA\merch-print-ready\shopify\pages\size-chart.html` | [ ] |
| 8 | Create FAQ page | `D:\4DA\merch-print-ready\shopify\pages\faq.html` | [ ] |
| 9 | Create About page | `D:\4DA\merch-print-ready\shopify\pages\about.html` | [ ] |
| 10 | Create Care Instructions page | `D:\4DA\merch-print-ready\shopify\pages\care-instructions.html` | [ ] |
| 11 | Set 4 legal policies | `D:\4DA\merch-print-ready\shopify\pages\privacy-policy.html` (+ 3 more) | [ ] |
| 12 | Add trust badges to product page | `D:\4DA\merch-print-ready\shopify\pages\trust-badges-snippet.html` | [ ] |
| 13 | Add structured data (JSON-LD product) | `D:\4DA\merch-print-ready\shopify\snippets\json-ld-product.liquid` | [ ] |
| 14 | Add structured data (JSON-LD org) | `D:\4DA\merch-print-ready\shopify\snippets\json-ld-organization.liquid` | [ ] |
| 15 | Customize 404 page | `D:\4DA\merch-print-ready\shopify\pages\custom-404.html` | [ ] |

---

## Phase 3: Navigation & Discovery (Day 2 — 20 min)

**Goal: Customers can find everything and never feel lost.**

| # | Task | Details | Done |
|---|------|---------|------|
| 16 | Update main menu | All / Tees / Layers / STREETS / Size Chart | [ ] |
| 17 | Update footer menu | FAQ / Size Chart / Care / Shipping / Returns / Privacy / Terms / About | [ ] |
| 18 | Create 6 collections | All Products, Tees, Sun, Developer Culture, STREETS, 4DA Logo | [ ] |
| 19 | Set announcement bar | `FREE SHIPPING OVER $120 AUD — WORLDWIDE SHIPPING — ALL SIGNAL. NO FEED.` | [ ] |
| 20 | Configure homepage sections | Announcement > Featured collection > Rich text > Logo collection | [ ] |

---

## Phase 4: Shipping & SEO (Day 2-3 — 30 min)

**Goal: Free shipping works, Google can find you, domain is professional.**

| # | Task | Details | Done |
|---|------|---------|------|
| 21 | Configure free shipping rule ($120 AUD) | Settings > Shipping > Add $0 rate for orders > $120 | [ ] |
| 22 | Connect shop.4da.ai | DNS CNAME: shop > shops.myshopify.com | [ ] |
| 23 | Set SEO meta for all 8 products | `D:\4DA\merch-print-ready\shopify\pages\product-seo-meta.html` | [ ] |
| 24 | Set SEO meta for all 6 collections | Same file, collection section | [ ] |
| 25 | Set SEO meta for all 4 pages | Same file, page section | [ ] |

---

## Phase 5: Cart Optimization (Day 3 — 15 min)

**Goal: Maximize average order value.**

| # | Task | File | Done |
|---|------|------|------|
| 26 | Add free shipping progress bar to cart | `D:\4DA\merch-print-ready\shopify\snippets\free-shipping-bar.liquid` | [ ] |
| 27 | Add cross-sell "Pairs well with" on product pages | `D:\4DA\merch-print-ready\shopify\snippets\cross-sell.liquid` | [ ] |

---

## Phase 6: Remove Password (Day 3-4)

**DECISION GATE — Check every item:**

- [ ] CSS v2 deployed, prices/sizes visible
- [ ] All 8 products have images and correct prices
- [ ] Checkout tested successfully (Printful received order)
- [ ] Size chart, FAQ, About, Care pages live
- [ ] 4 legal policies set
- [ ] Trust badges on product pages
- [ ] Announcement bar live
- [ ] Free shipping rule configured
- [ ] Navigation menus updated
- [ ] At least 1 collection exists

**If all checked: Remove password. Store is live.**

Shopify Admin > Online Store > Preferences > scroll to "Password protection" > Uncheck "Enable password" > Save.

---

## Phase 7: Launch Day (The Day Password Comes Off)

**Goal: Notify everyone who's been waiting. No fanfare, just signal.**

| # | Task | File | Done |
|---|------|------|------|
| 28 | Email every person who asked to buy | `D:\4DA\merch-print-ready\shopify\launch-copy\email-to-interested-buyers.md` | [ ] |
| 29 | Post X/Twitter launch thread | `D:\4DA\merch-print-ready\shopify\launch-copy\social-launch-posts.md` | [ ] |
| 30 | Post Discord announcement | Same file, Discord section | [ ] |
| 31 | Pin merch link in X bio | shop.4da.ai | [ ] |

---

## Phase 8: First Week (Days 4-10)

**Goal: Build social proof and automate follow-up.**

| # | Task | Details | Done |
|---|------|---------|------|
| 32 | Set up post-purchase email | `D:\4DA\merch-print-ready\shopify\email-templates\post-purchase-thank-you.html` | [ ] |
| 33 | Install reviews (Judge.me free) | Review request template: `D:\4DA\merch-print-ready\shopify\email-templates\review-request.html` | [ ] |
| 34 | Post individual product highlights on X | `social-launch-posts.md` — product section (1 per day) | [ ] |
| 35 | Connect Google Analytics 4 | Settings > Preferences > GA4 measurement ID | [ ] |
| 36 | Submit sitemap to Search Console | shop.4da.ai/sitemap.xml | [ ] |
| 37 | Mobile verification | Full walkthrough on phone (iPhone + Android) | [ ] |

---

## Phase 9: First Month (Days 10-30)

**Goal: Understand what sells, optimize, add cart fillers.**

| # | Task | Details | Done |
|---|------|---------|------|
| 38 | Review analytics: which products get the most views | Shopify Analytics > Products | [ ] |
| 39 | Review analytics: which products convert best | Same — compare views to purchases | [ ] |
| 40 | Identify the hero product (best seller) | This becomes your ad creative later | [ ] |
| 41 | Consider sticker pack ($12-15) as cart filler | 2 tees = $118, need $2 more for free shipping | [ ] |
| 42 | Email capture popup (Klaviyo or Shopify Email) | 10% off first order or just "Join the signal" | [ ] |

---

## Phase 10: Month 2+ (Only If Phase 9 Shows Demand)

| # | Task | Details |
|---|------|---------|
| 43 | Retargeting ads (people who visited but didn't buy) | $10-20/day, Facebook or Google |
| 44 | Product photography upgrade (if hero product emerges) | Lifestyle shots of best seller |
| 45 | Expand collection (desk mat, cap, pins) | Based on what customers actually ask for |
| 46 | Lookalike audience ads | Based on first 50 customers |

---

## Revenue Trajectory (Conservative, Organic Only)

| Period | Orders | AOV | Revenue | Margin (~70%) |
|--------|--------|-----|---------|---------------|
| Week 1 | 5-8 | $120 | $600-960 | $420-670 |
| Week 2-4 | 8-15 | $110 | $880-1,650 | $616-1,155 |
| Month 2 | 10-20 | $105 | $1,050-2,100 | $735-1,470 |
| Month 3+ | Depends on paid/organic growth | | | |

**First 90 days total (organic only): $2,500-4,700 revenue, $1,750-3,300 margin**

This is conservative. If the organic interest converts (and people are already emailing you), the actual numbers could be higher. The key insight: you have zero inventory risk and 70% margins. Even 5 orders in week 1 is a successful launch.

---

## Complete File Inventory — Full Paths

### Deploy to Shopify Theme (Edit Code)
```
D:\4DA\merch-print-ready\shopify\theme-custom-v2.css                    → assets/custom.css
D:\4DA\merch-print-ready\shopify\snippets\json-ld-product.liquid        → snippets/json-ld-product.liquid
D:\4DA\merch-print-ready\shopify\snippets\json-ld-organization.liquid   → snippets/json-ld-organization.liquid
D:\4DA\merch-print-ready\shopify\snippets\free-shipping-bar.liquid      → snippets/free-shipping-bar.liquid
D:\4DA\merch-print-ready\shopify\snippets\cross-sell.liquid             → snippets/cross-sell.liquid
```

### Paste into Shopify Pages (HTML mode)
```
D:\4DA\merch-print-ready\shopify\pages\size-chart.html
D:\4DA\merch-print-ready\shopify\pages\faq.html
D:\4DA\merch-print-ready\shopify\pages\about.html
D:\4DA\merch-print-ready\shopify\pages\care-instructions.html
D:\4DA\merch-print-ready\shopify\pages\custom-404.html
D:\4DA\merch-print-ready\shopify\pages\password-page.html
```

### Paste into Theme Customize > Custom Liquid
```
D:\4DA\merch-print-ready\shopify\pages\trust-badges-snippet.html
```

### Paste into Shopify Policies
```
D:\4DA\merch-print-ready\shopify\pages\privacy-policy.html
D:\4DA\merch-print-ready\shopify\pages\refund-policy.html
D:\4DA\merch-print-ready\shopify\pages\terms-of-service.html
D:\4DA\merch-print-ready\shopify\pages\shipping-policy.html
```

### SEO Reference (copy into each product/collection/page SEO field)
```
D:\4DA\merch-print-ready\shopify\pages\product-seo-meta.html
```

### Email Templates
```
D:\4DA\merch-print-ready\shopify\email-templates\post-purchase-thank-you.html
D:\4DA\merch-print-ready\shopify\email-templates\review-request.html
```

### Product CSV (only if re-importing)
```
D:\4DA\merch-print-ready\shopify-product-import-v2.csv
```

### Launch Copy (ready to post/send)
```
D:\4DA\merch-print-ready\shopify\launch-copy\social-launch-posts.md
D:\4DA\merch-print-ready\shopify\launch-copy\bundle-strategy.md
D:\4DA\merch-print-ready\shopify\launch-copy\email-to-interested-buyers.md
```

### Guides
```
D:\4DA\merch-print-ready\shopify\MASTER-PLAN.md                        ← THIS FILE (the plan)
D:\4DA\merch-print-ready\shopify\SHOPIFY-LAUNCH-GUIDE.md               ← detailed setup walkthrough
D:\4DA\merch-print-ready\shopify\PLAN-SHOPIFY-LAUNCH.md                ← phased checklist
```
