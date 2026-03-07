# 4DA Pre-Launch Plan

> Owner: Antony Lawrence Kiddie-Pasifa
> Created: 7 March 2026
> Status: ACTIVE
> Detailed IP playbook: D:\DOWNLOADS\4da\trademark\09-PRE-LAUNCH-IP-PLAYBOOK.md

This plan covers everything required before 4DA goes live — legal entity,
intellectual property, legal compliance, operational foundations, and the
things nobody tells you until they become expensive problems.

---

## How To Use This Plan

Each task has a priority tag:

- **[BLOCK]** — Launch cannot happen without this. Do it or don't launch.
- **[CRITICAL]** — Must be done within days of launch. Skipping creates serious risk.
- **[HIGH]** — Do before launch if possible, within 2 weeks after at latest.
- **[MEDIUM]** — Do within first month. Not urgent but compounds if ignored.
- **[LOW]** — Nice to have. Do when capacity allows.

---

## WEEK 1: Foundation (7-14 March 2026)

### Day 1-2: Free Actions (Today/Tomorrow)

- [ ] **[BLOCK] Reserve npm package name**
  - Sign into npmjs.com
  - Publish minimal placeholder: `npm init --scope=@4da -y && npm publish --access public`
  - Also publish `4da` as unscoped package if available
  - This prevents malicious/confusing packages appearing under your name

- [ ] **[BLOCK] Reserve crates.io name**
  - `cargo init 4da-placeholder && cd 4da-placeholder`
  - Edit Cargo.toml: name = "4da", add description and license
  - `cargo publish` — name reserved permanently
  - Also consider: `4da-core`, `4da-cli` if relevant

- [ ] **[BLOCK] Verify GitHub org**
  - Confirm github.com/runyourempire owns the 4DA repo
  - Consider creating github.com/4da-systems org as well (redirect or future use)

- [ ] **[CRITICAL] Capture evidence screenshots**
  - Create folder: `D:\DOWNLOADS\4da\trademark\evidence\`
  - Screenshot with system clock visible (Lightshot):
    - [ ] 4DA app running (title bar, main UI, logo)
    - [ ] 4da.ai website
    - [ ] GitHub repo page (creation date visible)
    - [ ] Shopify store
    - [ ] WHOIS for 4da.ai
    - [ ] `git log --oneline --reverse | head -30` output
    - [ ] X/Twitter @runyourempire profile
    - [ ] The LICENSE file in the repo
  - Naming: `evidence-[type]-20260307.png`

- [ ] **[HIGH] Check "4DA PTY LIMITED" on ASIC**
  - Search connectonline.asic.gov.au for "4DA PTY LIMITED"
  - Note: status (registered/deregistered), directors, activity
  - If active: assess whether they could oppose your trademark
  - If deregistered: no risk, but document it

### Day 2-3: Director ID + Company Preparation

- [ ] **[BLOCK] Get Director Identification Number**
  - Go to: abrs.gov.au/director-identification-number
  - You need myGovID (not myGov) — the identity app
  - Free, takes 15 minutes
  - Required by law before you can be a company director

- [ ] **[BLOCK] Check company name availability**
  - Search ASIC Connect for "4DA SYSTEMS PTY LTD"
  - If taken (possible given "4DA PTY LIMITED" exists), alternatives:
    - 4DA TECHNOLOGIES PTY LTD
    - FOUR DIMENSIONAL AUTONOMY PTY LTD
    - 4DA SOFTWARE PTY LTD
  - Reserve the name ($55, valid 2 months) while you prepare

### Day 3-5: Register the Company

- [ ] **[BLOCK] Register 4DA Systems Pty Ltd**
  - Where: ASIC Connect (connectonline.asic.gov.au)
  - Cost: ~$576
  - Details:
    - Company type: Proprietary company limited by shares
    - Registered office: Shop 2, 290 Boundary St, Spring Hill QLD 4000
    - Principal place of business: same
    - Director: Antony Lawrence Kiddie-Pasifa
    - Shareholder: same, 100 ordinary shares at $1.00
  - Save the ACN (Australian Company Number) when issued

- [ ] **[BLOCK] Apply for company ABN**
  - Via Australian Business Register: abr.gov.au
  - Use the new ACN
  - Usually instant if done during registration
  - This is the ABN for 4DA (separate from your personal ABN)

- [ ] **[HIGH] Register for GST (if expecting >$75k revenue within 12 months)**
  - Can be done during ABN registration
  - If unsure, register anyway — easier than backfilling BAS later
  - Note: digital products sold to AU consumers are GST-liable (10%)

---

## WEEK 2: IP Protection + Legal Pages (14-21 March 2026)

### Company Asset Transfers

- [ ] **[CRITICAL] Transfer business name "4DA SYSTEMS" to company**
  - ASIC Connect → Transfer business name
  - From: your personal ABN (75453268396)
  - To: company ABN (new)
  - Cost: ~$39

- [ ] **[CRITICAL] Assign 3 AU trademark applications to company**
  - IP Australia online services → Record change of ownership
  - File Deed of Assignment for each mark (template in 09-PRE-LAUNCH-IP-PLAYBOOK.md)
  - Assignor: you personally
  - Assignee: 4DA Systems Pty Ltd
  - Get one witness to sign
  - Cost: free if done online before registration, ~$100/mark after

- [ ] **[HIGH] Update Bustle Studios virtual office**
  - Email Bustle Studios
  - Change the agreement holder from personal name to company
  - This is your registered office — needs to match ASIC records

### US Trademark Filing

- [ ] **[CRITICAL] File US trademark — Word Mark "4DA"**
  - Via filing service (Trademarkia, Trademark Engine, or direct attorney)
  - Non-US applicants MUST have a US-licensed attorney of record
  - Filing basis: Section 44(d) — foreign application priority
  - Foreign application: AU TM 2629468 (or whichever is the word mark)
  - Foreign filing date: 27 February 2026
  - Class 9: downloadable computer software for aggregating, filtering, and
    displaying developer-relevant content from internet sources
  - Applicant: 4DA Systems Pty Ltd (if registered by then, otherwise personal name)
  - Cost: ~$250 (TEAS Plus) + ~$200-500 (attorney service)

- [ ] **[CRITICAL] File US trademark — Design Mark (the "4" logo)**
  - Same service, same basis, same class
  - Upload `!4da_logo_black.png`
  - Description: stylised numeral 4 in white on black square background
  - Cost: ~$250 (TEAS Plus) + ~$200-500 (attorney service)

- [ ] **[HIGH] Paris Convention priority claim**
  - VERIFY the filing service includes the Section 44(d) priority claim
  - This is the critical element — without it, you lose the AU priority date
  - Deadline: ~27 August 2026 (6 months from AU filing)
  - DO NOT let the service file under Section 1(b) alone without the priority claim

### EU Trademark Filing

- [ ] **[HIGH] File EU trademark — Word Mark "4DA"**
  - Via EUIPO (euipo.europa.eu) with professional representative
  - Online services: Trama, TrademarkNow, or direct EU attorney
  - Claim Paris Convention priority from AU filing dates
  - Class 9: same goods description
  - Cost: EUR 850 (filing) + EUR 200-500 (representative)

- [ ] **[HIGH] File EU trademark — Design Mark (the "4" logo)**
  - Same as above, separate application
  - Cost: EUR 850 (filing) + EUR 200-500 (representative)

### Legal Pages for Website + App

- [ ] **[BLOCK] Privacy Policy**
  - Required by: Australian Privacy Act (if revenue >$3M or handling personal info),
    GDPR (if EU users visit your site), CCPA (if California users), Apple/Google
    app store policies
  - Must cover:
    - What data 4DA collects (locally — emphasise this)
    - What data is sent externally (API calls to LLM providers using USER keys)
    - What telemetry exists (if any — crash reports, usage analytics)
    - How data is stored (local SQLite, never leaves machine)
    - Third-party services (Ollama, OpenAI, Anthropic — via user's own keys)
    - Contact information for privacy inquiries
    - Cookie policy (for the website, not the app)
  - Publish at: 4da.ai/privacy
  - Link from: app settings, website footer, Shopify store footer
  - Note: 4DA's "privacy by architecture" is genuinely strong — the privacy policy
    is easy to write because you're not doing anything invasive. This is a selling
    point, not just compliance.

- [ ] **[BLOCK] Terms of Service**
  - Required before accepting any payment or offering any service
  - Must cover:
    - License grant (reference the FSL-1.1-Apache-2.0)
    - Acceptable use
    - Pro tier terms (what they're paying for, refund policy)
    - Trial terms (duration, what happens when it expires)
    - Disclaimer of warranties (critical for a content scoring tool —
      "4DA does not guarantee the accuracy or relevance of scored content")
    - Limitation of liability
    - Governing law: Queensland, Australia
    - Dispute resolution
  - Publish at: 4da.ai/terms
  - Link from: app (first-run acceptance), website footer, Shopify store

- [ ] **[HIGH] EULA / License Agreement (in-app)**
  - Shown on first run or embedded in settings
  - Can reference the FSL license + Pro tier terms
  - User must accept before using the app
  - This creates the legal relationship between 4DA Systems Pty Ltd and the user

- [ ] **[MEDIUM] Contributor License Agreement (CLA)**
  - Since 4DA is FSL open source and accepts contributions (if it does):
  - A CLA ensures contributors assign or license their contributions to you
  - Without a CLA, each contributor retains copyright on their code, which means
    you can't change the license later without their permission
  - Use: CLA Assistant (GitHub app) or Developer Certificate of Origin (DCO)
  - This matters when: the FSL converts to Apache — you need clean IP ownership
    to make that conversion for the entire codebase

---

## WEEK 3: Financial + Operational (21-28 March 2026)

### Banking + Payments

- [ ] **[BLOCK] Open company bank account**
  - Recommended: Up Business, Macquarie Business, or Westpac
  - Need: ACN, ABN, Director ID, 100 points of ID
  - All business revenue goes here, not your personal account

- [ ] **[BLOCK] Set up payment processing**
  - For Pro license keys: determine provider
    - Stripe (most common for software, good AU support)
    - LemonSqueezy (handles tax compliance for you)
    - Paddle (reseller model — they handle all tax)
  - For Shopify merch: already set up (Shopify Payments)
  - Note: Payment provider account must be in the company name, not personal

- [ ] **[HIGH] Set up accounting**
  - Minimum: Xero or MYOB ($30-60/month)
  - Track: revenue, expenses, GST, BAS lodgements
  - Or: engage a bookkeeper ($100-200/month)
  - BAS lodgement is quarterly if registered for GST
  - Financial year ends 30 June — your first one is short (Mar-Jun 2026)

### Tax Compliance

- [ ] **[HIGH] Understand your tax obligations**
  - Company tax rate: 25% (base rate entity, <$50M turnover)
  - GST: 10% on domestic sales (lodge BAS quarterly)
  - Digital products sold to AU consumers: GST applies
  - Digital products sold to overseas consumers: GST-free (export)
  - PAYG: if paying yourself a salary from the company
  - Your accountant should set this up — budget $500-1,000 for initial setup

- [ ] **[MEDIUM] Register for PAYG withholding**
  - If you'll pay yourself a salary from the company
  - Via ATO Business Portal
  - Withhold tax from your own salary, remit to ATO

### Insurance

- [ ] **[MEDIUM] Professional indemnity insurance**
  - Covers claims that your software caused harm (bad recommendations,
    data loss, incorrect scoring leading to missed opportunities)
  - Cost: ~$300-800/year for a small software company
  - Not legally required for most software, but strongly recommended
  - Especially relevant because 4DA scores and ranks content — if a user
    claims they missed something important because 4DA scored it low,
    PI insurance covers the defence

- [ ] **[LOW] Cyber liability insurance**
  - Covers data breaches, cyber incidents
  - Less critical for 4DA (local-first, no central database of user data)
  - But covers scenarios like: your website is hacked, update mechanism
    is compromised, etc.
  - Cost: ~$500-1,500/year

---

## WEEK 4: Pre-Launch Polish (28 March - 4 April 2026)

### AU Class 42 Filing

- [ ] **[HIGH] File AU Class 42 via TM Headstart**
  - Mark: same as existing marks
  - Class 42: "Software as a service (SaaS) featuring software for aggregating,
    filtering, and displaying developer-relevant content; providing temporary use
    of online non-downloadable software for content intelligence"
  - Cost: $200 (part 1) + $130 (formalise) = $330

### Open Source Readiness

- [ ] **[BLOCK] Verify LICENSE file is correct and visible**
  - Confirm FSL-1.1-Apache-2.0 is in repo root
  - Confirm it displays correctly on GitHub
  - Confirm the copyright holder matches: "Copyright 2025-2026 4DA Systems Pty Ltd"
    (update from personal name to company name after assignment)

- [ ] **[CRITICAL] Add NOTICE file to repo**
  - List all third-party dependencies and their licenses
  - Required for good open source hygiene
  - Protects against claims of unlicensed use of dependencies

- [ ] **[HIGH] Set up GitHub issue templates**
  - Bug report template
  - Feature request template
  - Security vulnerability reporting template (SECURITY.md)
  - This channels community interaction into manageable workflows

- [ ] **[CRITICAL] Create SECURITY.md**
  - How to report security vulnerabilities privately
  - Email: security@4da.ai (or similar)
  - Expected response time
  - Scope of what you consider a vulnerability
  - This is critical for a BYOK app — if someone finds a way to exfiltrate
    API keys, you need them to tell YOU, not post it on Twitter

- [ ] **[HIGH] Review README.md**
  - Installation instructions
  - Quick start guide
  - Link to privacy policy
  - Link to contributing guidelines
  - License badge (FSL-1.1-Apache-2.0)
  - "4DA" trademark notice in footer

### Trademark Notices

- [ ] **[HIGH] Add trademark notice to README.md footer**
  ```
  "4DA" and the 4DA logo are trademarks of 4DA Systems Pty Ltd.
  The FSL-1.1-Apache-2.0 license does not grant rights to use
  these trademarks. See LICENSE for details.
  ```

- [ ] **[HIGH] Add trademark notice to website footer**
  - 4da.ai footer: "4DA is a trademark of 4DA Systems Pty Ltd"
  - Shopify store: same

- [ ] **[MEDIUM] Trademark usage guidelines**
  - Create a simple page: 4da.ai/brand or 4da.ai/trademark
  - State: how others may and may not use the 4DA name and logo
  - Examples: "You MAY say '4DA-compatible' or 'works with 4DA'. You MAY NOT
    name your product '4DA Plus' or use the 4DA logo as your app icon."
  - This gives you clear ground to stand on if someone misuses the mark

---

## ONGOING: Post-Launch Operations

### Monthly

- [ ] Check IP Australia inbox for any correspondence on your 3 marks
- [ ] Check email for USPTO/EUIPO communications
- [ ] Review GitHub for suspicious forks or confusingly-named repos
- [ ] Search npm/crates for new packages using "4da" in the name
- [ ] Back up trademark evidence folder

### Quarterly

- [ ] Lodge BAS (if registered for GST)
- [ ] Review and update evidence of use file (new screenshots)
- [ ] Check ASIC obligations (annual review, solvency resolution)

### Annually

- [ ] Company annual review with ASIC (~$310/year)
- [ ] Tax return for the company (engage accountant)
- [ ] Review insurance coverage
- [ ] Bustle Studios virtual office renewal
- [ ] Review trademark portfolio — any new filings needed?

### Key Dates Calendar

| Date | Event | Action |
|------|-------|--------|
| **~27 Aug 2026** | **Paris Convention priority expires** | **All international filings must be done** |
| ~Jun-Jul 2026 | AU formal examination (3 marks) | Respond if issues |
| ~Jul-Sep 2026 | AU published for opposition (2 months) | Monitor |
| ~Sep-Nov 2026 | AU registration expected | Switch to (R) symbol |
| ~Late 2026 | EU registration expected | Update branding |
| ~Mid 2027 | US registration expected | Update branding |
| 30 Jun 2026 | End of first financial year | Tax return due |
| 20 Feb 2029 | ASIC business name renewal | Renew |
| ~2036 | AU trademark renewals (10 years) | Renew |

---

## THINGS NOBODY TELLS YOU (Outside Scope But Will Save You Headaches)

### 1. Australian Consumer Law applies to your Pro tier

If you sell Pro licenses to Australian consumers, the Australian Consumer Law
(ACL) guarantees apply automatically. You cannot contract out of them.
This means:
- The software must be of acceptable quality
- The software must match its description
- You must provide remedies (refund, repair, replace) if it doesn't
- "No refunds" policies are illegal for Australian consumers for faulty goods

For digital products: if Pro features don't work as described, the customer
is entitled to a refund. Build your refund policy around this — don't fight it.

### 2. GDPR applies even though you're in Australia

If anyone in the EU uses 4DA (visits your website, downloads the app), GDPR
technically applies. For 4DA this is low risk because:
- You process minimal personal data
- Data stays on user's machine
- You don't track, profile, or sell data

But you still need:
- A privacy policy that meets GDPR standards
- A lawful basis for any data processing (legitimate interest is fine for crash reports)
- Cookie consent on your website (if using analytics cookies)

### 3. Export controls on encryption

Tauri apps using HTTPS, TLS, or any encryption may be subject to US export
controls (EAR) if distributed to US users. For standard commercial encryption
(HTTPS, TLS) this is usually covered by a mass-market exemption, but:
- You may need to file a self-classification report with the US Bureau of
  Industry and Security (BIS)
- This is a one-time filing, no approval needed, just a notification
- Most open source projects ignore this, but technically it applies

### 4. Keygen license validation and the FSL

Your Pro tier uses Keygen for license validation. Because the code is open
source (FSL), anyone can read how validation works. Consider:
- Server-side validation (phone home) vs offline validation
- If someone patches out the license check in a fork, the FSL "no competing use"
  clause is your legal remedy, not a technical one
- Accept that some people will bypass it — focus on making Pro genuinely valuable
  rather than technically unbypassable

### 5. The "bus factor" problem

You are a sole founder, sole developer, sole director. If something happens
to you:
- The company has no other directors
- The trademarks have no other owner
- The users have no one to contact
- The open source project has no maintainer

Consider:
- A basic will that addresses digital assets and IP
- A trusted person with access to critical accounts (GitHub, ASIC, IP Australia)
- A "key person" document stored securely (not in the repo) listing all
  accounts, credentials, and what to do

This isn't urgent, but it's the kind of thing that becomes urgent at the worst
possible time.

### 6. Separating personal and business finances from day one

The single most common mistake sole founder companies make:
- Using personal accounts for business expenses
- Not keeping receipts
- Mixing personal and business spending

From the day the company is registered:
- ALL business income goes to the company bank account
- ALL business expenses come from the company bank account
- Pay yourself a salary or dividends — don't just "take money out"
- Keep every receipt (digital is fine — use an app like Dext or Hubdoc)
- This makes tax time simple and protects the corporate veil (limited liability)

### 7. Domain portfolio

You have 4da.ai. Also consider registering:
- 4da.dev (developer audience)
- 4da.app (if available)
- 4dasystems.com (matches company name)
- 4dasystems.com.au (Australian presence)

Redirect all to 4da.ai. Costs ~$10-15/year each. Prevents squatting on obvious
variations. This is cheap insurance.

### 8. Automated security scanning

Since 4DA is open source and handles API keys:
- Enable GitHub Dependabot alerts
- Enable GitHub secret scanning
- Consider adding a SAST tool (CodeQL, Semgrep) to CI
- Publish a SECURITY.md with responsible disclosure process

A security incident on a "privacy-first" product is existential. Invest in
prevention.

### 9. What to do when (not if) someone opens a hostile issue

Open source projects attract occasional hostile community members. Someone will:
- Open an issue demanding you change the license
- Accuse you of "not really open source"
- Demand features for free
- Fork with loud announcements

Have a code of conduct (CONTRIBUTING.md), a clear governance model (BDFL — you),
and the discipline to close bad-faith issues without engaging in debate.

### 10. Tax deductibility of all the above

Every dollar you spend on trademarks, company registration, legal pages, insurance,
domains, virtual office, and filing services is a **business expense** that reduces
your taxable income. Keep receipts for everything. This includes:
- All IP Australia fees
- USPTO and EUIPO fees
- Filing service fees
- ASIC fees
- Bustle Studios monthly fee
- Domain registrations
- Insurance premiums
- Accounting software/services

At a 25% company tax rate, the government effectively pays 25% of your IP
protection costs.

---

## COMPLETE BUDGET SUMMARY

### One-Time Costs

| Item | Cost (AUD) | Priority |
|------|-----------|----------|
| Director ID | Free | BLOCK |
| 4DA Systems Pty Ltd registration | ~$576 | BLOCK |
| Company ABN | Free (with registration) | BLOCK |
| Business name transfer | ~$39 | CRITICAL |
| TM assignment to company (x3) | $0-300 | CRITICAL |
| US trademark — word mark (incl attorney) | ~$700 | CRITICAL |
| US trademark — design mark (incl attorney) | ~$700 | CRITICAL |
| EU trademark — word mark (incl representative) | ~$1,700 | HIGH |
| EU trademark — design mark (incl representative) | ~$1,700 | HIGH |
| AU Class 42 filing | ~$330 | HIGH |
| Privacy policy (draft yourself or legal template) | $0-500 | BLOCK |
| Terms of service (draft yourself or legal template) | $0-500 | BLOCK |
| Additional domains (3-4) | ~$50 | MEDIUM |
| **Total one-time** | **~$6,300-7,100** | |

### Ongoing Annual Costs

| Item | Cost (AUD/year) | Priority |
|------|----------------|----------|
| ASIC annual review | ~$310 | Required |
| Bustle Studios virtual office | ~$240 | Required |
| Accounting software (Xero/MYOB) | ~$400-720 | Required |
| Professional indemnity insurance | ~$300-800 | Recommended |
| Additional domains | ~$50 | Recommended |
| Accountant (tax + BAS) | ~$1,000-2,000 | Strongly recommended |
| **Total annual** | **~$2,300-4,120** | |

---

## EXECUTION PRIORITY (if budget is tight)

If you can't do everything at once, here's the minimum viable sequence:

**Tier 1 — Do this or don't launch (cost: ~$600):**
1. Register Pty Ltd + get company ABN ($576)
2. Reserve npm/crates names (free)
3. Capture evidence (free)
4. Write privacy policy + terms of service (free if you draft them)

**Tier 2 — Do within 2 weeks of launch (cost: ~$1,500):**
5. US trademark filings (~$1,400)
6. Transfer business name + TM assignments (~$100)
7. Open company bank account (free)

**Tier 3 — Do within 1 month (cost: ~$3,700):**
8. EU trademark filings (~$3,400)
9. AU Class 42 filing (~$330)

**Tier 4 — Do within 3 months (cost: ~$1,500/yr):**
10. Set up accounting ($400-720/yr)
11. Professional indemnity insurance ($300-800/yr)
12. Engage accountant for tax setup ($500-1,000)

---

## REFERENCES

| Document | Location |
|----------|----------|
| IP Australia filing details | `D:\DOWNLOADS\4da\trademark\08-MASTER-CHECKLIST.md` |
| Full IP playbook (US, EU, company) | `D:\DOWNLOADS\4da\trademark\09-PRE-LAUNCH-IP-PLAYBOOK.md` |
| Evidence of use checklist | `D:\DOWNLOADS\4da\trademark\04-EVIDENCE-OF-USE-CHECKLIST.md` |
| FSL License | `D:\4DA\LICENSE` |
| Tauri config | `D:\4DA\src-tauri\tauri.conf.json` |
| License validation | `D:\4DA\src-tauri\src\settings\license.rs` |
