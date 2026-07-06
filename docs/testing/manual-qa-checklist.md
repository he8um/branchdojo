# Manual QA Checklist

Run this before a public release.

## Environment Checks

- [ ] macOS basic run.
- [ ] Linux basic run.
- [ ] Windows basic run if available.
- [ ] Git missing behavior checked if practical.

## CLI Checks

- [ ] `branchdojo list` works.
- [ ] `branchdojo new conflict-basic --path ./dojo-conflict-basic` works.
- [ ] `branchdojo check --path ./dojo-conflict-basic` works.
- [ ] `branchdojo check --path ./dojo-conflict-basic --json` works.
- [ ] `branchdojo hint --path ./dojo-conflict-basic` works.
- [ ] `branchdojo reset --path ./dojo-conflict-basic` works.

## Exercise Checks

- [ ] Manually solve `conflict-basic`.
- [ ] Manually solve `revert-mistake`.
- [ ] Manually solve `wrong-branch-commit`.

## Safety Checks

- [ ] New refuses non-empty directory.
- [ ] Reset refuses non-BranchDojo directory.
- [ ] Check refuses missing metadata.
- [ ] Unsafe paths are refused.

## Documentation Checks

- [ ] README command examples are accurate.
- [ ] Generated README is accurate.
- [ ] Error messages match docs.
- [ ] Roadmap is current.
