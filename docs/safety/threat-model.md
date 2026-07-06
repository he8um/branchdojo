# Threat Model

## Threat 1: Accidental Deletion of User Files

Risk:

Reset deletes or overwrites files outside a disposable workspace.

Mitigation:

- Required `.branchdojo.json` for reset.
- Unsafe path rejection.
- Non-empty target rejection for new.
- Full reset only for valid BranchDojo workspaces.

Test coverage:

- Reset refuses non-BranchDojo directory.
- Reset refuses missing metadata.
- Reset refuses unsafe path.

## Threat 2: Running on a Real Repository

Risk:

User points BranchDojo at an existing project.

Mitigation:

- v0.1 does not support existing repositories.
- `new` refuses non-empty directories.
- `check/reset` require BranchDojo metadata.

## Threat 3: Shell Injection

Risk:

Exercise names or paths are passed into shell commands.

Mitigation:

- Do not invoke shell.
- Use `Command::new("git").args([...])`.

## Threat 4: Global Git Config Modification

Risk:

Tool changes user global identity.

Mitigation:

- Set only local repository config.
- Test local config behavior.

## Threat 5: Path Traversal

Risk:

Path inputs resolve to unsafe locations.

Mitigation:

- Canonicalize carefully.
- Reject parent/root/home/system paths.

## Threat 6: Corrupt Metadata

Risk:

Malformed state file causes unsafe behavior.

Mitigation:

- Strict state validation.
- Refuse unknown schema or exercise.

## Threat 7: Platform-Specific Path Bug

Risk:

Windows and Unix path differences bypass guards.

Mitigation:

- Use `PathBuf`.
- Add cross-platform tests where possible.
- Avoid hardcoded separators.
