# Module Map

## `src/main.rs`

Entrypoint. Should only initialize CLI handling and call the application command router.

## `src/cli.rs`

Defines command-line interface using `clap`.

Responsibilities:

- Command enum.
- Argument definitions.
- CLI parsing.
- Basic command routing.

## `src/git.rs`

Wrapper around Git commands.

Responsibilities:

- Check Git availability.
- Run Git commands with explicit args.
- Capture stdout/stderr.
- Set local identity.
- Query branch, status, history, file content.

## `src/state.rs`

Workspace metadata model.

Responsibilities:

- Define `BranchDojoState`.
- Serialize/deserialize `.branchdojo.json`.
- Validate required fields.
- Reject invalid tool/schema/exercise.

## `src/safety.rs`

Path and workspace safety checks.

Responsibilities:

- Unsafe path detection.
- Non-empty target detection.
- Workspace guard.
- Reset guard.

## `src/result.rs`

Validation result model.

Responsibilities:

- `CheckStatus`.
- `Severity`.
- `CheckResult`.
- `ValidationResult`.
- Score aggregation.
- Overall status calculation.

## `src/output.rs`

User output.

Responsibilities:

- Human-readable output.
- JSON output.
- Error formatting.

## `src/exercises/`

Exercise setup modules.

Each module should:

- Provide metadata.
- Create repository state.
- Write exercise README.
- Provide hints.

## `src/validators/`

Final-state validators.

Each module should:

- Run common checks.
- Run exercise-specific checks.
- Return `ValidationResult`.
