# Exercise: revert-mistake

## Summary

`revert-mistake` teaches the user to recover from a bad commit without destroying history.

## Difficulty

Beginner.

## Estimated Time

5 to 10 minutes.

## Skills

- Reading log history.
- Identifying a bad commit.
- Restoring safe file content.
- Preserving history.

## Setup

BranchDojo creates:

- `main` branch.
- `config.txt` with safe production config.
- Initial safe commit.
- One valid improvement commit.
- One intentionally bad commit that changes `config.txt` to an unsafe value.

## User Task

Fix the repository using a safe workflow. The bad commit should remain in history, but the final file content should be restored.

## Expected Final State

- User is on `main`.
- Working tree is clean.
- `config.txt` exists.
- Unsafe value is absent.
- Expected safe value exists.
- Bad commit still exists in history.
- At least one later commit exists after the bad commit.

## Warning Condition

If the final content is correct but the fix does not look like a revert-style workflow, return `WARNING` instead of `FAILED`.

## Important Policy

Do not require exact command sequence. A true `git revert` is ideal, but a history-preserving fix commit may be acceptable with warning.

## Suggested Hints

1. Run `git log --oneline`.
2. Find the commit that introduced the unsafe config.
3. Prefer a history-preserving recovery.
4. Confirm `config.txt` contains the safe value.
5. Confirm the bad commit still exists in history.
6. Run `branchdojo check --path .`.
