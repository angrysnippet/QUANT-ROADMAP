# Quant Arena fullstack server image (Phase 1) for Fly.io.
#
# NOTE: this project has not deployed fullstack before. The build commands below
# follow the standard Dioxus 0.7 fullstack pattern; confirm them locally with
# `dx serve --features server` and `dx bundle --help` before the first deploy
# (see docs/DEPLOY.md). The default `web`-only GitHub Pages build is unaffected.

# ---- build stage -----------------------------------------------------------
FROM rust:1-bookworm AS build
WORKDIR /app

# wasm target (client) + Dioxus CLI.
RUN rustup target add wasm32-unknown-unknown \
 && cargo install dioxus-cli --version '^0.7' --locked

# Cache deps first.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
 && cargo build --release --no-default-features --features server || true
RUN rm -rf src

COPY . .

# Build the fullstack bundle: client WASM/assets + the Axum server binary.
# `dx bundle` emits the server executable and a `public/` asset dir under
# target/dx/<crate>/release/web/.
RUN dx bundle --release --features server

# Normalise the output location for the runtime stage.
RUN set -eux; \
    SRV="$(find target/dx -type f -name 'quant-roadmap' -path '*release*' | head -1)"; \
    PUB="$(find target/dx -type d -name public -path '*release*' | head -1)"; \
    mkdir -p /out; cp "$SRV" /out/server; cp -r "$PUB" /out/public

# ---- runtime stage ---------------------------------------------------------
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=build /out/server /app/server
COPY --from=build /out/public /app/public

# The Dioxus server reads the port from the environment; Fly sets PORT.
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# DATABASE_URL, SUPABASE_JWT_SECRET, etc. are injected as Fly secrets at runtime.
CMD ["/app/server"]
