# Safety Model

BranchDojo manipulates local files and Git repositories, so safety is a core design requirement.

## Core Safety Principle

BranchDojo may only modify workspaces it created.

## Safety Controls

1. Required `--path` argument.
2. Unsafe path rejection.
3. Non-empty target rejection for `new`.
4. `.branchdojo.json` workspace guard.
5. Reset guard.
6. Local Git config only.
7. No shell execution.
8. No network access.
9. No existing repository support in v0.1.

## Safe Workspace

A safe workspace is:

- A dedicated exercise directory.
- Not root.
- Not home.
- Not a parent directory reference.
- Not a system directory.
- Contains valid BranchDojo metadata for check/reset.

## Dangerous Operations

The most dangerous operation is reset. Reset must always validate metadata before deleting or recreating anything.

## User Data Protection

BranchDojo should prefer refusing a command over risking user data.
