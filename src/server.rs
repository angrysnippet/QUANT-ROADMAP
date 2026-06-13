//! Server-only infrastructure for Quant Arena (compiled under the `server`
//! feature only). Holds the Postgres pool, Supabase JWT verification, and all
//! database logic behind the server functions in `server_fns.rs`.
//!
//! Nothing here is ever compiled into the WASM client: the pool, the
//! service-role connection, and the JWT secret live only on the server
//! (CLAUDE.md 4.1 / 4.8). All writes go through here using a service-role
//! Postgres connection that bypasses RLS; reads from the client are restricted
//! by RLS to the user's own rows.

use chrono::{Duration, NaiveDate, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sqlx::postgres::{PgPool, PgPoolOptions, Postgres};
use sqlx::{Row, Transaction};
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::api::MeSummary;

/// Result alias: server-side errors are surfaced as strings and mapped to
/// `ServerFnError` at the server-function boundary.
type R<T> = Result<T, String>;

// ---------------------------------------------------------------------------
// Database pool
// ---------------------------------------------------------------------------

static POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Lazily-initialised shared Postgres pool. Reads `DATABASE_URL` (the Supabase
/// connection string with the service role). Created once per process.
async fn pool() -> R<&'static PgPool> {
    POOL.get_or_try_init(|| async {
        let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set".to_string())?;
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .map_err(|e| e.to_string())
    })
    .await
}

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
struct Claims {
    sub: String,
    #[allow(dead_code)]
    exp: usize,
}

/// Verify a Supabase access token (HS256, signed with the project JWT secret)
/// and return the authenticated user id. The token is passed as an explicit
/// argument to each authenticated server function and verified here.
pub fn authed(token: &str) -> R<Uuid> {
    let secret =
        std::env::var("SUPABASE_JWT_SECRET").map_err(|_| "SUPABASE_JWT_SECRET not set".to_string())?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|e| format!("invalid token: {e}"))?;
    Uuid::parse_str(&data.claims.sub).map_err(|_| "invalid token subject".to_string())
}

// ---------------------------------------------------------------------------
// Profiles
// ---------------------------------------------------------------------------

/// 3-20 chars, ASCII alphanumeric or underscore. Mirrors the DB CHECK so we can
/// reject early with a friendly message before hitting the unique constraint.
fn valid_handle(handle: &str) -> bool {
    let len = handle.chars().count();
    (3..=20).contains(&len) && handle.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Create the profile (and its streak row) on first login if absent. Idempotent.
pub async fn ensure_profile(uid: Uuid, handle: &str, timezone: &str) -> R<()> {
    if !valid_handle(handle) {
        return Err("handle must be 3-20 letters, digits or underscores".to_string());
    }
    let pool = pool().await?;
    sqlx::query(
        "insert into profiles (id, handle, timezone) values ($1, $2, $3) \
         on conflict (id) do nothing",
    )
    .bind(uid)
    .bind(handle)
    .bind(timezone)
    .execute(pool)
    .await
    .map_err(|e| format!("could not create profile (handle may be taken): {e}"))?;

    sqlx::query("insert into streaks (user_id) values ($1) on conflict (user_id) do nothing")
        .bind(uid)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Game engine
// ---------------------------------------------------------------------------

/// Mark a day complete and award its XP, idempotently and transactionally.
/// Re-completing the same day grants 0 XP (ON CONFLICT DO NOTHING on both the
/// progress row and the ledger entry).
pub async fn complete_day(uid: Uuid, day: i64) -> R<()> {
    if !(1..=548).contains(&day) {
        return Err("day out of range (1-548)".to_string());
    }
    let pool = pool().await?;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    sqlx::query(
        "insert into progress (user_id, day, status, completed_at) \
         values ($1, $2, 'completed', now()) on conflict (user_id, day) do nothing",
    )
    .bind(uid)
    .bind(day)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    award_xp(&mut tx, uid, 35, "day_complete", &format!("day:{day}")).await?;
    update_streak(&mut tx, uid).await?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

/// One-shot legacy import of the browser `qrt_state`. Validates the one-shot
/// guard (`legacy_migrated`), caps the day count, and writes idempotent
/// day-complete entries for days 1..completed (CLAUDE.md 4.9). Reusing the
/// `day_complete` reason + `day:N` ref means importing then completing the same
/// day never double-awards.
pub async fn import_local_progress(uid: Uuid, current_day: i64) -> R<()> {
    let pool = pool().await?;
    let migrated: Option<bool> =
        sqlx::query_scalar("select legacy_migrated from profiles where id = $1")
            .bind(uid)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;
    match migrated {
        None => return Err("create a profile before importing".to_string()),
        Some(true) => return Err("legacy progress already imported".to_string()),
        Some(false) => {}
    }

    let completed = completed_days_from_legacy(current_day);
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    for day in 1..=completed {
        sqlx::query(
            "insert into progress (user_id, day, status, completed_at) \
             values ($1, $2, 'completed', now()) on conflict (user_id, day) do nothing",
        )
        .bind(uid)
        .bind(day)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
        award_xp(&mut tx, uid, 35, "day_complete", &format!("day:{day}")).await?;
    }
    sqlx::query("update profiles set legacy_migrated = true where id = $1")
        .bind(uid)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Compute the user's summary (xp, level, streak, progress, est completion).
pub async fn me_summary(uid: Uuid) -> R<MeSummary> {
    let pool = pool().await?;
    let prow = sqlx::query("select handle, title from profiles where id = $1")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
    let Some(prow) = prow else {
        return Err("profile not found".to_string());
    };
    let handle: String = prow.get("handle");
    let title: String = prow.get("title");

    let xp: i64 = sqlx::query_scalar(
        "select coalesce(sum(amount), 0)::bigint from xp_ledger where user_id = $1",
    )
    .bind(uid)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let completed_days: i64 = sqlx::query_scalar(
        "select count(*) from progress where user_id = $1 and status = 'completed'",
    )
    .bind(uid)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let srow = sqlx::query("select current, longest from streaks where user_id = $1")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
    let (streak_current, streak_longest) = match srow {
        Some(r) => (r.get::<i32, _>("current") as i64, r.get::<i32, _>("longest") as i64),
        None => (0, 0),
    };

    let remaining = (548 - completed_days).max(0);
    let est = (Utc::now().date_naive() + Duration::days(remaining)).to_string();

    Ok(MeSummary {
        handle,
        title,
        xp,
        level: level_for_xp(xp),
        current_day: (completed_days + 1).min(548),
        completed_days,
        streak_current,
        streak_longest,
        progress_pct: completed_days as f64 / 548.0 * 100.0,
        est_completion: Some(est),
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Append an idempotent XP award inside a transaction.
async fn award_xp(
    tx: &mut Transaction<'_, Postgres>,
    uid: Uuid,
    amount: i32,
    reason: &str,
    ref_id: &str,
) -> R<()> {
    // `reason` is a fixed enum literal chosen by the caller, not user input.
    let sql = format!(
        "insert into xp_ledger (user_id, amount, reason, ref_id) \
         values ($1, $2, '{reason}', $3) on conflict (user_id, reason, ref_id) do nothing"
    );
    sqlx::query(&sql)
        .bind(uid)
        .bind(amount)
        .bind(ref_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Bump the streak for "any qualifying activity today" (UTC for slice 1;
/// profile-timezone awareness and freeze auto-apply land in a later slice).
async fn update_streak(tx: &mut Transaction<'_, Postgres>, uid: Uuid) -> R<()> {
    sqlx::query("insert into streaks (user_id) values ($1) on conflict (user_id) do nothing")
        .bind(uid)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

    let row = sqlx::query(
        "select current, longest, last_active_date from streaks where user_id = $1 for update",
    )
    .bind(uid)
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| e.to_string())?;
    let current: i32 = row.get("current");
    let longest: i32 = row.get("longest");
    let last: Option<NaiveDate> = row.get("last_active_date");

    let today = Utc::now().date_naive();
    let new_current = streak_after(last, today, current);
    let new_longest = longest.max(new_current);

    sqlx::query(
        "update streaks set current = $1, longest = $2, last_active_date = $3 where user_id = $4",
    )
    .bind(new_current)
    .bind(new_longest)
    .bind(today)
    .bind(uid)
    .execute(&mut **tx)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// XP -> level. Level 1 at 0 XP, level 2 at 100, then ~100*level^1.6 cumulative
/// (CLAUDE.md 6). Pure function so it is unit-testable without a database.
pub fn level_for_xp(xp: i64) -> i64 {
    let mut level = 1i64;
    while level < 999 && xp >= level_threshold(level + 1) {
        level += 1;
    }
    level
}

/// Cumulative XP required to reach `level`. Anchored so level 1 = 0 and
/// level 2 = 100 XP, then `100 * (level-1)^1.6` (CLAUDE.md 6).
fn level_threshold(level: i64) -> i64 {
    if level <= 1 {
        0
    } else {
        (100.0 * ((level - 1) as f64).powf(1.6)).round() as i64
    }
}

/// Completed days implied by the legacy `current_day` (completed = day - 1),
/// clamped to the valid range. Pure so the import cap is unit-testable.
fn completed_days_from_legacy(current_day: i64) -> i64 {
    (current_day - 1).clamp(0, 548)
}

/// Next streak value given the last active date, today, and the current streak.
/// Same day -> unchanged; consecutive day -> +1; any gap (or first ever) -> 1.
/// Pure (no DB / no clock) so the timezone-agnostic edges are unit-testable.
fn streak_after(last: Option<NaiveDate>, today: NaiveDate, current: i32) -> i32 {
    match last {
        Some(d) if d == today => current,
        Some(d) if d == today - Duration::days(1) => current + 1,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_curve_is_monotonic_and_anchored() {
        assert_eq!(level_for_xp(0), 1);
        assert_eq!(level_for_xp(99), 1);
        assert_eq!(level_for_xp(100), 2); // level 2 at 100 XP (anchor)
        // Monotonic non-decreasing in XP.
        let mut last = 0;
        for xp in (0..50_000).step_by(250) {
            let l = level_for_xp(xp);
            assert!(l >= last, "level decreased at xp={xp}");
            last = l;
        }
    }

    #[test]
    fn handle_validation() {
        assert!(valid_handle("quant_kid"));
        assert!(valid_handle("abc"));
        assert!(!valid_handle("ab")); // too short
        assert!(!valid_handle("has space"));
        assert!(!valid_handle("bad-dash"));
        assert!(!valid_handle(&"x".repeat(21))); // too long
    }

    #[test]
    fn streak_transitions() {
        let today = NaiveDate::from_ymd_opt(2026, 6, 13).unwrap();
        let yesterday = today - Duration::days(1);
        let week_ago = today - Duration::days(7);
        // First ever activity.
        assert_eq!(streak_after(None, today, 0), 1);
        // Already counted today -> unchanged.
        assert_eq!(streak_after(Some(today), today, 5), 5);
        // Consecutive day -> increment.
        assert_eq!(streak_after(Some(yesterday), today, 5), 6);
        // Gap -> reset to 1.
        assert_eq!(streak_after(Some(week_ago), today, 5), 1);
    }

    #[test]
    fn legacy_import_cap() {
        assert_eq!(completed_days_from_legacy(1), 0); // on day 1 => 0 completed
        assert_eq!(completed_days_from_legacy(8), 7);
        assert_eq!(completed_days_from_legacy(0), 0); // never negative
        assert_eq!(completed_days_from_legacy(10_000), 548); // capped
    }
}
