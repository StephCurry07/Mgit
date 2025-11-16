use std::process::Command;

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn has_commits() -> bool {
    let result = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output();

    result.map(|o| o.status.success()).unwrap_or(false)
}

pub fn has_changes() -> bool {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to run git status");

    !String::from_utf8_lossy(&output.stdout).trim().is_empty()
}
