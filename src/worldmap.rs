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

const BG: Asset = asset!("/assets/background/image.png");

const W1: Asset = asset!("/assets/worlds/world1.png");
const W2: Asset = asset!("/assets/worlds/world2.png");
const W3: Asset = asset!("/assets/worlds/world3.png");
const W4: Asset = asset!("/assets/worlds/world4.png");
const W5: Asset = asset!("/assets/worlds/world5.png");
const W6: Asset = asset!("/assets/worlds/world6.png");
const W7: Asset = asset!("/assets/worlds/world7.png");
const W8: Asset = asset!("/assets/worlds/world8.png");
const W9: Asset = asset!("/assets/worlds/world9.png");

/// Castle art for world `n` (1-9).
fn castle(n: u32) -> Asset {
    match n {
        1 => W1,
        2 => W2,
        3 => W3,
        4 => W4,
        5 => W5,
        6 => W6,
        7 => W7,
        8 => W8,
        _ => W9,
    }
}

/// (top%, left%) of each world's castle along the painted path in the background
/// image (world 1..9, top -> bottom). Tweak these to line the castles up with
/// the path bends in `assets/background/image.png`.
const WORLD_POS: [(f32, f32); 9] = [
    (7.0, 46.0),
    (15.0, 62.0),
    (24.0, 39.0),
    (37.0, 49.0),
    (46.0, 61.0),
    (58.0, 35.0),
    (68.0, 58.0),
    (80.0, 36.0),
    (90.0, 46.0),
];

#[component]
pub fn WorldMap() -> Element {
    let app = use_context::<Signal<AppState>>();
    let me = use_me();
    let completed = completed_days(&me, &app);
    let current_day = completed + 1;

    rsx! {
        div { class: "view active wm-view",
            div { class: "view-title", "World Map" }
            div { class: "view-sub", "Your quant journey — Day {current_day} of 548" }

            div { class: "worldmap-scene",
                img { class: "wm-bg", src: BG, alt: "" }
                for (i, w) in MAP_WORLDS.iter().enumerate() {
                    {
                        let (top, left) = WORLD_POS[i];
                        let state = w.state(current_day);
                        let done = w.done(completed);
                        let total = w.days();
                        let pct = done * 100 / total;
                        let cls = match state {
                            WorldState::Completed => "wm-marker done",
                            WorldState::Current => "wm-marker current",
                            WorldState::Locked => "wm-marker locked",
                        };
                        let locked = state == WorldState::Locked;
                        let wn = w.n;
                        rsx! {
                            div {
                                key: "{w.n}",
                                class: "{cls}",
                                style: "top:{top}%;left:{left}%",
                                title: "{w.n}. {w.name} - {w.desc}",
                                onclick: move |_| {
                                    if !locked {
                                        navigator().push(Route::Chapter { world: wn });
                                    }
                                },
                                img { class: "wm-castle", src: castle(w.n), alt: "{w.name}" }
                                if state == WorldState::Completed {
                                    span { class: "wm-check", "✓" }
                                }
                                if locked {
                                    span { class: "wm-lock", "🔒" }
                                }
                                div { class: "wm-label", style: "border-color:{w.color}",
                                    if state == WorldState::Current {
                                        span { class: "wm-here", "YOU ARE HERE" }
                                    }
                                    div { class: "wm-name", "{w.emoji} {w.name}" }
                                    div { class: "wm-meta", "{done}/{total} · {pct}%" }
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
