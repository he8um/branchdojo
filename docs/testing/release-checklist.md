# Release Checklist

## Code Quality

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test`

## Functional

- [ ] All MVP commands work.
- [ ] All MVP exercises can be generated.
- [ ] All MVP exercises can be validated.
- [ ] Reset is safe.
- [ ] JSON output is valid.

## Docs

- [ ] README updated.
- [ ] CHANGELOG updated.
- [ ] Exercise docs updated.
- [ ] Safety docs updated.
- [ ] Command reference updated.

## Repository Hygiene

- [ ] No private `_dev` files committed.
- [ ] No logs committed.
- [ ] No generated exercise workspace committed.
- [ ] No secrets or tokens.

## Release Notes

Release notes should include:

- New features.
- Known limitations.
- Safety model summary.
- Installation instructions.
