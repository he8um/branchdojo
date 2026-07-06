# Git Wrapper

BranchDojo must use the local Git binary safely and explicitly.

## Rules

- Do not run shell scripts.
- Do not pass commands through `sh -c`, `bash -c`, `cmd.exe`, or PowerShell.
- Use `std::process::Command::new("git")`.
- Pass all Git arguments through `.args([...])`.
- Always set `current_dir` explicitly for repository operations.
- Capture stdout and stderr.
- Map Git failures into human-readable errors.
- Do not modify global Git config.

## Required Capabilities

The wrapper should expose functions such as:

- `ensure_git_available()`
- `init_repo(path)`
- `set_local_identity(path)`
- `checkout_new_branch(path, branch)`
- `checkout_branch(path, branch)`
- `add_all(path)`
- `commit(path, message)`
- `status_porcelain(path)`
- `current_branch(path)`
- `branch_exists(path, branch)`
- `file_content_at_branch(path, branch, file)`
- `log_contains_message(path, message)`
- `commit_count_after(path, commit_ref)`

## Local Identity

Each generated repository must receive local Git identity:

```bash
git config user.name "BranchDojo"
git config user.email "branchdojo@example.local"
```

This must be local repository config only.

## Git Status

Use stable machine-readable output:

```bash
git status --porcelain
```

An empty output means working tree is clean.

## Merge State Detection

A repository should be considered in an active operation if any of these exist in `.git/`:

- `MERGE_HEAD`
- `REBASE_HEAD`
- `CHERRY_PICK_HEAD`
- `REVERT_HEAD`

Implementation should account for `.git` being a file in worktree setups in future versions, but v0.1 can assume a normal generated repository.

## Error Mapping

Raw Git stderr should not be dumped alone. Wrap it with context:

```text
BD007: Repository is in an unexpected state.

Git reported:
<stderr>
```
