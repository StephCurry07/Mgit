use std::process::{Command, Stdio};

pub fn run(cmd: &str, args: &[&str]) {
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

pub fn git_add_specific(files: &[String]) {
    let mut cmd = Command::new("git");
    cmd.arg("add");
    for f in files {
        cmd.arg(f);
    }
    cmd.status().expect("Failed to stage files");
}

pub fn git_commit(msg: &str) {
    run("git", &["commit", "-m", msg]);
}

pub fn git_push() {
    run("git", &["push"]);
}
