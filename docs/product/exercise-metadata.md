# Exercise Metadata

## Status

Metadata foundation implemented for v0.3. Progress-aware hints and Markdown check reports are implemented.

## Goal

Make exercise definitions richer and more consistent across CLI listing, generated READMEs, hints, reports, and documentation.

## Implemented Fields

- `name`
- `title`
- `summary`
- `difficulty`
- `category`
- `estimated_time`
- `skills`
- `starting_branch`
- `expected_final_branch`
- `introduced_in`

## Internal Representation

Metadata can remain hardcoded in Rust for v0.3. External exercise definitions and exercise packs remain future work.

## Expected Uses

- Better `branchdojo list` output.
- More consistent generated `README.branchdojo.md` titles.
- Shared context for progress-aware hints.
- Report headers and summaries.
- Documentation consistency checks.

## Constraints

- Do not add custom external exercise packs in v0.3.
- Do not require network access.
- Keep metadata stable once released.
- Keep exercise IDs stable.

## Testing

- Unit-test metadata completeness for every exercise.
- Integration-test listing output.
- Regression-test generated metadata and README files.
- Add docs review checks manually before release.
