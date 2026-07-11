#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        match self {
            Difficulty::Beginner => "beginner",
            Difficulty::Intermediate => "intermediate",
            Difficulty::Advanced => "advanced",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ExerciseMetadata {
    pub name: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub difficulty: Difficulty,
    pub category: &'static str,
    pub estimated_time: &'static str,
    pub skills: &'static [&'static str],
    pub starting_branch: &'static str,
    pub expected_final_branch: &'static str,
    pub introduced_in: &'static str,
}

pub const CONFLICT_BASIC: ExerciseMetadata = ExerciseMetadata {
    name: "conflict-basic",
    title: "Resolve a basic merge conflict",
    summary: "Merge a feature branch and resolve a simple content conflict.",
    difficulty: Difficulty::Beginner,
    category: "Merge conflicts",
    estimated_time: "10-15 min",
    skills: &["merge", "conflict resolution", "working tree cleanup"],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.1.0",
};

pub const REVERT_MISTAKE: ExerciseMetadata = ExerciseMetadata {
    name: "revert-mistake",
    title: "Revert a bad commit",
    summary: "Undo an unsafe change while preserving project history.",
    difficulty: Difficulty::Beginner,
    category: "History repair",
    estimated_time: "10-15 min",
    skills: &["revert", "history preservation", "safe rollback"],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.1.0",
};

pub const WRONG_BRANCH_COMMIT: ExerciseMetadata = ExerciseMetadata {
    name: "wrong-branch-commit",
    title: "Move work from the wrong branch",
    summary: "Move accidental work from main to the intended feature branch.",
    difficulty: Difficulty::Beginner,
    category: "Branch recovery",
    estimated_time: "10-15 min",
    skills: &["branching", "commit movement", "branch cleanup"],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.1.0",
};

pub const STASH_SWITCH: ExerciseMetadata = ExerciseMetadata {
    name: "stash-switch",
    title: "Preserve work before switching branches",
    summary: "Protect local changes while switching to the correct branch.",
    difficulty: Difficulty::Intermediate,
    category: "Local changes",
    estimated_time: "10-15 min",
    skills: &["stash", "branch switching", "working tree cleanup"],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.2.0",
};

pub const CHERRY_PICK_BASIC: ExerciseMetadata = ExerciseMetadata {
    name: "cherry-pick-basic",
    title: "Cherry-pick a targeted fix",
    summary: "Apply one useful commit without merging unrelated branch history.",
    difficulty: Difficulty::Intermediate,
    category: "Selective history",
    estimated_time: "15-20 min",
    skills: &["cherry-pick", "commit selection", "branch isolation"],
    starting_branch: "release/current",
    expected_final_branch: "release/current",
    introduced_in: "0.2.0",
};

pub const DETACHED_HEAD_RECOVERY: ExerciseMetadata = ExerciseMetadata {
    name: "detached-head-recovery",
    title: "Recover detached HEAD work",
    summary: "Save useful work committed from detached HEAD onto a proper branch.",
    difficulty: Difficulty::Intermediate,
    category: "Recovery",
    estimated_time: "15-20 min",
    skills: &["detached HEAD", "branch creation", "commit recovery"],
    starting_branch: "detached HEAD",
    expected_final_branch: "recovery/detached-work",
    introduced_in: "0.2.0",
};

pub const INTERACTIVE_REBASE_BASIC: ExerciseMetadata = ExerciseMetadata {
    name: "interactive-rebase-basic",
    title: "Clean up a feature branch with interactive rebase",
    summary: "Remove noisy WIP history while preserving useful feature work.",
    difficulty: Difficulty::Advanced,
    category: "History rewriting",
    estimated_time: "20-30 min",
    skills: &[
        "interactive rebase",
        "history rewriting",
        "commit cleanup",
        "branch review",
    ],
    starting_branch: "feature/profile-copy",
    expected_final_branch: "feature/profile-copy",
    introduced_in: "0.4.0",
};

pub const TAG_RELEASE_FIX: ExerciseMetadata = ExerciseMetadata {
    name: "tag-release-fix",
    title: "Fix a release tag after a blocker",
    summary: "Move a local release tag so it points to the fixed release commit.",
    difficulty: Difficulty::Advanced,
    category: "Release recovery",
    estimated_time: "20-30 min",
    skills: &[
        "tagging",
        "release recovery",
        "ref inspection",
        "history verification",
    ],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.4.0",
};

pub const MERGE_VS_REBASE: ExerciseMetadata = ExerciseMetadata {
    name: "merge-vs-rebase",
    title: "Integrate a feature branch with a clean history",
    summary:
        "Update and integrate a feature branch while preserving both mainline and feature work.",
    difficulty: Difficulty::Advanced,
    category: "Branch integration",
    estimated_time: "20-30 min",
    skills: &[
        "rebase",
        "merge",
        "branch integration",
        "history inspection",
    ],
    starting_branch: "feature/pricing-copy",
    expected_final_branch: "main",
    introduced_in: "0.4.0",
};

pub const BISECT_BASIC: ExerciseMetadata = ExerciseMetadata {
    name: "bisect-basic",
    title: "Identify a regression with bisect-style debugging",
    summary: "Find the commit that introduced a deterministic regression.",
    difficulty: Difficulty::Advanced,
    category: "Debugging",
    estimated_time: "20-30 min",
    skills: &[
        "bisect",
        "debugging",
        "history inspection",
        "regression analysis",
    ],
    starting_branch: "main",
    expected_final_branch: "main",
    introduced_in: "0.5.0",
};

const ALL_METADATA: &[&ExerciseMetadata] = &[
    &CONFLICT_BASIC,
    &REVERT_MISTAKE,
    &WRONG_BRANCH_COMMIT,
    &STASH_SWITCH,
    &CHERRY_PICK_BASIC,
    &DETACHED_HEAD_RECOVERY,
    &INTERACTIVE_REBASE_BASIC,
    &TAG_RELEASE_FIX,
    &MERGE_VS_REBASE,
    &BISECT_BASIC,
];

pub fn all_metadata() -> &'static [&'static ExerciseMetadata] {
    ALL_METADATA
}

pub fn find_metadata(name: &str) -> Option<&'static ExerciseMetadata> {
    all_metadata()
        .iter()
        .copied()
        .find(|metadata| metadata.name == name)
}

#[cfg(test)]
pub fn exercise_names() -> Vec<&'static str> {
    all_metadata()
        .iter()
        .map(|metadata| metadata.name)
        .collect()
}

pub fn expected_final_branch_for(name: &str) -> Option<&'static str> {
    find_metadata(name).map(|metadata| metadata.expected_final_branch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_display_values_are_stable() {
        assert_eq!(Difficulty::Beginner.as_str(), "beginner");
        assert_eq!(Difficulty::Intermediate.as_str(), "intermediate");
        assert_eq!(Difficulty::Advanced.as_str(), "advanced");
    }

    #[test]
    fn metadata_includes_all_available_exercises() {
        assert_eq!(
            exercise_names(),
            vec![
                "conflict-basic",
                "revert-mistake",
                "wrong-branch-commit",
                "stash-switch",
                "cherry-pick-basic",
                "detached-head-recovery",
                "interactive-rebase-basic",
                "tag-release-fix",
                "merge-vs-rebase",
                "bisect-basic",
            ]
        );
    }

    #[test]
    fn expected_final_branches_are_available() {
        assert_eq!(
            expected_final_branch_for("cherry-pick-basic"),
            Some("release/current")
        );
        assert_eq!(
            expected_final_branch_for("detached-head-recovery"),
            Some("recovery/detached-work")
        );
        assert_eq!(
            expected_final_branch_for("interactive-rebase-basic"),
            Some("feature/profile-copy")
        );
        assert_eq!(expected_final_branch_for("tag-release-fix"), Some("main"));
        assert_eq!(expected_final_branch_for("merge-vs-rebase"), Some("main"));
        assert_eq!(expected_final_branch_for("bisect-basic"), Some("main"));
        assert_eq!(expected_final_branch_for("not-real"), None);
    }
}
