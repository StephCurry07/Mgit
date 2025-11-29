mod ai;
mod changelog;
mod config;
mod diff;
mod docs;
mod git;
mod git_remote;
mod git_status;
mod ssh_setup;
mod staging;

use clap::{Parser, Subcommand};
use config::Config;
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about = "gitbit - AI Git Assistant")]
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

    /// Generate release notes from commits since last version and update CHANGELOG.md
    Release {
        /// Version number (defaults to version in Cargo.toml)
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Generate and open API documentation
    Doc {
        /// Don't open browser automatically
        #[arg(long)]
        no_open: bool,
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

        /* ────────────────────────────────────────────────
           gitbit release
        ───────────────────────────────────────────────── */
        Commands::Release { version } => {
            if !git_status::is_git_repo() {
                println!("❌ Not a Git repository.");
                return;
            }

            let cfg = Config::load().expect("Run gitbit setup first.");
            let version = version.unwrap_or_else(|| changelog::get_current_version());
            
            println!("📝 Generating release notes for v{}...", version);
            
            let commits = changelog::get_all_commits_since_last_release();
            
            if commits.is_empty() {
                println!("⚠️  No commits found since last release.");
                return;
            }

            println!("📋 Found {} commits since last release", commits.len());
            
            let release_notes = changelog::generate_release_notes(&commits, &version, &cfg);
            
            println!("\n📄 Generated release notes:\n");
            println!("{}", release_notes);
            println!("\n");
            
            // Ask for confirmation before updating CHANGELOG.md
            print!("Update CHANGELOG.md? (y/n): ");
            io::stdout().flush().unwrap();
            
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();
            
            let confirm = confirm.trim().to_lowercase();
            if confirm == "y" || confirm == "yes" {
                match changelog::update_changelog(&release_notes) {
                    Ok(_) => println!("✅ CHANGELOG.md updated!"),
                    Err(e) => println!("❌ Failed to update CHANGELOG.md: {}", e),
                }
            } else {
                println!("❌ Skipped updating CHANGELOG.md");
            }
        }

        /* ────────────────────────────────────────────────
           gitbit doc
        ───────────────────────────────────────────────── */
        Commands::Doc { no_open } => {
            if let Err(e) = docs::ensure_doc_dir() {
                println!("❌ Failed to create doc/ directory: {}", e);
                return;
            }
            
            let doc_files = docs::list_doc_files();
            if !doc_files.is_empty() {
                println!("📄 Documentation files in doc/:");
                for file in &doc_files {
                    println!("   - {}", file);
                }
                println!();
            }
            
            if no_open {
                // Just generate docs without opening
                let status = Command::new("cargo")
                    .args(["doc", "--no-deps"])
                    .status();
                
                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Documentation generated at target/doc/");
                    }
                    Ok(_) => {
                        println!("❌ Failed to generate documentation");
                    }
                    Err(e) => {
                        println!("❌ Error running cargo doc: {}", e);
                    }
                }
            } else {
                // Generate and open in browser
                if let Err(e) = docs::generate_docs(true) {
                    println!("❌ Failed to generate documentation: {}", e);
                }
            }
        }
    }
}
