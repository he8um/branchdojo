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
fn list_prints_mvp_exercises() {
    let output = run(&["list"]);
    assert!(output.status.success());
    let output = stdout(&output);
    assert!(output.contains("Available exercises:"));
    assert!(output.contains("conflict-basic"));
    assert!(output.contains("revert-mistake"));
    assert!(output.contains("wrong-branch-commit"));
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
fn generated_exercises_include_metadata_readme_and_local_identity() {
    for exercise in ["conflict-basic", "revert-mistake", "wrong-branch-commit"] {
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
        assert_eq!(state["expected_branch"], "main");
        assert!(state["expected_files"].as_array().unwrap().len() == 1);
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
