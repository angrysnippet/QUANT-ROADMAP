//! Phase 0 fullstack proof: a single hello-world server function plus a tiny
//! probe component that calls it.
//!
//! This whole module is compiled only under the `fullstack` feature, so the
//! default `web` static build (and the GitHub Pages deploy) is byte-for-byte
//! unchanged. Per AGENTS.md (authoritative Dioxus 0.7 reference): the `#[get]`
//! macro turns this into an API endpoint on the server and an HTTP-calling stub
//! on the client; `use_server_future` runs it on the server during SSR and
//! ships the result to the client for a hydration-safe first render.
//!
//! There is intentionally no real logic here. It exists only to prove the
//! server feature can be enabled and exercised end to end. Real server
//! functions (auth, XP, grading) arrive in Phase 1.

use dioxus::prelude::*;

/// Returns a greeting from the server. Server-only body; the client gets a
/// generated stub that calls the `/api/hello` endpoint.
#[get("/api/hello")]
pub async fn hello_server() -> Result<String, ServerFnError> {
    Ok("Hello from the Quant Arena server".to_string())
}

/// A minimal, visually-hidden probe that calls `hello_server` once. Rendered
/// from the app root only under the `fullstack` feature so we can confirm the
/// round trip works during `dx serve` without touching any real UI.
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
            // Hidden: this is a build/round-trip proof, not real UI.
            style: "display:none",
            "data-hello-probe": "{text}",
        }
    }
}
