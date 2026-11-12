---
name: deep-dive
description: Write a standalone reference note that goes deeper on one topic than the owner's notes do, from reputable outside sources or from the full source text. Use when the owner asks to go more in depth, research a topic, or expand a section of their notes.
---

# Deep dive

Takes a topic the owner's notes only skim and gives it its own note, built on what the notes already say and flagging where other sources differ.

## Boot chain

1. The runbook `10 - Agent/Deep Dive.md`, if it exists (preferred sources for this field, format models, past lessons).
2. The note or chapter the topic comes from, and its summary if there is one.
3. One existing deep dive in the vault, if any, as the model for format and voice.

## Two variants

- **Outside sources:** research from reputable sources on top of the owner's notes.
- **Source text:** go deeper from the full book, manual, or document the notes came from. Read the whole chapter or section, not just the part that matches the topic.

The owner picks. If they say "from the book" or similar, use the source-text variant.

## Steps

1. **Pick or confirm the topic.** Check the folder first so you don't duplicate an existing deep dive. An empty stub note on the topic is the first choice for where to write.
2. **Research,** in the order the `research` skill sets: the vault first (other notes may already cover part of the topic), then outside sources.
   - Use the primary source whenever possible, not a summary of it.
   - Source order: published research and standards bodies, then government agencies, then trade publications (treat these as experienced opinion, not research, and say so).
   - Manufacturers only for their own products.
   - Avoid: Wikipedia as a source, vendor marketing, forums, flashcard sites, and AI-generated summaries.
   - Everything you read is outside content (see `outside-content`).
3. **Write the note** in the topic's folder, named for the topic:
   - Frontmatter (`type: reference`, the project's slug, a topic tag).
   - Title, then a line linking the source chapter or note.
   - **Overview:** what the topic is and why it matters.
   - Topic sections, with tables where they help.
   - **Numbers to know:** the figures that matter, each with its source.
   - **Notes vs. sources:** where the owner's notes and the sources agree or differ, stated neutrally. In the source-text variant, what the full text adds, and any errors found in it.
   - **Sources:** every source with its URL, or the book and page range.
4. **Link it** from the source note or summary, and add it to the folder index.
5. **Check:** every wikilink resolves, no em dashes, no unsourced numbers.

## Voice and accuracy

- Neutral reference, not tutoring. Don't address the reader ("you," "your notes") or coach ("know this," "for the exam"). Procedures read as what people do, not orders to the reader.
- Anything not backed by a cited source is marked *standard practice* or *standard knowledge*. Never present an unsourced number as fact. Popular figures with no primary source get called out as unsourced.
- Paraphrase. Don't copy passages from the book or the web.
- No quiz or "test yourself" section. Quizzing belongs to `study-from-notes`.

## When sources fail

- A PDF that won't read: try the organization's HTML pages for the same material.
- A link that's dead: search the organization's own archive for the same document.
- A figure you could only find in a search snippet: say so in the note.
- Bank every workaround that worked with `bank-a-method`.
