# PROGRESS.md - Quant Arena

Append-only phase log (CLAUDE.md Section 0.5). Newest phase at the bottom.

---

## Phase 0 - Audit & Scaffolding (complete)

**What was built:** Audited the existing QUANT-ROADMAP codebase and laid down the
scaffolding for the Quant Arena migration. Enabled the Dioxus fullstack `server`
feature behind a non-default flag and proved it with one hello-world server
function, without changing the live static build. No game features, no UI, no
database tables (as Phase 0 requires).

**File list:**
- `Cargo.toml` - added non-default features `fullstack = ["dioxus/fullstack"]`
  and `server = ["dioxus/server", "fullstack"]`. Default stays `["web"]`.
- `src/server_fns.rs` (new) - `#[get("/api/hello")] hello_server()` + hidden
  `HelloProbe` component; compiled only under `fullstack`.
- `src/main.rs` - `#[cfg(feature = "fullstack")] mod server_fns;` and a
  `hello_probe()` helper rendered in `App` (real node under `fullstack`, empty
  node otherwise).
- `docs/AUDIT.md` (new) - src map, `qrt_state` localStorage shape, `/schedule`
  format, deploy workflow, fullstack-migration risk list.
- `docs/BACKLOG.md`, `docs/DEV_SETUP.md`, `docs/PROGRESS.md` (new).
- `problems/SCHEMA.md` (new) + 3 samples: `probability/two-dice-sum-vs-four.toml`
  (mcq), `dsa/array-search-complexity.toml` (numeric), `dsa/sum-of-array.toml`
  (code). Answer/grader fields are in a `[server]` section to be stripped by the
  seeder.
- `supabase/migrations/.gitkeep`, `design/mockups/.gitkeep` (new, empty).
- `.github/workflows/ci.yml` (new) - build + test + clippy + cargo audit on
  push/PR. `deploy.yml` untouched.

**How to verify:**
- Default static build unchanged: `cargo check --manifest-path quant-roadmap/Cargo.toml`
  passes (verified); `dx build --release` is the production path and `deploy.yml`
  is untouched.
- Server feature compiles: `cargo check --manifest-path quant-roadmap/Cargo.toml
  --no-default-features --features server` passes (verified - pulls
  dioxus-fullstack/dioxus-server/axum and compiles the server fn).
- Tests: `cargo test` passes (0 tests; harness green).
- Full round trip: `dx serve --features server`, then check the `/api/hello`
  request returns "Hello from the Quant Arena server" (see docs/DEV_SETUP.md).

**Known gaps (by design):** fullstack is compile-verified on both paths but the
live `dx serve` round trip is local-only (not in CI) due to the rustup-vs-system
PATH wasm issue; no Supabase project, tables, auth, or real server functions yet;
hosting is still GitHub Pages. All of this is Phase 1.

**Next phase:** Phase 1 - Auth, Profiles & Single-Player Game Engine (server-side).

---

## Phase 1 - Slice 1 (in progress; vertical slice complete)

Phase 1 is being delivered as vertical slices. **Slice 1** is the deployable
end-to-end path: auth + profiles + complete_day + me_summary + legacy import.
Branch: `feat/phase-1-backend`.

**What was built:**
- `supabase/migrations/0001_init.sql` - `profiles`, `progress`, `xp_ledger`
  (append-only, idempotent via `unique(user_id, reason, ref_id)`), `streaks`,
  enums, RLS (own-row read; no client write policies so only the service role
  writes), leaderboard view stub.
- `src/server.rs` (server-only) - sqlx Postgres pool, Supabase JWT (HS256)
  verification, and transactional DB logic: `ensure_profile`, `complete_day`
  (+35 idempotent), `import_local_progress` (one-shot, replay-proof),
  `me_summary`, pure helpers (`level_for_xp`, `streak_after`,
  `completed_days_from_legacy`).
- `src/server_fns.rs` - the four `#[post]` server functions (token-authed) +
  the Phase 0 probe.
- `src/api.rs` - shared `MeSummary` (always compiled).
- `src/sync.rs` - client wrappers with an offline fallback so the non-fullstack
  `web` build still compiles/runs (local-only).
- `src/auth.rs` - Supabase Auth interop (email + Google + GitHub) via JS, the
  `/login` page, token provider.
- `src/today.rs` - "complete day" now calls the server when signed in (offline
  fallback otherwise); account/sync panel (summary, one-time import, sign out).
- Deploy: `Dockerfile`, `fly.toml`, `.env.example`, `.dockerignore`,
  `docs/DEPLOY.md`; `docs/DEV_SETUP.md` updated; `.env` gitignored.
- Cargo: optional `sqlx`/`jsonwebtoken`/`uuid`/`tokio` behind the `server`
  feature.

**How to verify:**
- Compiles on all paths: `cargo check` (web), `--features fullstack` (client),
  `--no-default-features --features server` (server) - all pass.
- `cargo test --no-default-features --features server` -> 4 pass (level curve,
  handle validation, streak transitions, legacy-import cap).
- `cargo clippy -D warnings` clean on default and server.
- Live: set Supabase env (docs/DEV_SETUP.md), `dx serve --features server`, sign
  up, complete a day, confirm XP/streak persist; `fly deploy` per docs/DEPLOY.md.

**Known gaps -> Slice 2+ (still Phase 1):** `submit_problem` + grading +
`submissions`/`problems` tables; streak-freeze logic + profile-timezone (slice 1
uses UTC); the problem seeder; full practice-write rewiring; top bar/right rail
fed by me_summary; retire GitHub Pages after a green Fly deploy. DB-level tests
(XP idempotency, import replay, RLS denial) are written as DB integration checks
to run against the dev project (cannot run in this environment).

---

## Phase 1 - Slice 2 (problem bank + grading; in progress)

Adds the server-graded problem bank and refines streaks. Branch:
`feat/phase-1-backend`.

**What was built:**
- `supabase/migrations/0002_problems.sql` - `problems` (public cols + SEALED
  answer cols; RLS on with NO client policy so the table is unreadable from the
  browser) and `submissions` (append-only, server-timestamped; own-row read).
- `src/server.rs` - `list_problems` (sealed, answers never selected for the
  client), `submit_problem` (server-side grading, append-only submission,
  idempotent XP by difficulty 10-50, streak bump); pure `grade_value` (mcq /
  numeric-tolerance / code-normalized-output) and freeze-aware `streak_next`
  (earn 1 freeze/7-day streak, max 2; a banked freeze covers a single missed
  day) + profile-timezone (`today_in_tz`). `award_xp` now reports rows affected.
- `src/server_fns.rs` + `src/sync.rs` - `list_problems` / `submit_problem`
  endpoints + offline-fallback wrappers.
- `src/api.rs` - `BankProblem` (sealed) and `GradeResult`.
- `src/bin/seed.rs` - seeder: parses `problems/**/*.toml`, validates, upserts
  into `problems` with answers in sealed columns. Built only with `--features
  server`.
- `src/bank.rs` + `/bank` route + a link from the Today panel - minimal UI to
  fetch sealed problems, answer, and submit for grading (solution revealed only
  in the result). Legacy Practice page untouched.
- Cargo: optional `toml` + `chrono-tz` under the server feature; `tokio` gains
  rt/macros for the seeder; `[[bin]] seed` gated by `required-features`.

**How to verify:**
- Compiles web / fullstack / server; wasm fullstack client builds.
- `cargo test --features server` -> 5 pass (adds `grading_by_kind`,
  `streak_transitions_and_freezes`). `cargo clippy -D warnings` clean (default +
  server).
- Live: apply `0002_problems.sql`, run the seeder
  (`DATABASE_URL=... cargo run --bin seed --no-default-features --features server`),
  then `/bank`: answer the seeded problems, confirm server grading, XP by
  difficulty, idempotency (re-solve grants 0), and that the solution appears only
  after submitting.

**Known gaps -> later (still Phase 1):** full rewire of the legacy Practice page
onto the problems table (needs content migration); retire GitHub Pages after a
green Fly deploy. Authoring the 40-problem bank is Phase 3. DB-level
idempotency/RLS checks run against the dev project.

---

## Phase 1 - shell wired to me_summary

Surfaced the server-authoritative summary in the persistent shell (was only on
the Today account panel).

**What was built:**
- `main.rs`: shared `Signal<Option<MeSummary>>` context, refetched on token change;
  `auth::use_me()` hook.
- `layout.rs`: top-bar chips (XP / streak / progress %) + a "Journey Stats" right
  rail (progress, days x/548, XP, level, current day, est finish); offline
  fallback derives progress/days from local state (XP/streak show "-").
- `assets/main.css`: chip + right-rail styles; rail hidden < 1100px, chips < 640px.
- `today.rs` / `bank.rs`: write the shared summary after complete_day /
  submit_problem so the shell updates live; removed today.rs's duplicate fetch.

**How to verify:** compiles web/fullstack/server + wasm; clippy clean; signed in,
the top-bar chips + rail show live server XP/streak/progress and update after
completing a day or solving a problem; signed out shows derived progress with
XP/streak dashed. Full Section 5 visuals (world map, achievements, quote) remain
Phase 2.

---

## Phase 1 - Practice rewire (Day 1; code + numeric server-graded)

Made the Practice page's solved state server-authoritative for gradeable
problems, replacing the gloo-storage `solved` writes for those. Scope: code
(graded by output) + numeric math/quant (graded by value). Open-ended quant +
Linux stay self-marked. Bounded to Days 1-3 (only days with legacy content);
**Day 1 migrated this batch**, Days 2-3 follow.

**What was built:**
- `server.rs` / `server_fns.rs` / `sync.rs`: `list_problems` gained a `day`
  filter; new `solved_problem_ids` (distinct correct submissions).
- `src/bin/seed.rs`: now also accepts a `[[problems]]` multi-problem file.
- `problems/migrated/day1.toml`: 30 Day-1 problems (9 code via `code_meta`
  sample I/O; 9 math + 12 quant as numeric with authored answers + tolerances),
  reusing legacy ids.
- `practice.rs`: signed in, the day's gradeable problems grade server-side - code
  via the in-browser runner -> `submit_problem`; numeric via a "Submit answer"
  button. Solved ids load from `solved_problem_ids` and cache into local state so
  the cards + Today practice-gate reflect server truth; shared `me` updates live.
  Non-migrated (Linux, conceptual quant) + offline keep local self-mark.

**How to verify:** compiles web/fullstack/server + wasm; clippy clean; 5 server
tests pass. After seeding (`cargo run --bin seed ...`), sign in and on Day 1:
run a code problem to a matching output -> server XP; submit a numeric answer ->
graded with XP; reload -> solved persists from the server.

**Deferred:** Linux + conceptual quant stay self-mark; Phase 3 practice engine
(filters, revision queue, daily quests).

**Update:** Days 2-3 migrated too - `problems/migrated/day2.toml` (9 code + 5
numeric) and `day3.toml` (6 code + 6 numeric), same pattern, legacy ids. All of
Days 1-3 (the only days with legacy practice content) are now server-gradeable;
the seeder pre-flight test validates all 59 problem files parse/derive/unique.

---

## Phase 1 - SSR landing + OpenGraph meta

Added title + meta/OpenGraph/Twitter tags in `main.rs`'s `App` (via
`document::Title` / `document::Meta`) so link previews and SEO work. They
server-render under fullstack SSR (fixing the empty-HTML-shell problem, CLAUDE.md
3); on the static build they inject client-side. `og:url`/`og:image` point at the
current host - update on the Fly deploy, and swap og:image for a designed social
card (user-supplied art). Compiles web/fullstack/server + wasm; clippy clean.

---

## Phase 1 - LIVE VERIFIED (engine complete)

The user ran the fullstack stack against a Supabase project and confirmed it
works end to end (Supabase-backed auth + server-authoritative game engine).
This satisfies the one path CI can't cover (the live DB round-trip), so the
Phase 1 single-player engine is functionally complete and verified.

**Verified working (live):** sign-in, server-authoritative XP/streak/progress,
server-side grading (code + numeric), problem bank + Practice (Days 1-3), the
shell chips/rail, and the one-shot legacy import - all persisting in Postgres.

**Only remaining Phase 1 step (operational, user-side):** deploy the fullstack
server to Fly.io (Dockerfile/fly.toml/.env.example/docs/DEPLOY.md are ready),
then retire or repoint the GitHub Pages workflow and update og:url to the Fly
domain. Must NOT retire Pages until the Fly deploy is confirmed live (do not
break the working app).

**Next phase:** Phase 2 - World Map UI & Three-View Navigation (new session,
per CLAUDE.md 0.2 one-phase-per-session).

---

## Phase 2 - Slice 1: World Map (additive view)

Started Phase 2 (user explicitly authorized starting it). Delivered as slices;
Slice 1 = the World Map level. Additive: existing Strategy/Progress untouched.

**What was built:**
- `roadmap.rs`: `MAP_WORLDS` - the 9 worlds keyed by DAY range (Section 5:
  Foundation 1-60 ... Interview Prep 481-548) with Section-5 accent colors, plus
  `MapWorld::{days,done,state}` + a `WorldState` enum.
- `src/worldmap.rs` + `/map` route + a "World Map" sidebar item: renders the 9
  worlds on a glowing dashed path with per-world progress and node state
  (completed / current "YOU ARE HERE" / locked), driven by `me_summary`
  completed_days when signed in, else local `current_day`. Gradient placeholder
  for world art (real illustrations are user-supplied). Unlocked world click ->
  existing Strategy view (interim; Chapter Zoom replaces it in Slice 2).
- `assets/main.css`: world-map styles; collapses to a vertical list < 768px.

**How to verify:** compiles web/fullstack/server + wasm; clippy clean. Open
/map (or the "World Map" sidebar item): worlds reflect progress, the current
world pulses with "YOU ARE HERE", future worlds are locked; resize < 768px for
the vertical fallback.

**Slices 2-3 (next):** Chapter Zoom (day-nodes winding path + boss banner
"N days to unlock"); Day Detail (title/tags/est time/progress ring/XP + TODAY'S
PLAN checklist -> complete_day) + right-rail Recent Achievements + quote card.

---

## Phase 2 - Slice 2: Chapter Zoom

- `worldmap.rs`: new `Chapter` component + `Route::Chapter { world }` at
  `/map/:world`. World Map cards now link here (replacing the interim Strategy
  link). Renders the world's days (lo..hi) as nodes with state completed /
  current / locked (from me_summary else local current_day), day title tooltip
  from DAYS, and a boss-project banner ("N days to unlock" / "cleared"). Day-node
  click -> Strategy for now (Day Detail replaces it in Slice 3). Breadcrumb back
  to the World Map.
- `assets/main.css`: chapter day-node + boss-banner styles.

Compiles web/fullstack/server + wasm; clippy clean. Slice 3 (Day Detail +
right-rail) next.
