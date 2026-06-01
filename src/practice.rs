//! Practice — the day's problems as clickable cards grouped by category
//! (CODE / LINUX / QUANT / MATH). Clicking a card opens a modal detail view
//! (no inline expansion). All inputs start empty; a "Load Solution" button
//! fills in the reference. Code problems run in-browser (Pyodide/JSCPP) and
//! auto-mark solved when output matches the expected sample; everything can
//! also be marked solved manually. Solved state is reflected on the cards.

use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::roadmap::{code_meta, practice_problems_for_day, PracticeCategory, PracticeProblem};
use crate::runner::{normalize_out, run_code, RunResult};
use crate::state::AppState;

// Display order requested for Day 1.
const CATEGORIES: [PracticeCategory; 4] = [
    PracticeCategory::Code,
    PracticeCategory::Linux,
    PracticeCategory::Quant,
    PracticeCategory::Math,
];

/// Truncate a prompt to a short card description.
fn brief(s: &str, n: usize) -> String {
    if s.chars().count() > n {
        let t: String = s.chars().take(n).collect();
        format!("{}…", t.trim_end())
    } else {
        s.to_string()
    }
}

#[component]
pub fn Practice() -> Element {
    let app = use_context::<Signal<AppState>>();
    let selected = use_signal(|| None::<String>); // open problem id (modal)

    // Modal session state (all start empty — no pre-fill).
    let lang = use_signal(|| "cpp".to_string());
    let code_buf = use_signal(HashMap::<String, String>::new); // "{id}_{lang}" -> code
    let input_buf = use_signal(HashMap::<String, String>::new); // id -> stdin / command / answer
    let outputs = use_signal(HashMap::<String, (String, String)>::new); // id -> (text, class)
    let running = use_signal(|| None::<String>);
    let revealed = use_signal(HashSet::<String>::new); // id -> solution shown (non-code)

    let day = app.read().current_day;
    let solved = app.read().solved.clone();

    let problems: Vec<&'static PracticeProblem> = practice_problems_for_day(day).collect();
    let total = problems.len();
    let solved_count = problems
        .iter()
        .filter(|p| solved.get(p.id).copied().unwrap_or(false))
        .count();

    let open = selected();

    rsx! {
        div { class: "view active",
            div { class: "prac-toolbar",
                div {
                    div { class: "view-title", "Practice · Day {day}" }
                    div { class: "view-sub", "Pick a problem to open it. Solve it, or hit Load Solution for the reference." }
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
                            div { class: "prac-grid",
                                for p in in_cat {
                                    {
                                        let is_solved = solved.get(p.id).copied().unwrap_or(false);
                                        let id = p.id;
                                        let mut sel = selected;
                                        rsx! {
                                            div {
                                                key: "{p.id}",
                                                class: if is_solved { "prac-card solved" } else { "prac-card" },
                                                onclick: move |_| sel.set(Some(id.to_string())),
                                                div { class: "prac-card-top",
                                                    span { class: "prac-diff diff-{p.diff}", "{p.diff}" }
                                                    span { class: "quiz-topic", "{p.topic}" }
                                                    if is_solved {
                                                        span { class: "prac-card-check", "✓ solved" }
                                                    }
                                                }
                                                div { class: "prac-card-title", "{p.title}" }
                                                div { class: "prac-card-desc", "{brief(p.prompt, 96)}" }
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

        if let Some(id) = open {
            {
                match practice_problems_for_day(day).find(|p| id == p.id) {
                    Some(p) => {
                        let is_solved = solved.get(p.id).copied().unwrap_or(false);
                        problem_modal(
                            p, is_solved, app, selected, lang, code_buf, input_buf, outputs,
                            running, revealed,
                        )
                    }
                    None => rsx! {},
                }
            }
        }
    }
}

/// The modal detail view for a single problem.
#[allow(clippy::too_many_arguments)]
fn problem_modal(
    p: &'static PracticeProblem,
    is_solved: bool,
    app: Signal<AppState>,
    mut selected: Signal<Option<String>>,
    lang: Signal<String>,
    code_buf: Signal<HashMap<String, String>>,
    input_buf: Signal<HashMap<String, String>>,
    outputs: Signal<HashMap<String, (String, String)>>,
    running: Signal<Option<String>>,
    revealed: Signal<HashSet<String>>,
) -> Element {
    let is_code = p.category == PracticeCategory::Code;

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| selected.set(None),
            div {
                class: "modal-panel",
                // Don't close when interacting inside the panel.
                onclick: move |e| e.stop_propagation(),

                div { class: "modal-header",
                    button {
                        class: "modal-back-btn",
                        onclick: move |_| selected.set(None),
                        "← Back to Day 1"
                    }
                    span { class: "modal-title", "{p.title}" }
                    span { class: "prac-diff diff-{p.diff}", "{p.diff}" }
                }

                div { class: "modal-statement", "{p.prompt}" }

                if is_code {
                    {code_body(p, is_solved, app, lang, code_buf, input_buf, outputs, running)}
                } else {
                    {answer_body(p, is_solved, app, input_buf, revealed)}
                }
            }
        }
    }
}

/// Code problem body: editor + stdin + run + terminal + load-solution.
#[allow(clippy::too_many_arguments)]
fn code_body(
    p: &'static PracticeProblem,
    is_solved: bool,
    mut app: Signal<AppState>,
    mut lang: Signal<String>,
    mut code_buf: Signal<HashMap<String, String>>,
    mut input_buf: Signal<HashMap<String, String>>,
    mut outputs: Signal<HashMap<String, (String, String)>>,
    mut running: Signal<Option<String>>,
) -> Element {
    let id = p.id;
    let lng = lang();
    let buf_key = format!("{id}_{lng}");
    let code = code_buf.read().get(&buf_key).cloned().unwrap_or_default();
    let stdin_val = input_buf.read().get(id).cloned().unwrap_or_default();
    let (out_text, out_cls) = outputs
        .read()
        .get(id)
        .cloned()
        .unwrap_or_else(|| ("Run your code to see the output…".to_string(), "muted".to_string()));
    let is_running = running.read().as_deref() == Some(id);
    let meta = code_meta(id);

    // Captured by the Run handler (reflect current edits).
    let run_code_str = code.clone();
    let run_stdin = stdin_val.clone();
    let run_lng = lng.clone();
    let expected = meta.map(|m| m.sample_out).unwrap_or("");

    rsx! {
        div { class: "prac-editor-bar",
            div { class: "prac-lang-toggle",
                button {
                    class: if lng == "cpp" { "prac-lang-btn active" } else { "prac-lang-btn" },
                    onclick: move |_| lang.set("cpp".to_string()),
                    "C++"
                }
                button {
                    class: if lng == "py" { "prac-lang-btn active" } else { "prac-lang-btn" },
                    onclick: move |_| lang.set("py".to_string()),
                    "Python"
                }
            }
            div { class: "modal-spacer" }
            button {
                class: "prac-run-btn",
                disabled: is_running,
                onclick: move |_| {
                    let id_s = id.to_string();
                    running.set(Some(id_s.clone()));
                    outputs.write().insert(id_s.clone(), ("Running…".to_string(), "muted".to_string()));
                    let lng2 = run_lng.clone();
                    let code2 = run_code_str.clone();
                    let stdin2 = run_stdin.clone();
                    spawn(async move {
                        let res = run_code(&lng2, code2, stdin2).await;
                        let (t, c) = build_output(&res, expected, &id_s, app);
                        outputs.write().insert(id_s.clone(), (t, c));
                        running.set(None);
                    });
                },
                if is_running { "running…" } else { "▶ Run" }
            }
        }

        div { class: "modal-section-label", "Your solution" }
        textarea {
            class: "prac-code",
            spellcheck: "false",
            placeholder: "Write your code here…",
            value: "{code}",
            oninput: move |e| {
                code_buf.write().insert(buf_key.clone(), e.value());
            },
        }
        // Load Solution sits directly below the editor.
        if meta.is_some() {
            button {
                class: "load-sol-btn",
                style: "margin-top:.6rem",
                onclick: move |_| {
                    if let Some(m) = meta {
                        let lng = lang();
                        let starter = if lng == "cpp" { m.starter_cpp } else { m.starter_py };
                        code_buf.write().insert(format!("{id}_{lng}"), starter.to_string());
                        input_buf.write().insert(id.to_string(), m.sample_in.to_string());
                    }
                },
                "⬇ Load Solution"
            }
        }

        div { class: "modal-section-label", "Input (stdin)" }
        textarea {
            class: "prac-stdin",
            spellcheck: "false",
            placeholder: "Type the program input here…",
            value: "{stdin_val}",
            oninput: move |e| {
                input_buf.write().insert(id.to_string(), e.value());
            },
        }

        div { class: "modal-section-label", "Terminal output" }
        div { class: "prac-output {out_cls}", "{out_text}" }

        div { class: "modal-actions",
            button {
                class: "quiz-check-btn",
                onclick: move |_| {
                    let cur = app.read().solved.get(id).copied().unwrap_or(false);
                    app.write().solved.insert(id.to_string(), !cur);
                },
                if is_solved { "✓ Solved — undo" } else { "Mark solved" }
            }
        }
    }
}

/// Linux / Quant / Math body: an answer field + load-solution reveal.
fn answer_body(
    p: &'static PracticeProblem,
    is_solved: bool,
    mut app: Signal<AppState>,
    mut input_buf: Signal<HashMap<String, String>>,
    mut revealed: Signal<HashSet<String>>,
) -> Element {
    let id = p.id;
    let answer = input_buf.read().get(id).cloned().unwrap_or_default();
    let show_sol = revealed.read().contains(id);
    let is_linux = p.category == PracticeCategory::Linux;
    let label = if is_linux { "Your command" } else { "Your answer" };
    let placeholder = if is_linux { "Type the command…" } else { "Type your answer…" };
    let has_sol = !p.solution.is_empty();

    rsx! {
        div { class: "modal-section-label", "{label}" }
        input {
            class: "modal-answer-input",
            r#type: "text",
            spellcheck: "false",
            placeholder,
            value: "{answer}",
            oninput: move |e| {
                input_buf.write().insert(id.to_string(), e.value());
            },
        }
        if has_sol {
            button {
                class: "load-sol-btn",
                style: "margin-top:.6rem",
                onclick: move |_| {
                    let mut r = revealed.write();
                    if !r.remove(id) {
                        r.insert(id.to_string());
                    }
                },
                if show_sol { "Hide Solution" } else { "⬇ Load Solution" }
            }
        }
        if has_sol && show_sol {
            div { class: "quiz-solution show",
                span { class: "sol-label", "Solution" }
                div { dangerous_inner_html: "{p.solution}" }
            }
        }
        div { class: "modal-actions",
            button {
                class: "quiz-check-btn",
                onclick: move |_| {
                    let cur = app.read().solved.get(id).copied().unwrap_or(false);
                    app.write().solved.insert(id.to_string(), !cur);
                },
                if is_solved { "✓ Solved — undo" } else { "Mark solved" }
            }
        }
    }
}

/// Build the (text, css-class) to show in the terminal, and auto-mark solved
/// when output matches the expected sample.
fn build_output(res: &RunResult, expected: &str, id: &str, mut app: Signal<AppState>) -> (String, String) {
    if let Some(err) = &res.error {
        let mut text = String::new();
        if !res.output.is_empty() {
            text.push_str(&res.output);
            text.push('\n');
        }
        text.push_str("⚠ ");
        text.push_str(err);
        return (text, "err".to_string());
    }

    let body = if res.output.is_empty() { "(no output)".to_string() } else { res.output.clone() };
    let exit = res.exit.map(|e| format!("\n[exit code {e}]")).unwrap_or_default();
    let checkable = !expected.is_empty() && !expected.contains("...");

    if checkable && normalize_out(&res.output) == normalize_out(expected) {
        if !app.read().solved.get(id).copied().unwrap_or(false) {
            app.write().solved.insert(id.to_string(), true);
        }
        (
            format!("{body}{exit}\n\n✓ Output matches the expected result — solved!"),
            "ok".to_string(),
        )
    } else if checkable {
        (
            format!("{body}{exit}\n\n↳ ran fine, but the output doesn't match the expected result yet."),
            "ok".to_string(),
        )
    } else {
        (format!("{body}{exit}"), "ok".to_string())
    }
}
