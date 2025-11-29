use std::process::Command;

pub fn get_remote_origin() -> Option<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if url.is_empty() { None } else { Some(url) }
}

pub fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Some(branch)
}

pub fn set_ssh_remote(user: &str, repo: &str) {
    let ssh_url = format!("git@github.com:{}/{}.git", user, repo);

    Command::new("git")
        .args(["remote", "set-url", "origin", &ssh_url])
        .status()
        .expect("Failed to set SSH remote");
}

pub fn get_repo_parts(url: &str) -> Option<(String, String)> {
    let mut split = url.split('/');
    let repo = split.next_back()?.replace(".git", "");
    let user = split.next_back()?.to_string();
    Some((user, repo))
}
