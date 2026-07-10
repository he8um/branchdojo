use std::path::Path;

use crate::error::AppResult;

pub mod cherry_pick_basic;
pub mod conflict_basic;
pub mod detached_head_recovery;
pub mod interactive_rebase_basic;
pub mod metadata;
pub mod revert_mistake;
pub mod stash_switch;
pub mod tag_release_fix;
pub mod wrong_branch_commit;

use metadata::ExerciseMetadata;

pub struct Exercise {
    pub metadata: &'static ExerciseMetadata,
    pub goal: &'static str,
    pub hints: &'static [&'static str],
    pub setup: fn(&Path) -> AppResult<()>,
}

pub fn all() -> Vec<&'static Exercise> {
    vec![
        &conflict_basic::EXERCISE,
        &revert_mistake::EXERCISE,
        &wrong_branch_commit::EXERCISE,
        &stash_switch::EXERCISE,
        &cherry_pick_basic::EXERCISE,
        &detached_head_recovery::EXERCISE,
        &interactive_rebase_basic::EXERCISE,
        &tag_release_fix::EXERCISE,
    ]
}

pub fn get(id: &str) -> Option<&'static Exercise> {
    all()
        .into_iter()
        .find(|exercise| exercise.metadata.name == id)
}

fn generated_readme(
    title: &str,
    goal: &str,
    branches: &str,
    files: &str,
    current_branch: &str,
    issue: &str,
    task: &str,
) -> String {
    format!(
        "\
# BranchDojo Exercise: {title}

## Goal

{goal}

## Starting State

BranchDojo has prepared this repository with:

- Branches: {branches}
- Files: {files}
- Current branch: {current_branch}
- Intentional issue: {issue}

## Your Task

{task}

## Rules

- Use real Git commands.
- Do not delete `.git`.
- Do not delete `.branchdojo.json`.
- Do not edit this README to satisfy validation.
- Solve the repository state.

## Useful Commands

These may help:

```bash
git status
git branch
git log --oneline --decorate --graph --all
```

## How to Check

```bash
branchdojo check --path .
```

## How to Reset

```bash
branchdojo reset --path .
```

## Hints

Run:

```bash
branchdojo hint --path .
```
"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_catalog_matches_metadata_catalog() {
        let exercise_names: Vec<_> = all()
            .into_iter()
            .map(|exercise| exercise.metadata.name)
            .collect();
        let metadata_names: Vec<_> = metadata::all_metadata()
            .iter()
            .map(|metadata| metadata.name)
            .collect();
        assert_eq!(exercise_names, metadata_names);
    }
}
