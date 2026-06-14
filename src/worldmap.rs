//! World Map (route `/map`) - Phase 2, Slice 1.
//!
//! The 9 worlds (CLAUDE.md Section 5) laid out on a glowing dashed path, each
//! with per-world progress and node state (completed / current / locked) driven
//! by the user's progress: `me_summary.completed_days` when signed in, else the
//! local `current_day`. World art is a gradient placeholder until the user
//! supplies illustrations (Section 5). Clicking an unlocked world opens the
//! existing Strategy view for now; the dedicated Chapter Zoom replaces that
//! target in Slice 2. Degrades to a vertical list on narrow screens.

use dioxus::prelude::*;

use crate::auth::use_me;
use crate::roadmap::{WorldState, MAP_WORLDS};
use crate::state::AppState;
use crate::Route;

#[component]
pub fn WorldMap() -> Element {
    let app = use_context::<Signal<AppState>>();
    let me = use_me();

    // Completed days: server summary when signed in, else local (current_day-1).
    let completed = me
        .read()
        .as_ref()
        .map(|m| m.completed_days.max(0) as u32)
        .unwrap_or_else(|| app.read().current_day.saturating_sub(1));
    let current_day = completed + 1;

    rsx! {
        div { class: "view active",
            div { class: "view-title", "World Map" }
            div { class: "view-sub", "Your journey across the 9 worlds — Day {current_day} of 548" }

            div { class: "worldmap",
                for w in MAP_WORLDS {
                    {
                        let state = w.state(current_day);
                        let done = w.done(completed);
                        let total = w.days();
                        let pct = done * 100 / total;
                        let (state_cls, badge) = match state {
                            WorldState::Completed => ("wm-node done", "✓"),
                            WorldState::Current => ("wm-node current", ""),
                            WorldState::Locked => ("wm-node locked", "🔒"),
                        };
                        let locked = state == WorldState::Locked;
                        rsx! {
                            div {
                                key: "{w.n}",
                                class: "{state_cls}",
                                onclick: move |_| {
                                    if !locked {
                                        navigator().push(Route::Strategy {});
                                    }
                                },
                                div {
                                    class: "wm-dot",
                                    style: "background:linear-gradient(135deg,{w.color}33,{w.color}11);border-color:{w.color};color:{w.color}",
                                    span { class: "wm-emoji", "{w.emoji}" }
                                    if !badge.is_empty() {
                                        span { class: "wm-badge", "{badge}" }
                                    }
                                }
                                div { class: "wm-card", style: "border-color:{w.color}55",
                                    if state == WorldState::Current {
                                        span { class: "wm-here", "YOU ARE HERE" }
                                    }
                                    div { class: "wm-name", "{w.n}. {w.name}" }
                                    div { class: "wm-range", "Days {w.lo}-{w.hi}" }
                                    div { class: "wm-desc", "{w.desc}" }
                                    div { class: "wm-bar-track",
                                        div { class: "wm-bar-fill", style: "width:{pct}%;background:{w.color}" }
                                    }
                                    div { class: "wm-meta", "{done}/{total} days · {pct}%" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
