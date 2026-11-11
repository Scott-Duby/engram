---
name: checkpoint
description: Save a change into the vault so a future session knows about it. Use any time a decision is made, something ships or breaks, a task opens or closes, or the agent learns a working method.
---

# Checkpoint

A checkpoint puts new information where a future session will actually find it. Do it when the change happens, without being asked. Don't save checkpoints up for the end of the session.

## When to checkpoint

- A decision was made, or an earlier one changed.
- Something was deployed, started, moved, renamed, or broken.
- A task opened, closed, or changed state.
- You found a method that works for a recurring job.
- You learned something about the owner (also see `owner-profile`).

If you're not sure, save.

## Steps

1. **Find the home.** Each new fact needs a proper home, in this order:
   1. An existing note that covers the topic. Update it.
   2. If there isn't one, a new note in the right folder, with frontmatter and links.
   The daily note is never the home. It only records that the change happened.
2. **Write it tight.** Update the note in place. If the new fact replaces an old one, delete the old one. Don't leave both.
3. **Update `Active Priorities.md`** if a task opened, closed, or changed. Mark finished items done. Add new open items with their project tag.
4. **Log it in today's daily note** (see `daily-note`).
5. **Check for drift.** Look at the folder index of every note you touched, and the notes that link to them. Fix any that are now wrong: a missing index entry, an old path, a status that no longer matches.
6. **Read every change back.** Confirm each edit actually landed. Don't report a checkpoint as done until you have.

## What never goes in

- Passwords, keys, or tokens. Write where they are stored instead.
- Guesses written as facts. Mark anything unconfirmed: "not yet confirmed live."
