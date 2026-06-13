//! Problem seeder: parse `problems/**/*.toml` and upsert into the `problems`
//! table. Server-only binary (built only with the `server` feature):
//!
//!   DATABASE_URL=... cargo run --bin seed --no-default-features --features server
//!
//! Public fields seed the public columns; the `[server]` answer/solution fields
//! go into the SEALED server-only columns, so no client artifact ever contains
//! an answer (CLAUDE.md 8). Re-running updates existing rows (upsert by id).

use std::path::{Path, PathBuf};

use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize)]
struct ServerSec {
    answer: toml::Value,
    #[serde(default)]
    tolerance: Option<f64>,
    #[serde(default)]
    solution_explanation: String,
}

#[derive(Deserialize)]
struct ProblemFile {
    id: String,
    track: String,
    world: i64,
    #[serde(default)]
    day: Option<i64>,
    difficulty: i64,
    #[serde(rename = "type")]
    kind: String,
    statement: String,
    #[serde(default)]
    choices: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    time_limit_seconds: i64,
    server: ServerSec,
}

/// A file holding many problems under a `[[problems]]` array.
#[derive(Deserialize)]
struct MultiFile {
    #[serde(default)]
    problems: Vec<ProblemFile>,
}

fn collect_toml(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                collect_toml(&p, out);
            } else if p.extension().is_some_and(|x| x == "toml") {
                out.push(p);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL not set")?;
    let root = std::env::args().nth(1).unwrap_or_else(|| "problems".to_string());

    let mut files = Vec::new();
    collect_toml(Path::new(&root), &mut files);
    if files.is_empty() {
        eprintln!("no .toml problems found under {root}");
        return Ok(());
    }

    let pool = PgPoolOptions::new().max_connections(2).connect(&url).await?;
    let mut n = 0;
    for f in &files {
        let src = f.display().to_string();
        let text = std::fs::read_to_string(f)?;
        // A file may hold a single problem, or many under a [[problems]] array.
        let has_array = toml::from_str::<toml::Value>(&text)
            .map_err(|e| format!("{src}: {e}"))?
            .get("problems")
            .is_some();
        let problems: Vec<ProblemFile> = if has_array {
            toml::from_str::<MultiFile>(&text).map_err(|e| format!("{src}: {e}"))?.problems
        } else {
            vec![toml::from_str::<ProblemFile>(&text).map_err(|e| format!("{src}: {e}"))?]
        };
        for p in &problems {
            upsert_problem(&pool, &src, p).await?;
            n += 1;
        }
    }
    println!("done: {n} problem(s) seeded");
    Ok(())
}

/// Validate one problem and upsert it (answers into the sealed columns).
async fn upsert_problem(
    pool: &sqlx::PgPool,
    src: &str,
    p: &ProblemFile,
) -> Result<(), Box<dyn std::error::Error>> {
    if !["dsa", "probability", "quantfin", "mentalmath"].contains(&p.track.as_str()) {
        return Err(format!("{src}: bad track '{}' ({})", p.track, p.id).into());
    }
    if !["mcq", "numeric", "code"].contains(&p.kind.as_str()) {
        return Err(format!("{src}: bad type '{}' ({})", p.kind, p.id).into());
    }
    if !(1..=5).contains(&p.difficulty) {
        return Err(format!("{src}: difficulty out of range ({})", p.id).into());
    }

    // Derive sealed answer columns from the [server] section by kind.
    let (answer_idx, answer_num, answer_tol, answer_text) = match p.kind.as_str() {
        "mcq" => (p.server.answer.as_integer().map(|i| i as i32), None, None, None),
        "numeric" => {
            let num = p
                .server
                .answer
                .as_float()
                .or_else(|| p.server.answer.as_integer().map(|i| i as f64));
            (None, num, Some(p.server.tolerance.unwrap_or(0.0)), None)
        }
        "code" => (None, None, None, p.server.answer.as_str().map(str::to_string)),
        _ => unreachable!(),
    };
    if p.kind == "mcq" && (answer_idx.is_none() || p.choices.is_empty()) {
        return Err(format!("{src}: mcq needs choices + an integer answer ({})", p.id).into());
    }
    if p.kind == "numeric" && answer_num.is_none() {
        return Err(format!("{src}: numeric needs a numeric answer ({})", p.id).into());
    }
    if p.kind == "code" && answer_text.is_none() {
        return Err(format!("{src}: code needs a string answer ({})", p.id).into());
    }

    sqlx::query(
        "insert into problems \
         (id, track, world, day, difficulty, kind, statement, choices, tags, \
          time_limit_seconds, answer_idx, answer_num, answer_tol, answer_text, \
          solution_explanation) \
         values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15) \
         on conflict (id) do update set \
          track=excluded.track, world=excluded.world, day=excluded.day, \
          difficulty=excluded.difficulty, kind=excluded.kind, \
          statement=excluded.statement, choices=excluded.choices, \
          tags=excluded.tags, time_limit_seconds=excluded.time_limit_seconds, \
          answer_idx=excluded.answer_idx, answer_num=excluded.answer_num, \
          answer_tol=excluded.answer_tol, answer_text=excluded.answer_text, \
          solution_explanation=excluded.solution_explanation",
    )
    .bind(&p.id)
    .bind(&p.track)
    .bind(p.world as i32)
    .bind(p.day.map(|d| d as i32))
    .bind(p.difficulty as i32)
    .bind(&p.kind)
    .bind(&p.statement)
    .bind(&p.choices)
    .bind(&p.tags)
    .bind(p.time_limit_seconds as i32)
    .bind(answer_idx)
    .bind(answer_num)
    .bind(answer_tol)
    .bind(answer_text)
    .bind(&p.server.solution_explanation)
    .execute(pool)
    .await?;
    println!("seeded {}", p.id);
    Ok(())
}
