//! Practice — the day's problems with self-marked "solved" tracking. Shows the
//! problems mapped to the current strategy day (`practice_problems_for_day`),
//! grouped by category. Marking every problem solved satisfies the practice
//! gate in the Today view. Solving is self-reported (no compiler/answer-check
//! in this pass); a worked solution can be revealed where one exists.

use std::collections::HashSet;

use dioxus::prelude::*;

use crate::roadmap::{practice_problems_for_day, PracticeCategory, PracticeProblem};
use crate::state::AppState;

const CATEGORIES: [PracticeCategory; 4] = [
    PracticeCategory::Code,
    PracticeCategory::Math,
    PracticeCategory::Quant,
    PracticeCategory::Linux,
];

#[component]
pub fn Practice() -> Element {
    let mut app = use_context::<Signal<AppState>>();
    let mut revealed = use_signal(HashSet::<String>::new);

    let day = app.read().current_day;
    let solved = app.read().solved.clone();

    let problems: Vec<&'static PracticeProblem> = practice_problems_for_day(day).collect();
    let total = problems.len();
    let solved_count = problems
        .iter()
        .filter(|p| solved.get(p.id).copied().unwrap_or(false))
        .count();

    rsx! {
        div { class: "view active",
            div { class: "prac-toolbar",
                div {
                    div { class: "view-title", "Practice · Day {day}" }
                    div { class: "view-sub", "Solve every problem to unlock the next day. Mark each one as you finish it." }
                }
                span { class: "prac-solved-count", "{solved_count}/{total} solved" }
            }

            if problems.is_empty() {
                div { class: "prac-empty", "No practice problems for this day." }
            }

            for cat in CATEGORIES {
                {
                    let in_cat: Vec<&'static PracticeProblem> =
                        problems.iter().copied().filter(|p| p.category == cat).collect();
                    if in_cat.is_empty() {
                        rsx! {}
                    } else {
                        rsx! {
                            div { class: "section-label", "{cat.label()}" }
                            div { class: "quiz-list",
                                for p in in_cat {
                                    {
                                        let is_solved = solved.get(p.id).copied().unwrap_or(false);
                                        let is_open = revealed.read().contains(p.id);
                                        let id_solve = p.id;
                                        let id_reveal = p.id;
                                        let has_sol = !p.solution.is_empty();
                                        rsx! {
                                            div {
                                                key: "{p.id}",
                                                class: if is_solved { "quiz-card solved" } else { "quiz-card" },
                                                div { class: "quiz-card-head",
                                                    div { class: if is_solved { "quiz-check solved" } else { "quiz-check" } }
                                                    div { class: "quiz-prompt",
                                                        strong { "{p.title}" }
                                                        br {}
                                                        "{p.prompt}"
                                                    }
                                                    div { class: "quiz-tags",
                                                        span { class: "quiz-topic", "{p.topic}" }
                                                        span { class: "prac-diff diff-{p.diff}", "{p.diff}" }
                                                    }
                                                }
                                                div { class: "quiz-answer-row",
                                                    button {
                                                        class: "quiz-check-btn",
                                                        onclick: move |_| {
                                                            let mut st = app.write();
                                                            let cur = st.solved.get(id_solve).copied().unwrap_or(false);
                                                            st.solved.insert(id_solve.to_string(), !cur);
                                                        },
                                                        if is_solved { "✓ Solved — undo" } else { "Mark solved" }
                                                    }
                                                    if has_sol {
                                                        button {
                                                            class: "quiz-reveal-btn",
                                                            onclick: move |_| {
                                                                let mut r = revealed.write();
                                                                if !r.remove(id_reveal) {
                                                                    r.insert(id_reveal.to_string());
                                                                }
                                                            },
                                                            if is_open { "Hide solution" } else { "Reveal solution" }
                                                        }
                                                    }
                                                }
                                                if has_sol {
                                                    div {
                                                        class: if is_open { "quiz-solution show" } else { "quiz-solution" },
                                                        span { class: "sol-label", "Solution" }
                                                        div { dangerous_inner_html: "{p.solution}" }
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
