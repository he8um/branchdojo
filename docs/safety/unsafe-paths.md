# Unsafe Paths

BranchDojo must reject paths that could cause accidental damage.

## Always Reject

- `/` on Unix.
- Home directory.
- Parent directory references used as target, such as `..`.
- System directories.
- Empty path.
- Paths that resolve outside expected local context when canonicalized.

## Reject for `new`

- Existing non-empty directory.
- Existing Git repository.
- Current BranchDojo project repository if detected.

## Reject for `check`, `hint`, and `reset`

- Missing `.branchdojo.json`.
- Invalid `.branchdojo.json`.
- Unsafe path.

## Windows Considerations

Reject likely dangerous roots and system paths, including:

- Drive root like `C:\`.
- User profile root.
- Windows system directories.

## Implementation Hint

Use canonicalization carefully. If a path does not exist yet, canonicalize the nearest existing parent and validate the final target name separately.
