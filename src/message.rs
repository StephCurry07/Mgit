use regex::Regex;

pub fn generate_message(diff: &str) -> String {
    if diff.trim().is_empty() {
        return "chore: minor updates".to_string();
    }

    let added = Regex::new(r"^\+[^+]").unwrap().find_iter(diff).count();
    let removed = Regex::new(r"^\-[^-]").unwrap().find_iter(diff).count();

    format!("update: {} additions, {} deletions", added, removed)
}
