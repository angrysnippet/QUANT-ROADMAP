//! Reflection journal — five daily prompts for the current strategy day. Each
//! answer is stored in `journal[current_day][jN]`; filling all five satisfies
//! the journal gate in the Today view. Prompts/markup mirror spec.html.

use dioxus::prelude::*;

use crate::state::AppState;

/// (field key, prompt label, full-width?) for the five prompts, in order.
const PROMPTS: [(&str, &str, bool); 5] = [
    ("j1", "01 — What problem confused me most?", false),
    ("j2", "02 — What clicked today?", false),
    ("j3", "03 — What bug wasted most time?", false),
    ("j4", "04 — Which concept is still weak?", false),
    ("j5", "05 — Did I panic while stuck? How did I handle it?", true),
];

#[component]
pub fn Journal() -> Element {
    let mut app = use_context::<Signal<AppState>>();

    let day = app.read().current_day;
    let day_key = day.to_string();
    let entries = app.read().journal.get(&day_key).cloned().unwrap_or_default();

    let filled = PROMPTS
        .iter()
        .filter(|(k, _, _)| entries.get(*k).is_some_and(|v| !v.trim().is_empty()))
        .count();

    rsx! {
        div { class: "view active",
            div { class: "view-title", "Reflection journal · Day {day}" }
            div { class: "view-sub", "{filled}/5 prompts filled — complete all five to satisfy the journal gate." }

            div { class: "journal-layout",
                for (key, label, full) in PROMPTS {
                    {
                        let val = entries.get(key).cloned().unwrap_or_default();
                        let day_key = day_key.clone();
                        rsx! {
                            div {
                                key: "{key}",
                                class: "journal-card",
                                style: if full { "grid-column:1/-1" } else { "" },
                                div { class: "journal-q", "{label}" }
                                textarea {
                                    class: "journal-textarea",
                                    style: if full { "min-height:80px" } else { "" },
                                    placeholder: "Write here...",
                                    value: "{val}",
                                    oninput: move |e| {
                                        let mut st = app.write();
                                        let day_map = st.journal.entry(day_key.clone()).or_default();
                                        day_map.insert(key.to_string(), e.value());
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
