# Validation Check Catalog

This catalog defines stable check IDs.

## Common Checks

### `metadata_exists`

- Label: Metadata exists
- Severity: required
- Pass: `.branchdojo.json` exists and is valid.
- Fail: Missing or invalid metadata.
- Next: Run the command inside a BranchDojo workspace.

### `git_directory_exists`

- Label: Git directory exists
- Severity: required
- Pass: `.git` exists.
- Fail: Repository metadata is missing.

### `current_branch_main`

- Label: Current branch is main
- Severity: required
- Pass: current branch is `main`.
- Fail: user is on another branch.

### `working_tree_clean`

- Label: Working tree is clean
- Severity: required
- Pass: `git status --porcelain` is empty.
- Fail: uncommitted changes exist.

### `no_active_git_operation`

- Label: No merge/rebase/cherry-pick/revert state is active
- Severity: required
- Pass: no operation state files exist.
- Fail: active operation state exists.

### `file_exists:<path>`

- Label: File exists
- Severity: required
- Pass: expected file exists.
- Fail: expected file is missing.

### `file_contains:<id>`

- Label: Expected content exists
- Severity: required
- Pass: file contains required normalized text.
- Fail: content missing.

### `file_not_contains:<id>`

- Label: Forbidden content is absent
- Severity: required
- Pass: file does not contain forbidden text.
- Fail: forbidden text exists.

### `branch_exists:<branch>`

- Label: Branch exists
- Severity: required
- Pass: branch exists.
- Fail: branch is missing.

## Exercise-Specific Checks

### `conflict_markers_removed`

- Exercise: conflict-basic
- Severity: required
- Pass: no `<<<<<<<`, `=======`, or `>>>>>>>` markers exist.

### `history_includes_feature_work`

- Exercise: conflict-basic
- Severity: required
- Pass: history includes expected feature commit or content lineage.

### `merge_commit_detected`

- Exercise: conflict-basic
- Severity: warning
- Warning: no merge commit detected while final state is valid.

### `bad_commit_still_exists`

- Exercise: revert-mistake
- Severity: required
- Pass: bad commit remains in history.

### `fix_commit_after_bad_commit`

- Exercise: revert-mistake
- Severity: required
- Pass: at least one later commit exists after bad commit.

### `revert_style_detected`

- Exercise: revert-mistake
- Severity: warning
- Warning: final state is valid but no revert-style commit message/history signal exists.

### `main_excludes_accidental_work`

- Exercise: wrong-branch-commit
- Severity: required
- Pass: accidental content is absent from main.

### `feature_includes_accidental_work`

- Exercise: wrong-branch-commit
- Severity: required
- Pass: accidental content is present on feature branch.

### `feature_settings_copy_exists`

- Exercise: stash-switch
- Severity: required
- Pass: expected settings copy is present on `feature/settings-copy`.

### `preserved_work_exists`

- Exercise: stash-switch
- Severity: required
- Pass: preserved local work is present on `main`.

### `wip_commit_detected`

- Exercise: stash-switch
- Severity: warning
- Warning: final state is valid, but history includes a WIP-style commit subject.

### `feature_copy_on_main`

- Exercise: stash-switch
- Severity: warning
- Warning: final state is valid, but feature copy appears on `main` too.

### `current_branch_release_current`

- Exercise: cherry-pick-basic
- Severity: required
- Pass: current branch is `release/current`.

### `bugfix_exists_on_release`

- Exercise: cherry-pick-basic
- Severity: required
- Pass: expected checkout bugfix content is present on `release/current`.

### `legacy_content_absent_on_release`

- Exercise: cherry-pick-basic
- Severity: required
- Pass: legacy-only support content is absent from `release/current`.

### `source_commit_still_exists`

- Exercise: cherry-pick-basic
- Severity: required
- Pass: original `Fix empty checkout cart` commit remains reachable from `support/legacy-fix`.

### `bugfix_remains_on_support`

- Exercise: cherry-pick-basic
- Severity: required
- Pass: bugfix content remains present on `support/legacy-fix`.

### `cherry_pick_style_detected`

- Exercise: cherry-pick-basic
- Severity: warning
- Warning: final state is valid, but no cherry-pick-style commit subject is detected on `release/current`.

### `broad_merge_shape_detected`

- Exercise: cherry-pick-basic
- Severity: warning
- Warning: final state is valid, but history shape includes a merge commit on `release/current`.

### `current_branch_recovery_detached_work`

- Exercise: detached-head-recovery
- Severity: required
- Pass: current branch is `recovery/detached-work`.

### `recovered_work_exists`

- Exercise: detached-head-recovery
- Severity: required
- Pass: recovered work content is present on `recovery/detached-work`.

### `not_detached_head`

- Exercise: detached-head-recovery
- Severity: required
- Pass: repository is on a named branch, not detached HEAD.

### `detached_commit_preserved`

- Exercise: detached-head-recovery
- Severity: warning
- Warning: final state is valid, but the original detached commit subject is not reachable from `recovery/detached-work`.

### `current_branch_feature_profile_copy`

- Exercise: interactive-rebase-basic
- Severity: required
- Pass: current branch is `feature/profile-copy`.

### `debug_file_absent`

- Exercise: interactive-rebase-basic
- Severity: required
- Pass: `debug.txt` is absent from `feature/profile-copy`.

### `debug_content_absent`

- Exercise: interactive-rebase-basic
- Severity: required
- Pass: temporary debug content is absent from `feature/profile-copy`.

### `wip_debug_commit_reachable`

- Exercise: interactive-rebase-basic
- Severity: warning
- Warning: final content is valid, but WIP/debug commit subjects remain reachable from `feature/profile-copy`.

### `reviewable_history_shape`

- Exercise: interactive-rebase-basic
- Severity: warning
- Warning: final content is valid, but branch history is broader or noisier than expected for review.
