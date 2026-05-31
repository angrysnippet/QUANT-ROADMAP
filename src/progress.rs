//! Progress overview — stat cards, the journey bar, the worlds grid and the
//! track breakdown. Ported from `renderSummary` / `renderTracks` in spec.html,
//! but **read-only**: progress is earned, not set. Every percentage is derived
//! from `completed_days` (`current_day - 1`) via `roadmap::topic_progress`, so
//! there are no sliders — the only way to move these bars is to complete days
//! in the Today view.

use std::collections::HashSet;

use dioxus::prelude::*;

use crate::calendar::make_ring;
use crate::roadmap::{
    active_tracks, all_topics, overall_pct, phase_label, stage_for, topic_has_autolink,
    topic_progress, topics_done, track_pct, TRACKS, WORLDS,
};
use crate::state::AppState;

#[component]
pub fn Progress() -> Element {
    let app = use_context::<Signal<AppState>>();
    // Which track cards are expanded (all collapsed by default, like spec.html).
    let mut open = use_signal(HashSet::<String>::new);

    // Reading `current_day` subscribes us to changes; progress is derived from
    // how many days have been completed.
    let completed_days = app.read().current_day.saturating_sub(1);

    let pct = overall_pct(completed_days);
    let done = topics_done(completed_days);
    let total = all_topics().count();
    let active = active_tracks(completed_days);
    let stage = stage_for(pct);
    let avatar_pos = pct.min(99); // keep the avatar on-track at 100%

    rsx! {
        div { class: "view active",
            div { class: "view-title", "Progress overview" }
            div { class: "view-sub", "{pct}% complete · {done}/{total} topics · {stage.label}" }

            // ── Stat cards ──
            div { class: "cols-4", style: "margin-bottom:1.5rem",
                div { class: "stat-card sc1",
                    div { class: "stat-label", "overall" }
                    div { class: "stat-val", "{pct}%" }
                    div { class: "stat-sub", "of all topics" }
                }
                div { class: "stat-card sc2",
                    div { class: "stat-label", "topics done" }
                    div { class: "stat-val", "{done}" }
                    div { class: "stat-sub", "of {total} total" }
                }
                div { class: "stat-card sc3",
                    div { class: "stat-label", "active tracks" }
                    div { class: "stat-val", "{active}" }
                    div { class: "stat-sub", "of {TRACKS.len()} tracks" }
                }
                div { class: "stat-card sc4",
                    div { class: "stat-label", "current stage" }
                    div { class: "stat-val", style: "font-size:22px", "{stage.emoji}" }
                    div { class: "stat-sub", "{stage.label}" }
                }
            }

            // ── Journey ──
            div { class: "journey-card",
                div { class: "journey-top",
                    span { class: "journey-stage", "{stage.label}" }
                    span { class: "journey-pct-badge", "{pct}%" }
                }
                div { class: "journey-track-wrap",
                    div { class: "journey-track",
                        div { class: "journey-fill", style: "width:{pct}%" }
                        div { class: "journey-avatar", style: "left:{avatar_pos}%", "{stage.emoji}" }
                        Milestone { at: 0, pct }
                        Milestone { at: 25, pct }
                        Milestone { at: 50, pct }
                        Milestone { at: 75, pct }
                        Milestone { at: 100, pct }
                    }
                }
                div { class: "journey-labels",
                    span { "🌱 Seed" }
                    span { "🌿 Plant" }
                    span { "🌳 Tree" }
                    span { "🧠 Thinker" }
                    span { "🚀 Quant" }
                }
                if let Some(reward) = stage.reward {
                    div { class: "reward-banner", "{reward}" }
                }
            }

            // ── Worlds ──
            div { class: "section-label", "Worlds" }
            div { class: "worlds-grid",
                for w in WORLDS {
                    {
                        let unlocked = pct >= w.lo;
                        let wp = if unlocked {
                            ((pct.saturating_sub(w.lo) as f64) / ((w.hi - w.lo) as f64) * 100.0)
                                .round()
                                .min(100.0) as u32
                        } else {
                            0
                        };
                        rsx! {
                            div {
                                key: "{w.name}",
                                class: if unlocked { "world-card unlocked" } else { "world-card locked" },
                                span { class: "world-emoji", "{w.emoji}" }
                                div { class: "world-name", "{w.name}" }
                                div { class: "world-sub", "{w.desc}" }
                                div { class: "world-bar-track",
                                    div {
                                        class: "world-bar-fill",
                                        style: "width:{wp}%;background:{w.color}",
                                    }
                                }
                                div { class: "world-pct",
                                    if unlocked { "{wp}% explored" } else { "locked" }
                                }
                                if !unlocked {
                                    div { class: "world-lock", "🔒" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Track breakdown ──
            div { class: "section-label", "Track breakdown" }
            div {
                for track in TRACKS {
                    {
                        let tid = track.id;
                        let is_open = open.read().contains(tid);
                        let tpct = track_pct(track, completed_days);
                        rsx! {
                            div { key: "{track.id}", class: "track-card",
                                div {
                                    class: "track-header",
                                    onclick: move |_| {
                                        let mut o = open.write();
                                        if !o.remove(tid) {
                                            o.insert(tid.to_string());
                                        }
                                    },
                                    div { class: "track-left",
                                        div {
                                            class: "track-icon",
                                            style: "background:{track.bg_color};color:{track.color}",
                                            "{track.icon}"
                                        }
                                        div {
                                            div { class: "track-name", "{track.name}" }
                                            div { class: "track-meta",
                                                "{track.topics.len()} topics · "
                                                span {
                                                    style: "color:var(--accent5);font-size:10px",
                                                    "⚡ auto-linked"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "track-right",
                                        div { class: "track-ring", {make_ring(tpct as f64, track.color, 40.0)} }
                                        span {
                                            class: if is_open { "chevron-icon open" } else { "chevron-icon" },
                                            "▾"
                                        }
                                    }
                                }
                                div {
                                    class: if is_open { "track-body open" } else { "track-body" },
                                    div { class: "topics-grid",
                                        for topic in track.topics {
                                            {
                                                let val = topic_progress(topic.id, completed_days);
                                                let auto = topic_has_autolink(topic.id);
                                                rsx! {
                                                    div { key: "{topic.id}", class: "topic-row",
                                                        div { class: "topic-top",
                                                            span { class: "topic-name", "{topic.name}" }
                                                            span {
                                                                class: if auto { "topic-auto-badge visible" } else { "topic-auto-badge" },
                                                                title: "auto-linked from daily blocks",
                                                                "⚡ auto"
                                                            }
                                                            span { class: "phase-pill {topic.phase}", "{phase_label(topic.phase)}" }
                                                        }
                                                        div { class: "topic-bottom",
                                                            div { class: "world-bar-track", style: "flex:1",
                                                                div {
                                                                    class: "world-bar-fill",
                                                                    style: "width:{val}%;background:{track.color}",
                                                                }
                                                            }
                                                            span { class: "topic-pct", "{val}%" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A single milestone dot on the journey track, lit once `pct` reaches `at`.
#[component]
fn Milestone(at: u32, pct: u32) -> Element {
    let active = pct >= at;
    rsx! {
        div { class: "journey-milestone", style: "left:{at}%",
            div { class: if active { "milestone-dot active" } else { "milestone-dot" } }
        }
    }
}
