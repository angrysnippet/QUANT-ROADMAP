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

/// A problem as shown to the client: SEALED - it carries no answer, tolerance,
/// or solution. Grading happens server-side; the solution is only returned in a
/// `GradeResult` after a submission (CLAUDE.md 4.4).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BankProblem {
    pub id: String,
    pub track: String,
    pub world: i64,
    pub difficulty: i64,
    pub kind: String, // "mcq" | "numeric" | "code"
    pub statement: String,
    pub choices: Vec<String>, // empty unless kind == "mcq"
    pub tags: Vec<String>,
    pub time_limit_seconds: i64,
}

/// Result of grading a submission. The solution is revealed only here, after the
/// attempt has been graded server-side.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GradeResult {
    pub correct: bool,
    pub xp_awarded: i64, // 0 if incorrect or already solved
    pub solution_explanation: String,
    pub summary: MeSummary,
}
