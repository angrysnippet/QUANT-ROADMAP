//! Server functions: the typed client <-> server API, plus the shared
//! request/response structs that compile on both sides.
//!
//! Compiled under the `fullstack` feature. The `#[get]`/`#[post]` macros emit a
//! real implementation on the server (where the `server` module's DB + auth are
//! available) and an HTTP-calling stub on the client. Authenticated endpoints
//! take the Supabase access `token` as an explicit argument and verify it
//! server-side (CLAUDE.md 4.1); the WASM client never holds DB or service keys.

use dioxus::prelude::*;

use crate::api::MeSummary;

/// Phase 0 hello-world server function (kept as a liveness probe).
#[get("/api/hello")]
pub async fn hello_server() -> Result<String, ServerFnError> {
    Ok("Hello from the Quant Arena server".to_string())
}

/// Create the caller's profile (and streak row) on first login if absent, then
/// return the fresh summary. `handle` is validated/sanitised server-side.
#[post("/api/ensure_profile")]
pub async fn ensure_profile(
    token: String,
    handle: String,
    timezone: String,
) -> Result<MeSummary, ServerFnError> {
    let uid = crate::server::authed(&token).map_err(ServerFnError::new)?;
    crate::server::ensure_profile(uid, &handle, &timezone)
        .await
        .map_err(ServerFnError::new)?;
    crate::server::me_summary(uid)
        .await
        .map_err(ServerFnError::new)
}

/// Mark `day` complete for the caller (idempotent +35 XP) and return the fresh
/// summary. Replaces the client-side `current_day += 1` in `today.rs`.
#[post("/api/complete_day")]
pub async fn complete_day(token: String, day: i64) -> Result<MeSummary, ServerFnError> {
    let uid = crate::server::authed(&token).map_err(ServerFnError::new)?;
    crate::server::complete_day(uid, day)
        .await
        .map_err(ServerFnError::new)?;
    crate::server::me_summary(uid)
        .await
        .map_err(ServerFnError::new)
}

/// Return the caller's current summary.
#[post("/api/me_summary")]
pub async fn me_summary(token: String) -> Result<MeSummary, ServerFnError> {
    let uid = crate::server::authed(&token).map_err(ServerFnError::new)?;
    crate::server::me_summary(uid)
        .await
        .map_err(ServerFnError::new)
}

/// One-shot import of the legacy browser `qrt_state`. `current_day` is the
/// legacy "current day" (completed = current_day - 1). Replay-proof: the server
/// rejects a second call once the profile is marked migrated.
#[post("/api/import_local_progress")]
pub async fn import_local_progress(
    token: String,
    current_day: i64,
) -> Result<MeSummary, ServerFnError> {
    let uid = crate::server::authed(&token).map_err(ServerFnError::new)?;
    crate::server::import_local_progress(uid, current_day)
        .await
        .map_err(ServerFnError::new)?;
    crate::server::me_summary(uid)
        .await
        .map_err(ServerFnError::new)
}

/// A minimal, visually-hidden probe that calls `hello_server` once (Phase 0
/// liveness check). Rendered from the app root only under `fullstack`.
#[component]
pub fn HelloProbe() -> Element {
    let greeting = use_server_future(hello_server)?;

    let text = match greeting() {
        Some(Ok(msg)) => msg,
        Some(Err(_)) => "server error".to_string(),
        None => "loading".to_string(),
    };

    rsx! {
        div {
            style: "display:none",
            "data-hello-probe": "{text}",
        }
    }
}
