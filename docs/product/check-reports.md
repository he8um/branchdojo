# Check Reports

## Status

Implemented for v0.3. Reports are compatible with v0.4 advanced exercise metadata.

## Goal

Allow users to save validation results as a local practice artifact.

## CLI

```bash
branchdojo check --path <path> --report <file>
```

Report generation should be opt-in. Running `check` without `--report` should not write files.

`branchdojo check --path <path> --json --report <file>` keeps stdout as JSON and writes the Markdown report to the requested file.

## Format

Markdown reports include:

- Exercise ID.
- Exercise title, difficulty, category, estimated time, and expected final branch.
- Status.
- Score.
- Required checks.
- Warning checks.
- Next steps.
- A note that BranchDojo validates final repository state, not exact command sequence.

JSON file export remains future work because `branchdojo check --json` already emits JSON to stdout.

## File Safety Rules

- Refuse unsafe report paths.
- Refuse to write inside `.git`.
- Refuse to overwrite existing files by default.
- Refuse missing parent directories.
- Do not create parent directories automatically.
- Return clear errors when writing fails.
- Do not provide `--force` overwrite behavior in v0.3.

## Validation Semantics

Reports should serialize the existing validation result. They must not change check status, scoring, warnings, or process exit behavior.

Reports do not track command history or shell history.

## Testing

- Report file is created when requested.
- Existing file refusal is tested.
- Unsafe path refusal is tested.
- Markdown content includes status, score, checks, and next steps.
- `--json --report` behavior keeps stdout parseable as JSON.
