# BACKLOG.md - Quant Arena

Out-of-scope ideas land here as one-liners (CLAUDE.md Section 0.4). They are not
implemented when logged; they are picked up in the phase that owns them.

Format: `- [Phase N | area] idea`

## Deferred from Phase 0 (belong to later phases)

- [Phase 1 | data] Supabase tables: profiles, progress, xp_ledger, streaks (+ RLS).
- [Phase 1 | auth] Supabase Auth (email + OAuth); JWT verified in every authenticated server fn.
- [Phase 1 | server] Real server functions: complete-day, submit-problem, me-summary.
- [Phase 1 | migration] One-shot import-local-progress endpoint (validates the `qrt_state` payload, idempotent, sets legacy_migrated).
- [Phase 1 | server] Replace gloo-storage progress writes with server calls; keep localStorage as offline read cache only.
- [Phase 1 | deploy] Move hosting off GitHub Pages to Fly.io/Railway; SSR landing page with OpenGraph meta tags; retire/repoint deploy.yml.
- [Phase 2 | ui] World Map / Chapter Zoom / Day Detail three-view navigation (CLAUDE.md Section 5).
- [Phase 3 | content] 40 original seed problems across the 4 tracks; Daily Quests; spaced-repetition revision queue.
- [Phase 4 | mp] Leaderboards, Async Duels, ratings/elo_history, friends, cheat_flags.
- [Phase 5 | mp] Live Blitz Battles over WebSockets.
- [Phase 6 | mp] Guilds, Seasons, Tournaments + full security pass.
