---
status: active
project: meta
type: reference
---
# Vault Settings

The owner's setup answers. Filled in at setup; change only when the owner asks, and log the change in [[Log]].

| Setting | Value |
|---|---|
| Vault type | {{new or existing}} |
| Set up on | {{YYYY-MM-DD}} |
| Numbered folders | {{yes or no}} |
| Tags as folders | {{yes or no}} |
| Daily notes | {{yes or no}} |
| Daily notes folder | {{folder, or "none"}} |
| Monthly subfolders | {{yes or no}} |
| Owner templates | {{yes or no}} ({{which ones}}) |
| Owner templates folder | {{folder, or "none"}} |
| Assistant templates | {{yes or no}} |

## What each setting changes

- **Numbered folders:** new owner folders get the next free `NN - ` prefix. Off: plain names. `Agent/` is never numbered.
- **Tags as folders:** when a note gets a new tag, create or update `Tags/<tag>.md` linking every note that carries it. Off: tags live in frontmatter only.
- **Daily notes on:** after logging a session in [[Log]], write a plain-language summary into the owner's daily note, built from the owner's daily note template. Never read daily notes to learn what the assistant did; [[Log]] is the record.
- **Daily notes off:** log only in [[Log]]. Never create a daily note.
- **Owner templates on:** new owner notes start from the matching template. Off: frontmatter only.
- **Assistant templates on:** log entries, runbooks, and indexes start from `Agent/Templates/`. Off: use the formats in the skills.
