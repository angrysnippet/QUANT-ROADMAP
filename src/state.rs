//! Shared application state, persisted to the browser's localStorage.
//!
//! Progress is *earned*, not set directly: topic percentages are derived from
//! how many strategy days have been completed (`current_day - 1`), so there is
//! no stored progress map. A day is completed — and `current_day` advanced —
//! only when its three gates pass (routine checked, practice solved, journal
//! filled). See `roadmap.rs` for the derivation and gate helpers.
//!
//! Data model:
//! - `current_day`   : the strategy day the user is currently on (1-based).
//! - `daily_checks`  : day id ("1", "2", …) -> { "block_item" -> done }
//! - `journal`       : day id -> { "j1".."j5" -> text }
//! - `schedule_start`: the calendar date that "Day 1" maps to (Calendar view).
//! - `solved`        : practice question/quiz id -> solved
//!
//! `daily_checks` / `journal` are keyed by strategy-day id (not calendar date)
//! so the per-day gates are self-paced; the Calendar projects them onto dates
//! via `strategy_day_for_date`.

use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STORAGE_KEY: &str = "qrt_state";

fn default_current_day() -> u32 {
    1
}

fn default_theme() -> String {
    "dark".to_string()
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default = "default_current_day")]
    pub current_day: u32,
    #[serde(default)]
    pub daily_checks: HashMap<String, HashMap<String, bool>>,
    #[serde(default)]
    pub journal: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub schedule_start: Option<String>,
    #[serde(default)]
    pub solved: HashMap<String, bool>,
    /// UI theme: "dark" (default) or "light".
    #[serde(default = "default_theme")]
    pub theme: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_day: default_current_day(),
            daily_checks: HashMap::new(),
            journal: HashMap::new(),
            schedule_start: None,
            solved: HashMap::new(),
            theme: default_theme(),
        }
    }
}

impl AppState {
    /// Load persisted state from localStorage, falling back to defaults.
    pub fn load() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Persist the current state to localStorage.
    pub fn save(&self) {
        let _ = LocalStorage::set(STORAGE_KEY, self);
    }
}

/// Provide the shared state signal to the component tree and register an
/// autosave effect that writes to localStorage whenever the state changes.
/// Call once at the app root.
pub fn use_app_state() -> Signal<AppState> {
    let state = use_context_provider(|| Signal::new(AppState::load()));
    use_effect(move || {
        // Reading the signal here subscribes the effect to every change.
        state.read().save();
    });
    state
}
