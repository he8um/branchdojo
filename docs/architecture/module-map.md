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

## `src/reports.rs`

Markdown check report rendering and writing.

Responsibilities:

- Render validation results as Markdown.
- Include exercise metadata in report summaries.
- Escape Markdown table cells.
- Refuse unsafe report paths, existing files, directories, missing parent directories, and paths inside `.git`.

## `src/hints.rs`

Progress-aware hint analysis.

Responsibilities:

- Read current repository state without mutating it.
- Detect common blockers such as wrong branch, dirty working tree, active Git operation, conflict markers, missing expected branch, and missing expected files.
- Route lightweight exercise-specific hint guidance.
- Preserve static exercise hints as the general fallback.

## `src/exercises/`

Exercise setup modules.

Each module should:

- Reference shared metadata.
- Create repository state.
- Write exercise README.
- Provide hints.

## `src/exercises/metadata.rs`

Exercise catalog metadata.

Responsibilities:

- Define exercise name, title, summary, difficulty, category, estimated time, skills, starting branch, expected final branch, and introduced version.
- Provide stable metadata lookup helpers.
- Provide the metadata source for `branchdojo list`.

## `src/validators/`

Final-state validators.

Each module should:

- Run common checks.
- Run exercise-specific checks.
- Return `ValidationResult`.

## v0.3 Notes

Modules touched by v0.3 work:

- `src/exercises/metadata.rs`: richer metadata fields support listing output and check reports.
- `src/hints.rs`: progress-aware hints use shared repository-state analysis and exercise-specific routing.
- `src/reports.rs`: report file rendering and write safety.

Exercise metadata foundation, progress-aware hints, and Markdown report output are implemented.

## v0.4 Planning Notes

Modules touched by v0.4 implementation work so far:

- `src/exercises/metadata.rs`: `advanced` difficulty support and `interactive-rebase-basic` metadata.
- `src/exercises/interactive_rebase_basic.rs`: setup for the first advanced built-in exercise.
- `src/validators/interactive_rebase_basic.rs`: final-state and history-shape validation for the first advanced exercise.
- `src/hints.rs`: progress-aware hint routing for `interactive-rebase-basic`.
- `src/reports.rs`: Markdown reports remain metadata-backed and require no exercise-specific report code.

Likely modules touched by future v0.4 work:

- `src/exercises/`: setup modules for any additional selected advanced built-in exercises.
- `src/validators/`: final-state validators for additional selected advanced exercises.
- `src/hints.rs`: progress-aware hints for additional selected advanced exercises.

Possible future areas:

- Learning-path documentation can live under `docs/product/` before any CLI support exists.
- Exercise-pack or custom-definition modules are deferred. Any future design must preserve workspace safety, deterministic validation, and no command-history tracking.
