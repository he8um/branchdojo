# Product Vision

## Summary

BranchDojo helps developers practice real Git workflows by generating disposable local repositories with controlled mistakes, branches, commits, and conflicts.

The product exists because many developers can use Git for simple workflows but freeze when they face:

- Merge conflicts.
- Wrong-branch commits.
- Bad commits that need safe reversal.
- Dirty working trees.
- Branch state confusion.
- History recovery decisions.

BranchDojo turns those situations into repeatable kata exercises.

## Vision Statement

Make Git workflow recovery practice as repeatable as coding katas.

## Product Promise

BranchDojo gives users a safe local place to break, fix, and understand Git workflows without risking real project history.

## What Makes It Different

BranchDojo is not a static tutorial. It creates a real Git repository. The user has to solve the state using real Git commands.

It does not need network access, GitHub accounts, tokens, or external services.

## User Outcome

After practicing with BranchDojo, users should be more confident when they see:

- `CONFLICT`
- `You are not currently on a branch`
- `Your branch and origin have diverged`
- `nothing to commit, working tree clean`
- wrong branch commits
- bad commits that should be reverted

## MVP Success Criteria

v0.1 is successful if:

- A user can install the CLI locally.
- A user can list available exercises.
- A user can generate each MVP exercise.
- A user can solve the exercise manually.
- BranchDojo correctly reports pass, warning, or fail.
- BranchDojo refuses unsafe paths and non-BranchDojo workspaces.
- Tests cover all MVP exercises.
