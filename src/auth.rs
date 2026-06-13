//! Client-side Supabase Auth interop (browser only) + the Login page.
//!
//! The browser talks to Supabase Auth directly (email/password + Google/GitHub
//! OAuth) via the official JS SDK, loaded on demand through `document::eval`.
//! We only keep the resulting access token, which is then passed to the
//! authenticated server functions and verified server-side. The anon URL/key
//! are public (safe in the client); the service-role key never lives here.
//!
//! Config comes from compile-time env: build the client with `SUPABASE_URL` and
//! `SUPABASE_ANON_KEY` set (see docs/DEV_SETUP.md). Without them the app still
//! builds and runs offline; auth calls just fail gracefully.

use dioxus::prelude::*;
use serde_json::{json, Value};

use crate::sync;
use crate::Route;

const SUPABASE_URL: Option<&str> = option_env!("SUPABASE_URL");
const SUPABASE_ANON_KEY: Option<&str> = option_env!("SUPABASE_ANON_KEY");

/// Shared auth token signal, provided at the app root. `None` = signed out.
pub fn use_token() -> Signal<Option<String>> {
    use_context::<Signal<Option<String>>>()
}

/// Shared server-summary signal, provided at the app root. `None` = not loaded
/// (signed out or still fetching). Pages may write it after a server action so
/// the shell reflects the change immediately.
pub fn use_me() -> Signal<Option<crate::api::MeSummary>> {
    use_context::<Signal<Option<crate::api::MeSummary>>>()
}

/// Serialize a Rust string as a safe JS string literal.
fn js_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

/// JS that lazily creates `window.__qa_supabase` (idempotent), wrapped per call
/// so we never depend on init ordering.
fn prelude() -> String {
    let mut s = String::new();
    s.push_str("if (!window.__qa_supabase) {");
    s.push_str("const m = await import('https://esm.sh/@supabase/supabase-js@2');");
    s.push_str("window.__qa_supabase = m.createClient(");
    s.push_str(&js_str(SUPABASE_URL.unwrap_or("")));
    s.push(',');
    s.push_str(&js_str(SUPABASE_ANON_KEY.unwrap_or("")));
    s.push_str(");}");
    s.push_str("const sb = window.__qa_supabase;");
    s
}

/// Run a Supabase op `body` (which must call `dioxus.send({ok, ...})`) and
/// return the decoded JSON result, with the client prelude + error handling.
async fn run(body: &str) -> Value {
    let mut script = String::new();
    script.push_str("try {");
    script.push_str(&prelude());
    script.push_str(body);
    script.push_str("} catch (e) { dioxus.send({ ok:false, msg:String(e) }); }");

    let mut eval = document::eval(&script);
    match eval.recv::<Value>().await {
        Ok(v) => v,
        Err(e) => json!({ "ok": false, "msg": format!("eval error: {e}") }),
    }
}

fn token_result(v: Value) -> Result<String, String> {
    if v.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        Ok(v.get("token").and_then(Value::as_str).unwrap_or("").to_string())
    } else {
        Err(v.get("msg").and_then(Value::as_str).unwrap_or("auth failed").to_string())
    }
}

/// Current access token from the persisted Supabase session, if signed in.
pub async fn current_token() -> Option<String> {
    let v = run("const { data } = await sb.auth.getSession(); \
                 dioxus.send({ ok:true, token: data.session ? data.session.access_token : null });")
        .await;
    v.get("token").and_then(Value::as_str).map(str::to_string)
}

/// The browser's IANA timezone (e.g. "Asia/Kolkata"), or "UTC".
pub async fn timezone() -> String {
    let v = run("dioxus.send({ ok:true, tz: (Intl.DateTimeFormat().resolvedOptions().timeZone || 'UTC') });").await;
    v.get("tz").and_then(Value::as_str).unwrap_or("UTC").to_string()
}

pub async fn sign_in_password(email: &str, password: &str) -> Result<String, String> {
    let mut body = String::from("const c = ");
    body.push_str(&json!({ "email": email, "password": password }).to_string());
    body.push_str(
        ";const { data, error } = await sb.auth.signInWithPassword({ email:c.email, password:c.password });\
         if (error) { dioxus.send({ ok:false, msg:error.message }); } \
         else { dioxus.send({ ok:true, token:data.session.access_token }); }",
    );
    token_result(run(&body).await)
}

pub async fn sign_up_password(email: &str, password: &str) -> Result<String, String> {
    let mut body = String::from("const c = ");
    body.push_str(&json!({ "email": email, "password": password }).to_string());
    body.push_str(
        ";const { data, error } = await sb.auth.signUp({ email:c.email, password:c.password });\
         if (error) { dioxus.send({ ok:false, msg:error.message }); } \
         else if (!data.session) { dioxus.send({ ok:false, msg:'Account created - confirm via email, then sign in.' }); } \
         else { dioxus.send({ ok:true, token:data.session.access_token }); }",
    );
    token_result(run(&body).await)
}

/// Start an OAuth redirect (provider = "google" | "github"). The browser
/// navigates away and returns with a session; `current_token` picks it up.
pub async fn sign_in_oauth(provider: &str) -> Result<(), String> {
    let mut body = String::from("const c = ");
    body.push_str(&json!({ "provider": provider }).to_string());
    body.push_str(
        ";const { error } = await sb.auth.signInWithOAuth({ provider:c.provider, options:{ redirectTo: window.location.href } });\
         if (error) { dioxus.send({ ok:false, msg:error.message }); } else { dioxus.send({ ok:true }); }",
    );
    let v = run(&body).await;
    if v.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        Ok(())
    } else {
        Err(v.get("msg").and_then(Value::as_str).unwrap_or("oauth failed").to_string())
    }
}

pub async fn sign_out() {
    let _ = run("await sb.auth.signOut(); dioxus.send({ ok:true });").await;
}

/// Derive a fallback handle from an email local-part (3-20 chars, [A-Za-z0-9_]).
fn derive_handle(email: &str) -> String {
    let local = email.split('@').next().unwrap_or("player");
    let mut h: String = local.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    while h.chars().count() < 3 {
        h.push('0');
    }
    h.chars().take(20).collect()
}

/// After a successful auth: store the token, ensure a profile exists, navigate
/// in. A free fn taking Copy signals by value so each handler can call it
/// without sharing a closure.
async fn finish_auth(
    tok: String,
    email: String,
    handle: String,
    mut token: Signal<Option<String>>,
    mut error: Signal<String>,
) {
    token.set(Some(tok.clone()));
    let tz = timezone().await;
    let h = if handle.trim().is_empty() { derive_handle(&email) } else { handle };
    if let Err(msg) = sync::ensure_profile(tok, h, tz).await {
        error.set(format!("Signed in, but profile setup failed: {msg}"));
    }
    navigator().push(Route::Today {});
}

/// Login / sign-up page (route `/login`). Email-password plus Google/GitHub.
#[component]
pub fn Login() -> Element {
    let token = use_token();

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut handle = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut busy = use_signal(|| false);

    // Email/password handlers. Each is independent (no shared closure) and
    // captures only Copy signals.
    let do_sign_in = move |_| {
        let (e, p, h) = (email(), password(), handle());
        busy.set(true);
        error.set(String::new());
        spawn(async move {
            match sign_in_password(&e, &p).await {
                Ok(tok) => finish_auth(tok, e, h, token, error).await,
                Err(msg) => error.set(msg),
            }
            busy.set(false);
        });
    };

    let do_sign_up = move |_| {
        let (e, p, h) = (email(), password(), handle());
        busy.set(true);
        error.set(String::new());
        spawn(async move {
            match sign_up_password(&e, &p).await {
                Ok(tok) => finish_auth(tok, e, h, token, error).await,
                Err(msg) => error.set(msg),
            }
            busy.set(false);
        });
    };

    let oauth = move |provider: &'static str| {
        move |_| {
            error.set(String::new());
            spawn(async move {
                if let Err(msg) = sign_in_oauth(provider).await {
                    error.set(msg);
                }
            });
        }
    };

    rsx! {
        div { class: "view active", style: "max-width:420px;margin:0 auto;padding-top:3rem",
            div { class: "view-title", "Sign in to Quant Arena" }
            div { class: "view-sub", "Your XP, streak and progress sync to the server." }

            input {
                class: "qa-input", r#type: "email", placeholder: "Email",
                value: "{email}", oninput: move |ev| email.set(ev.value()),
            }
            input {
                class: "qa-input", r#type: "password", placeholder: "Password",
                value: "{password}", oninput: move |ev| password.set(ev.value()),
            }
            input {
                class: "qa-input", placeholder: "Handle (for sign-up; 3-20 letters/digits)",
                value: "{handle}", oninput: move |ev| handle.set(ev.value()),
            }

            if !error().is_empty() {
                div { style: "color:#ff6b6b;font-size:13px;margin:.5rem 0", "{error}" }
            }

            div { style: "display:flex;gap:.5rem;margin-top:.5rem",
                button { class: "strat-practice-btn", disabled: busy(), onclick: do_sign_in, "Sign in" }
                button { class: "strat-practice-btn", disabled: busy(), onclick: do_sign_up, "Sign up" }
            }

            div { style: "margin-top:1rem;display:flex;flex-direction:column;gap:.5rem",
                button { class: "cal-sched-open-btn", onclick: oauth("google"), "Continue with Google" }
                button { class: "cal-sched-open-btn", onclick: oauth("github"), "Continue with GitHub" }
            }

            Link { to: Route::Today {}, class: "cal-sched-open-btn", style: "margin-top:1rem;display:inline-block",
                "Continue offline (no sync)"
            }
        }
    }
}
