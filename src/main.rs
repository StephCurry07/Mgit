mod config;
mod diff;
mod ai;
mod git;

use std::io::{self, Write};
use config::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: mgit push | setup");
        return;
    }

    match args[1].as_str() {
        "setup" => {
            print!("Enter your Gemini API key: ");
            io::stdout().flush().unwrap();

            let mut key = String::new();
            io::stdin().read_line(&mut key).unwrap();

            Config::save(key.trim());
            println!("API key saved!");
        }

        "push" => {
            let cfg = Config::load().expect("Run `mgit setup` first");

            let diff = diff::get_diff();
            if diff.trim().is_empty() {
                println!("No changes to commit.");
                return;
            }

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
