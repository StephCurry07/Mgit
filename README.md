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
cargo run
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
cargo run
```

## Development

```bash
# Run the project
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

