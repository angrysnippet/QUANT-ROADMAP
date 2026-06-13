# Problem Content Schema - Quant Arena

Formal definition of a problem file (CLAUDE.md Section 8). Problems are authored
as versioned files in this directory and seeded into Postgres by a script
(Phase 1+). **The seeder strips every server-only field from any client-readable
artifact** - the WASM client must never receive answers or graders before
grading (CLAUDE.md Sections 4.4 and 8).

## File format

Each problem is one TOML file: `problems/<track>/<id>.toml`. It has a public
section (safe to ship to the client) and a `[server]` section (never shipped).

### Public fields

| Field          | Type                                  | Notes |
|----------------|---------------------------------------|-------|
| `id`           | string, unique, kebab-case            | Stable identity; used for idempotent XP and revision scheduling. |
| `track`        | enum: `dsa` \| `probability` \| `quantfin` \| `mentalmath` | Arena/rating track. |
| `world`        | int 1-9                               | Maps to a CLAUDE.md Section 5 world. |
| `day`          | int 1-548, or null                    | Optional link to a curriculum day. |
| `difficulty`   | int 1-5                               | Drives XP (10/20/30/40/50) and anti-cheat thresholds. |
| `type`         | enum: `mcq` \| `numeric` \| `code`    | Determines grading mode. |
| `statement`    | string (markdown)                     | The problem text shown to the player. |
| `choices`      | array of strings                      | Required for `mcq`; omit otherwise. Order is presentation order. |
| `tags`         | array of strings                      | For Practice filters and revision. |
| `time_limit_seconds` | int                             | Soft per-question limit; also used by timing heuristics. |

### `[server]` section (server-only - stripped by the seeder)

| Field                   | Type                       | Notes |
|-------------------------|----------------------------|-------|
| `answer`                | int (mcq, 0-based index) / number (numeric) / string (code expected-output or test ref) | The graded answer. Never leaves the server. |
| `tolerance`             | number                     | Numeric only: abs tolerance for a correct match. |
| `solution_explanation`  | string (markdown)          | Revealed only after a graded submission (CLAUDE.md Section 4.4). |

## Grading by type

- **mcq:** correct iff submitted choice index == `server.answer`.
- **numeric:** correct iff `abs(submitted - server.answer) <= server.tolerance`.
- **code:** Phase 0 placeholder - compare normalized stdout to `server.answer`
  (full code execution/sandboxing is a later-phase concern; logged in BACKLOG).

## Validation rules (seeder rejects on failure)

1. `id` unique across all problem files; matches `^[a-z0-9-]+$`.
2. `track` and `type` are in their enums; `difficulty` in 1-5; `world` in 1-9.
3. `mcq` => `choices` non-empty and `server.answer` is a valid index into it.
4. `numeric` => `server.answer` is a number and `server.tolerance >= 0`.
5. `code` => `server.answer` present (string).
6. `[server]` section present; after stripping it, no answer/grader data remains
   in the client artifact.
7. `statement` non-empty; user-facing strings sanitized before storage/render.

## Sample files

Three samples live alongside this schema, one per type:

- `probability/two-dice-sum-vs-four.toml` (mcq)
- `dsa/array-search-complexity.toml` (numeric)
- `dsa/sum-of-array.toml` (code)

They are original wording inspired by the early curriculum (Days 1-2). They
exist to exercise the format; the real seed bank (40 problems) is Phase 3.
