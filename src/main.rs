mod config;
mod diff;
mod ai;
mod git;
mod git_status;
mod git_remote;
mod ssh_setup;
mod staging;

use clap::{Parser, Subcommand};
use std::io::{self, Write};
use config::Config;

#[derive(Parser)]
#[command(author, version, about="gitbit - AI Git Assistant")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save your Gemini API key
    Setup,

    /// Automatically fix "origin" to SSH and configure GitHub auth
    FixRemote,

    /// Stage files smartly, generate AI commit, push to remote
    Push {
        /// Exclude files by pattern: gitbit push -x file1 file2 "*.log"
        #[arg(short='x', long="exclude", num_args = 1.., value_delimiter = ' ')]
        exclude: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {

        /* ────────────────────────────────────────────────
           gitbit setup
        ───────────────────────────────────────────────── */
        Commands::Setup => {
            print!("Enter your Gemini API key: ");
            io::stdout().flush().unwrap();

            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();

            Config::save(key.trim());
            println!("🔑 API key saved!");
        }

        /* ────────────────────────────────────────────────
           gitbit fix-remote
        ───────────────────────────────────────────────── */
        Commands::FixRemote => {
            use git_remote::*;
            use ssh_setup::*;

            if !git_status::is_git_repo() {
                println!("❌ Not a Git repository.");
                return;
            }

            let origin = match get_remote_origin() {
                Some(o) => o,
                None => {
                    println!("❌ No remote 'origin' found.");
                    return;
                }
            };

            println!("📦 Current remote: {}", origin);

            if origin.starts_with("https://") {
                println!("🔄 Switching remote HTTPS → SSH...");

                if let Some((user, repo)) = get_repo_parts(&origin) {
                    set_ssh_remote(&user, &repo);
                    println!("✔ SSH remote set: git@github.com:{}/{}.git", user, repo);
                } else {
                    println!("❌ Could not parse remote URL.");
                    return;
                }
            } else {
                println!("✔ Remote already using SSH.");
            }

            ensure_ssh_key();
            show_public_key();
            test_github_connection();

            println!("🎉 fix-remote completed!");
        }

        /* ────────────────────────────────────────────────
           gitbit push (with -x exclusions)
        ───────────────────────────────────────────────── */
        Commands::Push { exclude } => {

            // 1. Must be a Git repo
            if !git_status::is_git_repo() {
                println!("❌ Not a Git repository.");
                return;
            }

            // 2. Must have at least one commit
            if !git_status::has_commits() {
                println!("❌ No commits yet. Create the first commit manually.");
                return;
            }

            // 3. Show remote + branch
            if let Some(remote) = git_remote::get_remote_origin() {
                println!("📦 Remote: {}", remote);
            } else {
                println!("❌ No remote 'origin' found.");
                println!("Run: git remote add origin <url>");
                return;
            }

            if let Some(branch) = git_remote::get_current_branch() {
                println!("🌿 Branch: {}", branch);
            }

            // 4. Get file changes
            let changes = staging::get_changes();

            if changes.is_empty() {
                println!("✔ No changes to commit.");
                return;
            }

            // 5. Smart filtering
            let (to_stage, ignored_default, ignored_user) =
                staging::filter_changes(&changes, &exclude);

            println!("\n📄 Detected changes:");
            for c in &changes {
                println!(" {} {}", c.status, c.path);
            }

            println!("\n🛑 Excluded by DEFAULT rules:");
            for f in ignored_default {
                println!(" - {}", f);
            }

            println!("\n🛑 Excluded by -x patterns:");
            for f in ignored_user {
                println!(" - {}", f);
            }

            println!("\n📦 Final files to stage:");
            for f in &to_stage {
                println!(" - {}", f);
            }

            if to_stage.is_empty() {
                println!("❌ Nothing to stage after exclusions.");
                return;
            }

            // 6. Stage only selected files
            git::git_add_specific(&to_stage);

            // 🟡 ASK FOR CONFIRMATION BEFORE COMMIT & PUSH
            use std::io::{self, Write};
            print!("\nProceed with commit and push? (y/n): ");
            io::stdout().flush().unwrap();

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();

            let confirm = confirm.trim().to_lowercase();
            if confirm != "y" && confirm != "yes" {
                println!("❌ Aborted by user.");
                return;
            }

            // 7. AI commit message
            let cfg = Config::load().expect("Run gitbit setup first.");
            let diff = diff::get_diff();
            
            if diff.trim().is_empty() {
                println!("⚠️  No staged changes detected. Using default commit message.");
            }
            
            let msg = ai::generate_message(&diff, &cfg);

            println!("\n🧠 Commit message:\n{}\n", msg);

            git::git_commit(&msg);
            git::git_push();

            println!("🚀 gitbit push complete!");
        }
    }
}
