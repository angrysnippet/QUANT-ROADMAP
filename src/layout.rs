//! The app shell: top bar + sidebar nav, with routed views rendered in the
//! content outlet. Markup/classes mirror `spec.html`.

use crate::state::AppState;
use crate::Route;
use chrono::Local;
use dioxus::prelude::*;

#[component]
pub fn AppShell() -> Element {
    let today = Local::now().format("%a, %d %b %Y").to_string();
    let mut app = use_context::<Signal<AppState>>();
    let is_dark = app.read().theme != "light";

    rsx! {
        div { class: "topbar",
            div { class: "topbar-left",
                div { class: "topbar-logo", "Quant ", span { "roadmap" } }
                div { class: "topbar-date", "{today}" }
            }
            div { class: "topbar-right",
                button {
                    class: "theme-toggle",
                    title: if is_dark { "Switch to light mode" } else { "Switch to dark mode" },
                    onclick: move |_| {
                        let mut st = app.write();
                        st.theme = if st.theme == "light" { "dark".to_string() } else { "light".to_string() };
                    },
                    if is_dark { "☀️" } else { "🌙" }
                }
                span { class: "autosave-label", "saved to ", span { "this browser" } }
            }
        }

        div { class: "shell",
            nav { class: "sidebar",
                div { class: "sidebar-section-label", "Views" }
                NavItem { to: Route::Progress {}, icon: "🗺", label: "Progress" }
                NavItem { to: Route::Today {}, icon: "📋", label: "Today" }
                NavItem { to: Route::Calendar {}, icon: "🗓", label: "Calendar" }
                NavItem { to: Route::Strategy {}, icon: "📅", label: "Strategy" }
                NavItem { to: Route::Practice {}, icon: "⌨️", label: "Practice" }
                NavItem { to: Route::Journal {}, icon: "📓", label: "Journal" }

                div { class: "sidebar-divider" }
                div { class: "sidebar-section-label", "Phases" }
                span { class: "phase-chip pc1", "🌱 Phase 1 — Foundation" }
                span { class: "phase-chip pc2", "⚙️ Phase 2 — Builder" }
                span { class: "phase-chip pc3", "🧠 Phase 3 — Quant Mind" }
                span { class: "phase-chip pc4", "🚀 Phase 4 — Trading" }

                div { class: "sidebar-bottom",
                    div { class: "sidebar-section-label", "Legend" }
                    div { style: "padding:0 1rem;display:flex;flex-direction:column;gap:5px",
                        LegendRow { color: "var(--accent)", label: "Phase 1" }
                        LegendRow { color: "var(--accent2)", label: "Phase 2" }
                        LegendRow { color: "var(--accent4)", label: "Phase 3" }
                        LegendRow { color: "var(--accent3)", label: "Phase 4" }
                    }
                }
            }

            div { class: "content",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn NavItem(to: Route, icon: String, label: String) -> Element {
    rsx! {
        Link {
            to,
            class: "nav-item",
            active_class: "active",
            span { class: "nav-item-icon", "{icon}" }
            " {label}"
        }
    }
}

#[component]
fn LegendRow(color: String, label: String) -> Element {
    rsx! {
        div {
            style: "display:flex;align-items:center;gap:7px;font-size:11px;color:var(--text3);font-family:var(--mono)",
            div { style: "width:8px;height:8px;border-radius:50%;background:{color}" }
            "{label}"
        }
    }
}
