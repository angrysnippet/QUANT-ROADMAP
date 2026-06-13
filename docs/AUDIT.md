# AUDIT.md - Quant Arena, Phase 0

Snapshot of the existing QUANT-ROADMAP codebase before the Quant Arena
migration. Written per CLAUDE.md Section 9, Phase 0. Facts here are verified
against the source, not assumed.

## 1. Source structure (`quant-roadmap/src/`)

| File          | Role |
|---------------|------|
| `main.rs`     | Entry point. `dioxus::launch(App)`; declares the `Route` enum and the `App` root component (favicon/CSS/font links + `Router`). |
| `state.rs`    | `AppState` struct + localStorage load/save (`gloo-storage`) + `use_app_state` autosave effect. |
| `roadmap.rs`  | ~1938 lines of static curriculum data: `DAILY_BLOCKS`, `DAYS: &[StrategyDay]`, plus derivation/gate helpers (`overall_pct`, `topic_progress`, `phase_label`, etc.). |
| `landing.rs`  | Route `/`: animated galaxy landing page; "click to enter" routes into the app. |
| `layout.rs`   | `AppShell` layout (sidebar + top bar + right rail) wrapping all in-app routes via `Outlet`. |
| `progress.rs` | `/progress`: stat cards, journey bar, worlds grid, track breakdown. Read-only; percentages derived from completed days. |
| `today.rs`    | `/today`: the per-day gates (routine checks, practice, journal) that advance `current_day`. |
| `calendar.rs` | `/calendar`: projects strategy days onto calendar dates from `schedule_start`. |
| `strategy.rs` | `/strategy`: renders a day's authored schedule (Markdown via `pulldown-cmark`) or the block/success tiles. |
| `practice.rs` | `/practice`: practice questions/quizzes; solved state in `AppState.solved`. |
| `journal.rs`  | `/journal`: 5 reflective questions per day. |
| `planner.rs`  | `/planner`: planning view. |
| `runner.rs`   | Shared helper module (not a route). |

Router (`main.rs`): `Landing {}` at `/`, then a `#[layout(AppShell)]` group over
`/progress`, `/today`, `/calendar`, `/strategy`, `/practice`, `/journal`,
`/planner`.

## 2. Persistence: browser localStorage only

There is **no backend, no auth, no database**. All state lives under a single
localStorage key.

- **Key:** `qrt_state` (`state.rs:25`, `const STORAGE_KEY`).
- **Shape:** `AppState` (serde Serialize/Deserialize), all fields `#[serde(default)]`:

  | Field            | Type                                          | Meaning |
  |------------------|-----------------------------------------------|---------|
  | `current_day`    | `u32` (default 1)                             | Strategy day the user is on (1-based). Progress is *derived* from `current_day - 1`; there is no stored progress map. |
  | `daily_checks`   | `HashMap<String, HashMap<String, bool>>`      | day id ("1", "2", ...) -> `"block_item"` -> done. Keyed by strategy-day id, not date. |
  | `journal`        | `HashMap<String, HashMap<String, String>>`    | day id -> `"j1".."j5"` -> text. |
  | `schedule_start` | `Option<String>`                              | Calendar date that "Day 1" maps to (Calendar view). |
  | `solved`         | `HashMap<String, bool>`                        | practice question/quiz id -> solved. |
  | `theme`          | `String` (default `"dark"`)                   | UI theme: `"dark"` or `"light"`. |

- **Access:** `AppState::load()` = `LocalStorage::get("qrt_state").unwrap_or_default()`;
  `AppState::save()` = `LocalStorage::set(...)`. `use_app_state()` provides the
  signal via context and registers a `use_effect` that saves on every change.
- **Migration relevance (Phase 1):** this is exactly the legacy payload the
  one-shot `import-local-progress` endpoint must validate and fold into the
  server-side `xp_ledger` / `progress` tables. `daily_checks` + `journal` +
  `current_day` are the meaningful signals; `theme` is cosmetic.

## 3. Curriculum content format (`/schedule` + `roadmap.rs`)

- `DAILY_BLOCKS` (`roadmap.rs`): 8 routine blocks (C++ theory, Implementation,
  LeetCode, Mathematics, Python, Linux, Quant thinking, Journal), 21 checkable
  items total (`total_daily_items()`).
- `DAYS: &[StrategyDay]`: the day-by-day schedule. `StrategyDay { id, phase,
  title, schedule_md, blocks, success }`.
  - **Days 1-3:** authored inline as `DAY1_SCHEDULE` / `DAY2_SCHEDULE` /
    `DAY3_SCHEDULE` string consts, with full `blocks` (raw-HTML `content`
    rendered via `dangerous_inner_html`) and `success` checklists.
  - **Days 4-548:** `schedule_md: include_str!("../schedule/DAY_N.md")` with
    empty `blocks`/`success`. The `.md` files are plain Markdown (headers,
    block sections, "Success Criteria" lists, "Tracker Update" percentages)
    rendered client-side by `pulldown-cmark`.
- `schedule/` holds `DAY_4.md` through `DAY_548.md` (Days 1-3 have no `.md`;
  they are the inline consts).
- `phase_color(phase)` maps phase labels ("Phase 1".."Phase 15") to tag colors.
  Note: the in-code phase labels (Phase 1-15) are finer-grained than the
  9 "worlds" defined in CLAUDE.md Section 5; the mapping is a Phase 2 concern.

## 4. Build & deploy (`.github/workflows/deploy.yml`)

- Trigger: push to `main` (and manual `workflow_dispatch`).
- Steps: checkout -> install Rust stable + `wasm32-unknown-unknown` -> cache ->
  install `cargo-binstall` -> install Dioxus CLI (`dx`) -> **inject
  `base_path = "QUANT-ROADMAP"` into `Dioxus.toml`** (project-site hosting under
  `/QUANT-ROADMAP/`; only in CI, so local `dx serve` stays at root) ->
  `dx build --release` (web only) -> copy dist to `_site`, add SPA fallback
  (`cp index.html 404.html`) + `.nojekyll` -> upload + `deploy-pages`.
- Cargo features: `default = ["web"]`, plus `web` / `desktop` / `mobile`. As of
  Phase 0 also `fullstack` and `server` (both **non-default**; see Section 6).
- `Dioxus.toml`: minimal; `[web.app] title`, empty resource lists. No base_path
  committed (injected in CI).

## 5. Fullstack-migration risk list

Risks to carry into Phase 1 when moving from static web to Dioxus Fullstack +
Supabase + a non-GitHub-Pages host:

1. **Browser-only APIs must be cfg-gated off the server build.**
   `gloo-storage` (`state.rs`) and `document::eval` (`main.rs` theme effect,
   `landing.rs` audio) are WASM/browser-only. Under SSR/the server platform
   they must be feature/cfg-gated or behind `use_effect` (post-hydration), or
   the server build breaks.
2. **Hydration mismatch.** Once SSR is on, the first client render must match
   the server's HTML. State that today loads from localStorage at startup will
   differ between server (no localStorage) and client; move it to
   `use_server_future` / post-hydration `use_effect`.
3. **`chrono` `wasmbind`.** Currently enabled for WASM clock. On the server the
   clock differs; ensure timezone/date logic (streaks in Phase 1) is
   server-authoritative and not relying on client `chrono` semantics.
4. **Hosting base path.** GitHub Pages serves under `/QUANT-ROADMAP/` via the
   injected `base_path`. A fullstack server (Fly.io/Railway) serves at root;
   asset paths, router base, and the deploy workflow all change. SSR also fixes
   the current empty-HTML-shell SEO/link-preview problem (CLAUDE.md Section 3).
5. **Secrets.** The Supabase service-role key must live in server env vars only,
   never compiled into WASM. `.env` gitignored; `cargo audit` in CI.
6. **Local toolchain.** A standalone Rust install shadows rustup on this
   machine's PATH and breaks wasm builds ("can't find crate for core"). Fullstack
   verification must use the rustup toolchain; documented in `DEV_SETUP.md`.

## 6. Phase 0 changes made (scaffolding only)

- `Cargo.toml`: added non-default `fullstack = ["dioxus/fullstack"]` and
  `server = ["dioxus/server", "fullstack"]` features. Default build untouched.
- `src/server_fns.rs`: one hello-world `#[get("/api/hello")]` server function +
  a hidden `HelloProbe` component, compiled only under `fullstack`.
- `src/main.rs`: `#[cfg(feature = "fullstack")] mod server_fns;` and a
  `hello_probe()` helper (real node under `fullstack`, empty node otherwise)
  rendered in `App`. The default `web` build emits identical output.
- New dirs/files: `docs/` (this audit + BACKLOG/DEV_SETUP/PROGRESS),
  `problems/` (SCHEMA + 3 samples), `supabase/migrations/` (empty),
  `design/mockups/` (empty; art assets are user-supplied - CLAUDE.md Section 5
  textual design system is the source of truth), `.github/workflows/ci.yml`.

No game features, no UI changes, no database tables were added in Phase 0.
