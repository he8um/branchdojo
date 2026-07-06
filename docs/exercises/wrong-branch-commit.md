# Exercise: wrong-branch-commit

## Summary

`wrong-branch-commit` teaches the user to move accidental work from the wrong branch to the correct feature branch.

## Difficulty

Beginner.

## Estimated Time

10 to 15 minutes.

## Skills

- Branch inspection.
- Moving work between branches.
- Restoring main branch state.
- Clean working tree validation.

## Setup

BranchDojo creates:

- `main` branch.
- `feature/profile-page` branch.
- An accidental commit on `main` that should belong to `feature/profile-page`.

## User Task

Move the accidental work from `main` to `feature/profile-page` and restore `main` to the correct clean state.

## Expected Final State

- `main` branch exists.
- `feature/profile-page` branch exists.
- `main` does not contain accidental file/content.
- `feature/profile-page` contains accidental file/content.
- Working tree is clean.
- `.git` directory is intact.
- `.branchdojo.json` is intact.

## Warning Condition

If final branch content is correct but history shape is unusual, return `WARNING`.

## Suggested Hints

1. Run `git branch`.
2. Run `git log --oneline --decorate --graph --all`.
3. Identify which commit belongs on the feature branch.
4. Move the work to `feature/profile-page`.
5. Restore `main` to the correct state.
6. Run `branchdojo check --path .`.

## Acceptable Final-State Principle

Do not require one exact solution. Cherry-pick, patch-based, or other safe approaches may be acceptable if the final branch state is correct.
