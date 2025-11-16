mod config;
mod diff;
mod ai;
mod git;
mod git_status;
mod git_remote;

use std::io::{self, Write};
use config::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: mgit push | setup");
        return;
    }

    match args[1].as_str() {
        "init" => {
            if git_status::is_git_repo() {
                println!("✔ This directory is already a Git repository.");
            } else {
                println!("Initializing Git repository...");
                git::run("git", &["init"]);
                println!("✔ Repository initialized.");
            }
        }

        "setup" => {
            print!("Enter your Gemini API key: ");
            io::stdout().flush().unwrap();

            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();

            Config::save(key.trim());
            println!("API key saved!");
        }

        "push" => {
            // 1. Check if this is a git repo
            if !git_status::is_git_repo() {
                println!("❌ Not a Git repository.");
                println!("Run `git init` or `mgit init` first.");
                return;
            }

            // 2. Check if repo has commits
            if !git_status::has_commits() {
                println!("❌ Repository has no commits yet.");
                println!("Run `mgit commit` or make the first commit manually.");
                return;
            }

            // check remote origin
            if let Some(remote) = git_remote::get_remote_origin() {
                println!("📦 Remote: {}", remote);
            } else {
                println!("❌ No remote 'origin' found.");
                println!("Add one with:");
                println!("  git remote add origin <url>");
                return;
            }
            
            if let Some(branch) = git_remote::get_current_branch() {
                println!("🌿 Branch: {}", branch);
            }            
            // 3. Check if repo has unstaged/unstaged changes
            if !git_status::has_changes() {
                println!("✔ No changes to commit.");
                return;
            }

            let cfg = Config::load().expect("Run `mgit setup` first");

            let diff = diff::get_diff();
            let msg = ai::generate_message(&diff, &cfg);
            println!("Generated commit message:\n{}\n", msg);

            git::git_add_all();
            git::git_commit(&msg);
            git::git_push();

            println!("🚀 mgit push complete!");
        }

        _ => println!("Unknown command"),
    }
}
