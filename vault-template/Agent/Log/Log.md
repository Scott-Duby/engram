---
status: active
project: meta
type: index
---
# Log

The assistant's record of every change, one file per day: `Log/YYYY-MM/Log YYYY-MM-DD.md` (the prefix keeps it from sharing a name with a daily note). Kept whether or not the owner uses daily notes. At startup the assistant reads the `## Index` block of the most recent day before today, plus today's if it exists.

## Format
- `# <Weekday, Month D, YYYY>` heading.
- `## Index`: one line per session, `- **Topic**: one-sentence outcome.`
- One `## Session N, H:MM PM: <topic>` section per session, with **What changed**, **Still open**, **Decisions**, **Notes touched**, and **Profile updates** (the `log` skill has the details).
- Append only. Never rewrite or delete an earlier entry; corrections go in a new session section.

## Months
- {{YYYY-MM}}
