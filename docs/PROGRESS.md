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
onto the problems table (needs content migration); top bar/right rail fed by
me_summary; retire GitHub Pages after a green Fly deploy. Authoring the 40-problem
bank is Phase 3. DB-level idempotency/RLS checks run against the dev project.
