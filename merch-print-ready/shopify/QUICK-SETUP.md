# 4DA Shopify Quick Setup — Copy & Paste Guide

Do these in order. Each step says exactly where to go and what to paste.

---

## 1. ANNOUNCEMENT BAR (2 min)
**Go to:** Theme Customize → click "Announcement bar" section at top

**Text:**
```
FREE SHIPPING OVER $120 AUD — WORLDWIDE SHIPPING — ALL SIGNAL. NO FEED.
```

**Link:** `/collections/all`

Save.

---

## 2. HERO BANNER (2 min)
**Go to:** Theme Customize → click the hero/image banner section

**Heading:**
```
4DA SYSTEMS
```

**Subheading/Text:**
```
Developer merch. All signal. No feed.
```

**Button label:**
```
VIEW COLLECTION
```

**Button link:** `/collections/all`

Save.

---

## 3. FEATURED COLLECTION (1 min)
**Go to:** Theme Customize → click the "Featured collection" / "Products" section

- Change collection to "All Products" (or whichever shows all 8)
- Set "Maximum products to show" to **8**
- Remove or change the heading from "Products" to nothing (leave blank)

Save.

---

## 4. ADD RICH TEXT SECTION (2 min)
**Go to:** Theme Customize → click "Add section" → choose "Rich text"
Drag it below the featured collection.

**Heading:**
```
4DA SYSTEMS
```

**Text:**
```
Premium developer merch. Printed on demand. Ships worldwide. Free shipping over $120 AUD. All products printed on Gildan blanks — heavyweight cotton, built to last.
```

Save.

---

## 5. STICKY HEADER (30 sec)
**Go to:** Theme Customize → click the "Header" section

- Look for "Enable sticky header" or "Sticky" → turn it **ON**

Save.

---

## 6. MAIN MENU (3 min)
**Go to:** Shopify Admin → Online Store → Navigation → Main menu

Delete existing items. Add these:

| Label     | Link                |
|-----------|---------------------|
| Shop      | /collections/all    |
| Size Chart| /pages/size-chart   |
| FAQ       | /pages/faq          |
| About     | /pages/about        |
| Contact   | /pages/contact      |

Save.

---

## 7. FOOTER MENU (3 min)
**Go to:** Shopify Admin → Online Store → Navigation → Footer menu

Delete existing items. Add these:

| Label              | Link                          |
|--------------------|-------------------------------|
| FAQ                | /pages/faq                    |
| Size Chart         | /pages/size-chart             |
| Care Instructions  | /pages/care-instructions      |
| About              | /pages/about                  |
| Shipping           | /policies/shipping-policy     |
| Returns            | /policies/refund-policy       |
| Privacy            | /policies/privacy-policy      |
| Terms              | /policies/terms-of-service    |

Save.

---

## 8. CREATE PAGES (5 min)
**Go to:** Shopify Admin → Online Store → Pages → Add page

For each page: enter the Title, then click the `<>` (HTML) button in the editor and paste the content.

### Page 1: Size Chart
- **Title:** Size Chart
- **Content:** Open `D:\4DA\merch-print-ready\shopify\pages\size-chart.html` → copy all → paste in HTML mode

### Page 2: FAQ
- **Title:** FAQ
- **Content:** Open `D:\4DA\merch-print-ready\shopify\pages\faq.html` → copy all → paste in HTML mode

### Page 3: About 4DA
- **Title:** About 4DA
- **Content:** Open `D:\4DA\merch-print-ready\shopify\pages\about.html` → copy all → paste in HTML mode

### Page 4: Care Instructions
- **Title:** Care Instructions
- **Content:** Open `D:\4DA\merch-print-ready\shopify\pages\care-instructions.html` → copy all → paste in HTML mode

---

## 9. SET LEGAL POLICIES (3 min)
**Go to:** Shopify Admin → Settings → Policies

For each policy, click the `<>` HTML button and paste:

| Policy           | File to paste                                              |
|------------------|------------------------------------------------------------|
| Privacy policy   | `D:\4DA\merch-print-ready\shopify\pages\privacy-policy.html`   |
| Refund policy    | `D:\4DA\merch-print-ready\shopify\pages\refund-policy.html`    |
| Terms of service | `D:\4DA\merch-print-ready\shopify\pages\terms-of-service.html` |
| Shipping policy  | `D:\4DA\merch-print-ready\shopify\pages\shipping-policy.html`  |

Save.

---

## 10. FOOTER CONTENT (2 min)
**Go to:** Theme Customize → scroll to Footer section

- Add a text block: `© 2026 4DA SYSTEMS — support@4da.ai`
- Add social links: X/Twitter → `https://x.com/runyourempire`
- Remove "Powered by Shopify" if possible (look for checkbox)

Save.

---

## 11. COLLECTIONS (3 min)
**Go to:** Shopify Admin → Products → Collections → Create collection

| Collection       | Type      | Products                                    |
|------------------|-----------|---------------------------------------------|
| All Products     | Automated | Product price > $0                          |
| Tees             | Automated | Product type is "T-Shirt"                   |
| 4DA Logo         | Manual    | Logo Tee, Logo Crewneck, Logo Hoodie        |
| Sun Collection   | Manual    | 4DA Sun Tee, 4DA Inferno Tee                |
| Developer Culture| Manual    | Code Fragment Tee, Void Pulse Tee           |
| STREETS          | Manual    | STREETS Tee                                 |

---

## DONE CHECKLIST
- [ ] Announcement bar updated
- [ ] Hero banner text updated
- [ ] Featured collection shows 8 products
- [ ] Rich text section added
- [ ] Sticky header enabled
- [ ] Main menu updated (Shop, Size Chart, FAQ, About, Contact)
- [ ] Footer menu updated (8 links)
- [ ] 4 pages created (Size Chart, FAQ, About, Care)
- [ ] 4 legal policies set
- [ ] Footer content updated
- [ ] 6 collections created
