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

## Phase 1 placeholders (not used yet)

Phase 1 introduces Supabase + secrets. When it lands:

- Create a `.env` (gitignored) in `quant-roadmap/` with, e.g.:
  ```
  SUPABASE_URL=...
  SUPABASE_SERVICE_ROLE_KEY=...   # server-only; never compiled into WASM
  SUPABASE_ANON_KEY=...
  ```
- Use Supabase local (`supabase start`) or a dev project. Migrations live in
  `supabase/migrations/` (empty in Phase 0).

Do not commit secrets. `cargo audit` runs in CI (`.github/workflows/ci.yml`).
