# Vault setup

How a vault gets Engram. Run by `brain init` once it exists; until then, the assistant follows these steps in a live session. This file is not copied into the vault.

## The interview

Ask one question at a time and wait for the answer. Offer the default; the owner can take it with a plain "yes."

1. **New vault or existing vault?**
   - If the installer already gave a vault path, don't ask this. Check the path instead: nothing there means a new vault (confirm the path once, then go on to question 2); an Obsidian vault there means an existing vault (handle it as below); anything else (a folder with no `.obsidian/`) means stop and ask.
   - New: ask where to create it (default `~/Documents/<Assistant name> Vault`).
   - Existing: ask for its path, then confirm it is an Obsidian vault (it has a `.obsidian/` folder). If an `Agent/` folder already exists there, stop and ask before going further.
2. **What should the assistant be called, and how should it talk?** First offer the presets in `Personalities/` (Butler, Wingman, Coach, Professional, Mentor, Minimal), each with its one line from the README, or the option to write one from scratch. A chosen preset fills the fields below as defaults the owner can change. Then confirm the name (default Engram), personality in a sentence or two, voice rules (optional), how to address the owner, and the greeting that opens every session (default "<Name> here. What are we working on?"). Goes into `Agent/Owner Profile.md`.
3. **Numbered folders?** (`00 - Inbox`, `01 - Daily Notes` ...) Default: yes for a new vault. For an existing vault, keep whatever it already does and ask only whether to number the folders this setup adds.
4. **Tags as folders?** A `Tags/` folder holding one note per tag, each linking the notes that carry it, so tags can be browsed like folders. Default: no.
5. **Daily notes?** Default: yes. If yes, ask which folder (default `Daily Notes`) and whether to use monthly subfolders (default yes, `NN - Month YYYY`).
6. **Templates for your notes?** Default: yes. Offers the note template, the project template, and (if daily notes are on) the daily note template. For an existing vault with its own templates folder, put them there.
7. **Templates for the assistant's notes?** Default: yes. Copies `Agent/Templates/` so its log entries, runbooks, and folder indexes stay consistent. If no, the formats come from the skills alone.

Hold the answers until the interview is done, then build. Every answer goes into `Agent/Vault Settings.md`; a check before the first log entry makes sure the settings and the vault match.

## Building a new vault

1. Create the vault folder.
2. Copy `Agent/` in. Drop `Agent/Templates/` if the owner said no to assistant templates.
3. Write every interview answer into `Agent/Vault Settings.md`.
4. Create the owner's folders, numbered if chosen, in this order:

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
5. Copy the chosen files from `User/Templates/` into the owner's `Templates` folder.
6. Fill in `Agent/Owner Profile.md` from the interview, `Agent/Vault Map.md` with the folders just created, and one index per owner folder in `Agent/Indexes/`, named without the folder's number (the daily notes index lists month subfolders, not each note).
7. If daily notes are on, write `.obsidian/daily-notes.json` pointing at the daily notes folder, the daily note template, and the format (`YYYY-MM-DD`, or `MM - MMMM YYYY/YYYY-MM-DD` with monthly subfolders, which files each note in its month's folder), and turn on the core Templates plugin with the templates folder.
8. Check that `Agent/Vault Settings.md` matches what was built: folder names and numbering, daily notes folder and subfolders, `.obsidian/daily-notes.json`, and which templates exist. Fix whichever side is wrong.
9. Write the first entry in `Agent/Log/` describing the setup.

## Adding Engram to an existing vault

1. Change nothing the owner already has. No moves, renames, frontmatter edits, or new files outside `Agent/` without asking first.
2. Copy `Agent/` in (minus `Agent/Templates/` if declined).
3. Write every interview answer into `Agent/Vault Settings.md`.
4. Read the vault's top-level folders and write them into `Agent/Vault Map.md` as they are. Build an index in `Agent/Indexes/` for each folder with real content; list notes by name only, without reading their contents unless the owner asks for a full read.
5. Add only the owner-side pieces the owner said yes to (daily notes folder, templates, `Tags/`), matching the vault's existing naming.
6. Check that `Agent/Vault Settings.md` matches what was added, and fix whichever side is wrong.
7. Write the first entry in `Agent/Log/` describing what was added and where.

## After setup

Tell the owner, in plain language: where the vault is, what was created, that `Agent/` is the assistant's folder and safe to ignore, and that any answer can be changed later by asking.
