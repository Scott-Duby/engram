---
name: study-from-notes
description: Build flashcard decks and exam-style quizzes from the owner's own notes, then drill them. Use when the owner asks for flashcards, a quiz, a practice test, or exam prep on a topic, chapter, or course.
---

# Study from notes

Turns the owner's course or topic notes into a flashcard deck and a quiz, then drills the owner with them. The goal is recall under exam conditions, not recognition. Lots of reps beat lots of cards.

## Boot chain

1. The runbook `Agent/Runbooks/Study From Notes.md`, if it exists (where this owner's decks live, what app or plugin reviews them, past lessons).
2. The notes for the topic, in full. If the notes are thin or end early, the source material they came from (slides, ebook, handouts).
3. `Agent/Active Priorities.md`, to weight the deck toward the next quiz or exam.

## Steps

1. **Confirm the scope.** Which chapter, topic, or exam. If the owner doesn't say, use the next dated exam in `Agent/Active Priorities.md`.
2. **Check coverage first.** Compare the notes against their source. Notes copied by hand often stop partway through. If the notes are missing material, build cards from the source for that part and tell the owner which part came from the source.
3. **Write the deck** in the project's flashcards folder (ask once where it should live if the runbook doesn't say, then record it there).
   - One card per line: `Question :: Answer`. This is the format the Obsidian Spaced Repetition plugin reads.
   - One fact per card. Split anything with two facts.
   - Group cards under headings by sub-topic.
   - End the deck with an empty `## Missed` section.
   - Numbers in file names are two digits (`Chapter 05 Flashcards.md`) so files list in order.
4. **Write the quiz** in the project's quizzes folder, about 20 questions per chapter, in the format below. Build it from the deck's facts.
5. **Check the answer key.** Count the correct answers. Spread them evenly across A to D and keep true/false close to half True. Hand-written keys drift toward one letter and toward False, which teaches guessing.
6. **Drill live** if the owner wants (see Drilling).

## Quiz format

```markdown
# Chapter 1 Quiz: <Title>

## <Sub-topic>

### 1-001
Which of these is NOT a <thing>?
- A) <choice>
- B) <choice>
- C) <choice>
- D) <choice>
Answer: C
Trap: except
Why: <one or two sentences that trace to the notes>.

### 1-002
True or false: <statement>.
Answer: False
Trap: number
Why: <the correct fact>.
```

- `### <chapter>-<number>` is a permanent ID. Never renumber or reuse one. New questions take the next free number.
- `Answer:` is a plain line, not a checkbox, so a stray click can't change the key.
- `Trap:` is one of `except`, `lookalike`, `absolute`, `number`, `none`.
- True/false questions have no choices.
- Tag quiz notes `Quiz`, never the flashcard tag, so the flashcard plugin skips them.

## Wording traps

Exams lose points on known material through wording. Build these into every quiz:
- **except:** "all of the following EXCEPT," "which is NOT."
- **lookalike:** distractors pulled from the same chapter that sound right.
- **absolute:** "always," "never," "only." Include true absolutes too, so "absolute means false" stops being a safe guess.
- **number:** swapped or near-miss numbers (distances, times, ratios, doses).

## Drilling

- One card or question at a time. The owner **writes out** the answer before you reveal it. Never show the answer first.
- Misses come back a few cards later. A card leaves the missed pile after two right in a row.
- After a session, add missed cards to the deck's `## Missed` section as question-only bullets (`- Question`), never `Question :: Answer`, so the plugin doesn't count duplicates.
- End with a score and the cards that still need reps.

## Quality bar

- Every card and answer traces to the owner's notes or their source. Never invent a fact. If the material is unclear or contradicts itself, say so.
- Paraphrase source material. Don't copy passages.
- After the work, bank anything new about this owner's setup with `bank-a-method`.
