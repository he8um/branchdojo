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
6. Report path guard.
7. Local Git config only.
8. No shell execution.
9. No network access.
10. No existing repository support in v0.1.

## Safe Workspace

A safe workspace is:

- A dedicated exercise directory.
- Not root.
- Not home.
- Not a parent directory reference.
- Not a system directory.
- Contains valid BranchDojo metadata for check/reset.

## Safe Report Path

A safe report path is:

- Explicitly provided with `--report <file>`.
- A regular file path, not a directory.
- Not an existing file.
- Not inside `.git`.
- Not root or the home directory itself.
- Not a parent-directory traversal path.
- Inside an existing parent directory.

BranchDojo does not create report parent directories and does not provide report overwrite or `--force` behavior.

## Dangerous Operations

The most dangerous operation is reset. Reset must always validate metadata before deleting or recreating anything.

## User Data Protection

BranchDojo should prefer refusing a command over risking user data.
