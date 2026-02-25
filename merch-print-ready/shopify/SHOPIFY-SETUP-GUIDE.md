# Shopify Store Setup Guide — 4DA SYSTEMS

> Step-by-step browser walkthrough. All files are in `merch-print-ready/shopify/`

---

## STEP 1: Theme Selection & Custom CSS

1. Shopify Admin → **Online Store** → **Themes**
2. Use **Dawn** theme (free, minimal, works best with our CSS override)
3. Click **Customize** → in the theme editor, click the **gear icon** (Theme settings)
4. Under **Colors**, set everything to black/dark (this gets overridden by our CSS but avoids flash of white)
5. To add custom CSS: **Actions** → **Edit Code** → open `assets/` folder
6. Create new file: `custom.css` → paste contents of `theme-custom.css`
7. Open `layout/theme.liquid` → add before `</head>`:
   ```html
   {{ 'custom.css' | asset_url | stylesheet_tag }}
   ```

---

## STEP 2: Store Branding

### Logo
- Use the 4DA sun logo (transparent PNG, white version for dark background)
- Shopify Admin → **Settings** → **Brand** → upload logo
- Recommended size: 200x200px or 400x100px horizontal

### Favicon
- Use the "4" mark as favicon (32x32 PNG)
- Settings → Brand → Favicon

### Store Name Display
- Theme Customize → Header → set to show logo image (not text)

---

## STEP 3: Legal Pages (copy-paste the HTML)

1. **Shopify Admin** → **Settings** → **Policies**
2. For each policy, switch to HTML mode (click `<>` icon) and paste:
   - **Privacy policy** → paste `pages/privacy-policy.html`
   - **Refund policy** → paste `pages/refund-policy.html`
   - **Terms of service** → paste `pages/terms-of-service.html`
   - **Shipping policy** → paste `pages/shipping-policy.html`

---

## STEP 4: Navigation Menus

### Main Menu (Header)
Shopify Admin → **Online Store** → **Navigation** → **Main menu**

| Label | Link |
|-------|------|
| All | /collections/all-products |
| Tees | /collections/tees |
| Layers | /collections/4da-logo |
| STREETS | /collections/streets |

### Footer Menu
Shopify Admin → **Online Store** → **Navigation** → **Footer menu**

| Label | Link |
|-------|------|
| Shipping | /policies/shipping-policy |
| Returns | /policies/refund-policy |
| Privacy | /policies/privacy-policy |
| Terms | /policies/terms-of-service |
| 4DA App | https://4da.ai |
| GitHub | https://github.com/runyourempire/4DA |

---

## STEP 5: Collections

Shopify Admin → **Products** → **Collections** → **Create collection**

| Collection | Type | Condition |
|------------|------|-----------|
| All Products | Automated | Product type is any |
| Tees | Automated | Product type is "T-Shirt" |
| Sun Collection | Manual | Add: 4DA Sun Tee (B+W), 4DA Inferno Tee (B+W) |
| Developer Culture | Manual | Add: Code Fragment Tee (B+W), Void Pulse Tee |
| STREETS | Manual | Add: STREETS Tee (B+W) |
| 4DA Logo | Manual | Add: Logo Tee (B+W), Logo Crewneck (B+W), Logo Hoodie (B+W) |

---

## STEP 6: Announcement Bar

Theme Customize → **Announcement bar** section:

**Text:** `ALL SIGNAL. NO FEED. — Free shipping on orders over $120 AUD`

(Or just: `ALL SIGNAL. NO FEED.` if you don't want to offer free shipping threshold)

---

## STEP 7: Homepage Layout

Theme Customize → arrange sections:

1. **Featured collection** → "All Products" or "Tees" — show 6 products
2. **Rich text** section:
   - Heading: `4DA SYSTEMS`
   - Text: `Developer merch. All signal. No feed.`
3. **Featured collection** → "4DA Logo" (tee + crewneck + hoodie)
4. **Newsletter** section (optional — skip for now)

---

## STEP 8: Payments

1. Shopify Admin → **Settings** → **Payments**
2. Click **Activate Shopify Payments**
3. Enter:
   - Business name: 4DA SYSTEMS
   - ABN: 75 453 268 396
   - Bank account details (BSB + account number)
   - Business address: Shop 2, 290 Boundary St, Spring Hill QLD 4000

---

## STEP 9: Shipping

1. Shopify Admin → **Settings** → **Shipping and delivery**
2. Printful manages shipping automatically — rates come from Printful
3. Verify shipping zones are set up (Printful should configure this on install)

---

## STEP 10: Domain (Optional)

Options:
- **Free:** yourstore.myshopify.com (fine for now)
- **Custom:** Buy through Shopify or connect existing domain
- **Recommended:** `shop.4da.ai` as a subdomain (add CNAME record in your DNS)

---

## STEP 11: Pre-Launch Checklist

- [ ] All products synced from Printful with correct prices ($59/$79/$89)
- [ ] Product descriptions match catalog
- [ ] Mockup images show design on main image
- [ ] Legal pages filled in (privacy, refund, terms, shipping)
- [ ] Shopify Payments activated
- [ ] Navigation menus set up (header + footer)
- [ ] Collections created
- [ ] Custom CSS applied (dark theme matches 4da.ai)
- [ ] Test checkout flow (use Shopify test mode)
- [ ] Place a test order to verify Printful receives it
