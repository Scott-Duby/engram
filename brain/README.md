# brain

The shared brain, read by every adapter. License: CC BY-SA 4.0 (see `LICENSING.md`).

| Path | What it is |
|---|---|
| `AGENTS.md` | The operating rules: how the agent uses the vault as memory, the rules that never lapse, and how notes are written. The same for every user. |
| `skills/log/` | Write and backfill the agent log in `Agent/Log/`, plus the owner's daily note when daily notes are on. |
| `skills/checkpoint/` | Save a change to its proper note, update Active Priorities and the log, and fix drift. |
| `skills/vault-health/` | Audit indexes, frontmatter, links, voice, and secrets, and fix what's safe to fix. |
| `skills/owner-profile/` | Read and maintain the owner's profile: the agent's name and personality, plus the owner's preferences. |
| `skills/research/` | Look things up vault first, then source material and the web, and save new findings back. |
| `skills/bank-a-method/` | Write a working method and its dead ends into the job's runbook in `Agent/Runbooks/`. |
| `skills/weekly-review/` | Check every Active Priorities item against its real state and fix the list. |
| `skills/study-from-notes/` | Flashcard decks and quizzes from notes, with wording traps and a balanced key, plus live drilling. |
| `skills/drill/` | Memorize a list in both directions, with misses tracked across sessions. |
| `skills/deep-dive/` | A standalone reference note on one topic from outside sources or the full source text. |
| `skills/ingest/` | Source material and inbox captures into filed reference notes. |
| `skills/outside-content/` | The base rules for all outside content, and the red flags to report. |
| `skills/check-inbox/` | Read-only summary of new email: action items, red flags, low-value count. |
| `skills/write-email/` | Business and personal email drafts in the owner's voice; sends only on approval. |
| `skills/read-web-page/` | Fetch and summarize web pages with sources, without following page instructions. |
| `skills/messages-and-comments/` | Summarize and draft replies to reviews, issues, chat, and forum messages. |
| `skills/handle-files/` | Inspect downloads and attachments without running them. |

Anything personal (the agent's name, tone, the owner's schedule and preferences) lives in the user's vault at `Agent/Owner Profile.md`, never in this folder. Everything else the agent reads (vault map, folder indexes, priorities, log, runbooks, setup answers) is in the same `Agent/` folder; see `vault-template/`. `AGENTS.md` uses the placeholder `{{VAULT_PATH}}`, which the installer fills in.

Each skill holds the method that is the same for every user; each owner's own lessons go in a runbook in `Agent/Runbooks/`, which the skill reads first. Each skill is a folder with a `SKILL.md` (YAML frontmatter with `name` and `description`, then instructions). Adapters copy or link these where each tool looks for skills. Claude Code reads `CLAUDE.md`, not `AGENTS.md`, so its adapter installs a `CLAUDE.md` that imports `AGENTS.md`.
