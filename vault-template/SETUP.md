# Vault setup

How a vault gets Engram. Run by `brain init` once it exists; until then, the assistant follows these steps in a live session. This file is not copied into the vault.

## The interview

Ask one question at a time and wait for the answer. Offer the default; the owner can take it with a plain "yes."

1. **New vault or existing vault?**
   - New: ask where to create it (default `~/Documents/<Assistant name> Vault`).
   - Existing: ask for its path, then confirm it is an Obsidian vault (it has a `.obsidian/` folder). If an `Agent/` folder already exists there, stop and ask before going further.
2. **What should the assistant be called, and how should it talk?** Name (default Engram), personality in a sentence or two, and how to address the owner. Goes into `Agent/Owner Profile.md`.
3. **Numbered folders?** (`00 - Inbox`, `01 - Daily Notes` ...) Default: yes for a new vault. For an existing vault, keep whatever it already does and ask only whether to number the folders this setup adds.
4. **Tags as folders?** A `Tags/` folder holding one note per tag, each linking the notes that carry it, so tags can be browsed like folders. Default: no.
5. **Daily notes?** Default: yes. If yes, ask which folder (default `Daily Notes`) and whether to use monthly subfolders (default yes, `NN - Month YYYY`).
6. **Templates for your notes?** Default: yes. Offers the note template, the project template, and (if daily notes are on) the daily note template. For an existing vault with its own templates folder, put them there.
7. **Templates for the assistant's notes?** Default: yes. Copies `Agent/Templates/` so its log entries, runbooks, and folder indexes stay consistent. If no, the formats come from the skills alone.

Write every answer into `Agent/Vault Settings.md` before building anything.

## Building a new vault

1. Create the vault folder.
2. Create the owner's folders, numbered if chosen, in this order:

   | Folder | Created when |
   |---|---|
   | Inbox | always |
   | Daily Notes | daily notes on |
   | Projects | always |
   | Resources | always |
   | Templates | either template answer is yes |
   | Tags | tags as folders on |
   | Archive | always |

   Numbering runs from `00` in that order, skipping folders that weren't created.
3. Copy `Agent/` in. Drop `Agent/Templates/` if the owner said no to assistant templates.
4. Copy the chosen files from `User/Templates/` into the owner's `Templates` folder.
5. Fill in `Agent/Owner Profile.md` from the interview, `Agent/Vault Map.md` with the folders just created, and one index per owner folder in `Agent/Indexes/`.
6. If daily notes are on, write `.obsidian/daily-notes.json` pointing at the daily notes folder, the daily note template, and the `YYYY-MM-DD` format, and turn on the core Templates plugin with the templates folder.
7. Write the first entry in `Agent/Log/` describing the setup.

## Adding Engram to an existing vault

1. Change nothing the owner already has. No moves, renames, frontmatter edits, or new files outside `Agent/` without asking first.
2. Copy `Agent/` in (minus `Agent/Templates/` if declined).
3. Read the vault's top-level folders and write them into `Agent/Vault Map.md` as they are. Build an index in `Agent/Indexes/` for each folder with real content; list notes by name only, without reading their contents unless the owner asks for a full read.
4. Add only the owner-side pieces the owner said yes to (daily notes folder, templates, `Tags/`), matching the vault's existing naming.
5. Write the first entry in `Agent/Log/` describing what was added and where.

## After setup

Tell the owner, in plain language: where the vault is, what was created, that `Agent/` is the assistant's folder and safe to ignore, and that any answer can be changed later by asking.
