---
name: weekly-review
description: Check every open item in Active Priorities against its real state and fix the list. Use when the owner asks for a weekly review or a priorities check, at the first session of a new week, or when the list has items past their date.
---

# Weekly review

`Active Priorities.md` is the system of record for what is still open. A list that says something is open when it's done, or forgets something that's open, is worse than no list. `vault-health` checks the vault's structure. This checks whether the list is true.

## Steps

1. **Read the runbook** `10 - Agent/Weekly Review.md` if it exists.
2. **Get the real date** from the system.
3. **Read the whole list,** then go through every open item, one at a time:
   1. **Check its real state.** Read the notes, files, or commands the item points at. Never close or keep an item on memory alone.
   2. **Done?** Mark it `- [x]` with the date, or move it to Completed Tasks.
   3. **Date passed?** An event, deadline, or exam that is behind the current date: if the vault shows the outcome, record it and close the item. If it doesn't, ask the owner.
   4. **Stale?** No movement in the vault for 14 days or more (check the daily notes and the item's linked notes). List it for the owner to keep, park, or drop.
   5. **Blocked?** If it's waiting on something, name what it's waiting on in the item.
   6. **Wrong?** If the item's details no longer match the vault (a file moved, a count changed), fix the item.
4. **Look for missing items.** Scan the daily notes since the last review for "In Progress" work and decisions that never reached the list. Add them with their project tag.
5. **Look ahead.** List anything with a date in the next 14 days, soonest first.
6. **Save it** with a `checkpoint`. Record the review date at the top of the runbook so the next review knows where to start.

## Report

Keep it short:
- **Closed:** what you marked done, and the evidence.
- **Fixed:** items you corrected.
- **Added:** items that were missing.
- **Coming up:** dated items in the next 14 days.
- **Needs your call:** stale items, and dates that passed with no outcome recorded.

Ask about the "needs your call" items one at a time, starting with the most urgent. Ask one and stop.

## Rules

- Closing an item needs evidence. Dropping or parking one needs the owner.
- Never archive or delete an item on your own. Parking means it stays on the list, marked parked.
- Don't rewrite items in your own words for style. Fix only what's wrong.
