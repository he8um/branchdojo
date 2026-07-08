# Exercise Metadata

## Status

Design proposal for v0.3. Not implemented.

## Goal

Make exercise definitions richer and more consistent across CLI listing, generated READMEs, hints, reports, and documentation.

## Proposed Fields

- `id`
- `title`
- `difficulty`
- `category`
- `estimated_time`
- `skills`
- `description`
- `starting_branch`
- `expected_final_branch`
- `introduced_in`
- `files`
- `branches`

## Internal Representation

Metadata can remain hardcoded in Rust for v0.3. External exercise definitions and exercise packs remain future work.

## Expected Uses

- Better `branchdojo list` output.
- More consistent generated `README.branchdojo.md`.
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
