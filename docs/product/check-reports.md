# Check Reports

## Status

Design proposal for v0.3. Not implemented.

## Goal

Allow users to save validation results as a local practice artifact.

## CLI Proposal

```bash
branchdojo check --path <path> --report <file>
```

Report generation should be opt-in. Running `check` without `--report` should not write files.

## Format Proposal

Markdown first:

- Exercise ID.
- Status.
- Score.
- Required checks.
- Warning checks.
- Next steps.

JSON file export can remain future work because `branchdojo check --json` already emits JSON to stdout.

## File Safety Rules

- Refuse unsafe report paths.
- Refuse to write inside `.git`.
- Refuse to overwrite existing files by default.
- Create parent directories only if that behavior is explicitly designed and tested.
- Return clear errors when writing fails.

## Validation Semantics

Reports should serialize the existing validation result. They must not change check status, scoring, warnings, or process exit behavior.

## Testing

- Report file is created when requested.
- Existing file refusal is tested if overwrite is disallowed.
- Unsafe path refusal is tested.
- Markdown content includes status, score, checks, and next steps.
- `--json` behavior remains unchanged.
