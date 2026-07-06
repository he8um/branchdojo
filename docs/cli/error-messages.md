# Error Message Catalog

BranchDojo errors should include a code, a short message, the likely cause, and a next step.

## BD001: Git Not Found

Message:

```text
BD001: Git is not installed or not available in PATH.
```

Cause:

BranchDojo needs the local Git binary to create and validate exercises.

Next step:

Install Git and run the command again.

## BD002: Target Path Is Not Empty

Message:

```text
BD002: Target path already exists and is not empty.
```

Cause:

`branchdojo new` refuses to create an exercise inside a non-empty directory.

Next step:

Choose a new empty path.

## BD003: Unsafe Path Refused

Message:

```text
BD003: Unsafe path refused.
```

Cause:

The path is root, home, parent, system-like, or otherwise unsafe.

Next step:

Use a dedicated local exercise path such as `./dojo-conflict-basic`.

## BD004: Missing State File

Message:

```text
BD004: .branchdojo.json is missing. This is not a BranchDojo workspace.
```

Cause:

`check`, `hint`, or `reset` was run outside a generated exercise workspace.

Next step:

Create an exercise with `branchdojo new <exercise> --path <path>`.

## BD005: Invalid State File

Message:

```text
BD005: .branchdojo.json is invalid.
```

Cause:

The file is malformed, missing required fields, or does not identify BranchDojo.

Next step:

Reset only if the workspace is valid, or create a new exercise.

## BD006: Unsupported Exercise

Message:

```text
BD006: Unsupported exercise: <name>.
```

Cause:

The exercise ID is unknown.

Next step:

Run `branchdojo list`.

## BD007: Unexpected Git State

Message:

```text
BD007: Repository is in an unexpected state.
```

Cause:

The generated repository is missing `.git`, has corrupted history, or cannot be inspected.

Next step:

Run `branchdojo reset --path <path>` if the workspace metadata is valid.

## BD008: Reset Refused

Message:

```text
BD008: Reset refused because this is not a valid BranchDojo workspace.
```

Cause:

Reset is only allowed with valid BranchDojo metadata.

Next step:

Create a new exercise or use a valid BranchDojo workspace.
