use std::process::{Command, Stdio};

fn run(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to run command");

    if !status.success() {
        eprintln!("Command failed: {} {:?}", cmd, args);
        std::process::exit(1);
    }
}

pub fn git_add_all() {
    run("git", &["add", "."]);
}

pub fn git_commit(msg: &str) {
    run("git", &["commit", "-m", msg]);
}

pub fn git_push() {
    run("git", &["push"]);
}
