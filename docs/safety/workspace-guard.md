# Workspace Guard

The workspace guard determines whether a path is a valid BranchDojo workspace.

## Guard Inputs

- Path.
- Safety check result.
- `.branchdojo.json` existence.
- `.branchdojo.json` content.
- Supported exercise registry.

## Guard Rules

A path is a BranchDojo workspace only if:

- It passes safety checks.
- It contains `.branchdojo.json`.
- The state file parses correctly.
- `tool` is `branchdojo`.
- `validation_policy` is `final-state`.
- `exercise` is supported.

## Commands Using Guard

- `check`
- `hint`
- `reset`

## Failure Behavior

Return a human-readable error. Do not attempt to infer exercise type.

## Security Note

A state file can be copied by a user. Therefore the guard reduces accidental damage but does not establish a complete trust boundary. It must be combined with unsafe path checks.
