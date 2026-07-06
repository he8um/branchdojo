use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[test]
fn list_prints_mvp_exercises() {
    let output = run(&["list"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("conflict-basic"));
    assert!(stdout.contains("revert-mistake"));
    assert!(stdout.contains("wrong-branch-commit"));
}

#[test]
fn conflict_basic_fails_before_solving_and_json_is_valid_shape() {
    let path = temp_path("conflict");
    let path_arg = path.to_string_lossy().to_string();
    let new_output = run(&["new", "conflict-basic", "--path", &path_arg]);
    assert!(new_output.status.success());

    let check_output = run(&["check", "--path", &path_arg]);
    assert!(check_output.status.success());
    assert!(String::from_utf8_lossy(&check_output.stdout).contains("Status: FAILED"));

    let json_output = run(&["check", "--path", &path_arg, "--json"]);
    assert!(json_output.status.success());
    let stdout = String::from_utf8_lossy(&json_output.stdout);
    assert!(stdout.contains("\"exercise\": \"conflict-basic\""));
    assert!(stdout.contains("\"status\": \"failed\""));

    fs::remove_dir_all(path).unwrap();
}

#[test]
fn conflict_basic_valid_merge_passes() {
    let path = temp_path("conflict-pass");
    let path_arg = path.to_string_lossy().to_string();
    assert!(run(&["new", "conflict-basic", "--path", &path_arg]).status.success());

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
    assert!(run(&["new", "revert-mistake", "--path", &path_arg]).status.success());

    fs::write(path.join("config.txt"), "mode=safe\nfeature_flags=basic,search\n").unwrap();
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
    assert!(run(&["new", "wrong-branch-commit", "--path", &path_arg]).status.success());

    let hash_output = Command::new("git")
        .current_dir(&path)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(hash_output.status.success());
    let hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();
    git(&path, &["checkout", "feature/profile-page"]);
    git(&path, &["cherry-pick", &hash]);
    git(&path, &["checkout", "main"]);
    git(&path, &["reset", "--hard", "HEAD~1"]);

    let output = run(&["check", "--path", &path_arg]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Status: PASSED"));

    fs::remove_dir_all(path).unwrap();
}
