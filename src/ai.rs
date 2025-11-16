use serde_json::json;
use crate::config::Config;

pub fn generate_message(diff: &str, cfg: &Config) -> String {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        cfg.gemini_api_key
    );

    let prompt = format!(
        "Generate a concise git commit message based only on this diff:\n\n{}",
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

    res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("update: minor changes")
        .to_string()
}
