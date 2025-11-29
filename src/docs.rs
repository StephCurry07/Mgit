use std::process::Command;
use std::path::Path;

pub fn generate_docs(open_browser: bool) -> std::io::Result<()> {
    println!("📚 Generating documentation...");
    
    // Run cargo doc
    let mut cmd = Command::new("cargo");
    cmd.args(["doc", "--no-deps"]);
    
    if open_browser {
        cmd.arg("--open");
    }
    
    let status = cmd.status()?;
    
    if !status.success() {
        eprintln!("❌ Failed to generate documentation");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "cargo doc failed"
        ));
    }
    
    if open_browser {
        println!("✅ Documentation generated and opened in browser");
    } else {
        println!("✅ Documentation generated at target/doc/");
        println!("   Open with: cargo doc --open");
    }
    
    Ok(())
}

pub fn ensure_doc_dir() -> std::io::Result<()> {
    if !Path::new("doc").exists() {
        std::fs::create_dir("doc")?;
        println!("📁 Created doc/ directory");
    }
    Ok(())
}

pub fn list_doc_files() -> Vec<String> {
    let doc_dir = Path::new("doc");
    if !doc_dir.exists() {
        return Vec::new();
    }
    
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(doc_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".md") {
                    files.push(name.to_string());
                }
            }
        }
    }
    files.sort();
    files
}

