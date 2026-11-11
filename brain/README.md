# brain

The shared brain, read by every adapter. License: CC BY-SA 4.0 (see `LICENSING.md`).

| Path | What it is |
|---|---|
| `AGENTS.md` | The operating rules: how the agent uses the vault as memory, the rules that never lapse, and how notes are written. The same for every user. |
| `skills/daily-note/` | Create, append to, and backfill the daily note. |
| `skills/checkpoint/` | Save a change to its proper note, update Active Priorities and the daily note, and fix drift. |
| `skills/vault-health/` | Audit indexes, frontmatter, links, voice, and secrets, and fix what's safe to fix. |
| `skills/owner-profile/` | Read and maintain the owner's profile: the agent's name and personality, plus the owner's preferences. |

Anything personal (the agent's name, tone, the owner's schedule and preferences) lives in the user's vault at `10 - Agent/Owner Profile.md`, never in this folder. `AGENTS.md` uses the placeholder `{{VAULT_PATH}}`, which the installer fills in.

Each skill is a folder with a `SKILL.md` (YAML frontmatter with `name` and `description`, then instructions). Adapters copy or link these where each tool looks for skills. Claude Code reads `CLAUDE.md`, not `AGENTS.md`, so its adapter installs a `CLAUDE.md` that imports `AGENTS.md`.
