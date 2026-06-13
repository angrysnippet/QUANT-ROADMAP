# CLAUDE.md — Quant Arena
# This file is the single source of truth for this project.
# Claude Code: read this entire file before doing ANYTHING. The user will not
# re-explain the project. Everything you need is here.

---

## 0. HOW YOU (CLAUDE CODE) MUST OPERATE — READ FIRST

1. **Orient before acting.** At the start of every session: read this file fully,
   read `AGENTS.md` (Dioxus 0.7 API reference — it is authoritative for all Dioxus
   code), then read `docs/PROGRESS.md` to determine which phase is active.
   If `docs/PROGRESS.md` does not exist, you are at Phase 0.
2. **One phase per session.** Work ONLY on the active phase defined in Section 9.
   Never start the next phase in the same session, even if asked vaguely to
   "continue" — finish the active phase, update `docs/PROGRESS.md`, and stop.
3. **Plan first, always.** Before writing any code in a phase, produce a written
   plan (files to create/modify, schema changes, endpoints, tests) and ask the
   user to confirm with a single yes/no. Ask ALL clarifying questions in that
   one message, not scattered through the session.
4. **Scope discipline.** Any idea outside the active phase → append one line to
   `docs/BACKLOG.md` and move on. Do not implement it.
5. **End-of-phase ritual (mandatory):** all tests pass (`cargo test` + any added
   test scripts) → append a 5–10 line summary to `docs/PROGRESS.md` (phase number,
   what was built, file list, how to verify, known gaps) → tell the user the phase
   is complete and which phase comes next.
6. **Never break the working app.** Every phase must end in a buildable,
   deployable state. If a migration is risky, feature-flag it.
7. **Plain ASCII in code and config.** No decorative unicode in source files.
8. If anything in this file conflicts with `AGENTS.md` on Dioxus API usage,
   `AGENTS.md` wins. On product/architecture decisions, this file wins.

---

## 1. WHAT THIS PROJECT IS

Quant Arena is the evolution of **QUANT-ROADMAP**
(repo: angrysnippet/QUANT-ROADMAP, live: https://angrysnippet.github.io/QUANT-ROADMAP),
a 548-day quant-finance learning curriculum, into a **gamified multiplayer
learning world** where students, traders, developers, and working quants learn,
compete, duel, and rank up across C++, DSA, probability & statistics,
derivatives, fixed income, stat-arb, market microstructure, ML-for-finance,
time series, and portfolio/risk.

**Core loop:** Learn (daily lesson) → Practice (problems) → Compete (duels,
battles, leagues) → Rank up (ELO + XP + titles) → Unlock (worlds, boss
projects, cosmetics).

**Target users:** complete beginners through working professionals. The ranking
system (Section 6) is what makes them coexist — a derivatives VP can be
Grandmaster in Quant Finance and Silver in DSA simultaneously.

---

## 2. CURRENT STATE OF THE CODEBASE (verified facts — do not assume otherwise)

- Frontend: **Dioxus 0.7.1** (Rust → WASM), `router` feature, deployed as a
  static site to GitHub Pages via `.github/workflows`.
- Dependencies: `serde`, `serde_json`, `chrono`, `pulldown-cmark` (lesson
  markdown rendering), **`gloo-storage`** — ALL user progress, XP, and streaks
  currently live in **browser localStorage**. There is no backend, no auth,
  no database. This is the single biggest thing this project changes.
- Curriculum content lives under `/schedule` (markdown, rendered client-side).
- Styling: Tailwind (`tailwind.css` at root). Lint config: `clippy.toml`.
- `AGENTS.md` at root documents Dioxus 0.7 including **Fullstack** (server
  functions via `#[post]`/`#[get]`, `use_server_future`, SSR/hydration).
- Cargo features: `web` (default), `desktop`, `mobile`.
- Design mockups: `design/mockups/` (4 images). If the folder is missing, ask
  the user to add the images, but do not block — the design system in Section 5
  is the textual source of truth.

---

## 3. TARGET ARCHITECTURE

- **One repo, Dioxus 0.7 Fullstack.** Add the `server` feature alongside `web`.
  All game logic (XP, grading, streaks, ELO, duels) lives in **server
  functions** — type-safe, shared structs between client and server. Dioxus
  fullstack runs on Axum under the hood.
- **Database & auth: Supabase** (Postgres + Auth). Accessed ONLY from server
  functions via `postgrest`/`sqlx` + the service role key in env vars.
  The WASM client NEVER holds service keys and NEVER writes to the database
  directly. Supabase Auth JWTs verified server-side on every authenticated call.
- **Realtime (Phase 5):** dedicated WebSocket routes mounted on the same Axum
  server that serves the Dioxus app.
- **Hosting:** moves off GitHub Pages to **Fly.io or Railway** (user will choose
  when Phase 1 deploys; default Fly.io). SSR from fullstack also fixes the
  current empty-HTML-shell SEO/link-preview problem — ensure the landing page
  server-renders with proper meta/OpenGraph tags.
- **Problem content:** versioned files in `/problems` (format defined Phase 0),
  seeded into Postgres by a script. **Answer fields are server-only and must
  never appear in any payload sent to the client before grading.**
- **Local dev:** `dx serve` with fullstack; Supabase local or a dev project;
  `.env` for secrets (gitignored). Document setup in `docs/DEV_SETUP.md` (Phase 0).

---

## 4. NON-NEGOTIABLE SECURITY & ANTI-CHEAT RULES (apply to every phase)

1. **Server-authoritative everything.** XP, ELO, streaks, timers, grading.
   The client only renders. Any endpoint that lets a client self-report a
   score, an XP amount, or a completion timestamp is a bug — reject the design.
2. **XP is an append-only ledger** (`xp_ledger`), never a mutable column.
   Totals are SUM over the ledger. Awards are idempotent: re-submitting a
   solved problem or re-completing a day grants 0.
3. **Row Level Security on every table.** Users may read their own rows (and
   public leaderboard views); all writes go through server functions using the
   service role.
4. **Answers are sealed.** Duel/battle/problem payloads to the client exclude
   answer and grader fields. Solutions are revealed only after a graded
   submission (or after both duelists finish).
5. **Server timestamps every submission.** Anti-cheat heuristics: flag (do not
   auto-ban) sub-3-second solves on difficulty ≥3, accuracy streaks beyond
   statistical plausibility, and identical answer-timing patterns across
   accounts. Flags go to a `cheat_flags` table for manual review.
6. **Rate limiting** on auth, submission, matchmaking, and challenge endpoints
   (per-user and per-IP).
7. **Input validation at the boundary** with strong Rust types; parameterized
   queries only; sanitize all user-generated strings (handles, guild names,
   chat) before storage and rendering.
8. **Secrets in env vars only.** Never committed, never compiled into WASM.
   `.env` in `.gitignore`. Run `cargo audit` in CI.
9. **Legacy import is one-shot.** The localStorage migration endpoint (Phase 1)
   validates payloads (schema, caps, date sanity), runs once per account, and
   marks the account migrated so it cannot be replayed for free XP.

---

## 5. VISUAL DESIGN SYSTEM (source of truth; mockups in design/mockups/ illustrate it)

**Do not attempt to generate or recreate the fantasy castle illustrations** —
those are art assets the user supplies separately. Build every world/node with
an image slot (styled gradient placeholder until assets arrive). Implement
everything else in code: cards, paths, nodes, rings, chips, sidebars, legends.

- Theme: dark fantasy-tech. Background `#0A0E1A`→`#0D1220`. Panels `#131A2B`
  with 1px `#232C44` border, 12–16px radius. Glow effects in phase accent colors.
- Phase accents: P1 purple `#A855F7` · P2 teal `#2DD4BF` · P3 green `#84CC16`
  · P4 amber `#F59E0B` · P5 blue `#3B82F6` · P6 violet `#8B5CF6`
  · P7 indigo `#6366F1` · P8 pink `#EC4899` · P9 orange-red `#F97316`.
- World map: glowing dashed SVG paths connect worlds/day-nodes. Node states:
  completed (check), current ("YOU ARE HERE" pill), locked (padlock).
- Top bar chips: XP (gold star) · Day Streak (flame) · Overall Progress (ring %).
- Right rail: Journey Stats (progress ring, completed days x/548, XP, current
  day, estimated completion date), Recent Achievements, quote card.
- Left sidebar views: Progress, Today, Calendar, Strategy (world map),
  Practice, Journal — plus new: **Arena, Leaderboard, Guilds**.
- Three-level navigation: World Map → Chapter Zoom (day-nodes on a winding
  path, boss-project banner with "N days to unlock") → Day Detail (title, tags,
  estimated time, TODAY'S PLAN checklist, day progress ring, XP earned).
- Typography: bold condensed headers, uppercase phase labels, monospace numerals.
- Responsive: map degrades to a vertical list under 768px.

**The 9 worlds** (mirror existing roadmap phases):
1 Foundation (1–60) · 2 DSA Core (61–120) · 3 Data Structures (121–180)
· 4 Algorithms (181–240) · 5 Quant Math (241–300) · 6 Python & Data (301–360)
· 7 Linux & Tools (361–400) · 8 Projects (401–480) · 9 Interview Prep (481–548).
Boss Project gates the end of each world.

---

## 6. GAME ECONOMY (server-authoritative, exact numbers)

- **XP awards:** lesson/day complete +35 · problem solved +10/20/30/40/50 by
  difficulty 1–5 · daily quest +25 each · duel win +ELO-scaled XP (base 50,
  scaled by rating gap) · boss project +500.
- **Streaks:** any qualifying activity per local day (timezone from profile).
  Earn 1 streak-freeze per 7-day streak, max 2 banked; freezes auto-apply
  before a streak breaks.
- **Levels & titles:** XP thresholds (quadratic curve, level 1 at 0, level 2 at
  100, then `100 * level^1.6` cumulative). Titles: Intern → Analyst →
  Associate → VP → Desk Head → Quant Grandmaster.
- **ELO/MMR:** separate rating per arena track: `dsa`, `probability`,
  `quantfin`, `mentalmath`. Start 1200. K=32 for first 30 games on a track,
  K=16 after. League tiers: Bronze <1300, Silver <1500, Gold <1700,
  Platinum <1900, Diamond <2100, Grandmaster ≥2100.
- **Matchmaking constraint:** opponents within ±150 ELO on the chosen track.
  Never match across that bound; widen to ±250 only after 120s queue time.
- **Seasons:** 8-week cycles. Soft reset: rating ← rating + 0.25*(1500 − rating).
  Seasonal leaderboards + cosmetic title rewards. Lifetime stats never reset.
- **Monetization principle (future):** cosmetics only. Never sell XP, ELO, or
  competitive advantage.

---

## 7. MULTIPLAYER MODES (build order is fixed)

1. **Leaderboards** — global + friends; XP, streak, ELO per track; per season.
2. **Async Duels** — challenge a specific player or matchmake; both receive an
   identical sealed 5-problem set; 24h to complete; per-question server
   timestamps; graded on second completion or expiry; ELO exchanged; result
   screen shows per-question comparison only after both finish.
3. **Live Blitz Battles** — real-time 1v1 over WebSockets; 7 questions,
   60–90s server-side timer each; opponent's "answered" status visible, never
   their answer; 15s disconnect grace then forfeit.
4. **Guilds ("Firms")** — 5–20 members, firm page, weekly firm-vs-firm XP wars.
5. **Tournaments** — weekend single-elimination brackets per league tier,
   built on the battle engine.

---

## 8. PROBLEM CONTENT SCHEMA (define formally in Phase 0)

Each problem: `id`, `track` (dsa|probability|quantfin|mentalmath), `world`,
`day` (nullable), `difficulty` 1–5, `statement` (markdown),
`type` (mcq|numeric|code), `choices` (mcq), `answer` + `tolerance` (numeric)
— **answer/grader fields live in a server-only section of the file, stripped
by the seeder from any client-readable artifact** — `solution_explanation`,
`tags`, `time_limit_seconds`. Validate on seed; reject malformed files.

---

## 9. EXECUTION PLAN — PHASES (work strictly in order; one per session)

### PHASE 0 — Audit & Scaffolding
- Read `AGENTS.md` first. Audit the repo and write `docs/AUDIT.md`: exact src/
  structure, every gloo-storage localStorage key and its shape, the `/schedule`
  content format, the GitHub Pages deploy workflow, and a fullstack-migration
  risk list.
- Enable the Dioxus `server` feature alongside `web` (per AGENTS.md Fullstack
  section); prove it with one hello-world server function called from one page;
  keep the existing static build path working until Phase 1 deploys.
- Create: `/problems` (schema doc + 3 sample problems in the Section 8 format),
  `/supabase/migrations` (empty), `docs/` (AUDIT.md, BACKLOG.md, PROGRESS.md,
  DEV_SETUP.md), `design/mockups/` if absent.
- CI: build + `cargo test` + `cargo audit` on push.
- NO features, NO UI, NO database tables. Stop after scaffolding.

### PHASE 1 — Auth, Profiles & Single-Player Game Engine (server-side)
- Supabase migrations: `profiles` (id, handle UNIQUE 3–20 chars sanitized,
  avatar, title, timezone, created_at, legacy_migrated bool),
  `progress` (user_id, day, status enum, completed_at),
  `xp_ledger` (append-only: user_id, amount, reason enum, ref_id, created_at),
  `streaks` (user_id, current, longest, last_active_date, freezes_banked).
  RLS per Section 4.
- Server functions: complete-day, submit-problem (server-side grading vs
  seeded answers, idempotent XP), me-summary (xp, level, streak, progress %,
  estimated completion date), **import-local-progress** (one-shot legacy
  localStorage migration per Section 4.9 — on first login the client posts its
  legacy payload, server validates and writes to the ledger, sets
  legacy_migrated=true).
- Supabase Auth integration (email + OAuth); JWT verified in every
  authenticated server function.
- Streak logic with profile timezone; lazy daily check applying freezes.
- Problem seeder script (answers stripped from client paths).
- Replace gloo-storage progress writes with server calls; keep localStorage
  only as an offline read cache.
- Tests: XP idempotency, streak/timezone/freeze edges, import replay rejection,
  RLS denial of direct client writes.
- Deploy: Fly.io (or Railway) for the fullstack server; SSR landing page with
  proper meta/OpenGraph tags; document in `docs/DEPLOY.md`; retire the GitHub
  Pages workflow or repoint it.

### PHASE 2 — World Map UI & Three-View Navigation
- Implement Section 5 exactly, wired to live me-summary data: World Map
  (9 world cards, glowing path, image slots), Chapter Zoom (day-nodes, boss
  banner), Day Detail (TODAY'S PLAN checklist triggering Phase 1 endpoints),
  persistent top bar and right rail, responsive fallback.
- Reuse existing tracker components where sensible. No multiplayer UI yet.

### PHASE 3 — Practice Engine & Problem Bank
- Practice view: filters (track/world/difficulty/tag); players for mcq,
  numeric (tolerance grading), code-stub types; post-submit solution reveal;
  revision queue implementing the Day 0/3/7/21/60 spaced-repetition schedule.
- Author 40 original seed problems across the 4 tracks, difficulty 1–5,
  quant-interview realistic, original wording.
- Daily Quests: server selects 3/day (lesson, problem, revision), +25 XP each,
  validated server-side.
- Tests: numeric tolerance, spaced-repetition scheduling.

### PHASE 4 — Leaderboards & Async Duels (first multiplayer)
- `ratings` (user_id, track, rating, games_played, season) and append-only
  `elo_history`; K-factors and tiers per Section 6.
- Leaderboards: cached/materialized queries refreshed ≤60s; global + friends;
  league-tier badges in UI.
- Async Duels per Section 7.2, sealed sets per Section 4.4, matchmaking bound
  per Section 6. Duel inbox/history UI and result screen.
- Friends: request/accept, sanitized handle search.
- `cheat_flags` table + timing heuristics from Section 4.5.
- Tests: ELO math, seal integrity (client can never fetch answers
  pre-deadline), ±150 matchmaking bound, rate limits.

### PHASE 5 — Live Blitz Battles (real-time)
- Propose the WS protocol in the plan before implementing. Mount WS routes on
  the same Axum server. Matchmaking queue (in-memory with persistence fallback;
  Redis/Upstash only if justified in the plan). Battle flow per Section 7.3,
  all timing server-clocked, JWT-verified WS upgrade, one active battle per user.
- Battle UI: split-screen avatars, question card, countdown ring, live score
  bar, ELO-delta result animation — Section 5 styling.
- Load-test script simulating 50 concurrent battles; results to `docs/`.

### PHASE 6 — Guilds, Seasons, Tournaments & Hardening
- Guilds, seasons (soft reset job), weekend tournaments per Sections 6–7.
- Full security pass against Section 4: verify RLS on every table, rate limits
  on every mutating endpoint, dependency audit clean, secrets scan, and an
  adversarial review of duel/battle endpoints for answer leakage and replay
  attacks. Findings + fixes → `docs/SECURITY_REVIEW.md`.
- Production runbook → `docs/DEPLOY.md`.

---

## 10. DEFINITION OF DONE (every phase)

- Builds with `dx build` (and server build) cleanly; clippy passes per
  `clippy.toml`; all tests green; no secrets in the diff; `docs/PROGRESS.md`
  updated; the user can verify the phase with the steps you list there.