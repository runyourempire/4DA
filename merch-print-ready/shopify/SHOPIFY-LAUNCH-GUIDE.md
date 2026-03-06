# Shopify Store Launch Guide — 4DA SYSTEMS v2.0

> Complete browser walkthrough. All files are in `merch-print-ready/shopify/`
> Supersedes the original SHOPIFY-SETUP-GUIDE.md

---

## PHASE 1: Critical Fixes (Do First)

### 1.1 Replace Custom CSS (Fixes Price/Size Visibility)

1. Shopify Admin > **Online Store** > **Themes** > **Actions** > **Edit Code**
2. In `assets/` folder, find `custom.css` (or create it)
3. **Delete all existing content** and paste the entire contents of `theme-custom-v2.css`
4. In `layout/theme.liquid`, verify this line exists before `</head>`:
   ```html
   {{ 'custom.css' | asset_url | stylesheet_tag }}
   ```
5. **Save** and preview — prices, sizes, and variant labels should now be visible

**What this fixes:**
- Price text invisible on dark background (nuclear fallback: forces white text globally)
- Size selector buttons invisible or unreadable
- Variant labels ("Color", "Size") not showing
- Cart totals and line items invisible
- All form inputs, breadcrumbs, pagination styled correctly

### 1.2 Verify Printful Product Sync

Check that ALL 8 products are synced from Printful:

| Product | SKUs | Status |
|---------|------|--------|
| 4DA Sun Tee | BLK, WHT | Check |
| 4DA Inferno Tee | BLK, WHT | Check |
| Code Fragment Tee | BLK, WHT | Check |
| STREETS Tee | BLK, WHT | Check |
| 4DA Logo Tee | BLK, WHT | Check |
| Void Pulse Tee | BLK only | Check |
| **4DA Logo Crewneck** | BLK, WHT | **NEW — may need to create in Printful** |
| **4DA Logo Hoodie** | BLK, WHT | **NEW — may need to create in Printful** |

**If Crewneck/Hoodie are not in Printful yet:**
1. Printful Dashboard > Add Product
2. Crewneck: Gildan 18000, DTG, left chest `4-logo-v2-black.png` / `4-logo-v2-white.png`
3. Hoodie: Gildan 18500, DTG, left chest `4-logo-v2-black.png` / `4-logo-v2-white.png`
4. Set prices: Crewneck $79 AUD, Hoodie $89 AUD
5. Sync to Shopify

**If re-importing products via CSV:**
Use `shopify-product-import-v2.csv` — includes Crewneck + Hoodie with correct weights, SKUs, and descriptions.

### 1.3 Verify Product Images

Every product MUST have mockup images (not just the print file). For each product, verify in Shopify Admin > Products:

- [ ] Main image: flat lay front, design clearly visible
- [ ] At least 2-3 additional images (model shot, close-up, lifestyle)

If Printful auto-generated mockups, these should already be there. If not, use Printful's mockup generator to create them.

### 1.4 Test Checkout Flow

1. Shopify Admin > Settings > Payments > Enable test mode (Bogus Gateway)
2. Go to your storefront, add a tee to cart, go through checkout
3. Verify: price visible, size selectable, cart totals correct, Printful receives the order
4. Disable test mode when done

---

## PHASE 2: Trust & Conversion (Before Removing Password)

### 2.1 Create Shopify Pages

Shopify Admin > **Online Store** > **Pages** > **Add page**

Create each page. Switch to HTML mode (click `<>`) and paste the file contents:

| Page Title | Handle (URL) | Source File |
|------------|-------------|-------------|
| Size Chart | `size-chart` | `pages/size-chart.html` |
| FAQ | `faq` | `pages/faq.html` |
| About 4DA | `about` | `pages/about.html` |
| Care Instructions | `care-instructions` | `pages/care-instructions.html` |

### 2.2 Set Up Legal Pages

Shopify Admin > **Settings** > **Policies**

For each policy, switch to HTML mode and paste:

| Policy | Source File |
|--------|------------|
| Privacy policy | `pages/privacy-policy.html` |
| Refund policy | `pages/refund-policy.html` |
| Terms of service | `pages/terms-of-service.html` |
| Shipping policy | `pages/shipping-policy.html` |

### 2.3 Configure Announcement Bar

Theme Customize > **Announcement bar** section:

**Text:**
```
FREE SHIPPING OVER $120 AUD — WORLDWIDE SHIPPING — ALL SIGNAL. NO FEED.
```

**Settings:**
- Show announcement: Yes
- Color scheme: Keep dark (CSS handles styling — gold text on dark bg)
- Link: `/collections/all` (optional — makes the whole bar clickable to the shop)

### 2.4 Add Trust Badges to Product Pages

1. Theme Customize > Product page template
2. Add section: **Custom Liquid** (below the product form / Add to Cart button)
3. Paste contents of `pages/trust-badges-snippet.html`
4. Save

This adds: Secure Checkout | Worldwide Shipping | Quality Guarantee | Free Over $120 AUD

### 2.5 Update Navigation Menus

#### Main Menu (Header)

Shopify Admin > **Online Store** > **Navigation** > **Main menu**

| Label | Link |
|-------|------|
| All | /collections/all |
| Tees | /collections/tees |
| Layers | /collections/4da-logo |
| STREETS | /collections/streets |
| Size Chart | /pages/size-chart |

#### Footer Menu

Shopify Admin > **Online Store** > **Navigation** > **Footer menu**

| Label | Link |
|-------|------|
| FAQ | /pages/faq |
| Size Chart | /pages/size-chart |
| Care Instructions | /pages/care-instructions |
| Shipping | /policies/shipping-policy |
| Returns | /policies/refund-policy |
| Privacy | /policies/privacy-policy |
| Terms | /policies/terms-of-service |
| About | /pages/about |

### 2.6 Configure Free Shipping

Shopify Admin > **Settings** > **Shipping and delivery**

1. In your shipping profile, click **Add rate** for each zone
2. Add a rate: "Free shipping" — Price: $0.00 — Condition: Minimum order price > $120.00 AUD
3. Keep Printful's calculated rates as the default for orders under $120

**Alternative (if Printful manages rates):**
- Install a free shipping bar app (e.g., "Starter Free Shipping Bar" — free)
- Set threshold to $120 AUD
- Or handle via Shopify Scripts if on Shopify Plus

### 2.7 Connect Custom Domain

**Recommended: `shop.4da.ai`**

1. In your DNS provider, add a CNAME record:
   - Name: `shop`
   - Target: `shops.myshopify.com`
2. Shopify Admin > Settings > Domains > Connect existing domain > `shop.4da.ai`
3. Wait for SSL certificate (automatic, takes up to 48 hours)

### 2.8 Set SEO Meta Descriptions

For each product and collection, scroll to "Search engine listing preview" and paste the title + meta from `pages/product-seo-meta.html`.

Do this for:
- [ ] All 8 products
- [ ] All 6 collections
- [ ] All 4 custom pages (Size Chart, FAQ, About, Care)

### 2.9 Homepage Layout

Theme Customize > arrange sections:

1. **Announcement bar** — Free shipping + worldwide + all signal
2. **Image banner** (optional) — Hero image with "4DA SYSTEMS" heading
3. **Featured collection** > "All Products" — show 8 products
4. **Rich text** section:
   - Heading: `4DA SYSTEMS`
   - Text: `Developer merch. All signal. No feed. Ships worldwide.`
5. **Featured collection** > "4DA Logo" (tee + crewneck + hoodie)
6. **Custom Liquid** — trust badges (optional on homepage too)

---

## PHASE 3: Growth Setup (Week 1 After Launch)

### 3.1 Email Capture

**Option A: Shopify Email (free for first 10,000 emails/month)**
1. Shopify Admin > Marketing > Automations
2. Create: "Welcome new subscriber" automation
3. Add popup via Theme Customize > add "Email signup" section to footer

**Option B: Klaviyo (free up to 250 contacts)**
1. Install Klaviyo app
2. Create a popup: "Join the signal" — 10% off first order
3. Set up welcome flow: Welcome > Bestsellers > Brand story

### 3.2 Post-Purchase Email

Shopify Admin > Marketing > Automations > Create automation:

**Flow: "First purchase thank you"**
- Trigger: First order placed
- Wait 2 hours
- Email:
  ```
  Subject: Your order is being printed — here's what to expect
  Body: Your [product] is being custom-printed right now. Production takes 2-5 business days, then it ships with tracking. Questions? Reply to this email.
  ```

### 3.3 Reviews

**Option A: Shopify Product Reviews (free)**
1. Install from Shopify App Store
2. Enable on product pages via Theme Customize

**Option B: Judge.me (free tier)**
1. Install Judge.me
2. Auto-sends review request 14 days after delivery
3. Displays stars on product cards + product pages

### 3.4 Google Analytics / Search Console

1. Shopify Admin > Online Store > Preferences
2. Add Google Analytics 4 tracking ID
3. Submit sitemap to Google Search Console: `shop.4da.ai/sitemap.xml`

---

## COLLECTIONS

Create in Shopify Admin > **Products** > **Collections**:

| Collection | Type | Condition / Products |
|------------|------|---------------------|
| All Products | Automated | Product price > $0 |
| Tees | Automated | Product type is "T-Shirt" |
| Sun Collection | Manual | 4DA Sun Tee, 4DA Inferno Tee |
| Developer Culture | Manual | Code Fragment Tee, Void Pulse Tee |
| STREETS | Manual | STREETS Tee |
| 4DA Logo | Manual | Logo Tee, Logo Crewneck, Logo Hoodie |

---

## PRE-LAUNCH CHECKLIST

### Critical (Cannot Launch Without)
- [ ] CSS v2 applied — prices, sizes, variants all visible on dark background
- [ ] All 8 products synced from Printful with correct prices ($59/$79/$89)
- [ ] Every product has at least 1 clear mockup image
- [ ] Size chart page created and linked from product descriptions
- [ ] Test checkout completed — Printful receives order correctly
- [ ] Shopify Payments activated (Stripe)
- [ ] Legal pages filled in (privacy, refund, terms, shipping)

### Important (Should Have for Launch)
- [ ] FAQ page created
- [ ] About page created
- [ ] Care instructions page created
- [ ] Announcement bar: "FREE SHIPPING OVER $120 AUD — WORLDWIDE SHIPPING — ALL SIGNAL. NO FEED."
- [ ] Trust badges on product pages (Custom Liquid snippet)
- [ ] Free shipping rule configured ($120 AUD threshold)
- [ ] Navigation menus updated (header + footer with new pages)
- [ ] Collections created (all 6)
- [ ] Custom domain connected (`shop.4da.ai`)
- [ ] SEO meta descriptions set for all products + collections + pages

### Nice to Have (Week 1)
- [ ] Email capture popup or footer signup
- [ ] Post-purchase email automation
- [ ] Reviews app installed
- [ ] Google Analytics connected
- [ ] Sitemap submitted to Search Console
- [ ] Social media links in footer (X, Discord)
- [ ] Mobile testing on actual devices (iPhone + Android)

---

## STORE SETTINGS REFERENCE

- **Store name:** 4DA SYSTEMS
- **Legal business name:** 4DA SYSTEMS (ABN 75 453 268 396)
- **Store email:** runyourempirehq@gmail.com
- **Currency:** AUD
- **Payments:** Shopify Payments (Stripe)
- **Address:** Shop 2, 290 Boundary St, Spring Hill QLD 4000
- **Theme:** Dawn (free) with custom CSS v2
- **Fulfillment:** Printful (print on demand)
