use dioxus::prelude::*;

mod state;
mod roadmap;
mod landing;
mod layout;
mod progress;
mod runner;
mod today;
mod calendar;
mod strategy;
mod practice;
mod journal;
mod planner;
mod auth;
mod api;
mod sync;
mod bank;
// Server functions (Phase 1). The fullstack feature compiles the function
// signatures + client stubs; the server feature additionally compiles the
// server-only `server` module (DB + auth) that the bodies call into.
#[cfg(feature = "fullstack")]
mod server_fns;
#[cfg(feature = "server")]
mod server;

use landing::Landing;
use layout::AppShell;
use progress::Progress;
use today::Today;
use calendar::Calendar;
use strategy::Strategy;
use practice::Practice;
use journal::Journal;
use planner::Planner;
use auth::Login;
use bank::Bank;
use state::use_app_state;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Landing {},
    #[route("/login")]
    Login {},
    #[layout(AppShell)]
        #[route("/progress")]
        Progress {},
        #[route("/today")]
        Today {},
        #[route("/calendar")]
        Calendar {},
        #[route("/strategy")]
        Strategy {},
        #[route("/practice")]
        Practice {},
        #[route("/bank")]
        Bank {},
        #[route("/journal")]
        Journal {},
        #[route("/planner")]
        Planner {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

/// Phase 0 fullstack probe. Renders a hidden component that exercises one
/// hello-world server function under the `fullstack` feature; a no-op empty
/// node otherwise, so the default `web` build emits identical output.
#[cfg(feature = "fullstack")]
fn hello_probe() -> Element {
    rsx! { server_fns::HelloProbe {} }
}

#[cfg(not(feature = "fullstack"))]
fn hello_probe() -> Element {
    rsx! {}
}

#[component]
fn App() -> Element {
    // Provide shared state + register localStorage autosave.
    let app = use_app_state();

    // Server-auth token (None = signed out), shared with every page via context.
    // Hydrated from the persisted Supabase session on mount.
    let mut token = use_context_provider(|| Signal::new(Option::<String>::None));
    use_effect(move || {
        spawn(async move {
            if let Some(t) = auth::current_token().await {
                token.set(Some(t));
            }
        });
    });

    // Apply the persisted theme to <html data-theme> after mount and on change.
    use_effect(move || {
        let theme = app.read().theme.clone();
        let _ = document::eval(&format!(
            "document.documentElement.setAttribute('data-theme','{theme}')"
        ));
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=Sora:wght@400;500;600&display=swap",
        }
        Router::<Route> {}
        {hello_probe()}
    }
}
