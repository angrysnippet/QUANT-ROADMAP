//! The app shell: top bar + sidebar nav, with routed views rendered in the
//! content outlet. Markup/classes mirror `spec.html`.

use crate::auth::use_me;
use crate::roadmap::overall_pct;
use crate::state::AppState;
use crate::Route;
use chrono::Local;
use dioxus::prelude::*;

#[component]
pub fn AppShell() -> Element {
    let today = Local::now().format("%a, %d %b %Y").to_string();
    let mut app = use_context::<Signal<AppState>>();
    let is_dark = app.read().theme != "light";

    // Server-authoritative summary when signed in; otherwise derive what we can
    // from local state (XP/streak are server-only, shown as "—" offline).
    let me = use_me();
    let m = me.read().clone();
    let completed_local = app.read().current_day.saturating_sub(1);
    let pct = m
        .as_ref()
        .map(|x| x.progress_pct)
        .unwrap_or_else(|| overall_pct(completed_local) as f64);
    let pct_display = format!("{pct:.0}");
    let completed_days = m.as_ref().map(|x| x.completed_days).unwrap_or(completed_local as i64);
    let current_day = m.as_ref().map(|x| x.current_day).unwrap_or(app.read().current_day as i64);
    let xp_display = m.as_ref().map(|x| x.xp.to_string()).unwrap_or_else(|| "—".to_string());
    let level_display = m.as_ref().map(|x| x.level.to_string()).unwrap_or_else(|| "—".to_string());
    let streak_display = m.as_ref().map(|x| x.streak_current.to_string()).unwrap_or_else(|| "—".to_string());
    let est_display = m.as_ref().and_then(|x| x.est_completion.clone()).unwrap_or_else(|| "—".to_string());

    rsx! {
        div { class: "topbar",
            div { class: "topbar-left",
                div { class: "topbar-logo", "Quant ", span { "roadmap" } }
                div { class: "topbar-date", "{today}" }
            }
            div { class: "topbar-right",
                div { class: "topbar-chips",
                    span { class: "tb-chip", "⭐ ", span { "{xp_display}" } }
                    span { class: "tb-chip", "🔥 ", span { "{streak_display}" } }
                    span { class: "tb-chip", "📊 ", span { "{pct_display}%" } }
                }
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
                NavItem { to: Route::Planner {}, icon: "✦", label: "AI Planner" }

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

            aside { class: "rightrail",
                div { class: "rail-card",
                    div { class: "rail-title", "Journey Stats" }
                    RailStat { label: "Progress", value: "{pct_display}%" }
                    RailStat { label: "Days done", value: "{completed_days}/548" }
                    RailStat { label: "XP", value: "{xp_display}" }
                    RailStat { label: "Level", value: "{level_display}" }
                    RailStat { label: "Current day", value: "{current_day}" }
                    RailStat { label: "Est. finish", value: "{est_display}" }
                }
            }
        }
    }
}

#[component]
fn RailStat(label: String, value: String) -> Element {
    rsx! {
        div { class: "rail-stat",
            span { class: "rail-stat-label", "{label}" }
            span { class: "rail-stat-value", "{value}" }
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
