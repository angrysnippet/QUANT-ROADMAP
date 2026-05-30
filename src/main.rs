use dioxus::prelude::*;

mod state;
mod layout;
mod progress;
mod today;
mod calendar;
mod strategy;
mod practice;
mod journal;

use layout::AppShell;
use progress::Progress;
use today::Today;
use calendar::Calendar;
use strategy::Strategy;
use practice::Practice;
use journal::Journal;
use state::use_app_state;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Progress {},
        #[route("/today")]
        Today {},
        #[route("/calendar")]
        Calendar {},
        #[route("/strategy")]
        Strategy {},
        #[route("/practice")]
        Practice {},
        #[route("/journal")]
        Journal {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide shared state + register localStorage autosave.
    use_app_state();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=Sora:wght@400;500;600&display=swap",
        }
        Router::<Route> {}
    }
}
