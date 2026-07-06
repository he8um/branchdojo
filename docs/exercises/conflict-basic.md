# Exercise: conflict-basic

## Summary

`conflict-basic` teaches the user to resolve a real merge conflict in a small repository.

## Difficulty

Beginner.

## Estimated Time

5 to 10 minutes.

## Skills

- Branch awareness.
- Merge conflict resolution.
- Working tree inspection.
- Clean merge completion.

## Setup

BranchDojo creates:

- `main` branch.
- `feature/landing-copy` branch.
- `app.txt`.
- One initial commit.
- One commit on feature changing CTA text.
- One commit on main changing headline text.

The repository is left on `main`, ready for the user to merge `feature/landing-copy`.

## User Task

Merge `feature/landing-copy` into `main` and resolve the conflict so `app.txt` keeps both:

- The updated headline from `main`.
- The CTA from `feature/landing-copy`.

## Expected Final State

- User is on `main`.
- No merge, rebase, cherry-pick, or revert operation is active.
- Working tree is clean.
- `app.txt` exists.
- `app.txt` has no conflict markers.
- `app.txt` contains expected headline.
- `app.txt` contains expected CTA.
- `feature/landing-copy` branch still exists.
- History includes work from both main and feature branch.

## Warning Condition

If final content is correct but no merge commit is detected, return `WARNING` instead of `FAILED`.

## Suggested Hints

1. Start with `git status`.
2. Make sure you are on `main`.
3. Merge `feature/landing-copy`.
4. Open `app.txt` and remove conflict markers.
5. Keep both required changes.
6. Add and commit the resolved file.
7. Run `branchdojo check --path .`.

## Example Next Step on Failure

```text
Open app.txt, remove conflict markers, preserve both required lines, then run branchdojo check --path . again.
```
