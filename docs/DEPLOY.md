# DEPLOY.md - Quant Arena (Phase 1)

Production runbook for the fullstack server on **Fly.io** + **Supabase**. The
legacy `web`-only GitHub Pages deploy (`.github/workflows/deploy.yml`) stays as-is
until a green Fly deploy is confirmed; only then do we retire/repoint it.

> Status: Slice 1. I (Claude) cannot run Supabase/Fly/wasm here, so these steps
> are written to be run by you. Confirm the `dx bundle` command and asset path
> against your local `dx serve --features server` before the first deploy.

## 1. Supabase project + schema

1. Create a Supabase project (hosted). Note from **Project Settings**:
   - API: `Project URL` (SUPABASE_URL), `anon public` key (SUPABASE_ANON_KEY),
     `JWT Secret` (SUPABASE_JWT_SECRET).
   - Database: the `URI` connection string (DATABASE_URL).
2. Apply migrations + seed the problem bank:
   ```sh
   # with the Supabase CLI, linked to your project:
   supabase link --project-ref YOUR-REF
   supabase db push        # applies 0001_init.sql + 0002_problems.sql
   # seed problems (answers go into the sealed server-only columns):
   DATABASE_URL='postgres://...' cargo run --bin seed --no-default-features --features server
   ```
   (Or paste the `supabase/migrations/*.sql` files into the SQL editor; then run
   the seeder.)
3. Enable auth providers (Authentication > Providers): Email, Google, GitHub.
   For Google/GitHub create OAuth apps and paste their client id/secret; set the
   redirect URL to `https://<your-domain>/` (and `http://localhost:8080/` for
   local dev). Add your site URL under Authentication > URL Configuration.

## 2. Build config (client env)

The client bakes in the public Supabase URL/key at build time. Export them
before `dx`/`docker build` (the Dockerfile passes the build env through):
```sh
export SUPABASE_URL=https://YOUR-PROJECT.supabase.co
export SUPABASE_ANON_KEY=YOUR-ANON-KEY
```

## 3. Fly.io

```sh
fly auth login
fly launch --no-deploy            # claim app name/region; keep the provided fly.toml
fly secrets set \
  DATABASE_URL="postgres://postgres:PASSWORD@db.YOUR-PROJECT.supabase.co:5432/postgres" \
  SUPABASE_JWT_SECRET="YOUR-JWT-SECRET" \
  SUPABASE_URL="https://YOUR-PROJECT.supabase.co" \
  SUPABASE_ANON_KEY="YOUR-ANON-KEY"
fly deploy
```
`fly deploy` builds the `Dockerfile` (client bundle + server binary) and runs it.
Verify: open the app URL, sign up, complete a day, confirm XP/streak persist
across reload, and that `/api/*` calls succeed (browser network tab).

## 4. Retire / repoint GitHub Pages (only after a green Fly deploy)

Once Fly serves the SSR app correctly, either delete
`.github/workflows/deploy.yml` or repoint the domain to Fly. SSR also fixes the
old empty-HTML-shell SEO/link-preview problem.

## Secrets & anti-cheat reminders

- Service-role/DB creds live ONLY in Fly secrets / local `.env` (gitignored),
  never in the WASM client (CLAUDE.md 4.8).
- All writes go through server functions using the DB service connection; RLS
  denies direct client writes (CLAUDE.md 4.1/4.3).
- `cargo audit` runs in CI (`.github/workflows/ci.yml`).
