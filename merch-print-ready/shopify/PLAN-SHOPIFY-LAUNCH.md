# Shopify Launch Plan — 4DA SYSTEMS

> Status: All code/content created. Awaiting browser execution.
> Estimated total time: ~90 minutes of browser work across 3 sessions

---

## Phase 1: Fix Critical Visibility Issues (20 min)

These are conversion killers. Nothing else matters until these work.

### Task 1.1: Deploy CSS v2
- Open `merch-print-ready/shopify/theme-custom-v2.css`
- Shopify Admin > Online Store > Themes > Actions > Edit Code
- In `assets/`, open or create `custom.css`
- Delete ALL existing content, paste the entire v2 CSS
- In `layout/theme.liquid`, verify before `</head>`:
  ```
  {{ 'custom.css' | asset_url | stylesheet_tag }}
  ```
- Save. Preview. Verify:
  - [ ] Product prices visible (white text, JetBrains Mono font)
  - [ ] Size selector buttons visible (border, readable labels)
  - [ ] Color variant labels visible
  - [ ] Add to Cart button visible (gold background, dark text)
  - [ ] Cart drawer totals visible

### Task 1.2: Verify All 8 Products Exist in Printful + Shopify
- Open Printful dashboard > Stores > Shopify
- Confirm ALL 8 products synced:
  - [ ] 4DA Sun Tee (Black, White)
  - [ ] 4DA Inferno Tee (Black, White)
  - [ ] Code Fragment Tee (Black, White)
  - [ ] STREETS Tee (Black, White)
  - [ ] 4DA Logo Tee (Black, White)
  - [ ] Void Pulse Tee (Black only)
  - [ ] 4DA Logo Crewneck (Black, White) -- IF MISSING: create in Printful
  - [ ] 4DA Logo Hoodie (Black, White) -- IF MISSING: create in Printful
- Verify prices: Tees $59, Crewneck $79, Hoodie $89

### Task 1.3: Verify Product Images
- Shopify Admin > Products
- Click each product, verify it has at least 1 mockup image
- If any product has NO images:
  - Option A: Printful auto-generates mockups on sync
  - Option B: Use Printful mockup generator manually
  - Option C: Upload flat-lay mockup images

### Task 1.4: Test Checkout
- Shopify Admin > Settings > Payments > Use Bogus Gateway
- Browse store > Add item to cart > Complete checkout
- Verify:
  - [ ] Prices correct at each step
  - [ ] Size selection works
  - [ ] Printful receives the test order
- Disable test mode after

**GATE: Do not proceed to Phase 2 until prices and sizes are visually confirmed working.**

---

## Phase 2: Pages, Navigation & Trust (30 min)

### Task 2.1: Create 4 Shopify Pages
Shopify Admin > Online Store > Pages > Add page

For each: type the Title, then switch to HTML mode (click `<>`) and paste:

| # | Title | File to paste |
|---|-------|--------------|
| 1 | Size Chart | `shopify/pages/size-chart.html` |
| 2 | FAQ | `shopify/pages/faq.html` |
| 3 | About 4DA | `shopify/pages/about.html` |
| 4 | Care Instructions | `shopify/pages/care-instructions.html` |

Verify each page renders correctly after saving.

### Task 2.2: Set Legal Pages
Shopify Admin > Settings > Policies

For each policy, click `<>` to switch to HTML mode and paste:

| Policy | File |
|--------|------|
| Privacy | `shopify/pages/privacy-policy.html` |
| Refund | `shopify/pages/refund-policy.html` |
| Terms | `shopify/pages/terms-of-service.html` |
| Shipping | `shopify/pages/shipping-policy.html` |

### Task 2.3: Update Navigation
Shopify Admin > Online Store > Navigation

**Main menu:**
| Label | Link |
|-------|------|
| All | /collections/all |
| Tees | /collections/tees |
| Layers | /collections/4da-logo |
| STREETS | /collections/streets |
| Size Chart | /pages/size-chart |

**Footer menu:**
| Label | Link |
|-------|------|
| FAQ | /pages/faq |
| Size Chart | /pages/size-chart |
| Care | /pages/care-instructions |
| Shipping | /policies/shipping-policy |
| Returns | /policies/refund-policy |
| Privacy | /policies/privacy-policy |
| Terms | /policies/terms-of-service |
| About | /pages/about |

### Task 2.4: Announcement Bar
Theme Customize > Announcement bar:
```
FREE SHIPPING OVER $120 AUD — WORLDWIDE SHIPPING — ALL SIGNAL. NO FEED.
```
Enable: Yes. Color: dark scheme (CSS handles it).

### Task 2.5: Trust Badges on Product Page
Theme Customize > Product page template:
1. Below the product form, click "Add block" or "Add section"
2. Choose "Custom Liquid"
3. Paste contents of `shopify/pages/trust-badges-snippet.html`
4. Save

### Task 2.6: Create Collections
Shopify Admin > Products > Collections

| Collection | Type | Condition |
|------------|------|-----------|
| All Products | Automated | Product price > $0 |
| Tees | Automated | Product type is "T-Shirt" |
| Sun Collection | Manual | 4DA Sun Tee + 4DA Inferno Tee |
| Developer Culture | Manual | Code Fragment Tee + Void Pulse Tee |
| STREETS | Manual | STREETS Tee |
| 4DA Logo | Manual | Logo Tee + Logo Crewneck + Logo Hoodie |

---

## Phase 3: Shipping, Domain & SEO (25 min)

### Task 3.1: Configure Free Shipping Rule
Shopify Admin > Settings > Shipping and delivery

For EACH shipping zone (Australia, International, etc.):
1. Click "Add rate"
2. Rate name: "Free shipping"
3. Price: $0.00
4. Condition: Based on order price > Minimum: $120.00
5. Save

Keep Printful's calculated rates as the default for orders under $120.

### Task 3.2: Connect Custom Domain
In your DNS provider (wherever 4da.ai is registered):
1. Add CNAME record:
   - Name: `shop`
   - Target: `shops.myshopify.com`
   - TTL: 3600

Then in Shopify:
1. Settings > Domains > Connect existing domain
2. Enter: `shop.4da.ai`
3. Verify and wait for SSL (up to 48h)

### Task 3.3: Set SEO Meta Descriptions
For EACH product (8 products):
1. Shopify Admin > Products > [Product]
2. Scroll to "Search engine listing preview" > Edit
3. Copy the Title and Meta from `shopify/pages/product-seo-meta.html`

For EACH collection (6 collections):
1. Products > Collections > [Collection]
2. Scroll to SEO > Edit
3. Copy from `product-seo-meta.html`

For EACH page (4 pages):
1. Online Store > Pages > [Page]
2. Scroll to SEO > Edit
3. Copy from `product-seo-meta.html`

**Total: 18 SEO entries to set.**

### Task 3.4: Homepage Layout
Theme Customize > Home page:
1. Announcement bar (already done)
2. Image banner or rich text hero:
   - Heading: "4DA SYSTEMS"
   - Subtext: "Developer merch. All signal. No feed. Ships worldwide."
3. Featured collection: "All Products" (show 8)
4. Featured collection: "4DA Logo" (tee + crewneck + hoodie)

---

## Phase 4: Growth (Week 1 after removing password)

### Task 4.1: Email Capture
- Install Shopify Email (free) or Klaviyo (free up to 250 contacts)
- Add email signup to footer section in Theme Customize
- Optional: popup offering 10% off first order

### Task 4.2: Post-Purchase Automation
Shopify Admin > Marketing > Automations > Create:
- Trigger: First order placed
- Wait: 2 hours
- Send email using template from `email-templates/post-purchase-thank-you.html`

### Task 4.3: Review Request Automation
- Install Judge.me (free) or Shopify Product Reviews
- Configure auto-request: 14 days after delivery
- Email template: `email-templates/review-request.html`

### Task 4.4: Analytics
- Settings > Online Store > Preferences
- Add Google Analytics 4 measurement ID
- Submit `shop.4da.ai/sitemap.xml` to Google Search Console

### Task 4.5: Mobile Verification
Browse the full store on your phone:
- [ ] Announcement bar readable, not cut off
- [ ] Product cards: images, prices, sizes all visible
- [ ] Product page: variant buttons tappable (44px+ touch targets)
- [ ] Cart: totals visible, checkout button works
- [ ] Trust badges wrap correctly on small screens

---

## Password Removal Decision Gate

Remove password ONLY when ALL of these are true:
- [ ] CSS v2 deployed and visually verified
- [ ] All 8 products have images and correct prices
- [ ] Test checkout completed successfully
- [ ] Size chart page exists and is linked
- [ ] Legal pages (4) are filled in
- [ ] Announcement bar is live with free shipping + worldwide messaging
- [ ] Trust badges visible on product pages
- [ ] Free shipping rule configured in shipping settings
- [ ] At least header navigation updated

---

## Files Inventory (What Claude Created)

All ready to copy-paste. No code changes needed — just paste into Shopify.

| File | Paste Into |
|------|-----------|
| `shopify/theme-custom-v2.css` | Themes > Edit Code > assets/custom.css |
| `shopify/pages/size-chart.html` | Pages > Size Chart (HTML mode) |
| `shopify/pages/faq.html` | Pages > FAQ (HTML mode) |
| `shopify/pages/about.html` | Pages > About 4DA (HTML mode) |
| `shopify/pages/care-instructions.html` | Pages > Care Instructions (HTML mode) |
| `shopify/pages/trust-badges-snippet.html` | Theme Customize > Product page > Custom Liquid |
| `shopify/pages/product-seo-meta.html` | Each product/collection/page SEO field |
| `shopify/pages/privacy-policy.html` | Settings > Policies > Privacy |
| `shopify/pages/refund-policy.html` | Settings > Policies > Refund |
| `shopify/pages/terms-of-service.html` | Settings > Policies > Terms |
| `shopify/pages/shipping-policy.html` | Settings > Policies > Shipping |
| `shopify/email-templates/post-purchase-thank-you.html` | Marketing > Automations |
| `shopify/email-templates/review-request.html` | Judge.me or Automations |
| `shopify-product-import-v2.csv` | Products > Import (only if re-importing) |

### Code Changes (already applied to codebase)

| File | Change |
|------|--------|
| `site/src/merch.njk` | Free shipping banner, AUD labels on prices, size chart links on cards, updated footer links, trust icons in CTA section |
| `site/shopify-theme.css` | Marked as deprecated, points to v2 |
