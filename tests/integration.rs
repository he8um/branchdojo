use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_branchdojo"))
}

fn temp_path(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("branchdojo-test-{name}-{nonce}"))
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(bin()).args(args).output().unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn git(path: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_may_fail(path: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .unwrap()
}

fn git_output(path: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn list_prints_available_exercises() {
    let output = run(&["list"]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Available exercises:"));
    assert!(output.contains("cherry-pick-basic"));
    assert!(output.contains("conflict-basic"));
    assert!(output.contains("detached-head-recovery"));
    assert!(output.contains("interactive-rebase-basic"));
    assert!(output.contains("merge-vs-rebase"));
    assert!(output.contains("revert-mistake"));
    assert!(output.contains("stash-switch"));
    assert!(output.contains("tag-release-fix"));
    assert!(output.contains("wrong-branch-commit"));
    assert!(output.contains("advanced"));
    assert!(output.contains("beginner"));
    assert!(output.contains("intermediate"));
    assert!(output.contains("History rewriting"));
    assert!(output.contains("Branch integration"));
    assert!(output.contains("Release recovery"));
    assert!(output.contains("Merge conflicts"));
    assert!(output.contains("Selective history"));
    assert!(output.contains("Recovery"));
}

#[test]
fn root_help_succeeds() {
    let output = run(&["--help"]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Usage:"));
    assert!(output.contains("Commands:"));
    assert!(output.contains("list"));
    assert!(output.contains("new"));
    assert!(output.contains("check"));
}

#[test]
fn root_version_succeeds() {
    let output = run(&["--version"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "branchdojo 0.3.0\n");
}

#[test]
fn subcommand_help_succeeds() {
    for command in ["list", "new", "check"] {
        let output = run(&[command, "--help"]);
        assert!(output.status.success());
        let output = stdout(&output);
        assert!(output.contains("Usage:"));
    }
}

#[test]
fn missing_path_for_new_fails_cleanly() {
    let output = run(&["new", "conflict-basic"]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("required"));
    assert!(output.contains("--path"));
}

#[test]
fn missing_path_for_check_fails_cleanly() {
    let output = run(&["check"]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("required"));
    assert!(output.contains("--path"));
}

#[test]
fn unknown_command_fails_cleanly() {
    let output = run(&["unknown"]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("unrecognized subcommand"));
    assert!(output.contains("Usage:"));
}

#[test]
fn unsupported_exercise_returns_useful_error() {
    let path = temp_path("unsupported");
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["new", "not-real", "--path", &path_arg]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("BD006"));
    assert!(output.contains("Unsupported exercise: not-real."));
    assert!(output.contains("branchdojo list"));
}

#[test]
fn hint_prints_progress_aware_and_general_sections_for_all_exercises() {
    for exercise in [
        "conflict-basic",
        "revert-mistake",
        "wrong-branch-commit",
        "stash-switch",
        "cherry-pick-basic",
        "detached-head-recovery",
        "interactive-rebase-basic",
        "merge-vs-rebase",
        "tag-release-fix",
    ] {
        let path = temp_path(&format!("hint-{exercise}"));
        let path_arg = path.to_string_lossy().to_string();
        assert!(run(&["new", exercise, "--path", &path_arg])
            .status
            .success());

        let output = run(&["hint", "--path", &path_arg]);
        assert!(output.status.success());
        let output = stdout(&output);
        assert!(output.contains(&format!("Hints for {exercise}:")));
        assert!(output.contains("Progress-aware hints:"));
        assert!(output.contains("General hints:"));

        fs::remove_dir_all(path).unwrap();
    }
}

#[test]
fn hint_refuses_missing_workspace_metadata() {
    let path = temp_path("hint-missing-metadata");
    fs::create_dir_all(&path).unwrap();
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["hint", "--path", &path_arg]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("BD004"));
    assert!(output.contains(".branchdojo.json is missing."));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn conflict_basic_hint_mentions_unresolved_conflict_markers() {
    let path = temp_path("hint-conflict-markers");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let merge = git_may_fail(&path, &["merge", "feature/landing-copy"]);
    assert!(!merge.status.success());

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("Conflict markers"));
    assert!(output.contains("remove conflict markers"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_hint_mentions_preserving_dirty_local_work() {
    let path = temp_path("hint-stash-dirty");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "stash-switch", "--path", &path_arg])
        .status
        .success());

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("local work"));
    assert!(output.contains("Preserve"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_hint_mentions_release_branch_and_target_bugfix() {
    let path = temp_path("hint-cherry");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "main"]);

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("release/current"));
    assert!(output.contains("target bugfix"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_hint_mentions_detached_head_and_recovery_branch() {
    let path = temp_path("hint-detached");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("detached HEAD"));
    assert!(output.contains("recovery/detached-work"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn new_refuses_non_empty_directory() {
    let path = temp_path("non-empty");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("file.txt"), "existing data").unwrap();
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["new", "conflict-basic", "--path", &path_arg]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("BD002"));
    assert!(path.join("file.txt").exists());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn invalid_workspace_metadata_returns_useful_error() {
    let path = temp_path("invalid-metadata");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join(".branchdojo.json"), "{ invalid json").unwrap();
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["check", "--path", &path_arg]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("BD005"));
    assert!(output.contains(".branchdojo.json is invalid."));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn missing_workspace_metadata_returns_useful_error() {
    let path = temp_path("missing-metadata");
    fs::create_dir_all(&path).unwrap();
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["check", "--path", &path_arg]);
    assert!(!output.status.success());
    let output = stderr(&output);
    assert!(output.contains("BD004"));
    assert!(output.contains(".branchdojo.json is missing."));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn conflict_basic_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("conflict");
    let path_arg = path.to_string_lossy().to_string();
    let new_output = run(&["new", "conflict-basic", "--path", &path_arg]);
    assert!(new_output.status.success());

    let check_output = run(&["check", "--path", &path_arg]);
    assert!(check_output.status.success());
    let human_output = stdout(&check_output);
    assert!(human_output.contains("Status: FAILED"));
    assert!(!human_output.contains("⚠️"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let stdout = String::from_utf8_lossy(&json_output.stdout);
    let json: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["exercise"], "conflict-basic");
    assert_eq!(json["status"], "failed");
    assert!(json["score"].is_number());
    assert!(json["total"].is_number());
    assert!(json["checks"].is_array());
    assert!(json["next_steps"].is_array());
    let first_check = &json["checks"][0];
    assert!(first_check["id"].is_string());
    assert!(first_check["label"].is_string());
    assert_eq!(first_check["status"], "passed");
    assert_eq!(first_check["severity"], "required");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn check_report_creates_markdown_and_keeps_human_output() {
    let path = temp_path("report-human");
    let report = temp_path("report-human-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(output.status.success());
    let output_text = stdout(&output);
    assert!(output_text.contains("BranchDojo Result"));
    assert!(output_text.contains("Status: FAILED"));
    assert!(output_text.contains("Report written:"));

    let report_text = fs::read_to_string(&report).unwrap();
    assert!(report_text.contains("# BranchDojo Check Report"));
    assert!(report_text.contains("- Exercise: `conflict-basic`"));
    assert!(report_text.contains("- Title: Resolve a basic merge conflict"));
    assert!(report_text.contains("- Status: FAILED"));
    assert!(report_text.contains("- Score:"));
    assert!(report_text.contains("| Severity | Status | Check | Details |"));
    assert!(report_text.contains("## Next Steps"));

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn check_without_report_does_not_create_report_file() {
    let path = temp_path("report-absent");
    let report = temp_path("report-absent-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(!report.exists());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn check_json_with_report_keeps_stdout_parseable_json() {
    let path = temp_path("report-json");
    let report = temp_path("report-json-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&[
        "check",
        "--path",
        &path_arg,
        "--json",
        "--report",
        &report_arg,
    ]);
    assert!(output.status.success());
    let output_text = stdout(&output);
    let json: Value = serde_json::from_str(&output_text).unwrap();
    assert_eq!(json["exercise"], "conflict-basic");
    assert!(!output_text.contains("Report written"));
    assert!(report.exists());

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn check_report_refuses_existing_file() {
    let path = temp_path("report-existing");
    let report = temp_path("report-existing-md").with_extension("md");
    fs::write(&report, "existing").unwrap();
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("BD007"));
    assert!(error.contains("Report file already exists."));

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn check_report_refuses_directory_path() {
    let path = temp_path("report-directory-workspace");
    let report_dir = temp_path("report-directory");
    fs::create_dir_all(&report_dir).unwrap();
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report_dir.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("BD008"));
    assert!(error.contains("Unsafe report path."));

    fs::remove_dir_all(path).unwrap();
    fs::remove_dir_all(report_dir).unwrap();
}

#[test]
fn check_report_refuses_path_inside_git() {
    let path = temp_path("report-git");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());
    let report = path.join(".git").join("branchdojo-report.md");
    let report_arg = report.to_string_lossy().to_string();

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("BD008"));
    assert!(error.contains("Unsafe report path."));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn check_report_refuses_missing_parent_directory() {
    let path = temp_path("report-missing-parent");
    let report = temp_path("report-missing-parent-dir")
        .join("missing")
        .join("report.md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("BD009"));
    assert!(error.contains("Report parent directory does not exist."));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn generated_exercises_include_metadata_readme_and_local_identity() {
    for exercise in [
        "cherry-pick-basic",
        "conflict-basic",
        "detached-head-recovery",
        "interactive-rebase-basic",
        "merge-vs-rebase",
        "revert-mistake",
        "stash-switch",
        "tag-release-fix",
        "wrong-branch-commit",
    ] {
        let path = temp_path(exercise);
        let path_arg = path.to_string_lossy().to_string();
        let output = run(&["new", exercise, "--path", &path_arg]);
        assert!(output.status.success());

        assert!(path.join(".git").exists());
        assert!(path.join(".branchdojo.json").exists());
        assert!(path.join("README.branchdojo.md").exists());
        let state = fs::read_to_string(path.join(".branchdojo.json")).unwrap();
        let state: Value = serde_json::from_str(&state).unwrap();
        assert_eq!(state["tool"], "branchdojo");
        assert_eq!(state["schema_version"], "0.1.0");
        assert_eq!(state["exercise"], exercise);
        let expected_branch = if exercise == "cherry-pick-basic" {
            "release/current"
        } else if exercise == "detached-head-recovery" {
            "recovery/detached-work"
        } else if exercise == "interactive-rebase-basic" {
            "feature/profile-copy"
        } else {
            "main"
        };
        assert_eq!(state["expected_branch"], expected_branch);
        let expected_file_count = if matches!(exercise, "tag-release-fix" | "merge-vs-rebase") {
            2
        } else {
            1
        };
        assert_eq!(
            state["expected_files"].as_array().unwrap().len(),
            expected_file_count
        );
        assert_eq!(state["validation_policy"], "final-state");
        let created_at = state["created_at"].as_str().unwrap();
        assert!(!created_at.is_empty());
        assert_ne!(created_at, "2026-07-07T00:00:00Z");
        assert!(created_at.contains('T'));
        assert!(created_at.ends_with('Z'));
        assert_eq!(git_output(&path, &["config", "user.name"]), "BranchDojo");
        assert_eq!(
            git_output(&path, &["config", "user.email"]),
            "branchdojo@example.local"
        );

        fs::remove_dir_all(path).unwrap();
    }
}

#[test]
fn merge_vs_rebase_new_creates_expected_starting_state() {
    let path = temp_path("merge-rebase-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "merge-vs-rebase", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("pricing.txt").exists());
    assert!(path.join("checkout.txt").exists());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "feature/pricing-copy"
    );
    assert!(!git_output(&path, &["rev-parse", "--verify", "main"]).is_empty());
    assert!(!git_output(&path, &["rev-parse", "--verify", "feature/pricing-copy"]).is_empty());
    assert!(
        git_output(&path, &["show", "main:checkout.txt"]).contains("Your payment is protected.")
    );
    assert!(
        git_output(&path, &["show", "feature/pricing-copy:pricing.txt"])
            .contains("You can change plans at any time.")
    );
    assert!(
        !git_output(&path, &["show", "feature/pricing-copy:checkout.txt"])
            .contains("Your payment is protected.")
    );
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("merge-rebase-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let human = stdout(&output);
    assert!(human.contains("Status: FAILED"));
    assert!(human.contains("Current branch is main"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "merge-vs-rebase");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_linear_solution_passes() {
    let path = temp_path("merge-rebase-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["rebase", "main"]);
    git(&path, &["switch", "main"]);
    git(&path, &["merge", "--ff-only", "feature/pricing-copy"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_merge_commit_solution_warns() {
    let path = temp_path("merge-rebase-warning");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "main"]);
    git(
        &path,
        &[
            "merge",
            "--no-ff",
            "feature/pricing-copy",
            "-m",
            "Merge pricing copy",
        ],
    );

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("Merge commit integration check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_wrong_branch_fails() {
    let path = temp_path("merge-rebase-wrong-branch");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Current branch is main"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_dirty_working_tree_fails() {
    let path = temp_path("merge-rebase-dirty");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    fs::write(path.join("notes.txt"), "integration notes\n").unwrap();

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Working tree is clean"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_missing_pricing_content_fails() {
    let path = temp_path("merge-rebase-missing-pricing");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "main"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Main has pricing headline"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_missing_checkout_content_fails() {
    let path = temp_path("merge-rebase-missing-checkout");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["rebase", "main"]);
    git(&path, &["switch", "main"]);
    git(&path, &["merge", "--ff-only", "feature/pricing-copy"]);
    fs::write(
        path.join("checkout.txt"),
        "Checkout\nStandard payment flow.\n",
    )
    .unwrap();
    git(&path, &["add", "checkout.txt"]);
    git(&path, &["commit", "-m", "Remove checkout trust copy"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Main has checkout trust copy"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_missing_feature_branch_fails() {
    let path = temp_path("merge-rebase-missing-feature");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["rebase", "main"]);
    git(&path, &["switch", "main"]);
    git(&path, &["merge", "--ff-only", "feature/pricing-copy"]);
    git(&path, &["branch", "-d", "feature/pricing-copy"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Branch `feature/pricing-copy` exists"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_hint_mentions_integration() {
    let path = temp_path("merge-rebase-hint");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Hints for merge-vs-rebase:"));
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("feature/pricing-copy"));
    assert!(output.contains("integrate it into `main`"));
    assert!(output.contains("git log --oneline --decorate --graph --all"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn merge_vs_rebase_report_includes_metadata() {
    let path = temp_path("merge-rebase-report");
    let report = temp_path("merge-rebase-report-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(output.status.success());
    assert!(report.exists());
    let report_text = fs::read_to_string(&report).unwrap();
    assert!(report_text.contains("- Exercise: `merge-vs-rebase`"));
    assert!(report_text.contains("- Title: Integrate a feature branch with a clean history"));
    assert!(report_text.contains("- Difficulty: advanced"));
    assert!(report_text.contains("- Category: Branch integration"));

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn merge_vs_rebase_reset_works() {
    let path = temp_path("merge-rebase-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "merge-vs-rebase", "--path", &path_arg])
        .status
        .success());

    git(&path, &["rebase", "main"]);
    git(&path, &["switch", "main"]);
    git(&path, &["merge", "--ff-only", "feature/pricing-copy"]);

    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "feature/pricing-copy"
    );
    assert!(
        git_output(&path, &["show", "main:checkout.txt"]).contains("Your payment is protected.")
    );
    assert!(
        !git_output(&path, &["show", "feature/pricing-copy:checkout.txt"])
            .contains("Your payment is protected.")
    );
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_new_creates_expected_starting_state() {
    let path = temp_path("tag-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "tag-release-fix", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("VERSION").exists());
    assert!(path.join("app.txt").exists());
    assert_eq!(git_output(&path, &["branch", "--show-current"]), "main");
    assert!(!git_output(&path, &["rev-parse", "--verify", "main"]).is_empty());
    assert!(!git_output(&path, &["rev-parse", "--verify", "v1.0.0"]).is_empty());
    assert_eq!(git_output(&path, &["cat-file", "-t", "v1.0.0"]), "tag");
    assert!(git_output(&path, &["show", "v1.0.0:app.txt"]).contains("release_blocker=true"));
    assert!(fs::read_to_string(path.join("app.txt"))
        .unwrap()
        .contains("release_blocker=false"));
    assert!(fs::read_to_string(path.join("app.txt"))
        .unwrap()
        .contains("release_ready=true"));
    assert_eq!(fs::read_to_string(path.join("VERSION")).unwrap(), "1.0.0\n");
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("tag-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let human = stdout(&output);
    assert!(human.contains("Status: FAILED"));
    assert!(human.contains("Tag points to release blocker fix"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "tag-release-fix");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_annotated_solution_passes() {
    let path = temp_path("tag-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    git(&path, &["tag", "-d", "v1.0.0"]);
    git(&path, &["tag", "-a", "v1.0.0", "-m", "Release v1.0.0"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_lightweight_tag_warns() {
    let path = temp_path("tag-warning-lightweight");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    git(&path, &["tag", "-d", "v1.0.0"]);
    git(&path, &["tag", "v1.0.0"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("Release tag annotation check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_missing_tag_fails() {
    let path = temp_path("tag-missing");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    git(&path, &["tag", "-d", "v1.0.0"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Tag `v1.0.0` exists"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_old_tag_target_fails() {
    let path = temp_path("tag-old-target");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Tagged commit excludes old blocker"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_dirty_working_tree_fails() {
    let path = temp_path("tag-dirty");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    fs::write(path.join("notes.txt"), "uncommitted release notes\n").unwrap();

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Working tree is clean"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_wrong_branch_fails() {
    let path = temp_path("tag-wrong-branch");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "-c", "release/check"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Current branch is main"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_missing_fixed_content_fails() {
    let path = temp_path("tag-missing-fixed");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    fs::write(path.join("app.txt"), "release_blocker=false\n").unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Break release readiness"]);
    git(&path, &["tag", "-d", "v1.0.0"]);
    git(&path, &["tag", "-a", "v1.0.0", "-m", "Release v1.0.0"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Main is release ready"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_hint_mentions_tag_target() {
    let path = temp_path("tag-hint");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Hints for tag-release-fix:"));
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("v1.0.0"));
    assert!(output.contains("old release content"));
    assert!(output.contains("git show v1.0.0"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn tag_release_fix_report_includes_metadata() {
    let path = temp_path("tag-report");
    let report = temp_path("tag-report-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(output.status.success());
    assert!(report.exists());
    let report_text = fs::read_to_string(&report).unwrap();
    assert!(report_text.contains("- Exercise: `tag-release-fix`"));
    assert!(report_text.contains("- Title: Fix a release tag after a blocker"));
    assert!(report_text.contains("- Difficulty: advanced"));
    assert!(report_text.contains("- Category: Release recovery"));

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn tag_release_fix_reset_works() {
    let path = temp_path("tag-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "tag-release-fix", "--path", &path_arg])
        .status
        .success());

    git(&path, &["tag", "-d", "v1.0.0"]);
    git(&path, &["tag", "-a", "v1.0.0", "-m", "Release v1.0.0"]);

    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert_eq!(git_output(&path, &["branch", "--show-current"]), "main");
    assert!(git_output(&path, &["show", "v1.0.0:app.txt"]).contains("release_blocker=true"));
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_new_creates_expected_starting_state() {
    let path = temp_path("rebase-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "interactive-rebase-basic", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("profile.txt").exists());
    assert!(path.join("debug.txt").exists());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "feature/profile-copy"
    );
    assert!(!git_output(&path, &["rev-parse", "--verify", "main"]).is_empty());
    assert!(!git_output(&path, &["rev-parse", "--verify", "feature/profile-copy"]).is_empty());
    let log = git_output(&path, &["log", "--format=%s"]);
    assert!(log.contains("Add profile copy draft"));
    assert!(log.contains("WIP debug profile copy"));
    assert!(log.contains("Fix profile copy typo"));
    assert!(log.contains("Polish profile copy"));
    let profile = fs::read_to_string(path.join("profile.txt")).unwrap();
    assert!(profile.contains("Profile page"));
    assert!(profile.contains("Welcome to your profile."));
    assert!(profile.contains("Manage your account details here."));
    assert!(fs::read_to_string(path.join("debug.txt"))
        .unwrap()
        .contains("temporary debug notes"));
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("rebase-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let human = stdout(&output);
    assert!(human.contains("Status: FAILED"));
    assert!(human.contains("Debug file is absent"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "interactive-rebase-basic");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_clean_history_solution_passes() {
    let path = temp_path("rebase-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    git(&path, &["reset", "--soft", "main"]);
    fs::remove_file(path.join("debug.txt")).unwrap();
    fs::write(
        path.join("profile.txt"),
        "Profile page\nWelcome to your profile.\nManage your account details here.\n",
    )
    .unwrap();
    git(&path, &["add", "-A"]);
    git(&path, &["commit", "-m", "Add polished profile copy"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_content_correct_but_messy_history_warns() {
    let path = temp_path("rebase-warning");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    fs::remove_file(path.join("debug.txt")).unwrap();
    git(&path, &["add", "-A"]);
    git(&path, &["commit", "-m", "Remove debug notes"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("WIP/debug commit cleanup check"));
    assert!(output.contains("Reviewable history shape check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_wrong_branch_fails() {
    let path = temp_path("rebase-wrong-branch");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    git(&path, &["switch", "main"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Current branch is feature/profile-copy"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_dirty_working_tree_fails() {
    let path = temp_path("rebase-dirty");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    fs::write(path.join("notes.txt"), "uncommitted notes\n").unwrap();

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Working tree is clean"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_debug_content_remaining_fails() {
    let path = temp_path("rebase-debug-fail");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Debug content is absent"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_missing_profile_content_fails() {
    let path = temp_path("rebase-missing-content");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    fs::remove_file(path.join("debug.txt")).unwrap();
    fs::write(
        path.join("profile.txt"),
        "Profile page\nWelcome to your profile.\n",
    )
    .unwrap();
    git(&path, &["add", "-A"]);
    git(&path, &["commit", "-m", "Remove debug notes"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Profile account-management copy exists"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_hint_mentions_history_cleanup() {
    let path = temp_path("rebase-hint");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    let output = run(&["hint", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Hints for interactive-rebase-basic:"));
    assert!(output.contains("Progress-aware hints:"));
    assert!(output.contains("Debug/WIP work"));
    assert!(output.contains("Clean the branch history"));
    assert!(output.contains("interactive rebase"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn interactive_rebase_basic_report_includes_metadata() {
    let path = temp_path("rebase-report");
    let report = temp_path("rebase-report-md").with_extension("md");
    let path_arg = path.to_string_lossy().to_string();
    let report_arg = report.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    let output = run(&["check", "--path", &path_arg, "--report", &report_arg]);
    assert!(output.status.success());
    assert!(report.exists());
    let report_text = fs::read_to_string(&report).unwrap();
    assert!(report_text.contains("- Exercise: `interactive-rebase-basic`"));
    assert!(report_text.contains("- Title: Clean up a feature branch with interactive rebase"));
    assert!(report_text.contains("- Difficulty: advanced"));
    assert!(report_text.contains("- Category: History rewriting"));

    fs::remove_dir_all(path).unwrap();
    fs::remove_file(report).unwrap();
}

#[test]
fn interactive_rebase_basic_reset_works() {
    let path = temp_path("rebase-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(
        run(&["new", "interactive-rebase-basic", "--path", &path_arg])
            .status
            .success()
    );

    fs::remove_file(path.join("debug.txt")).unwrap();
    git(&path, &["add", "-A"]);
    git(&path, &["commit", "-m", "Remove debug notes"]);

    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "feature/profile-copy"
    );
    assert!(path.join("debug.txt").exists());
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_new_creates_expected_starting_state() {
    let path = temp_path("detached-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "detached-head-recovery", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("recovered-note.txt").exists());
    assert!(git_output(&path, &["branch", "--show-current"]).is_empty());
    assert!(!git_output(&path, &["rev-parse", "--verify", "main"]).is_empty());
    assert_eq!(
        git_output(&path, &["log", "-1", "--format=%s"]),
        "Add detached work note"
    );
    assert!(fs::read_to_string(path.join("recovered-note.txt"))
        .unwrap()
        .contains("Recovered detached HEAD work"));
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("detached-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: FAILED"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "detached-head-recovery");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_branch_preserving_solution_passes() {
    let path = temp_path("detached-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "-c", "recovery/detached-work"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_manual_content_equivalent_solution_warns() {
    let path = temp_path("detached-warning");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "main"]);
    git(&path, &["switch", "-c", "recovery/detached-work"]);
    fs::write(
        path.join("recovered-note.txt"),
        "Recovered detached HEAD work\n",
    )
    .unwrap();
    git(&path, &["add", "recovered-note.txt"]);
    git(&path, &["commit", "-m", "Recreate recovered work"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("Detached commit preservation check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_missing_recovered_work_fails() {
    let path = temp_path("detached-missing-work");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    git(&path, &["switch", "main"]);
    git(&path, &["switch", "-c", "recovery/detached-work"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Recovered work exists on recovery branch"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_still_detached_fails() {
    let path = temp_path("detached-still-detached");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    git(&path, &["branch", "recovery/detached-work"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Current branch is recovery/detached-work"));
    assert!(output.contains("Repository is not in detached HEAD"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn detached_head_recovery_reset_and_hint_work() {
    let path = temp_path("detached-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "detached-head-recovery", "--path", &path_arg])
        .status
        .success());

    let hint = run(&["hint", "--path", &path_arg]);
    assert!(hint.status.success());
    assert!(stdout(&hint).contains("Hints for detached-head-recovery:"));

    git(&path, &["switch", "-c", "recovery/detached-work"]);
    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert!(git_output(&path, &["branch", "--show-current"]).is_empty());
    assert_eq!(
        git_output(&path, &["log", "-1", "--format=%s"]),
        "Add detached work note"
    );
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_new_creates_expected_starting_state() {
    let path = temp_path("cherry-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "cherry-pick-basic", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("app.txt").exists());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "release/current"
    );
    assert!(!git_output(&path, &["rev-parse", "--verify", "support/legacy-fix"]).is_empty());
    assert!(!git_output(&path, &["rev-parse", "--verify", "release/current"]).is_empty());
    assert!(
        git_output(&path, &["log", "support/legacy-fix", "--format=%s"])
            .contains("Fix empty checkout cart")
    );
    assert!(
        git_output(&path, &["show", "support/legacy-fix:legacy.txt"])
            .contains("Legacy support mode enabled")
    );
    assert!(git_output(&path, &["show", "support/legacy-fix:app.txt"])
        .contains("Fix: handle empty checkout cart"));
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("cherry-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: FAILED"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "cherry-pick-basic");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_valid_cherry_pick_solution_passes() {
    let path = temp_path("cherry-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    let hash = git_output(
        &path,
        &[
            "log",
            "support/legacy-fix",
            "--format=%H",
            "--grep",
            "^Fix empty checkout cart$",
            "-n",
            "1",
        ],
    );
    git(&path, &["cherry-pick", &hash]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_manual_final_state_warns() {
    let path = temp_path("cherry-warning");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    fs::write(
        path.join("app.txt"),
        "Checkout: standard cart flow\nFix: handle empty checkout cart\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Apply checkout fix manually"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("Cherry-pick-style workflow check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_rejects_unrelated_legacy_content_on_release() {
    let path = temp_path("cherry-legacy-fail");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    git(&path, &["merge", "support/legacy-fix"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: FAILED"));
    assert!(output.contains("Release branch excludes legacy-only content"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn cherry_pick_basic_reset_and_hint_work() {
    let path = temp_path("cherry-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "cherry-pick-basic", "--path", &path_arg])
        .status
        .success());

    let hint = run(&["hint", "--path", &path_arg]);
    assert!(hint.status.success());
    assert!(stdout(&hint).contains("Hints for cherry-pick-basic:"));

    fs::write(
        path.join("app.txt"),
        "Checkout: standard cart flow\nFix: handle empty checkout cart\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Apply checkout fix manually"]);
    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert_eq!(
        git_output(&path, &["branch", "--show-current"]),
        "release/current"
    );
    assert!(!git_output(&path, &["rev-parse", "--verify", "support/legacy-fix"]).is_empty());
    assert!(git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_new_creates_expected_starting_state() {
    let path = temp_path("stash-new");
    let path_arg = path.to_string_lossy().to_string();
    let output = run(&["new", "stash-switch", "--path", &path_arg]);
    assert!(output.status.success());

    assert!(path.join(".git").exists());
    assert!(path.join(".branchdojo.json").exists());
    assert!(path.join("README.branchdojo.md").exists());
    assert!(path.join("app.txt").exists());
    assert_eq!(git_output(&path, &["branch", "--show-current"]), "main");
    let feature_hash = git_output(&path, &["rev-parse", "--verify", "feature/settings-copy"]);
    assert!(!feature_hash.is_empty());
    assert!(fs::read_to_string(path.join("app.txt"))
        .unwrap()
        .contains("Local note: Keep dark mode feedback for follow-up."));
    assert!(!git_output(&path, &["status", "--porcelain"]).is_empty());

    let checkout = Command::new("git")
        .current_dir(&path)
        .args(["checkout", "feature/settings-copy"])
        .output()
        .unwrap();
    assert!(!checkout.status.success());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("stash-unsolved");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "stash-switch", "--path", &path_arg])
        .status
        .success());

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: FAILED"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let json: Value = serde_json::from_slice(&json_output.stdout).unwrap();
    assert_eq!(json["exercise"], "stash-switch");
    assert_eq!(json["status"], "failed");
    for check in json["checks"].as_array().unwrap() {
        let status = check["status"].as_str().unwrap();
        assert!(matches!(status, "passed" | "warning" | "failed"));
        let severity = check["severity"].as_str().unwrap();
        assert!(matches!(severity, "required" | "warning"));
    }

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_valid_stash_style_solution_passes() {
    let path = temp_path("stash-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "stash-switch", "--path", &path_arg])
        .status
        .success());

    git(&path, &["stash", "push", "-m", "preserve local notes"]);
    git(&path, &["checkout", "feature/settings-copy"]);
    fs::write(
        path.join("app.txt"),
        "Welcome: BranchDojo demo app\nSettings: Save preferences with confidence.\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Update settings copy"]);
    git(&path, &["checkout", "main"]);
    git(&path, &["stash", "pop"]);
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Preserve local follow-up note"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_commit_before_switch_warns() {
    let path = temp_path("stash-warning");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "stash-switch", "--path", &path_arg])
        .status
        .success());

    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "WIP preserve local notes"]);
    git(&path, &["checkout", "feature/settings-copy"]);
    fs::write(
        path.join("app.txt"),
        "Welcome: BranchDojo demo app\nSettings: Save preferences with confidence.\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Update settings copy"]);
    git(&path, &["checkout", "main"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Status: WARNING"));
    assert!(output.contains("WIP commit check"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn stash_switch_reset_and_hint_work() {
    let path = temp_path("stash-reset");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "stash-switch", "--path", &path_arg])
        .status
        .success());

    let hint = run(&["hint", "--path", &path_arg]);
    assert!(hint.status.success());
    assert!(stdout(&hint).contains("Hints for stash-switch:"));

    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "WIP preserve local notes"]);
    let reset = run(&["reset", "--path", &path_arg]);
    assert!(reset.status.success());
    assert_eq!(git_output(&path, &["branch", "--show-current"]), "main");
    assert!(fs::read_to_string(path.join("app.txt"))
        .unwrap()
        .contains("Local note: Keep dark mode feedback for follow-up."));
    assert!(!git_output(&path, &["status", "--porcelain"]).is_empty());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn conflict_basic_valid_merge_passes() {
    let path = temp_path("conflict-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    let merge = Command::new("git")
        .current_dir(&path)
        .args(["merge", "feature/landing-copy"])
        .output()
        .unwrap();
    assert!(!merge.status.success());
    fs::write(
        path.join("app.txt"),
        "Headline: Build better Git habits\nCTA: Start practicing\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(&path, &["commit", "-m", "Resolve landing copy merge"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Status: PASSED"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("\nNext:"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn dirty_working_tree_fails_required_validation() {
    let path = temp_path("dirty");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "revert-mistake", "--path", &path_arg])
        .status
        .success());
    fs::write(path.join("notes.txt"), "uncommitted notes\n").unwrap();

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Status: FAILED"));
    assert!(stdout.contains("❌ Working tree is clean"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn conflict_markers_fail_required_validation() {
    let path = temp_path("markers");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg])
        .status
        .success());

    fs::write(
        path.join("app.txt"),
        "<<<<<<< HEAD\nHeadline: Build better Git habits\n=======\nCTA: Start practicing\n>>>>>>> feature/landing-copy\n",
    )
    .unwrap();
    git(&path, &["add", "app.txt"]);
    git(
        &path,
        &["commit", "-m", "Commit unresolved conflict markers"],
    );

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Status: FAILED"));
    assert!(stdout.contains("❌ Conflict markers removed"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn reset_refuses_non_branchdojo_folder() {
    let path = temp_path("plain");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("file.txt"), "user data").unwrap();
    let path_arg = path.to_string_lossy().to_string();

    let output = run(&["reset", "--path", &path_arg]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("BD008"));
    assert!(path.join("file.txt").exists());

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn revert_mistake_history_preserving_fix_warns_without_revert_message() {
    let path = temp_path("revert");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "revert-mistake", "--path", &path_arg])
        .status
        .success());

    fs::write(
        path.join("config.txt"),
        "mode=safe\nfeature_flags=basic,search\n",
    )
    .unwrap();
    git(&path, &["add", "config.txt"]);
    git(&path, &["commit", "-m", "Restore safe production config"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Status: WARNING"));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn wrong_branch_commit_can_be_solved_by_cherry_pick_and_reset() {
    let path = temp_path("wrong-branch");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "wrong-branch-commit", "--path", &path_arg])
        .status
        .success());

    let hash_output = Command::new("git")
        .current_dir(&path)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(hash_output.status.success());
    let hash = String::from_utf8_lossy(&hash_output.stdout)
        .trim()
        .to_string();
    git(&path, &["checkout", "feature/profile-page"]);
    git(&path, &["cherry-pick", &hash]);
    git(&path, &["checkout", "main"]);
    git(&path, &["reset", "--hard", "HEAD~1"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}
