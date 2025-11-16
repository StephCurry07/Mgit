use glob::Pattern;
use std::process::Command;

#[derive(Debug)]
pub struct Change {
    pub status: String,
    pub path: String,
}

pub fn default_ignores() -> Vec<&'static str> {
    vec![
        "target/*",
        "node_modules/*",
        "*.log",
        "*.tmp",
        ".vscode/*",
        ".idea/*",
        "*.env"
    ]
}

pub fn get_changes() -> Vec<Change> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to run git status");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut changes = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() { continue; }

        let status = line[0..2].trim().to_string();
        let path = line[3..].trim().to_string();

        changes.push(Change { status, path });
    }

    changes
}

pub fn filter_changes(
    changes: &[Change], 
    user_excludes: &[String]
) -> (Vec<String>, Vec<String>, Vec<String>) {

    let mut ignored_default = Vec::new();
    let mut ignored_user = Vec::new();
    let mut final_files = Vec::new();

    for ch in changes {
        let path = &ch.path;

        if default_ignores().iter().any(|p| Pattern::new(p).unwrap().matches(path)) {
            ignored_default.push(path.clone());
            continue;
        }

        if user_excludes.iter().any(|p| Pattern::new(p).unwrap().matches(path)) {
            ignored_user.push(path.clone());
            continue;
        }

        final_files.push(path.clone());
    }

    (final_files, ignored_default, ignored_user)
}
