//! Day-by-day strategy — a three-level drill-down ported from the
//! `renderStratDays` / `renderStratBlocks` / `renderStratBlockDetail` flow in
//! spec.html: (1) a searchable, phase-filterable list of days, (2) the block
//! tiles + "success criteria" for a chosen day, and (3) a single block's detail
//! (raw HTML content) with prev/next nav.
//!
//! Navigation state lives in local signals; block `content` is injected via
//! `dangerous_inner_html` (the same HTML spec.html assigns to `innerHTML`).

use std::collections::HashSet;

use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

use crate::roadmap::{phase_color, StrategyDay, DAYS};
use crate::Route;

/// How many "blocks" to advertise on a day card. Authored (Markdown) days have
/// no `blocks` array, so count their `## Block …` sections instead.
fn day_block_count(d: &StrategyDay) -> usize {
    if d.schedule_md.is_empty() {
        d.blocks.len()
    } else {
        d.schedule_md.lines().filter(|l| l.starts_with("## Block")).count()
    }
}

/// Render authored Markdown (a day's schedule) to an HTML string.
fn md_to_html(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

/// Split a day's Markdown into a preamble (everything before the first `## `
/// heading) and a list of `(heading, body)` sections — one per `## ` block —
/// so each can be shown as a collapsible panel.
fn split_sections(md: &str) -> (String, Vec<(String, String)>) {
    let mut preamble = String::new();
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut cur: Option<(String, String)> = None;
    for line in md.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            if let Some(sec) = cur.take() {
                sections.push(sec);
            }
            cur = Some((h.trim().to_string(), String::new()));
        } else if let Some((_, body)) = cur.as_mut() {
            body.push_str(line);
            body.push('\n');
        } else {
            preamble.push_str(line);
            preamble.push('\n');
        }
    }
    if let Some(sec) = cur.take() {
        sections.push(sec);
    }
    (preamble, sections)
}

/// (label, filter value) for the phase tabs, in display order.
const PHASE_TABS: [(&str, &str); 14] = [
    ("All", "all"),
    ("Phase 1", "Phase 1"),
    ("Phase 2", "Phase 2"),
    ("Phase 3", "Phase 3"),
    ("Phase 4", "Phase 4"),
    ("Phase 5", "Phase 5"),
    ("Phase 6", "Phase 6"),
    ("Phase 7", "Phase 7"),
    ("Phase 8", "Phase 8"),
    ("Phase 9", "Phase 9"),
    ("Phase 10", "Phase 10"),
    ("Phase 11", "Phase 11"),
    ("Phase 12", "Phase 12"),
    ("Phase 13", "Phase 13"),
];

#[component]
pub fn Strategy() -> Element {
    let search = use_signal(String::new);
    let phase_filter = use_signal(|| "all".to_string());
    let active_day = use_signal(|| None::<i64>);
    let active_block = use_signal(|| None::<usize>);
    let open_secs = use_signal(HashSet::<String>::new); // expanded "{day}|{heading}" sections

    let day_id = active_day();
    let block_idx = active_block();

    rsx! {
        div { class: "view active",
            div { class: "view-title", "Day-by-day strategy" }
            div { class: "view-sub", "Full daily breakdown — resources, tasks, practice problems" }

            if let (Some(d), Some(bi)) = (day_id, block_idx) {
                {block_detail(d, bi, active_day, active_block)}
            } else if let Some(d) = day_id {
                {day_blocks(d, active_day, active_block, open_secs)}
            } else {
                {day_list(search, phase_filter, active_day, active_block)}
            }
        }
    }
}

/// Level 1 — searchable, phase-filtered list of days.
fn day_list(
    mut search: Signal<String>,
    mut phase_filter: Signal<String>,
    mut active_day: Signal<Option<i64>>,
    mut active_block: Signal<Option<usize>>,
) -> Element {
    let raw = search();
    let q = raw.to_lowercase();
    let pf = phase_filter();

    let filtered: Vec<&'static StrategyDay> = DAYS
        .iter()
        .filter(|d| {
            let phase_match = pf == "all" || pf == d.phase;
            let text_match = q.is_empty()
                || format!("day {}", d.id).contains(q.as_str())
                || d.title.to_lowercase().contains(q.as_str())
                || d.phase.to_lowercase().contains(q.as_str())
                || d.blocks.iter().any(|b| b.title.to_lowercase().contains(q.as_str()));
            phase_match && text_match
        })
        .collect();

    rsx! {
        div { class: "strategy-toolbar", style: "display:flex",
            input {
                class: "strat-search",
                r#type: "text",
                placeholder: "Search day, topic, block...",
                value: "{raw}",
                oninput: move |e| {
                    search.set(e.value());
                    // Searching always returns to the day list.
                    active_day.set(None);
                    active_block.set(None);
                },
            }
            div { class: "strat-phase-tabs",
                for (label, val) in PHASE_TABS {
                    button {
                        key: "{val}",
                        class: if pf == val { "strat-ptab active" } else { "strat-ptab" },
                        onclick: move |_| {
                            phase_filter.set(val.to_string());
                            active_day.set(None);
                            active_block.set(None);
                        },
                        "{label}"
                    }
                }
            }
        }
        div {
            if filtered.is_empty() {
                div {
                    style: "text-align:center;color:var(--text3);font-size:13px;padding:3rem",
                    "No days match."
                }
            } else {
                for d in filtered {
                    {
                        let (pbg, pc) = phase_color(d.phase);
                        let id = d.id;
                        rsx! {
                            div { key: "{d.id}", class: "day-card",
                                div {
                                    class: "day-card-header",
                                    onclick: move |_| {
                                        active_day.set(Some(id));
                                        active_block.set(None);
                                    },
                                    div { class: "day-header-left",
                                        span { class: "day-number", "Day {d.id}" }
                                        span { class: "day-title", "{d.title}" }
                                    }
                                    div { style: "display:flex;align-items:center;gap:8px",
                                        span { class: "strat-block-count", "{day_block_count(d)} blocks" }
                                        span {
                                            class: "day-phase-tag",
                                            style: "background:{pbg};color:{pc}",
                                            "{d.phase}"
                                        }
                                        span { style: "color:var(--text3);font-size:14px", "›" }
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

/// Level 2 — block tiles + success criteria for a chosen day.
fn day_blocks(
    day_id: i64,
    mut active_day: Signal<Option<i64>>,
    mut active_block: Signal<Option<usize>>,
    open_secs: Signal<HashSet<String>>,
) -> Element {
    let Some(d) = DAYS.iter().find(|x| x.id == day_id) else {
        return rsx! {};
    };
    let (pbg, pc) = phase_color(d.phase);

    // Authored days render their Markdown as a page: an intro preamble plus one
    // collapsible panel per `## ` section (Block 1, Block 2, … collapsed first).
    if !d.schedule_md.is_empty() {
        let (preamble, sections) = split_sections(d.schedule_md);
        let preamble_html = md_to_html(&preamble);
        let has_preamble = !preamble.trim().is_empty();
        return rsx! {
            div { class: "strat-crumb",
                span {
                    class: "strat-crumb-link",
                    onclick: move |_| {
                        active_day.set(None);
                        active_block.set(None);
                    },
                    "← All days"
                }
                span { class: "strat-crumb-sep", "/" }
                span { class: "strat-crumb-current", "Day {d.id} · {d.title}" }
                span {
                    class: "day-phase-tag",
                    style: "background:{pbg};color:{pc};margin-left:auto",
                    "{d.phase}"
                }
            }
            if has_preamble {
                div { class: "schedule-md", style: "margin-bottom:1rem", dangerous_inner_html: "{preamble_html}" }
            }
            for (i, (heading, body)) in sections.iter().enumerate() {
                {
                    let key = format!("{}|{}", d.id, heading);
                    let is_open = open_secs.read().contains(&key);
                    let body_html = if is_open { md_to_html(body) } else { String::new() };
                    let title = heading.clone();
                    let mut os = open_secs;
                    rsx! {
                        div { key: "{i}", class: "strat-sec",
                            button {
                                class: if is_open { "strat-sec-header open" } else { "strat-sec-header" },
                                onclick: move |_| {
                                    let mut s = os.write();
                                    if !s.remove(&key) {
                                        s.insert(key.clone());
                                    }
                                },
                                span { class: "strat-sec-chevron", if is_open { "▾" } else { "▸" } }
                                span { class: "strat-sec-title", "{title}" }
                            }
                            if is_open {
                                div { class: "strat-sec-body",
                                    div { class: "schedule-md", dangerous_inner_html: "{body_html}" }
                                }
                            }
                        }
                    }
                }
            }
            button {
                class: "strat-practice-btn",
                style: "margin-top:1rem",
                onclick: move |_| {
                    navigator().push(Route::Practice {});
                },
                "⌨️ Practice Day {d.id} problems ›"
            }
        };
    }

    rsx! {
        div { class: "strat-crumb",
            span {
                class: "strat-crumb-link",
                onclick: move |_| {
                    active_day.set(None);
                    active_block.set(None);
                },
                "← All days"
            }
            span { class: "strat-crumb-sep", "/" }
            span { class: "strat-crumb-current", "Day {d.id} · {d.title}" }
            span {
                class: "day-phase-tag",
                style: "background:{pbg};color:{pc};margin-left:auto",
                "{d.phase}"
            }
        }
        div { class: "strat-block-grid",
            for (bi, b) in d.blocks.iter().enumerate() {
                {
                    let idx = bi;
                    rsx! {
                        div {
                            key: "{bi}",
                            class: "strat-block-tile",
                            onclick: move |_| active_block.set(Some(idx)),
                            div { class: "strat-tile-top",
                                div { class: "block-color-dot", style: "background:{b.color}" }
                                span { class: "strat-tile-time", "{b.time}" }
                            }
                            div { class: "strat-tile-title", "{b.title}" }
                            div { class: "strat-tile-open", "Open ›" }
                        }
                    }
                }
            }
        }
        if !d.success.is_empty() {
            div { class: "strat-success",
                div { class: "strat-success-title",
                    "🎯 By the end of Day {d.id} you should be able to:"
                }
                for (i, s) in d.success.iter().enumerate() {
                    div { key: "s{i}", class: "strat-success-item",
                        span { class: "strat-success-tick", "✓" }
                        span { "{s}" }
                    }
                }
                // Every ported day has practice problems, so the button always shows.
                button {
                    class: "strat-practice-btn",
                    onclick: move |_| {
                        navigator().push(Route::Practice {});
                    },
                    "⌨️ Practice Day {d.id} problems ›"
                }
            }
        }
    }
}

/// Level 3 — a single block's detail with prev/next navigation.
fn block_detail(
    day_id: i64,
    block_idx: usize,
    mut active_day: Signal<Option<i64>>,
    mut active_block: Signal<Option<usize>>,
) -> Element {
    let Some(d) = DAYS.iter().find(|x| x.id == day_id) else {
        return rsx! {};
    };
    let Some(b) = d.blocks.get(block_idx) else {
        return rsx! {};
    };

    let prev = block_idx
        .checked_sub(1)
        .map(|i| (i, d.blocks[i].title));
    let next = if block_idx + 1 < d.blocks.len() {
        Some((block_idx + 1, d.blocks[block_idx + 1].title))
    } else {
        None
    };

    rsx! {
        div { class: "strat-crumb",
            span {
                class: "strat-crumb-link",
                onclick: move |_| {
                    active_day.set(None);
                    active_block.set(None);
                },
                "All days"
            }
            span { class: "strat-crumb-sep", "/" }
            span {
                class: "strat-crumb-link",
                onclick: move |_| active_block.set(None),
                "Day {d.id}"
            }
            span { class: "strat-crumb-sep", "/" }
            span { class: "strat-crumb-current", "{b.title}" }
        }
        div { class: "strat-detail-card",
            div { class: "strat-detail-header",
                div {
                    class: "block-color-dot",
                    style: "background:{b.color};width:10px;height:10px",
                }
                span { class: "strat-detail-title", "{b.title}" }
                span { class: "strat-detail-time", "{b.time}" }
            }
            div { class: "strat-detail-body", dangerous_inner_html: "{b.content}" }
        }
        div { class: "strat-block-nav",
            if let Some((pi, ptitle)) = prev {
                button {
                    class: "strat-nav-btn",
                    onclick: move |_| active_block.set(Some(pi)),
                    "← {ptitle}"
                }
            } else {
                span {}
            }
            if let Some((ni, ntitle)) = next {
                button {
                    class: "strat-nav-btn",
                    onclick: move |_| active_block.set(Some(ni)),
                    "{ntitle} →"
                }
            } else {
                span {}
            }
        }
    }
}
