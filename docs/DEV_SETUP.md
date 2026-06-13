# DEV_SETUP.md - Quant Arena

How to build and run the project locally. Phase 0 keeps the default static build
working and adds an optional, non-default fullstack path.

## Prerequisites

- Rust (stable) via **rustup**, with the wasm target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- Dioxus CLI (`dx`), version `^0.7`:
  ```sh
  cargo binstall dioxus-cli --version '^0.7'   # or: cargo install dioxus-cli
  ```

### Windows toolchain gotcha (this machine)

A standalone Rust install can shadow rustup on the system PATH and break wasm
builds with `error: can't find crate for core`. If you hit that, make sure the
**rustup** toolchain is first on PATH for the shell you build in (or invoke via
`rustup run stable dx ...`). This is why Phase 0 fullstack verification is done
locally rather than in CI.

## Default build (static web - the current production path)

This is what GitHub Pages deploys. Nothing about it changed in Phase 0.

```sh
cd quant-roadmap
dx serve            # dev server at http://localhost:8080 (root path)
dx build --release  # production wasm build under target/dx
```

The CI deploy injects `base_path = "QUANT-ROADMAP"` into `Dioxus.toml` so the
project site works under `/QUANT-ROADMAP/`; locally you serve at root.

## Fullstack build (Phase 0 proof - optional, non-default)

Enabled by the `fullstack` / `server` Cargo features (see `Cargo.toml`). This
compiles the hello-world server function in `src/server_fns.rs` and the hidden
`HelloProbe` that calls it.

```sh
cd quant-roadmap
dx serve --features server        # builds client (web+fullstack) + server binary
```

Confirm it works: the page is unchanged visually, but the network tab shows a
request to `/api/hello` returning `Hello from the Quant Arena server`, and the
hidden `div[data-hello-probe]` in the DOM carries that text. (Verify locally; not
run in CI yet due to the toolchain note above.)

To prove the default build is untouched, build without the feature - the
`server_fns` module and probe are cfg'd out entirely.

## Phase 1 - fullstack dev (Supabase + server functions)

Slice 1 adds auth, profiles, the XP/streak engine, and the legacy import,
talking to Supabase from server functions via `sqlx`.

### 1. Supabase dev project

Create a hosted Supabase project (recommended over local Docker on Windows).
Apply the schema:
```sh
supabase link --project-ref YOUR-REF
supabase db push          # supabase/migrations/0001_init.sql
```
Enable Email + Google + GitHub providers (Authentication > Providers); set the
local redirect to `http://localhost:8080/`. Full steps: docs/DEPLOY.md.

### 2. Environment

Copy `.env.example` to `.env` (gitignored) and fill in. The two `SUPABASE_URL`
/ `SUPABASE_ANON_KEY` values are public and baked into the client at build time;
`DATABASE_URL` and `SUPABASE_JWT_SECRET` are server-only.

```sh
# client build env (public):
export SUPABASE_URL=https://YOUR-PROJECT.supabase.co
export SUPABASE_ANON_KEY=YOUR-ANON-KEY
# server runtime env (secret):
export DATABASE_URL='postgres://postgres:PASSWORD@db.YOUR-PROJECT.supabase.co:5432/postgres'
export SUPABASE_JWT_SECRET=YOUR-JWT-SECRET
```

### 3. Run fullstack locally

```sh
cd quant-roadmap
dx serve --features server      # client (web+fullstack) + Axum server
```
Then sign up at `/login`, complete a day, and confirm XP/streak persist across a
reload. Without the env set, the app still runs offline: `today.rs` falls back to
the local `current_day` increment and the account panel shows "Offline mode".

### Notes

- `sqlx` uses runtime-checked queries, so no live DB is needed to compile. To
  switch to compile-time-checked `query!` later, run `cargo sqlx prepare` against
  the dev DB.
- Server-only deps (`sqlx`, `jsonwebtoken`, `uuid`, `tokio`) are gated behind the
  `server` feature and never compiled into the WASM client.
- Do not commit secrets; `.env` is gitignored. `cargo audit` runs in CI.
