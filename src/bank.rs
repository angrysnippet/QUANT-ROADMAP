//! Problem Bank (route `/bank`) - a minimal surface that exercises the
//! server-graded problem bank: it fetches SEALED problems (no answers) via
//! `list_problems`, lets the user answer, and submits each to `submit_problem`
//! for server-side grading. The solution is revealed only in the graded result.
//! Requires sign-in; the legacy Practice page is left untouched.

use dioxus::prelude::*;

use crate::api::{BankProblem, GradeResult};
use crate::auth::{use_me, use_token};
use crate::sync;
use crate::Route;

#[component]
pub fn Bank() -> Element {
    let token = use_token();
    let mut problems = use_signal(Vec::<BankProblem>::new);
    let mut status = use_signal(String::new);

    // Load sealed problems whenever we have a token.
    use_effect(move || {
        if let Some(tok) = token() {
            spawn(async move {
                match sync::list_problems(tok, None, None).await {
                    Ok(ps) => problems.set(ps),
                    Err(e) => status.set(e),
                }
            });
        }
    });

    if token().is_none() {
        return rsx! {
            div { class: "view active",
                div { class: "view-title", "Problem Bank" }
                div { class: "view-sub", "Server-graded practice problems." }
                div { class: "prac-empty", "Sign in to use the problem bank." }
                Link { to: Route::Login {}, class: "cal-sched-open-btn", "Sign in ›" }
            }
        };
    }

    let plist = problems();
    let n = plist.len();
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Problem Bank" }
            div { class: "view-sub", "Answers are graded on the server. {n} problem(s) available." }
            if !status().is_empty() {
                div { style: "color:#ff6b6b;font-size:13px", "{status}" }
            }
            for p in plist {
                ProblemCard { key: "{p.id}", p: p.clone(), token }
            }
        }
    }
}

#[component]
fn ProblemCard(p: BankProblem, token: Signal<Option<String>>) -> Element {
    let mut answer = use_signal(String::new);
    let mut result = use_signal(|| None::<GradeResult>);
    let mut err = use_signal(String::new);
    let mut me = use_me();

    let problem_id = p.id.clone();
    let submit = move |_| {
        let ans = answer.peek().clone();
        let pid = problem_id.clone();
        if ans.trim().is_empty() {
            err.set("Enter an answer first.".to_string());
            return;
        }
        err.set(String::new());
        if let Some(tok) = token() {
            spawn(async move {
                match sync::submit_problem(tok, pid, ans).await {
                    Ok(r) => {
                        // Update the shared summary so the shell reflects new XP.
                        me.set(Some(r.summary.clone()));
                        result.set(Some(r));
                    }
                    Err(e) => err.set(e),
                }
            });
        }
    };

    rsx! {
        div { class: "strat-success", style: "margin-bottom:1rem",
            div { class: "strat-success-title", "[{p.track} · difficulty {p.difficulty}] {p.statement}" }

            if p.kind == "mcq" {
                for (i, choice) in p.choices.iter().enumerate() {
                    {
                        let idx = i.to_string();
                        let selected = answer() == idx;
                        rsx! {
                            button {
                                key: "{i}",
                                class: if selected { "prac-lang-btn active" } else { "prac-lang-btn" },
                                style: "display:block;margin:.25rem 0;text-align:left;width:100%",
                                onclick: move |_| answer.set(idx.clone()),
                                "{choice}"
                            }
                        }
                    }
                }
            } else {
                input {
                    class: "modal-answer-input",
                    r#type: "text",
                    placeholder: if p.kind == "numeric" { "Your numeric answer" } else { "Your program's output" },
                    value: "{answer}",
                    oninput: move |e| answer.set(e.value()),
                }
            }

            button {
                class: "strat-practice-btn",
                style: "margin-top:.5rem",
                onclick: submit,
                "Submit"
            }

            if !err().is_empty() {
                div { style: "color:#ff6b6b;font-size:12px;margin-top:.3rem", "{err}" }
            }

            if let Some(r) = result() {
                div { style: "margin-top:.5rem",
                    if r.correct {
                        div { style: "color:#84cc16;font-weight:600", "✓ Correct — +{r.xp_awarded} XP" }
                    } else {
                        div { style: "color:#ff6b6b;font-weight:600", "✗ Not quite — try again" }
                    }
                    if !r.solution_explanation.is_empty() {
                        div { style: "font-size:12px;color:var(--text3);margin-top:.3rem", "{r.solution_explanation}" }
                    }
                }
            }
        }
    }
}
