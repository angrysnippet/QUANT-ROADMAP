-- Quant Arena - Phase 1, Slice 1 schema.
--
-- Server-authoritative game state (CLAUDE.md Sections 4 and 6). Identity comes
-- from Supabase Auth (auth.users). Every table has Row Level Security: a normal
-- (authenticated/anon) client may read ONLY its own rows and NEVER write. All
-- writes happen in server functions using the service-role key, which bypasses
-- RLS. XP is an append-only ledger; totals are SUM(amount), never a column.

-- ---------------------------------------------------------------------------
-- Enums
-- ---------------------------------------------------------------------------
create type day_status as enum ('in_progress', 'completed');

-- Slice 1 uses 'day_complete' and 'legacy_import'; the rest are reserved so the
-- enum is stable for later slices/phases.
create type xp_reason as enum (
  'day_complete',
  'problem_solved',
  'daily_quest',
  'duel_win',
  'boss_project',
  'legacy_import'
);

-- ---------------------------------------------------------------------------
-- profiles
-- ---------------------------------------------------------------------------
create table profiles (
  id              uuid primary key references auth.users (id) on delete cascade,
  handle          text unique not null
                    check (handle ~ '^[a-zA-Z0-9_]{3,20}$'),
  avatar          text,
  title           text not null default 'Intern',
  timezone        text not null default 'UTC',
  legacy_migrated boolean not null default false,
  created_at      timestamptz not null default now()
);

-- ---------------------------------------------------------------------------
-- progress  (one row per completed/started day)
-- ---------------------------------------------------------------------------
create table progress (
  id           bigint generated always as identity primary key,
  user_id      uuid not null references profiles (id) on delete cascade,
  day          int  not null check (day between 1 and 548),
  status       day_status not null default 'completed',
  completed_at timestamptz,
  unique (user_id, day)
);

create index progress_user_idx on progress (user_id);

-- ---------------------------------------------------------------------------
-- xp_ledger  (APPEND-ONLY - never updated, never deleted)
-- ---------------------------------------------------------------------------
-- ref_id scopes the award (e.g. 'day:7', 'problem:two-dice-...'). The unique
-- (user_id, reason, ref_id) constraint makes awards idempotent at the database
-- level: server functions insert with ON CONFLICT DO NOTHING, so re-completing
-- a day or re-submitting a solved problem grants 0 XP (CLAUDE.md 4.2).
create table xp_ledger (
  id         bigint generated always as identity primary key,
  user_id    uuid not null references profiles (id) on delete cascade,
  amount     int  not null check (amount >= 0),
  reason     xp_reason not null,
  ref_id     text not null,
  created_at timestamptz not null default now(),
  unique (user_id, reason, ref_id)
);

create index xp_ledger_user_idx on xp_ledger (user_id);

-- ---------------------------------------------------------------------------
-- streaks  (one row per user)
-- ---------------------------------------------------------------------------
create table streaks (
  user_id          uuid primary key references profiles (id) on delete cascade,
  current          int  not null default 0,
  longest          int  not null default 0,
  last_active_date date,
  freezes_banked   int  not null default 0 check (freezes_banked between 0 and 2)
);

-- ---------------------------------------------------------------------------
-- Row Level Security
-- ---------------------------------------------------------------------------
-- Enable RLS everywhere. With RLS on and NO write policies, authenticated/anon
-- clients cannot INSERT/UPDATE/DELETE at all; only the service role (used by the
-- server) can write. Read policies expose a user only their own rows.
alter table profiles  enable row level security;
alter table progress  enable row level security;
alter table xp_ledger enable row level security;
alter table streaks   enable row level security;

create policy profiles_select_own on profiles
  for select using (auth.uid() = id);

create policy progress_select_own on progress
  for select using (auth.uid() = user_id);

create policy xp_ledger_select_own on xp_ledger
  for select using (auth.uid() = user_id);

create policy streaks_select_own on streaks
  for select using (auth.uid() = user_id);

-- ---------------------------------------------------------------------------
-- Leaderboard view (STUB - Phase 4 builds the real cached/materialized one)
-- ---------------------------------------------------------------------------
-- Aggregates public-safe columns only (handle, title, total XP). Full public
-- exposure / friends scoping is a Phase 4 concern; kept here so me_summary and
-- early UI have something to read.
create view leaderboard as
  select
    p.id,
    p.handle,
    p.title,
    coalesce(sum(x.amount), 0)::bigint as xp
  from profiles p
  left join xp_ledger x on x.user_id = p.id
  group by p.id, p.handle, p.title;
