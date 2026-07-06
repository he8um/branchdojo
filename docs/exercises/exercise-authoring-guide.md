# Exercise Authoring Guide

This guide defines how to add new BranchDojo exercises.

## Good Exercise Criteria

A good exercise is:

- Realistic.
- Small enough to finish in 5 to 15 minutes.
- Deterministic.
- Offline.
- Safe.
- Validatable through final repository state.
- Useful for Git workflow muscle memory.

## Avoid

- Exercises that require remote repositories.
- Exercises that require GitHub accounts or tokens.
- Exercises that rely on exact shell history.
- Exercises that require time-sensitive behavior.
- Exercises that need network access.
- Exercises that can only be solved one way.
- Exercises that encourage destructive commands without safety context.

## Authoring Steps

1. Define metadata.
2. Define starting repository state.
3. Define user task.
4. Define required final state.
5. Define warning conditions.
6. Define static hints.
7. Write generated README content.
8. Implement setup.
9. Implement validator.
10. Add unit tests.
11. Add integration tests.
12. Add documentation.

## Validation Design

Prefer this pattern:

```text
Required checks:
- final state is correct
- repository is clean
- expected files exist
- expected branch state exists

Warnings:
- solution is acceptable but not the intended workflow shape
```

## Generated README Rules

Every exercise README should include:

- Goal.
- Starting state.
- User task.
- Rules.
- Useful commands.
- How to check.
- How to reset.
- Hints.
