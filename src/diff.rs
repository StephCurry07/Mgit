use std::process::Command;

pub fn get_diff() -> String {
    let output = Command::new("git")
        .args(["diff"])
        .output()
        .expect("Failed to run git diff");

    String::from_utf8_lossy(&output.stdout).to_string()
}
