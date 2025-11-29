# Mgit

A lightweight Git automation tool that generates commit messages from diffs using AI.

## Setup

### Windows (PowerShell)

#### 1. Install Rust

If you don't have Rust installed, install it using [rustup](https://rustup.rs/):

```powershell
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

#### 2. Install Visual Studio Build Tools (Required for Windows)

The MSVC toolchain requires Visual Studio Build Tools. Install them using one of these methods:

**Option A: Using winget (Recommended)**
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```
Then manually run the installer and select "Desktop development with C++" workload.

**Option B: Direct Download**
1. Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
2. Run the installer
3. Select "Desktop development with C++" workload
4. Click Install

**Option C: Using Chocolatey**
```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

#### 3. Verify Installation

After installing the build tools, restart your terminal and verify:

```powershell
cargo --version
rustc --version
```

#### 4. Build the Project

```powershell
cargo build
cargo run -- <command>
```

### WSL (Windows Subsystem for Linux)

#### 1. Install Rust

If you don't have Rust installed, install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and select the default installation options. After installation, restart your terminal or run:

```bash
source $HOME/.cargo/env
```

#### 2. Install Build Dependencies

WSL uses the GNU toolchain, so you'll need to install the necessary build tools:

```bash
# For Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential

# For Fedora
sudo dnf install -y gcc

# For Arch Linux
sudo pacman -S base-devel
```

#### 3. Verify Installation

Verify that Rust is installed correctly:

```bash
cargo --version
rustc --version
```

#### 4. Build the Project

```bash
cargo build
cargo run -- <command>
```

## Usage

This tool provides several commands (run via cargo run -- <command> when developing, or via the built binary).

Note: Many commands rely on an AI service (Gemini) and require saving your API key first (see `Setup`).

Common usage examples:

- Save Gemini API key (interactive):

```bash
cargo run -- setup
# prompts: Enter your Gemini API key:
```

- Convert HTTPS remote to SSH and configure GitHub SSH auth:

```bash
cargo run -- fix-remote
```

- Stage, generate an AI commit message, and push. Exclude files using -x:

```bash
cargo run -- push -x "*.log" "node_modules/*"
```

- Generate release notes from commits and optionally update CHANGELOG.md (use -v/--version to set the version):

```bash
cargo run -- release --version 1.2.3
# or
cargo run -- release -v 1.2.3
```

- Generate and open API docs (use --no-open to only generate without opening a browser):

```bash
cargo run -- doc
cargo run -- doc --no-open
```

## Commands (behavior highlights)

- setup
  - Prompts for and saves your Gemini API key. Many AI features (commit message generation, release notes) require this.

- fix-remote
  - Ensures the remote `origin` uses SSH (converts HTTPS remotes when possible), sets up/checks SSH keys, and tests the GitHub connection.
  - Requires being inside a Git repository with a remote named `origin`.

- push
  - Smartly stages files, generates an AI commit message from the git diff, and pushes to the remote.
  - Requires a Git repository with at least one commit and a configured remote `origin`.
  - You can exclude files by patterns using `-x` multiple times or with multiple values, e.g. `-x "*.log" "secret.txt"`.
  - Prompts for confirmation before committing and pushing.
  - If the diff is empty, a default commit message is used.

- release
  - Finds the latest git tag (if any), collects commits since that tag, and generates release notes via the Gemini API.
  - Prompts before writing to `CHANGELOG.md`.
  - If Gemini fails or returns an error-like response, a simple fallback changelog is created from commit messages.

- doc
  - Generates Rust documentation (`cargo doc --no-deps`) and can open it in the browser unless `--no-open` is passed.

## Development

```bash
# Run the project
cargo run -- <command>

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Notes

- Be sure to run `setup` and provide a valid Gemini API key before using `push` or `release` — both features call the Gemini API.
- The tool performs safety checks and asks for confirmation before potentially destructive actions (committing, pushing, updating CHANGELOG.md).
- If you see messages like "Not a Git repository" or "No remote 'origin' found", run the commands from within a Git repository and ensure a remote is configured.
