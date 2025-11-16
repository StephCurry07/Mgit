use std::process::Command;
use std::fs;

pub fn ensure_ssh_key() {
    let ssh_key = format!("{}/.ssh/id_ed25519", std::env::var("HOME").unwrap());
    if !std::path::Path::new(&ssh_key).exists() {
        println!("🔑 No SSH key found. Creating one...");
        Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-C", "mgit-auto"])
            .status()
            .expect("Failed to generate SSH key");
    }
}

pub fn show_public_key() {
    let pubkey_path = format!("{}/.ssh/id_ed25519.pub", std::env::var("HOME").unwrap());
    let pubkey = fs::read_to_string(pubkey_path)
        .expect("Could not read public key");
    println!("📋 Add this SSH key to GitHub:\n\n{}", pubkey);
}

pub fn test_github_connection() {
    println!("🔗 Testing GitHub SSH connection...");
    let status = Command::new("ssh")
        .args(["-T", "git@github.com"])
        .status()
        .expect("SSH test failed");

    if status.success() {
        println!("✅ SSH authentication OK!");
    } else {
        println!("⚠️ GitHub SSH authentication failed.");
    }
}
