# vault-template

What a new Engram user gets in their Obsidian vault. License: CC BY-SA 4.0 (see `LICENSING.md`).

The design rule: **the assistant stays out of the owner's way.** Everything the assistant reads to do its job (its map of the vault, the folder indexes, open priorities, its own log, runbooks, the owner profile) lives in one folder, `Agent/`. The rest of the vault belongs to the owner and only holds what they asked for.

## What's here

| Path | What it is | Copied into the vault? |
|---|---|---|
| `SETUP.md` | The setup interview and assembly steps the installer (or the assistant, until `brain init` exists) follows. | No |
| `Agent/` | The assistant's folder. Always installed, always named `Agent/`, never numbered, so it sorts after the owner's folders. | Yes, always |
| `Agent/Templates/` | Templates for the assistant's own notes (log entry, runbook, folder index). | Only if the owner opts in |
| `User/Templates/` | Templates for the owner's notes, including the daily note template. | Only the ones the owner opts in to, into the owner's templates folder |

Folders for the owner (inbox, projects, resources, archive, daily notes, tags) are not stored here as empty folders; `SETUP.md` lists them and creates the ones the owner chose, numbered or not.

## How the assistant's memory is split

- **Agent log** (`Agent/Log/`): every change the assistant makes, by date, with an Index block at the top of each day. Always kept, whether or not the owner uses daily notes. This is what the assistant reads at startup.
- **Daily notes** (owner's folder, optional): written for the owner, following the owner's daily note template. The assistant writes a plain-language summary there when daily notes are on, but never reads them to know what it did. If the owner turns daily notes off, changes are logged only in `Agent/Log/`.

---

Adapted from [AI Memory Vault](https://github.com/jaredrhod/ai-memory-vault) by Jared Rhodenizer. Licensed CC BY-SA 4.0.
