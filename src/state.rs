//! Shared application state, persisted to the browser's localStorage.
//!
//! The data model mirrors the original `spec.html` store:
//! - `progress`      : topic id -> 0..=100 percent
//! - `daily_checks`  : date ("YYYY-MM-DD") -> { "block_item" -> done }
//! - `journal`       : date -> { "j1".."j5" -> text }
//! - `schedule_start`: the calendar date that "Day 1" of the strategy maps to
//! - `solved`        : practice question/quiz id -> solved

use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const STORAGE_KEY: &str = "qrt_state";

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppState {
    #[serde(default)]
    pub progress: HashMap<String, u32>,
    #[serde(default)]
    pub daily_checks: HashMap<String, HashMap<String, bool>>,
    #[serde(default)]
    pub journal: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub schedule_start: Option<String>,
    #[serde(default)]
    pub solved: HashMap<String, bool>,
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

/// Convenience accessor for descendants of the root provider.
pub fn use_state_ctx() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}
