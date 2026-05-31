//! In-browser code runner for the Practice view. Bridges to JS engines via
//! Dioxus `document::eval`: Python through Pyodide (dynamic import) and C++
//! through JSCPP (CDN script). User code/stdin are passed as data (never
//! string-interpolated into JS). Ported from spec.html's run logic.
//!
//! Both engines load from a CDN on first use, so running requires an internet
//! connection; failures are reported in `RunResult.error` rather than panicking.

use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Default, Deserialize)]
pub struct RunResult {
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub exit: Option<i64>,
}

/// Run `code` (lang = "cpp" | "py") with `stdin`, returning captured output.
pub async fn run_code(lang: &str, code: String, stdin: String) -> RunResult {
    let mut eval = document::eval(RUN_JS);
    if let Err(e) = eval.send(serde_json::json!({ "lang": lang, "code": code, "stdin": stdin })) {
        return RunResult {
            error: Some(format!("could not start runner: {e}")),
            ..Default::default()
        };
    }
    match eval.recv::<String>().await {
        Ok(json) => serde_json::from_str(&json).unwrap_or(RunResult {
            error: Some("runner returned malformed result".into()),
            ..Default::default()
        }),
        Err(e) => RunResult {
            error: Some(format!("runner failed: {e}")),
            ..Default::default()
        },
    }
}

/// Normalize program output for comparison with the expected sample. Ported
/// from spec's `normalizeOut`: strip `\r`, trim trailing whitespace per line,
/// trim trailing blank lines, then trim the whole thing.
pub fn normalize_out(s: &str) -> String {
    let joined = s
        .replace('\r', "")
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    joined.trim_end_matches('\n').trim().to_string()
}

const RUN_JS: &str = r##"
const { lang, code, stdin } = await dioxus.recv();

const normErr = (e) => String((e && e.message) || e);

function loadScriptOnce(url) {
  return new Promise((resolve) => {
    const s = document.createElement('script');
    s.src = url; s.async = true;
    let done = false;
    const finish = (ok) => { if (done) return; done = true; if (!ok && s.parentNode) s.parentNode.removeChild(s); resolve(ok); };
    s.onload = () => finish(true);
    s.onerror = () => finish(false);
    setTimeout(() => finish(false), 12000);
    document.head.appendChild(s);
  });
}

const JSCPP_URLS = [
  'https://cdn.jsdelivr.net/npm/JSCPP@2.0.0/dist/JSCPP.es5.min.js',
  'https://cdn.jsdelivr.net/npm/JSCPP/dist/JSCPP.es5.min.js',
  'https://unpkg.com/JSCPP@2.0.0/dist/JSCPP.es5.min.js',
  'https://unpkg.com/JSCPP/dist/JSCPP.es5.min.js',
  'https://cdn.jsdelivr.net/npm/jscpp/dist/JSCPP.es5.min.js',
  'https://unpkg.com/jscpp/dist/JSCPP.es5.min.js'
];

async function ensureJSCPP() {
  if (typeof window.JSCPP !== 'undefined') return true;
  for (const url of JSCPP_URLS) {
    const ok = await loadScriptOnce(url);
    if (ok && typeof window.JSCPP !== 'undefined') return true;
  }
  return false;
}

async function ensurePyodide() {
  if (window.__qrPyodide) return window.__qrPyodide;
  const mod = await import('https://cdn.jsdelivr.net/pyodide/v0.26.2/full/pyodide.mjs');
  window.__qrPyodide = await mod.loadPyodide();
  return window.__qrPyodide;
}

async function runCpp(code, stdin) {
  const ok = await ensureJSCPP();
  if (!ok) return { output: '', error: 'Could not load the C++ engine. This needs an internet connection the first time — check your connection and run again.', exit: null };
  let out = '';
  const config = { stdio: { write: (s) => { out += s; } }, unsigned_overflow: 'warn' };
  try {
    const exitCode = window.JSCPP.run(code, stdin, config);
    return { output: out, error: null, exit: exitCode };
  } catch (e) {
    return { output: out, error: normErr(e), exit: null };
  }
}

async function runPy(code, stdin) {
  let py;
  try { py = await ensurePyodide(); }
  catch (e) { return { output: '', error: 'Python engine failed to load. Check your connection and retry.', exit: null }; }
  py.runPython(`import sys, io\nsys.stdin=io.StringIO(${JSON.stringify(stdin)})\n_OUT=io.StringIO()\nsys.stdout=_OUT\nsys.stderr=_OUT`);
  let err = null;
  try { py.runPython(code); }
  catch (e) { err = normErr(e); }
  const out = py.runPython('_OUT.getvalue()');
  py.runPython('import sys\nsys.stdout=sys.__stdout__\nsys.stderr=sys.__stderr__\nsys.stdin=sys.__stdin__');
  if (err) {
    const tail = err.split('\n').filter((l) => l.trim()).slice(-6).join('\n');
    return { output: out, error: tail, exit: null };
  }
  return { output: out, error: null, exit: null };
}

let result;
try {
  result = lang === 'cpp' ? await runCpp(code, stdin) : await runPy(code, stdin);
} catch (e) {
  result = { output: '', error: normErr(e), exit: null };
}
dioxus.send(JSON.stringify(result));
"##;
