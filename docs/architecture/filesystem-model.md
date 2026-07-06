# Filesystem Model

BranchDojo creates and manages disposable local exercise directories.

## Workspace Root

A generated workspace contains:

```text
<workspace>/
├── .git/
├── .branchdojo.json
├── README.branchdojo.md
└── exercise files
```

## Creation Rules

`branchdojo new` may create a directory if:

- The path is safe.
- The path does not exist, or exists and is empty.
- The path is not root, home, parent, or a system directory.
- The exercise ID is supported.

## Reset Rules

`branchdojo reset` may remove and recreate a workspace only if:

- The path is safe.
- `.branchdojo.json` exists.
- `.branchdojo.json` is valid.
- `tool == branchdojo`.
- The exercise is supported.

## Non-Empty Directory Rule

`new` must fail for non-empty directories. This avoids accidental overwrite.

## Generated Files

Generated files should be simple text files with deterministic content. Avoid binary files in v0.1.

## Path Handling

Use `Path` and `PathBuf`. Do not concatenate paths with `/`.

## Line Endings

Validators should normalize line endings before content checks.
