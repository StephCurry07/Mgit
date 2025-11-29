use serde_json::json;
use crate::config::Config;

pub fn generate_message(diff: &str, cfg: &Config) -> String {
    // If diff is empty, return a default message
    if diff.trim().is_empty() {
        return "chore: update files".to_string();
    }

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        cfg.gemini_api_key
    );

    let prompt = format!(
        "Generate a concise git commit message (one line, max 72 characters) based only on this diff. Return only the commit message, nothing else:\n\n{}",
        diff
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

    let message = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("update: minor changes")
        .to_string()
        .trim()
        .to_string();

    // Validate the response - if it looks like an error message, use fallback
    if is_error_message(&message) {
        return "chore: update files".to_string();
    }

    message
}

fn is_error_message(msg: &str) -> bool {
    let lower = msg.to_lowercase();
    lower.contains("please provide") ||
    lower.contains("i need to see") ||
    lower.contains("error") ||
    lower.contains("cannot") ||
    lower.contains("unable") ||
    lower.contains("diff is required") ||
    lower.contains("no diff") ||
    lower.starts_with("sorry") ||
    (lower.len() > 100 && lower.contains("?")) // Long messages with questions are likely errors
}
