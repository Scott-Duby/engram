---
name: vault-health
description: Audit the vault for drift and fix it. Use when the owner asks for a health check, or when you notice a missing index entry, bad frontmatter, or a broken link.
---

# Vault health

A vault that drifts gives future sessions a wrong map. This check finds the drift and fixes it.

## What to check

Check the whole vault, or the folders the owner names. Read every note in scope. Don't sample.

1. **Folder indexes.**
   - Every folder with real content (5 or more notes, or a separate area of its own) has an index named after the folder.
   - Every note in the folder is listed in its index, and every entry in the index points to a note that exists.
   - Every top-level folder appears in the layout in `VAULT-INDEX.md`.
2. **Frontmatter.**
   - Every note has `status`, `project`, and `type`, with values from the allowed lists in `AGENTS.md` and `VAULT-INDEX.md`.
   - Status matches the content: unchecked tasks mean `active`, all done means `completed`, notes in the archive are `archived`.
3. **Links.**
   - Every `[[link]]` resolves to a note that exists.
   - No two notes share a filename. Duplicate names make links ambiguous.
4. **Voice.**
   - Notes outside `10 - Agent/` don't talk about the owner in the third person.
5. **Secrets.**
   - No note contains anything that looks like a password, API key, or token. If you find one, tell the owner right away. Don't copy the value anywhere, including into your report.
6. **Active Priorities.**
   - Items marked open that the vault shows are done, and done work that's missing from the list.

## Fixing

- **Safe to fix on your own:** missing frontmatter, missing or stale index entries, wrong status values.
- **Ask first:** renames (they break links unless done in the Obsidian app), moves, archiving, deletions, and anything touching the Owner Profile's protected sections.
- When you fix a broken link, find the note it was supposed to point to. If you can't tell which note that is, list it for the owner instead of guessing.

## Report

Give the owner a short list: what you fixed, and what needs their call. Then log the check in today's daily note.
