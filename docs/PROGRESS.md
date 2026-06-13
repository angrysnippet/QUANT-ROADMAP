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
