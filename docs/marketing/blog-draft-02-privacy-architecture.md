# Why 4DA Is a Desktop App (And Why Your Dev Tools Should Be Too)

*Reading time: ~8 minutes*
*Platform: Dev.to, Hashnode, personal blog*
*Suggested tags: #privacy #security #devtools #architecture*

---

When I tell developers that 4DA reads their codebase to score content relevance, I get two reactions:

1. "That's brilliant -- my Cargo.toml IS the best filter."
2. "Wait, you want to read my project files? Hard pass."

Both reactions are correct. The insight is powerful. And the trust concern is completely valid.

This is why 4DA is a desktop app, not a SaaS product. Not because we could not build a web service. Because we should not.

## The Trust Problem With Cloud Dev Tools

Here is what a cloud-based version of 4DA would require:

- Upload your `Cargo.toml`, `package.json`, `go.mod`
- Upload your Git history (commit messages, branch names, file change patterns)
- Upload your reading behavior (what you click, save, dismiss)
- Trust us to store it securely, not sell it, not get breached, and not shut down

That last point is not hypothetical. Pocket shut down in July 2025 and millions of users lost their saved content. A cloud service can pivot, get acquired, or disappear. Your local data cannot be taken from you by a corporate strategy decision.

For a content aggregator, losing your reading list is annoying. For a tool that knows your entire technology stack, your dependencies, your architecture patterns, and your professional interests -- a breach or acquisition is a much bigger problem.

## What "Zero Telemetry" Actually Means

When companies say "we respect your privacy," they usually mean "we anonymize your data before selling behavioral patterns to advertisers." When they say "no tracking," they mean "no third-party tracking -- we do our own."

4DA means something different:

**Zero telemetry means zero.** Not anonymized. Not aggregated. Not "we only collect crash reports." The application does not contain code that sends data to any server we operate. There is no analytics SDK. No error reporting service. No "phone home" on startup. Nothing.

I genuinely cannot tell you how many people use 4DA. I do not know. The application does not report its existence to anyone.

This is verifiable. The source code is public under FSL-1.1-Apache-2.0. You can read every line. You can build it from source and monitor its network traffic. You will find exactly two categories of outbound requests:

1. **Content fetching** -- pulling articles from HN, Reddit, arXiv, etc. (the sources you configured)
2. **AI API calls** -- only if you configured a BYOK API key, only to the provider you chose

That is it. No third category. No surprise beacons. No update checks that happen to include a device fingerprint.

## The BYOK Model

BYOK (Bring Your Own Key) is not a limitation. It is a privacy architecture decision.

When you use an AI feature in 4DA (daily briefings, score autopsy), your request goes directly from your machine to the AI provider (Anthropic, OpenAI). 4DA does not proxy it. We do not see the request. We do not see the response. We do not store it.

Your API key is stored in `data/settings.json` on your local filesystem. It is never transmitted to any server we operate.

And if you do not want to use external APIs at all: Ollama. Fully local inference. Your content scoring runs without any network dependency beyond fetching the actual content from sources.

## The Architecture

```
Your Machine
+--------------------------------------------------+
|                                                  |
|  4DA Desktop App                                 |
|  +--------------------------------------------+  |
|  |  ACE Scanner (reads your codebase locally)  |  |
|  |  PASIFA Scorer (5-axis, all local)         |  |
|  |  SQLite + sqlite-vec (local database)      |  |
|  |  Behavior Learning (local feedback loop)   |  |
|  +--------------------------------------------+  |
|         |                        |               |
|    [fetch content]          [BYOK API calls]     |
|         |                   (optional, direct)   |
+---------|-----------------------|----------------+
          v                       v
   HN, Reddit, arXiv,     Anthropic/OpenAI
   GitHub, RSS, etc.       (or Ollama locally)
```

There is no 4DA server in this diagram because there is no 4DA server. Period.

The database is a SQLite file at `data/4da.db`. It is a regular file on your disk. You can open it with any SQLite client. Back it up, inspect it, or delete it. When you uninstall 4DA, the data is gone.

## Why This Matters More Now

Three trends make local-first architecture increasingly important for developer tools:

**1. AI tools need deeper context.** The more useful an AI tool is, the more it needs to know about your work. GitHub Copilot reads your code. Cursor reads your codebase. 4DA reads your project metadata. As these tools get better, the sensitivity of the data they require goes up. The architecture question -- cloud or local -- becomes increasingly critical.

**2. Cloud services shut down.** Pocket. Google Reader. Sunrise Calendar. Every cloud service you depend on is one board meeting away from being "sunset." Local-first tools give you data sovereignty by default.

**3. Supply chain attacks are increasing.** Your `package.json` and `Cargo.toml` are a map of your attack surface. Uploading them to a cloud service creates one more place that map can be compromised. Keeping it local eliminates that vector entirely.

## The Tradeoff

Local-first is not free. Here is what 4DA gives up:

- **No sync across devices.** Your 4DA data lives on one machine. (We may add encrypted local export/import later, but never cloud sync.)
- **No social features.** No "trending in the 4DA community." No shared reading lists. The product is individual by design.
- **Harder to build.** Tauri 2.0 + Rust is a harder stack than Next.js + Vercel. Desktop distribution is harder than a URL. Auto-updates are harder than a deploy.

These are real costs. We chose them deliberately because the alternative -- asking developers to upload their codebase metadata to our servers -- is worse.

## Source Available, Apache 2.0 After Two Years

4DA is licensed under FSL-1.1-Apache-2.0. This means:

- **Today:** Source is public. You can read, audit, and run it. Commercial competing services are restricted for 2 years.
- **After 2 years:** The code converts to Apache 2.0. Full open source. No restrictions. Forever.

This is intentional. We want you to verify our privacy claims. We want security researchers to audit the code. We want the developer community to trust 4DA not because we say "trust us" but because they can read every line.

## Try the Local-First Approach

Download 4DA. Install it. Open a network monitor. Watch what it does.

You will see content fetches from your configured sources. If you added an API key, you will see direct calls to your AI provider. You will not see anything else. No analytics. No telemetry. No surprise connections.

That is the product working as designed.

---

**Download 4DA:** [https://4da.ai](https://4da.ai)

**View the source:** [https://github.com/runyourempire/4DA](https://github.com/runyourempire/4DA)
