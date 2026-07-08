use std::path::Path;

use crate::error::AppResult;

pub mod cherry_pick_basic;
pub mod conflict_basic;
pub mod detached_head_recovery;
pub mod revert_mistake;
pub mod stash_switch;
pub mod wrong_branch_commit;

pub struct Exercise {
    pub id: &'static str,
    pub title: &'static str,
    pub difficulty: &'static str,
    pub estimated_time: &'static str,
    pub skills: &'static [&'static str],
    pub description: &'static str,
    pub goal: &'static str,
    pub hints: &'static [&'static str],
    pub setup: fn(&Path) -> AppResult<()>,
}

pub fn all() -> Vec<&'static Exercise> {
    vec![
        &cherry_pick_basic::EXERCISE,
        &conflict_basic::EXERCISE,
        &detached_head_recovery::EXERCISE,
        &revert_mistake::EXERCISE,
        &stash_switch::EXERCISE,
        &wrong_branch_commit::EXERCISE,
    ]
}

pub fn get(id: &str) -> Option<&'static Exercise> {
    all().into_iter().find(|exercise| exercise.id == id)
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
