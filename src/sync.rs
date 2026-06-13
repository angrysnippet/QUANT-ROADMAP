//! Client-facing wrappers over the server functions, with an offline fallback.
//!
//! Under `fullstack` these call the real server functions. In the plain `web`
//! build (no fullstack - e.g. CI's default `cargo build`, or an offline static
//! bundle) the server functions do not exist, so these degrade to an error and
//! the UI falls back to local-only behaviour. Centralising the `cfg` here keeps
//! `auth.rs` and `today.rs` free of feature gates.

use crate::api::MeSummary;

#[cfg(feature = "fullstack")]
pub async fn ensure_profile(token: String, handle: String, tz: String) -> Result<MeSummary, String> {
    crate::server_fns::ensure_profile(token, handle, tz)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(feature = "fullstack"))]
pub async fn ensure_profile(_t: String, _h: String, _tz: String) -> Result<MeSummary, String> {
    Err("sync unavailable in this build".to_string())
}

#[cfg(feature = "fullstack")]
pub async fn me_summary(token: String) -> Result<MeSummary, String> {
    crate::server_fns::me_summary(token).await.map_err(|e| e.to_string())
}

#[cfg(not(feature = "fullstack"))]
pub async fn me_summary(_t: String) -> Result<MeSummary, String> {
    Err("sync unavailable in this build".to_string())
}

#[cfg(feature = "fullstack")]
pub async fn complete_day(token: String, day: i64) -> Result<MeSummary, String> {
    crate::server_fns::complete_day(token, day).await.map_err(|e| e.to_string())
}

#[cfg(not(feature = "fullstack"))]
pub async fn complete_day(_t: String, _day: i64) -> Result<MeSummary, String> {
    Err("sync unavailable in this build".to_string())
}

#[cfg(feature = "fullstack")]
pub async fn import_local_progress(token: String, current_day: i64) -> Result<MeSummary, String> {
    crate::server_fns::import_local_progress(token, current_day)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(feature = "fullstack"))]
pub async fn import_local_progress(_t: String, _d: i64) -> Result<MeSummary, String> {
    Err("sync unavailable in this build".to_string())
}
