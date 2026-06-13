//! Shared client <-> server data types. Always compiled (no server-only or
//! fullstack-only deps) so both the WASM client and the server agree on the
//! wire format, and the non-fullstack `web` build still type-checks.

use serde::{Deserialize, Serialize};

/// Snapshot of a user's server-authoritative game state. Uses only
/// primitive/`String` fields so nothing server-only (uuid/sqlx) crosses the
/// boundary or leaks into the WASM client.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MeSummary {
    pub handle: String,
    pub title: String,
    pub xp: i64,
    pub level: i64,
    pub current_day: i64,
    pub completed_days: i64,
    pub streak_current: i64,
    pub streak_longest: i64,
    pub progress_pct: f64,
    pub est_completion: Option<String>,
}
