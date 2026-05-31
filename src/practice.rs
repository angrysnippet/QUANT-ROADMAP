//! Practice — the day's problems. Code problems get a real in-browser editor
//! (C++ via JSCPP, Python via Pyodide; see `runner`): write code, give stdin,
//! Run, and see terminal output. When the program's output matches the
//! expected sample the problem is auto-marked solved (feeding the Today
//! practice gate). Math/Quant/Linux problems stay self-marked with a reveal.

use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::roadmap::{code_meta, practice_problems_for_day, PracticeCategory, PracticeProblem};
use crate::runner::{normalize_out, run_code};
use crate::state::AppState;

const CATEGORIES: [PracticeCategory; 4] = [
    PracticeCategory::Code,
    PracticeCategory::Math,
    PracticeCategory::Quant,
    PracticeCategory::Linux,
];

#[component]
pub fn Practice() -> Element {
    // These signals are Copy and are passed by value into the card helpers
    // (which hold their own mutable copies), so they don't need `mut` here.
    let app = use_context::<Signal<AppState>>();
    let revealed = use_signal(HashSet::<String>::new);

    // Code-editor session state (not persisted).
    let open_id = use_signal(|| None::<String>);
    let lang = use_signal(|| "cpp".to_string());
    let buffers = use_signal(HashMap::<String, String>::new); // "{id}_{lang}" -> code
    let stdins = use_signal(HashMap::<String, String>::new); // id -> stdin
    let outputs = use_signal(HashMap::<String, (String, String)>::new); // id -> (text, class)
    let running = use_signal(|| None::<String>);

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
                    div { class: "view-sub", "Run your code against the sample — matching output marks it solved. Solve every problem to unlock the next day." }
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
                                        let is_code = p.category == PracticeCategory::Code;
                                        rsx! {
                                            div {
                                                key: "{p.id}",
                                                class: if is_solved { "quiz-card solved" } else { "quiz-card" },
                                                if is_code {
                                                    {code_card(p, is_solved, app, open_id, lang, buffers, stdins, outputs, running)}
                                                } else {
                                                    {quiz_card(p, is_solved, app, revealed)}
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

/// A non-code problem: prompt, self-marked solved, reveal solution.
fn quiz_card(
    p: &'static PracticeProblem,
    is_solved: bool,
    mut app: Signal<AppState>,
    mut revealed: Signal<HashSet<String>>,
) -> Element {
    let is_open = revealed.read().contains(p.id);
    let id = p.id;
    let has_sol = !p.solution.is_empty();
    rsx! {
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
                    let cur = app.read().solved.get(id).copied().unwrap_or(false);
                    app.write().solved.insert(id.to_string(), !cur);
                },
                if is_solved { "✓ Solved — undo" } else { "Mark solved" }
            }
            if has_sol {
                button {
                    class: "quiz-reveal-btn",
                    onclick: move |_| {
                        let mut r = revealed.write();
                        if !r.remove(id) { r.insert(id.to_string()); }
                    },
                    if is_open { "Hide solution" } else { "Reveal solution" }
                }
            }
        }
        if has_sol {
            div { class: if is_open { "quiz-solution show" } else { "quiz-solution" },
                span { class: "sol-label", "Solution" }
                div { dangerous_inner_html: "{p.solution}" }
            }
        }
    }
}

/// A code problem: header + (when opened) the in-browser editor/terminal.
#[allow(clippy::too_many_arguments)]
fn code_card(
    p: &'static PracticeProblem,
    is_solved: bool,
    mut app: Signal<AppState>,
    mut open_id: Signal<Option<String>>,
    mut lang: Signal<String>,
    mut buffers: Signal<HashMap<String, String>>,
    mut stdins: Signal<HashMap<String, String>>,
    mut outputs: Signal<HashMap<String, (String, String)>>,
    mut running: Signal<Option<String>>,
) -> Element {
    let id = p.id;
    let is_open = open_id.read().as_deref() == Some(id);
    let Some(meta) = code_meta(id) else {
        return rsx! {};
    };

    // Header (click toggles the editor open/closed).
    let head = rsx! {
        div {
            class: "quiz-card-head",
            style: "cursor:pointer",
            onclick: move |_| {
                let now_open = open_id.read().as_deref() == Some(id);
                open_id.set(if now_open { None } else { Some(id.to_string()) });
            },
            div { class: if is_solved { "quiz-check solved" } else { "quiz-check" } }
            div { class: "quiz-prompt",
                strong { "{p.title}" }
                br {}
                "{p.prompt}"
            }
            div { class: "quiz-tags",
                span { class: "quiz-topic", "{p.topic}" }
                span { class: "prac-diff diff-{p.diff}", "{p.diff}" }
                span { class: "quiz-reveal-btn", if is_open { "▴ editor" } else { "▾ editor" } }
            }
        }
    };

    if !is_open {
        return rsx! { {head} };
    }

    let lng = lang();
    let buf_key = format!("{id}_{lng}");
    let starter = if lng == "cpp" { meta.starter_cpp } else { meta.starter_py };
    let code = buffers.read().get(&buf_key).cloned().unwrap_or_else(|| starter.to_string());
    let stdin_val = stdins.read().get(id).cloned().unwrap_or_else(|| meta.sample_in.to_string());
    let (out_text, out_cls) = outputs
        .read()
        .get(id)
        .cloned()
        .unwrap_or_else(|| ("Run your code to see output…".to_string(), "muted".to_string()));
    let is_running = running.read().as_deref() == Some(id);

    // Values captured by the Run handler (reflect the current edits).
    let run_code_str = code.clone();
    let run_stdin = stdin_val.clone();
    let run_lng = lng.clone();
    let expected = meta.sample_out;

    rsx! {
        {head}
        div { class: "prac-sample",
            b { "Sample input" }
            pre { "{meta.sample_in}" }
            b { style: "display:inline-block;margin-top:6px", "Expected output" }
            pre { "{meta.sample_out}" }
        }
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
            button {
                class: "prac-reset-btn",
                onclick: {
                    let bk = buf_key.clone();
                    move |_| { buffers.write().remove(&bk); }
                },
                "↺ Clear"
            }
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
                        let (text, cls) = build_output(&res, expected, &id_s, &mut app);
                        outputs.write().insert(id_s.clone(), (text, cls));
                        running.set(None);
                    });
                },
                if is_running { "running…" } else { "▶ Run" }
            }
        }
        textarea {
            class: "prac-code",
            spellcheck: "false",
            placeholder: "Write your code, then hit Run…",
            value: "{code}",
            oninput: move |e| {
                buffers.write().insert(buf_key.clone(), e.value());
            },
        }
        div { class: "prac-io-row",
            div { class: "prac-io-col",
                div { class: "prac-io-label", "stdin (input)" }
                textarea {
                    class: "prac-stdin",
                    spellcheck: "false",
                    value: "{stdin_val}",
                    oninput: move |e| {
                        stdins.write().insert(id.to_string(), e.value());
                    },
                }
            }
            div { class: "prac-io-col",
                div { class: "prac-io-label", "terminal output" }
                div { class: "prac-output {out_cls}", "{out_text}" }
            }
        }
        div { class: "prac-sol-bar",
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
/// when output matches the expected sample. Mirrors spec's runCode result path.
fn build_output(
    res: &crate::runner::RunResult,
    expected: &str,
    id: &str,
    app: &mut Signal<AppState>,
) -> (String, String) {
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
