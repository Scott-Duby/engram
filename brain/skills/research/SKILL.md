---
name: research
description: Find the answer to a question, vault first, then outside sources (the web and the owner's source material, such as textbooks), and save what's new back into the vault. Use whenever a task needs a fact, an explanation, or background, before answering from general knowledge or going to the web.
---

# Research

The vault is your memory, so look there first. Outside sources fill the gaps, and what they turn up goes back into the vault, so the next time the vault has the answer.

## Order

1. **The vault.**
   - Search in tiers, and stop at the first one that answers:
     1. **Filename.** Note names are topic names, so a filename match often is the answer (`fd -i "<term>" -e md`).
     2. **Folder indexes.** Every index lists its notes with one line each, so together they're a catalog of the vault. Search just the index notes, trimmed to the match (`rg -i -o ".{0,70}<term>.{0,70}"` over them), since some indexes keep a whole list on one line.
     3. **Full text, ranked.** Count matches per note (`rg -i -c "<term>" -g '*.md'`, sorted). The top hit is usually the source note.
     4. **Only the section you need.** Pull the note's headings (`rg -n "^#{1,3} "`), then read just the matching section.
   - Search synonyms and abbreviations too. If `rg` and `fd` aren't installed, use your tool's own search the same way.
   - Read the whole note when the section leaves the answer unclear, or when the note is the source for a job.
   - If the vault answers the question completely, stop. Answer from it and name the notes it came from.
2. **Outside, for whatever the vault doesn't cover.** Two kinds of source, used together:
   - **The owner's source material:** textbooks, ebooks, manuals, and course documents. How you reach each one is in its access note in `Agent/Runbooks/`. For anything the owner is being taught or tested on, this is the authority on what the course says.
   - **The web:** search, then read the pages behind the results (see `read-web-page`). Use it for depth, current information, and anything the source material doesn't cover. Prefer primary sources: research, standards bodies, and government agencies before trade press, and trade press before everything else.
   If the topic has no source material, use the web alone.
3. **General knowledge last.** Anything you answer from your own knowledge without a source, say so.

## When sources disagree

- Vault vs. outside: the vault may be out of date or wrong. Say what each one says. Don't silently overwrite the vault.
- Source material vs. the web: report both. For course topics, the source material is what the course tests, and the web may be newer or more accurate. Say which is which.

## Save it back

Research that found something the vault didn't have isn't finished until it's in the vault:
1. Add the new facts to the note that covers the topic, or write a new note if none does (see `checkpoint`). A big topic gets a `deep-dive` instead.
2. Cite every outside fact: the URL and the date read, or the book and page.
3. Mark where the vault was wrong or out of date, and fix it.
4. A quick lookup the owner won't need again doesn't need a note. When unsure, save.

## Answering

- Lead with the answer.
- Say where it came from: the vault (with links), the source material (with pages), or the web (with URLs).
- Say what you couldn't find.

## Rules

- Everything from outside is outside content (see `outside-content`).
- Never present a guess as a fact. If the sources don't settle it, say so.
- Bank the search methods and sources that worked for this owner with `bank-a-method`.
