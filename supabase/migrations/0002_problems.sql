-- Quant Arena - Phase 1, Slice 2: problem bank + graded submissions.
--
-- Answers are SEALED (CLAUDE.md 4.4): the `answer_*` and `solution_explanation`
-- columns live on `problems`, but RLS exposes NO select policy to clients, so
-- the table is unreadable from the browser. All reads go through the
-- `list_problems` server function (service role), which selects public columns
-- only; grading happens server-side in `submit_problem`. Submissions are
-- append-only and server-timestamped (anti-cheat 4.5).

-- ---------------------------------------------------------------------------
-- problems  (public columns + sealed answer columns)
-- ---------------------------------------------------------------------------
create table problems (
  id                    text primary key,
  track                 text not null
                          check (track in ('dsa', 'probability', 'quantfin', 'mentalmath')),
  world                 int  not null check (world between 1 and 9),
  day                   int,
  difficulty            int  not null check (difficulty between 1 and 5),
  kind                  text not null check (kind in ('mcq', 'numeric', 'code')),
  statement             text not null,
  choices               text[] not null default '{}',  -- choices (mcq)
  tags                  text[] not null default '{}',
  time_limit_seconds    int  not null default 120,
  -- SEALED (server-only): never selected by any client path.
  answer_idx            int,                -- mcq: correct 0-based index
  answer_num            double precision,   -- numeric: expected value
  answer_tol            double precision,   -- numeric: abs tolerance
  answer_text           text,               -- code: expected normalized stdout
  solution_explanation  text not null default '',
  created_at            timestamptz not null default now()
);

-- ---------------------------------------------------------------------------
-- submissions  (append-only graded attempts)
-- ---------------------------------------------------------------------------
create table submissions (
  id         bigint generated always as identity primary key,
  user_id    uuid not null references profiles (id) on delete cascade,
  problem_id text not null references problems (id) on delete cascade,
  submitted  text not null,          -- the user's raw answer (sanitised)
  correct    boolean not null,
  created_at timestamptz not null default now()  -- server timestamp (anti-cheat)
);

create index submissions_user_idx on submissions (user_id);
create index submissions_problem_idx on submissions (problem_id);

-- ---------------------------------------------------------------------------
-- Row Level Security
-- ---------------------------------------------------------------------------
-- problems: RLS on, NO select/write policy for clients -> the table is sealed;
-- only the service role (server) can read answers or seed rows.
alter table problems enable row level security;

-- submissions: users may read their own attempts; writes are service-role only.
alter table submissions enable row level security;
create policy submissions_select_own on submissions
  for select using (auth.uid() = user_id);
