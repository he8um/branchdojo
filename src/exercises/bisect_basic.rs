use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const APP_FILE: &str = "app.txt";
pub const CHECK_FILE: &str = "check.txt";
pub const DEBUG_README_FILE: &str = "README.debug.md";
pub const DIAGNOSIS_FILE: &str = "diagnosis.md";
pub const EXPECTED_MARKER: &str = "expected_discount_total=correct";
pub const BAD_MARKER: &str = "discount_total=incorrect";
pub const CULPRIT_COMMIT_MESSAGE: &str = "Introduce discount regression";
pub const INITIAL_COMMIT_MESSAGE: &str = "Initialize sample app";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::BISECT_BASIC,
    goal: "Practice identifying the commit that introduced a deterministic regression.",
    hints: &[
        "Inspect history with `git log --oneline --decorate`.",
        "Compare file contents across commits with `git show <commit>:app.txt`.",
        "Use `git bisect` or manual history inspection to narrow the culprit.",
        "Record the culprit in `diagnosis.md`.",
        "Clean up any active bisect state before final check.",
        "Keep the working tree clean and run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_app(
        path,
        "\
app=sample
login=basic
checkout=none
discount_total=correct
copy=plain
label=standard
",
    )?;
    write_check(path)?;
    fs::write(
        path.join(DEBUG_README_FILE),
        "\
# Debug Notes

The expected discount marker is in check.txt.
Find the commit where app.txt first changes away from it.
",
    )
    .map_err(|error| AppError::io("Could not write README.debug.md.", error))?;
    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.metadata.name,
            metadata::expected_final_branch_for(EXERCISE.metadata.name)
                .expect("exercise metadata should define expected final branch"),
            vec![
                APP_FILE.to_string(),
                CHECK_FILE.to_string(),
                DIAGNOSIS_FILE.to_string(),
            ],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main",
            "app.txt, check.txt, README.debug.md, diagnosis.md",
            "main",
            "A discount calculation regression was introduced in the history.",
            "Inspect the history to find the regression-introducing commit, create `diagnosis.md` with the culprit commit subject or hash, do not fix the regression, end on `main`, and leave the working tree clean.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, INITIAL_COMMIT_MESSAGE)?;

    write_app(
        path,
        "\
app=sample
login=stable
checkout=none
discount_total=correct
copy=plain
label=standard
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Add stable login flow")?;

    write_app(
        path,
        "\
app=sample
login=stable
checkout=calculated
discount_total=correct
copy=plain
label=standard
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Add checkout calculation")?;

    write_app(
        path,
        "\
app=sample
login=stable
checkout=calculated
discount_total=incorrect
copy=plain
label=standard
",
    )?;
    git::add_all(path)?;
    git::commit(path, CULPRIT_COMMIT_MESSAGE)?;

    write_app(
        path,
        "\
app=sample
login=stable
checkout=calculated
discount_total=incorrect
copy=clear
label=standard
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Update copy text")?;

    write_app(
        path,
        "\
app=sample
login=stable
checkout=calculated
discount_total=incorrect
copy=clear
label=checkout-summary
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Refactor display labels")?;
    Ok(())
}

fn write_app(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}

fn write_check(path: &Path) -> AppResult<()> {
    fs::write(
        path.join(CHECK_FILE),
        format!("{EXPECTED_MARKER}\nregression_file=app.txt\n"),
    )
    .map_err(|error| AppError::io("Could not write check.txt.", error))
}
