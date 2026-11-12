---
name: bank-a-method
description: Save a working method for a recurring job so no future session has to rediscover it. Use when a recurring job failed on the first approach and another approach worked, when the owner corrects how a job is done, or when a new recurring job appears that has no runbook yet.
---

# Bank a method

Skills hold the method that works for everyone. Runbooks hold what this owner's setup has taught you. A runbook is a note in `Agent/Runbooks/` named after the job (`Agent/Runbooks/Check Inbox.md`). Every skill reads its runbook first, if one exists, and the runbook wins where they disagree.

This is how you get better the longer you're used. A lesson that isn't banked is paid for again by the next session.

## When to bank

- A recurring job failed on your first approach, and you found one that works.
- The owner corrected how you did a job ("don't do it that way, do this").
- A tool, site, or file behaved in a way the skill doesn't mention (a page that won't load, a file format that needs a different reader, a folder that moved).
- A job has come up a second time and has no runbook yet.

Skip one-off fixes. If the job won't come up again, the lesson has no reader.

## Steps

1. **Find the runbook.** Check `Agent/Runbooks/` and its index (`Agent/Runbooks/Runbooks.md`) for a note covering this job. If none exists, create one (see the format below) and add it to that index.
2. **Write the lesson.** Under `## Lessons`, one bullet per lesson:
   - What works, specifically enough to repeat (the command, the setting, the order of steps).
   - The dead end to skip, and why it failed.
   - The date you learned it.
3. **Fold, don't pile.** If the lesson replaces or sharpens an older one, rewrite that bullet. Don't leave both. If a lesson changes the main procedure, update the procedure and drop the bullet.
4. **Correct the profile if needed.** A correction about how the owner wants things done in general (not just in this job) goes in the Owner Profile's Standing Calls instead (see `owner-profile`).
5. **Log it** with a `checkpoint` and read the change back.

## Runbook format

```markdown
---
status: active
project: meta
type: guide
---
# <Job name>

**The job:** what it does and when it fires.
**Skill:** `<skill-name>` (or "none, runbook only").

## Boot chain
The notes to read, in order, before doing the job.

## Procedure
Only the steps that differ from the skill, or all of them if there is no skill.

## Lessons
- 2026-11-11: <what works>. Skip <dead end>: <why it failed>.
```

## Rules

- No secrets in a runbook. Write where a credential is stored, never its value.
- Lessons are facts you verified, not guesses. Mark anything unconfirmed.
- If a lesson would help every user, not just this owner, tell the owner it could go into the shared skill.
