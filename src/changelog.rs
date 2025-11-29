use std::process::Command;
use crate::config::Config;
use serde_json::json;

pub fn get_commits_since_tag(tag: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%s", &format!("{}..HEAD", tag)])
        .output()
        .ok();

    if let Some(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|s| s.to_string())
                .collect();
        }
    }
    Vec::new()
}

pub fn get_latest_tag() -> Option<String> {
    let output = Command::new("git")
        .args(["tag", "--sort=-version:refname"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines()
        .find(|l| !l.trim().is_empty())
        .map(|s| s.trim().to_string())
}

pub fn get_all_commits_since_last_release() -> Vec<String> {
    // Try to get the latest tag
    if let Some(tag) = get_latest_tag() {
        println!("📌 Found latest tag: {}", tag);
        return get_commits_since_tag(&tag);
    }

    // If no tags, get all commits (first release)
    println!("📌 No tags found, getting all commits");
    let output = Command::new("git")
        .args(["log", "--pretty=format:%s"])
        .output()
        .ok();

    if let Some(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|s| s.to_string())
                .collect();
        }
    }
    Vec::new()
}

pub fn generate_release_notes(commits: &[String], version: &str, cfg: &Config) -> String {
    if commits.is_empty() {
        return format!("# Changelog - v{}\n\nNo changes since last release.", version);
    }

    let commits_text = commits.join("\n");
    
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        cfg.gemini_api_key
    );

    let prompt = format!(
        "Generate release notes in Markdown format for version {} based on these git commit messages.\n\n\
        Format:\n\
        - Use a clear heading: # Changelog - v{}\n\
        - Group changes by type (Features, Bug Fixes, Improvements, etc.)\n\
        - Use bullet points\n\
        - Be concise but descriptive\n\
        - Skip merge commits and trivial updates\n\n\
        Commit messages:\n{}\n\n\
        Return only the formatted Markdown release notes, nothing else:",
        version, version, commits_text
    );

    let body = json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }]
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(&url)
        .json(&body)
        .send()
        .expect("Failed to contact Gemini")
        .json::<serde_json::Value>()
        .expect("Invalid response from Gemini");

    let notes = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string()
        .trim()
        .to_string();

    if notes.is_empty() || is_error_message(&notes) {
        // Fallback: simple formatted list
        format!(
            "# Changelog - v{}\n\n## Changes\n\n{}",
            version,
            commits.iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        )
    } else {
        notes
    }
}

fn is_error_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("please provide")
        || lower.contains("i need to see")
        || lower.contains("error")
        || lower.contains("cannot")
        || lower.contains("unable")
        || lower.contains("diff is required")
        || lower.contains("no diff")
        || lower.starts_with("sorry")
}

pub fn get_current_version() -> String {
    use std::fs;
    
    // Read version from Cargo.toml
    if let Ok(content) = fs::read_to_string("Cargo.toml") {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("version") && line.contains('=') {
                if let Some(version_part) = line.split('=').nth(1) {
                    let version = version_part.trim().trim_matches('"').trim_matches('\'');
                    if !version.is_empty() {
                        return version.to_string();
                    }
                }
            }
        }
    }
    "0.1.0".to_string()
}

pub fn update_changelog(release_notes: &str) -> std::io::Result<()> {
    use std::fs;
    
    let changelog_path = "CHANGELOG.md";
    
    // Read existing changelog if it exists
    let existing = fs::read_to_string(changelog_path).ok();
    
    let new_content = if let Some(existing) = existing {
        // Prepend new release notes to existing changelog
        format!("{}\n\n---\n\n{}", release_notes, existing)
    } else {
        // Create new changelog
        release_notes.to_string()
    };
    
    fs::write(changelog_path, new_content)?;
    Ok(())
}

