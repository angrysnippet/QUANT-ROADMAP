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
use crate::roadmap::{WorldState, DAYS, MAP_WORLDS};
use crate::state::AppState;
use crate::Route;

/// Completed days from the server summary when signed in, else local state.
fn completed_days(me: &Signal<Option<crate::api::MeSummary>>, app: &Signal<AppState>) -> u32 {
    me.read()
        .as_ref()
        .map(|m| m.completed_days.max(0) as u32)
        .unwrap_or_else(|| app.read().current_day.saturating_sub(1))
}

#[component]
pub fn WorldMap() -> Element {
    let app = use_context::<Signal<AppState>>();
    let me = use_me();
    let completed = completed_days(&me, &app);
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
                        let wn = w.n;
                        rsx! {
                            div {
                                key: "{w.n}",
                                class: "{state_cls}",
                                onclick: move |_| {
                                    if !locked {
                                        navigator().push(Route::Chapter { world: wn });
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

/// Chapter Zoom (route `/map/:world`) - Phase 2, Slice 2. One world's days as
/// nodes on a winding path, with a boss-project banner. A day-node click opens
/// the existing Strategy view for now (Day Detail replaces this in Slice 3).
#[component]
pub fn Chapter(world: u32) -> Element {
    let app = use_context::<Signal<AppState>>();
    let me = use_me();
    let completed = completed_days(&me, &app);
    let current_day = completed + 1;

    let Some(w) = MAP_WORLDS.iter().find(|w| w.n == world) else {
        return rsx! {
            div { class: "view active",
                div { class: "view-title", "Unknown world" }
                Link { to: Route::WorldMap {}, class: "cal-sched-open-btn", "← World Map" }
            }
        };
    };

    let world_done = w.done(completed);
    let world_total = w.days();
    let boss_cleared = world_done >= world_total;
    let boss_remaining = world_total.saturating_sub(world_done);

    rsx! {
        div { class: "view active",
            div { class: "strat-crumb",
                span {
                    class: "strat-crumb-link",
                    onclick: move |_| { navigator().push(Route::WorldMap {}); },
                    "← World Map"
                }
                span { class: "strat-crumb-sep", "/" }
                span { class: "strat-crumb-current", "World {w.n}: {w.name}" }
            }
            div { class: "view-sub", "Days {w.lo}-{w.hi} · {world_done}/{world_total} complete" }

            div { class: "chapter-path", style: "--world-color:{w.color}",
                for day in w.lo..=w.hi {
                    {
                        let cls = if day < current_day {
                            "ch-node done"
                        } else if day == current_day {
                            "ch-node current"
                        } else {
                            "ch-node locked"
                        };
                        let title = DAYS.iter().find(|d| d.id as u32 == day).map(|d| d.title).unwrap_or("");
                        let locked = day > current_day;
                        rsx! {
                            div {
                                key: "{day}",
                                class: "{cls}",
                                title: "{title}",
                                onclick: move |_| {
                                    if !locked {
                                        navigator().push(Route::Strategy {});
                                    }
                                },
                                "{day}"
                            }
                        }
                    }
                }
            }

            div { class: "boss-banner", style: "border-color:{w.color}55",
                span { class: "boss-emoji", "👑" }
                if boss_cleared {
                    span { "Boss Project cleared — {w.name} complete!" }
                } else {
                    span { "Boss Project — {boss_remaining} days to unlock" }
                }
            }
        }
    }
}
