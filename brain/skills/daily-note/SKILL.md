---
name: daily-note
description: Create, append to, or backfill the vault's daily note. Use at session start (to check the previous note), when logging work, and when the owner signals they are done for the day.
---

# Daily note

The daily note is the log of what happened across every session in a day. Several sessions, and sometimes several AI tools, write to the same day. Each one adds what it knows.

## Where it lives

- Folder: `01 - Daily Notes/NN - Month YYYY/` (for example `01 - Daily Notes/11 - November 2026/`). Use the month subfolder from the first note of the month.
- Filename: `YYYY-MM-DD.md`.
- Template: `01 - Daily Notes/Daily Note Template.md`. Always build a new note from it. Never write a bare heading.

## Steps

1. **Get the real date and time** from the system. Don't trust the date the session started on.
2. **If today's note doesn't exist,** create it from the template:
   - Frontmatter: `status: active`, `project: personal`, `type: log`.
   - A readable date heading: `# Wednesday, November 11, 2026`.
   - An `## Index` block right under the heading.
   - The session section from the template.
3. **If today's note exists,** don't overwrite anything. Add a new `## Session N, H:MM PM: <topic>` section at the end and a line for it in the Index.
4. **Fill the session section:**
   - **What Got Done:** finished work, with links to the notes it touched.
   - **What's Still In Progress:** what's open right now. This is a snapshot. `Active Priorities.md` is the live list.
   - **Decisions Made:** written as facts.
   - **Notes Touched:** wikilinks.
   - **Profile Updates:** every change to the Owner Profile (see the `owner-profile` skill).
5. **Index line:** `- **<Bold topic>**: <one sentence on the outcome>.`
6. Read the note back and confirm your change is there.

## At session start: check the previous note

Look at the most recent daily note before today.
- **It exists:** read it. If you have context it's missing, append a section. Otherwise leave it alone.
- **It doesn't exist and you have context for that day:** create it from the template and mark it as reconstructed and possibly incomplete.
- **No context for that day:** assume a day off. Don't create an empty note.

Mention briefly what you did, then move on.

## When the owner says they're done

When they say they're done ("calling it," "goodnight"), offer to update today's note. Never ask whether they're done, and never suggest stopping.

## Rules

- Neutral voice: it's a log, not a diary about the owner.
- The log is append-only. Never de-duplicate across days or rewrite an earlier session.
- A daily-note entry is never the only record of something new. Pair it with a `checkpoint`.
