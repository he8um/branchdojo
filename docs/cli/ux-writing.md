# UX Writing Guidelines

BranchDojo should feel like a precise developer tool that is still helpful to beginners.

## Tone

- Direct.
- Practical.
- Calm.
- No blame.
- No jokes in errors.

## Error Message Pattern

Use this pattern:

```text
<CODE>: <problem>

Cause:
<why this happened>

Next:
<what the user should do>
```

## Validation Message Pattern

Each failed check should explain the missing final state, not accuse the user of using a wrong command.

Good:

```text
Conflict markers are still present in app.txt.
```

Bad:

```text
You did not resolve the merge correctly.
```

## Avoid

- Long theory in command output.
- Ambiguous words like "bad" without context.
- Suggesting destructive Git commands without explanation.
- Mentioning internal implementation details.

## Prefer

- Specific file names.
- Specific branch names.
- Specific next commands when safe.
- Clear distinction between `FAILED` and `WARNING`.
