//! Calendar view — activity heatmap + scheduled strategy days, ported from
//! the original `spec.html` calendar. The month grid colours each day by how
//! many daily-routine items were checked off that date; a dot marks days that
//! map to a strategy day (via `schedule_start`) and another marks days with a
//! journal entry. Clicking a date opens a detail panel.

use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;

use crate::roadmap::{strategy_day_for_date, total_daily_items, DAILY_BLOCKS};
use crate::state::AppState;
use crate::Route;

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// (label, journal field key) for the five journal prompts, in display order.
const JOURNAL_PROMPTS: [(&str, &str); 5] = [
    ("Confused me most", "j1"),
    ("What clicked", "j2"),
    ("Worst bug", "j3"),
    ("Weak concept", "j4"),
    ("Panic / handling", "j5"),
];

fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

fn today_key() -> String {
    today_date().format("%Y-%m-%d").to_string()
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day()
}

/// Heatmap fill colour for a completion ratio (0..=1). Mirrors `heatFill`.
fn heat_fill(ratio: f64) -> &'static str {
    if ratio <= 0.0 {
        "var(--surface2)"
    } else if ratio < 0.26 {
        "rgba(6,214,160,0.25)"
    } else if ratio < 0.51 {
        "rgba(6,214,160,0.5)"
    } else if ratio < 0.76 {
        "rgba(6,214,160,0.75)"
    } else {
        "rgba(6,214,160,1)"
    }
}

#[component]
pub fn Calendar() -> Element {
    let mut app = use_context::<Signal<AppState>>();

    let today = today_date();
    let mut cal_year = use_signal(|| today.year());
    let mut cal_month = use_signal(|| today.month()); // 1..=12
    let mut selected = use_signal(|| None::<String>);

    // Snapshot the shared state we need (clones are cheap for these maps).
    let snap = app.read();
    let daily_checks = snap.daily_checks.clone();
    let journal = snap.journal.clone();
    let schedule_start = snap.schedule_start.clone();
    drop(snap);

    let year = cal_year();
    let month = cal_month();
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first.weekday().num_days_from_sunday(); // 0 = Sun
    let dim = days_in_month(year, month);
    let tk = today_key();
    let sel = selected();
    let total = total_daily_items();

    let month_label = first.format("%B %Y").to_string();
    let has_start = schedule_start.is_some();
    let start_value = schedule_start.clone().unwrap_or_default();
    let hint = if has_start {
        "Day 1 begins on this date"
    } else {
        "Set a date to schedule strategy days"
    };

    // Precompute each day cell.
    struct Cell {
        day: u32,
        key: String,
        fill: &'static str,
        num_class: &'static str,
        today: bool,
        selected: bool,
        sched: bool,
        journal: bool,
    }

    let cells: Vec<Cell> = (1..=dim)
        .map(|d| {
            let key = format!("{year}-{month:02}-{d:02}");
            // Routine/journal are keyed by strategy-day id; project this date
            // onto its scheduled day (if any) and look completion up by that id.
            let sched_day = strategy_day_for_date(&key, &schedule_start);
            let day_key = sched_day.map(|s| s.id.to_string());
            let done = day_key
                .as_ref()
                .and_then(|k| daily_checks.get(k))
                .map(|m| m.values().filter(|v| **v).count())
                .unwrap_or(0);
            let ratio = if total > 0 { done as f64 / total as f64 } else { 0.0 };
            let num_class = if ratio >= 0.76 {
                "on-fill"
            } else if ratio <= 0.0 {
                "dim"
            } else {
                ""
            };
            let has_journal = day_key
                .as_ref()
                .and_then(|k| journal.get(k))
                .map(|m| m.values().any(|v| !v.trim().is_empty()))
                .unwrap_or(false);
            Cell {
                day: d,
                today: key == tk,
                selected: Some(&key) == sel.as_ref(),
                sched: sched_day.is_some(),
                journal: has_journal,
                fill: heat_fill(ratio),
                num_class,
                key,
            }
        })
        .collect();

    // Build the grid (leading blanks + day cells) before the macro to keep the
    // `cells` move out of the rsx expansion.
    let grid_nodes: Vec<Element> = (0..first_weekday)
        .map(|i| rsx! { div { key: "e{i}", class: "cal-cell empty" } })
        .chain(cells.into_iter().map(|cell| {
            let key = cell.key.clone();
            let class = format!(
                "cal-cell{}{}",
                if cell.today { " today" } else { "" },
                if cell.selected { " selected" } else { "" },
            );
            rsx! {
                div {
                    key: "{cell.key}",
                    class: "{class}",
                    style: "background:{cell.fill}",
                    onclick: move |_| selected.set(Some(key.clone())),
                    span { class: "cal-cell-num {cell.num_class}", "{cell.day}" }
                    if cell.sched {
                        span { class: "cal-sched-dot" }
                    }
                    if cell.journal {
                        span { class: "cal-jrnl-dot" }
                    }
                }
            }
        }))
        .collect();

    rsx! {
        div { class: "view active",
            div { class: "view-title", "Calendar" }
            div { class: "view-sub",
                "Activity heatmap · click any date to browse · scheduled strategy days"
            }

            div { class: "cal-schedule-bar",
                span { class: "cal-sched-label", "📅 Strategy start date" }
                input {
                    r#type: "date",
                    class: "cal-date-input",
                    value: "{start_value}",
                    onchange: move |e| {
                        let v = e.value();
                        app.write().schedule_start = if v.is_empty() { None } else { Some(v) };
                    },
                }
                span { class: "cal-sched-hint", "{hint}" }
            }

            div { class: "cal-layout",
                div { class: "cal-main",
                    div { class: "cal-header",
                        button {
                            class: "cal-nav-btn",
                            onclick: move |_| {
                                let (mut y, mut m) = (cal_year(), cal_month());
                                if m == 1 { m = 12; y -= 1; } else { m -= 1; }
                                cal_year.set(y);
                                cal_month.set(m);
                            },
                            "‹"
                        }
                        div { class: "cal-month-label", "{month_label}" }
                        button {
                            class: "cal-nav-btn",
                            onclick: move |_| {
                                let (mut y, mut m) = (cal_year(), cal_month());
                                if m == 12 { m = 1; y += 1; } else { m += 1; }
                                cal_year.set(y);
                                cal_month.set(m);
                            },
                            "›"
                        }
                        button {
                            class: "cal-today-btn",
                            onclick: move |_| {
                                let t = today_date();
                                cal_year.set(t.year());
                                cal_month.set(t.month());
                                selected.set(Some(t.format("%Y-%m-%d").to_string()));
                            },
                            "Today"
                        }
                    }

                    div { class: "cal-weekdays",
                        for w in WEEKDAYS {
                            div { key: "{w}", class: "cal-weekday", "{w}" }
                        }
                    }

                    div { class: "cal-grid",
                        {grid_nodes.into_iter()}
                    }

                    div { class: "cal-legend",
                        span { class: "cal-legend-label", "Less" }
                        span { class: "cal-legend-cell", style: "background:var(--surface2)" }
                        span { class: "cal-legend-cell", style: "background:rgba(6,214,160,0.25)" }
                        span { class: "cal-legend-cell", style: "background:rgba(6,214,160,0.5)" }
                        span { class: "cal-legend-cell", style: "background:rgba(6,214,160,0.75)" }
                        span { class: "cal-legend-cell", style: "background:rgba(6,214,160,1)" }
                        span { class: "cal-legend-label", "More" }
                        span {
                            style: "margin-left:auto;display:flex;align-items:center;gap:6px",
                            span { class: "cal-sched-dot" }
                            span { class: "cal-legend-label", "Strategy day" }
                        }
                    }
                }

                CalDetail { selected: sel }
            }
        }
    }
}

/// Right-hand detail panel for the selected date.
#[component]
fn CalDetail(selected: Option<String>) -> Element {
    let app = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    let Some(dk) = selected else {
        return rsx! {
            div { class: "cal-detail",
                div { class: "cal-detail-empty",
                    "Click any date to see"
                    br {}
                    "that day's progress,"
                    br {}
                    "schedule and journal."
                }
            }
        };
    };

    let snap = app.read();
    let schedule_start = snap.schedule_start.clone();
    // Routine/journal are keyed by the strategy-day id this date maps to.
    let sched = strategy_day_for_date(&dk, &schedule_start);
    let day_key = sched.map(|s| s.id.to_string());
    let checks = day_key
        .as_ref()
        .and_then(|k| snap.daily_checks.get(k))
        .cloned()
        .unwrap_or_default();
    let journal = day_key
        .as_ref()
        .and_then(|k| snap.journal.get(k))
        .cloned()
        .unwrap_or_default();
    drop(snap);

    let date = NaiveDate::parse_from_str(&dk, "%Y-%m-%d").ok();
    let date_label = date
        .map(|d| d.format("%A, %d %B %Y").to_string())
        .unwrap_or_else(|| dk.clone());
    let is_today = dk == today_key();

    let total = total_daily_items();
    let done_total = checks.values().filter(|v| **v).count();
    let pct = if total > 0 {
        (done_total as f64 / total as f64 * 100.0).round()
    } else {
        0.0
    };

    let journal_filled: Vec<(&str, String)> = JOURNAL_PROMPTS
        .iter()
        .filter_map(|(label, k)| {
            journal
                .get(*k)
                .filter(|v| !v.trim().is_empty())
                .map(|v| (*label, v.clone()))
        })
        .collect();

    rsx! {
        div { class: "cal-detail",
            div { class: "cal-detail-date",
                "{date_label}"
                if is_today {
                    " · Today"
                }
            }
            div { class: "cal-detail-sub", "{dk}" }

            if let Some(day) = sched {
                div { class: "cal-detail-section-label", "Scheduled" }
                div { class: "cal-sched-card",
                    div { class: "cal-sched-card-title", "Day {day.id} · {day.title}" }
                    div { class: "cal-sched-card-meta", "{day.phase} · {day.blocks.len()} blocks" }
                    button {
                        class: "cal-sched-open-btn",
                        onclick: move |_| {
                            nav.push(Route::Strategy {});
                        },
                        "Open in Strategy ›"
                    }
                }
            }

            div { class: "cal-detail-section-label", "Completion" }
            div { class: "cal-completion-ring",
                {make_ring(pct, "#06d6a0", 44.0)}
                div { class: "cal-completion-text",
                    strong { "{done_total}" }
                    " / {total} items done"
                }
            }
            for (bi, b) in DAILY_BLOCKS.iter().enumerate() {
                {
                    let b_done = (0..b.items.len())
                        .filter(|ii| checks.get(&format!("{bi}_{ii}")).copied().unwrap_or(false))
                        .count();
                    let full = b_done == b.items.len();
                    let count_class = if full { "cal-mini-count full" } else { "cal-mini-count" };
                    rsx! {
                        div { key: "{bi}", class: "cal-mini-block",
                            span { class: "cal-mini-dot", style: "background:{b.color}" }
                            span { class: "cal-mini-name", "{b.title}" }
                            span { class: "{count_class}", "{b_done}/{b.items.len()}" }
                        }
                    }
                }
            }

            div { class: "cal-detail-section-label", "Journal" }
            if journal_filled.is_empty() {
                div { class: "cal-jrnl-empty", "No journal entry for this date." }
            } else {
                for (i, (label, text)) in journal_filled.into_iter().enumerate() {
                    div { key: "{i}", class: "cal-jrnl-preview",
                        span { class: "jq", "{label}" }
                        "{text}"
                    }
                }
            }

            if is_today {
                button {
                    class: "cal-open-today-btn",
                    onclick: move |_| {
                        nav.push(Route::Today {});
                    },
                    "Open Today's routine ›"
                }
            }
        }
    }
}

/// SVG progress ring, matching `makeSVGRing` from spec.html (uses the existing
/// `.ring-svg/.ring-bg/.ring-fg/.ring-text` styles in main.css).
pub(crate) fn make_ring(pct: f64, color: &str, size: f64) -> Element {
    let r = size / 2.0 - 4.0;
    let circ = 2.0 * std::f64::consts::PI * r;
    let offset = circ - (pct / 100.0) * circ;
    let half = size / 2.0;
    let pct_i = pct as i64;
    rsx! {
        svg {
            class: "ring-svg",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 {size} {size}",
            circle { class: "ring-bg", cx: "{half}", cy: "{half}", r: "{r}" }
            circle {
                class: "ring-fg",
                cx: "{half}",
                cy: "{half}",
                r: "{r}",
                stroke: "{color}",
                stroke_dasharray: "{circ}",
                stroke_dashoffset: "{offset}",
            }
            text {
                class: "ring-text",
                x: "{half}",
                y: "{half}",
                transform: "rotate(90 {half} {half})",
                "{pct_i}%"
            }
        }
    }
}
