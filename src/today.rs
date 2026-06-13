//! Today's routine — the daily-loop hub. Shows the daily routine checklist for
//! the current strategy day, the three completion gates (strategy / practice /
//! journal), and the gated "complete day" control. Completing the routine here
//! is what feeds earned progress (`roadmap::topic_progress`) and satisfies the
//! strategy gate. A day can only be completed — advancing `current_day` — once
//! all three gates pass.

use dioxus::prelude::*;

use crate::roadmap::{
    all_topics, journal_complete, practice_complete, practice_problems_for_day, routine_complete,
    total_daily_items, BLOCK_TOPIC_MAP, DAILY_BLOCKS, DAYS,
};
use crate::state::AppState;
use crate::api::MeSummary;
use crate::auth::use_token;
use crate::sync;
use crate::Route;

/// Display name for a topic id (for the "⚡ links →" line); empty if unknown.
fn topic_name(id: &str) -> &'static str {
    all_topics().find(|t| t.id == id).map(|t| t.name).unwrap_or("")
}

/// "name · name · name +N" summary of the topics a block auto-links to.
fn linked_label(block_idx: usize) -> String {
    let Some(block) = BLOCK_TOPIC_MAP.get(block_idx) else {
        return String::new();
    };
    let names: Vec<&str> = block.iter().take(3).map(|c| topic_name(c.topic_id)).collect();
    let mut label = names.join(" · ");
    if block.len() > 3 {
        label.push_str(&format!(" +{}", block.len() - 3));
    }
    label
}

#[component]
pub fn Today() -> Element {
    let mut app = use_context::<Signal<AppState>>();

    // Server-auth token + the server summary (None when offline / signed out).
    let mut token = use_token();
    let mut me = use_signal(|| Option::<MeSummary>::None);
    let mut sync_msg = use_signal(String::new);

    // Pull the server summary whenever a token is present.
    use_effect(move || {
        if let Some(tok) = token() {
            spawn(async move {
                if let Ok(s) = sync::me_summary(tok).await {
                    me.set(Some(s));
                }
            });
        }
    });

    let day = app.read().current_day;
    let day_key = day.to_string();

    // Snapshot the slices this render needs.
    let snap = app.read();
    let checks = snap.daily_checks.get(&day_key).cloned().unwrap_or_default();
    let day_journal = snap.journal.get(&day_key).cloned().unwrap_or_default();
    let solved = snap.solved.clone();
    drop(snap);

    // Past the last defined day → the whole program is complete.
    if day as usize > DAYS.len() {
        return rsx! {
            div { class: "view active",
                div { class: "view-title", "Today's routine" }
                div { class: "view-sub", "You've completed every day of the roadmap." }
                div { class: "strat-success",
                    div { class: "strat-success-title", "🚀 Program complete — all {DAYS.len()} days done!" }
                    div { class: "strat-success-item",
                        span { class: "strat-success-tick", "✓" }
                        span { "Head to Progress to see your final standing." }
                    }
                }
            }
        };
    }

    let strategy_ok = routine_complete(&checks);
    let practice_ok = practice_complete(day, &solved);
    let journal_ok = journal_complete(&day_journal);
    let all_ok = strategy_ok && practice_ok && journal_ok;

    let total = total_daily_items();
    let done_total = checks.values().filter(|v| **v).count();

    let prac_total = practice_problems_for_day(day).count();
    let prac_done = practice_problems_for_day(day)
        .filter(|p| solved.get(p.id).copied().unwrap_or(false))
        .count();
    let jrnl_done = ["j1", "j2", "j3", "j4", "j5"]
        .iter()
        .filter(|k| day_journal.get(**k).is_some_and(|v| !v.trim().is_empty()))
        .count();

    let day_title = DAYS.iter().find(|d| d.id as u32 == day).map(|d| d.title).unwrap_or("");

    rsx! {
        div { class: "view active",
            div { class: "view-title", "Today's routine · Day {day}" }
            div { class: "view-sub",
                if day_title.is_empty() {
                    "Check off every item — completing the day is the only way to earn progress."
                } else {
                    "Day {day} — {day_title}. Finish all three gates to unlock the next day."
                }
            }

            // ── Sync / account panel ──
            div { class: "strat-success", style: "margin-bottom:1rem",
                if let Some(s) = me() {
                    div { class: "strat-success-title",
                        "☁ Synced as {s.handle} — Lv {s.level} · {s.xp} XP · 🔥 {s.streak_current}"
                    }
                    button {
                        class: "cal-sched-open-btn",
                        onclick: move |_| {
                            let cur = app.read().current_day as i64;
                            if let Some(tok) = token() {
                                spawn(async move {
                                    match sync::import_local_progress(tok, cur).await {
                                        Ok(s) => {
                                            me.set(Some(s));
                                            sync_msg.set("Imported local progress.".to_string());
                                        }
                                        Err(e) => sync_msg.set(e),
                                    }
                                });
                            }
                        },
                        "Import local progress (one-time)"
                    }
                    button {
                        class: "cal-sched-open-btn",
                        style: "margin-left:.5rem",
                        onclick: move |_| {
                            spawn(async move {
                                crate::auth::sign_out().await;
                                token.set(None);
                                me.set(None);
                            });
                        },
                        "Sign out"
                    }
                } else if token().is_some() {
                    div { class: "strat-success-title", "☁ Signing in…" }
                } else {
                    div { class: "strat-success-title", "Offline mode — progress saved on this device only." }
                    Link { to: Route::Login {}, class: "cal-sched-open-btn", "Sign in to sync ›" }
                }
                if !sync_msg().is_empty() {
                    div { style: "font-size:12px;color:var(--text3);margin-top:.4rem", "{sync_msg}" }
                }
            }

            // ── Gate panel ──
            div { class: "strat-success",
                div { class: "strat-success-title", "🔓 Complete all three to unlock Day {day + 1}:" }
                GateRow {
                    met: strategy_ok,
                    label: "Strategy routine",
                    detail: format!("{done_total}/{total} items checked"),
                    route: None,
                }
                GateRow {
                    met: practice_ok,
                    label: "Practice questions",
                    detail: format!("{prac_done}/{prac_total} solved"),
                    route: Some(Route::Practice {}),
                }
                GateRow {
                    met: journal_ok,
                    label: "Journal",
                    detail: format!("{jrnl_done}/5 prompts filled"),
                    route: Some(Route::Journal {}),
                }
                button {
                    class: "strat-practice-btn",
                    disabled: !all_ok,
                    style: if all_ok { "" } else { "opacity:.45;cursor:not-allowed" },
                    onclick: move |_| {
                        if !all_ok {
                            return;
                        }
                        match token() {
                            // Authenticated: the server is authoritative. It awards
                            // XP idempotently and returns the fresh summary; we mirror
                            // current_day locally as an offline cache.
                            Some(tok) => {
                                spawn(async move {
                                    match sync::complete_day(tok, day as i64).await {
                                        Ok(s) => {
                                            app.write().current_day = s.current_day as u32;
                                            me.set(Some(s));
                                        }
                                        Err(e) => sync_msg.set(format!("Sync failed: {e}")),
                                    }
                                });
                            }
                            // Offline / signed out: local-only advance (legacy behaviour).
                            None => {
                                app.write().current_day += 1;
                            }
                        }
                    },
                    if all_ok {
                        "✓ Complete Day {day} → unlock Day {day + 1}"
                    } else {
                        "🔒 Finish all three gates to advance"
                    }
                }
            }

            // ── Daily routine checklist ──
            div { class: "section-label", "Daily routine" }
            div { class: "today-grid",
            for (bi, block) in DAILY_BLOCKS.iter().enumerate() {
                {
                    let b_done = (0..block.items.len())
                        .filter(|ii| checks.get(&format!("{bi}_{ii}")).copied().unwrap_or(false))
                        .count();
                    let b_total = block.items.len();
                    let fill = b_done * 100 / b_total.max(1);
                    let all_block = b_done == b_total;
                    let links = linked_label(bi);
                    rsx! {
                        div { key: "{bi}", class: "daily-block",
                            div { class: "daily-block-header",
                                div { class: "block-dot", style: "background:{block.color}" }
                                span { "{block.title}" }
                                span { class: "daily-count", "{b_done}/{b_total}" }
                                span {
                                    class: if all_block { "block-complete-badge visible" } else { "block-complete-badge" },
                                    "✓ done"
                                }
                            }
                            div { class: "block-progress-bar",
                                div { class: "block-progress-fill", style: "width:{fill}%;background:{block.color}" }
                            }
                            div { class: "daily-items",
                                for (ii, item) in block.items.iter().enumerate() {
                                    {
                                        let key = format!("{bi}_{ii}");
                                        let is_done = checks.get(&key).copied().unwrap_or(false);
                                        let day_key = day_key.clone();
                                        rsx! {
                                            div {
                                                key: "{ii}",
                                                class: "daily-item",
                                                onclick: move |_| {
                                                    let mut st = app.write();
                                                    let day_map = st.daily_checks.entry(day_key.clone()).or_default();
                                                    let e = day_map.entry(key.clone()).or_insert(false);
                                                    *e = !*e;
                                                },
                                                div { class: if is_done { "check-box checked" } else { "check-box" } }
                                                span { class: if is_done { "daily-label checked" } else { "daily-label" }, "{item}" }
                                            }
                                        }
                                    }
                                }
                            }
                            if !links.is_empty() {
                                div {
                                    style: "padding:.5rem 1.25rem .625rem;font-size:10px;color:var(--text3);font-family:var(--mono);border-top:1px solid var(--border)",
                                    "⚡ links → {links}"
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

/// One gate row in the unlock panel: a tick when met, otherwise a link to the
/// view where it can be satisfied.
#[component]
fn GateRow(met: bool, label: String, detail: String, route: Option<Route>) -> Element {
    rsx! {
        div { class: "strat-success-item",
            span {
                class: "strat-success-tick",
                style: if met { "" } else { "color:var(--text3);border-color:var(--border2)" },
                if met { "✓" } else { "○" }
            }
            span { style: if met { "" } else { "color:var(--text2)" }, "{label} — {detail}" }
            if !met {
                if let Some(r) = route {
                    Link { to: r, class: "cal-sched-open-btn", style: "margin-left:auto", "Go ›" }
                }
            }
        }
    }
}
