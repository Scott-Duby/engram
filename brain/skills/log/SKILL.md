---
name: log
description: Write the agent log in Agent/Log/, and the owner's daily note when daily notes are on. Use at session start (to check the previous day's log), when logging work, and when the owner signals they are done for the day.
---

# Log

`Agent/Log/` is your record of what happened across every session in a day. Several sessions, and sometimes several AI tools, write to the same day. Each one adds what it knows. It is kept whether or not the owner uses daily notes.

Daily notes are different: they belong to the owner. When `Agent/Vault Settings.md` has daily notes on, you also write a plain-language summary into the owner's daily note. You never read daily notes to learn what you did. The log is the record.

## Where the log lives

- Folder: `Agent/Log/YYYY-MM/` (for example `Agent/Log/2026-11/`). Add each new month to `Agent/Log/Log.md`.
- Filename: `Log YYYY-MM-DD.md` (for example `Log 2026-11-11.md`). The `Log ` prefix keeps it from sharing a name with the owner's daily note for the same day, which would make `[[links]]` ambiguous.
- Template: `Agent/Templates/Log Entry Template.md` if it exists; otherwise the format below.

## Steps

1. **Get the real date and time** from the system, and read the result before writing anything. Never write a date or time you haven't read from the clock, and don't trust the date the session started on.
2. **If today's log doesn't exist,** create it:
   - Frontmatter: `status: active`, `project: meta`, `type: log`.
   - A readable date heading: `# Wednesday, November 11, 2026`.
   - An `## Index` block right under the heading.
   - The session section.
3. **If today's log exists,** don't overwrite anything. Add a new `## Session N, H:MM PM: <topic>` section at the end and a line for it in the Index.
4. **Fill the session section:**
   - **What changed:** finished work, with links to the notes it touched, and any files changed outside the vault (with their path).
   - **Still open:** what's open right now. This is a snapshot. `Agent/Active Priorities.md` is the live list.
   - **Decisions:** written as facts.
   - **Notes touched:** wikilinks.
   - **Profile updates:** every change to the Owner Profile (see the `owner-profile` skill).
5. **Index line:** `- **<Bold topic>**: <one sentence on the outcome>.`
6. **Daily note, if on.** Find or create today's daily note where `Agent/Vault Settings.md` says, built from the owner's daily note template. Never write a bare heading. If it starts a new month subfolder, add the month to the daily notes folder's index. Fill only the template's own sections, in plain language a person would write for themselves: what got done, not how you did it. Leave out paths, commit hashes, and your own working notes; those stay in the log. Never overwrite what the owner wrote there.
7. Read every change back and confirm it's there.

## At session start: check the previous day

Look at the most recent log day before today in `Agent/Log/`.
- **It exists:** read its `## Index` block first; that's enough to tell whether it covers what you know. If you have context it's missing, read the full entry and append a section. Otherwise leave it alone.
- **It doesn't exist and you have context for that day:** create it and mark it as reconstructed and possibly incomplete.
- **No context for that day:** assume a day off. Don't create an empty entry.

Mention briefly what you did, then move on.

## When the owner says they're done

When they say they're done ("calling it," "goodnight"), offer to update today's log (and daily note, if on). Never ask whether they're done, and never suggest stopping.

## Rules

- Neutral voice in the daily note: it's the owner's page, not a report about them.
- The log and daily notes are append-only. Never de-duplicate across days or rewrite an earlier session.
- A log entry is never the only record of something new. Pair it with a `checkpoint`.
- Daily notes off: never create one, and never ask the owner to turn them on.
