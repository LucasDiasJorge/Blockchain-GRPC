# Build Troubleshooting Guide

## Common Build Errors and Solutions

### Error: "Could not find `protoc`"

**Symptoms:**
```
Error: Custom { kind: NotFound, error: "Could not find `protoc`. If `protoc` is installed, try setting the `PROTOC` environment variable to the path of the `protoc` binary." }
```

**Cause:**
The Protocol Buffers compiler (`protoc`) is required for gRPC code generation during the build process. This is used by `build.rs` via `tonic-build` to generate Rust bindings from the `.proto` files.

**Solutions:**

#### Windows (PowerShell)

**Option 1: Manual Installation (Recommended)**
```powershell
# Create directory for protoc
New-Item -ItemType Directory -Path "C:\protoc" -Force

# Download latest protoc for Windows
Invoke-WebRequest -Uri "https://github.com/protocolbuffers/protobuf/releases/download/v28.3/protoc-28.3-win64.zip" -OutFile "C:\protoc\protoc.zip"

# Extract the archive
Expand-Archive -Path "C:\protoc\protoc.zip" -DestinationPath "C:\protoc" -Force

# Set environment variables (current session)
$env:PROTOC = 'C:\protoc\bin\protoc.exe'
$env:PATH = $env:PATH + ';C:\protoc\bin'

# Set environment variables persistently
[Environment]::SetEnvironmentVariable('PROTOC', 'C:\protoc\bin\protoc.exe', 'User')
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';C:\protoc\bin', 'User')
```

**Option 2: Using winget**
```powershell
winget install ProtocolBuffers.Protoc
```

**Option 3: Using Chocolatey (requires admin)**
```powershell
choco install protoc --confirm
```

#### Linux/macOS

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y protobuf-compiler
```

**Fedora:**
```bash
sudo dnf install protobuf-compiler
```

**Arch Linux:**
```bash
sudo pacman -S protobuf
```

**macOS (Homebrew):**
```bash
brew install protobuf
```

**Verification:**
```bash
protoc --version
# Expected output: libprotoc 28.3 (or similar version)
```

### Error: "Unable to find libclang"

**Symptoms:**
```
thread 'main' panicked at C:\Users\Lucas\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bindgen-0.72.1\lib.rs:616:27:
Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found"
```

**Cause:**
The `bindgen` crate (used by dependencies like `zstd-sys`) requires LLVM/Clang to generate FFI bindings. The `libclang.dll` library is part of the LLVM installation.

**Solutions:**

#### Windows (PowerShell)

**Option 1: Using winget (Recommended)**
```powershell
winget install LLVM.LLVM
```

**Option 2: Manual Download**
1. Download LLVM from: https://github.com/llvm/llvm-project/releases
2. Install to default location (usually `C:\Program Files\LLVM`)

**Set Environment Variable:**
```powershell
# Current session
$env:LIBCLANG_PATH = 'C:\Program Files\LLVM\bin'

# Persistent
[Environment]::SetEnvironmentVariable('LIBCLANG_PATH', 'C:\Program Files\LLVM\bin', 'User')
```

#### Linux

**Ubuntu/Debian:**
```bash
sudo apt install -y clang libclang-dev
```

**Fedora:**
```bash
sudo dnf install clang clang-devel
```

**Arch Linux:**
```bash
sudo pacman -S clang
```

#### macOS

Clang is usually pre-installed. If needed:
```bash
xcode-select --install
```

**Verification:**
```powershell
# Windows
Test-Path "C:\Program Files\LLVM\bin\libclang.dll"

# Linux/macOS
ls /usr/lib/libclang.so*  # or appropriate path
```

### Error: "the name `server` is defined multiple times"

**Symptoms:**
```
error[E0428]: the name `server` is defined multiple times
  --> src\infrastructure\grpc\mod.rs:11:1
   |
9  | pub mod server;
   | --------------- previous definition of the module `server` here
10 |
11 | pub mod server;
   | ^^^^^^^^^^^^^^^ `server` redefined here
```

**Cause:**
Duplicate module declaration in `src/infrastructure/grpc/mod.rs`.

**Solution:**
Remove the duplicate line:
```rust
// Before (incorrect)
pub mod server;
pub mod server;

// After (correct)
pub mod server;
```

### Error: Conflicting Axum Versions in http_proxy

**Symptoms:**
```
error[E0308]: mismatched types
... (various type mismatch errors related to Axum)
```

**Cause:**
The `http_proxy` binary uses Axum 0.7, but `tonic` brings in Axum 0.6 as a transitive dependency, causing version conflicts.

**Solutions:**

1. **Pin Axum version in Cargo.toml:**
```toml
[dependencies]
axum = "0.7"
# Ensure all axum-related dependencies use the same version
axum-core = "0.4"
```

2. **Use feature flags to avoid conflicts:**
```toml
[dependencies]
tonic = { version = "0.11", default-features = false, features = ["transport", "codegen", "prost"] }
```

3. **Build only the main server:**
```bash
cargo build --bin blockchain-grpc  # Skip http_proxy
```

### Visual Studio Build Tools (Windows)

**Symptoms:**
Native build failures with MSVC-related errors.

**Solution:**
Install "Visual Studio Build Tools" with the "Desktop development with C++" workload:
1. Download from: https://visualstudio.microsoft.com/downloads/
2. Choose "Build Tools for Visual Studio"
3. Select "Desktop development with C++" workload
4. Install

### General Build Verification

After fixing the above issues, verify the build:

```bash
# Clean and rebuild
cargo clean
cargo build

# Build specific components
cargo build --lib                    # Library only
cargo build --bin blockchain-grpc    # Main gRPC server
cargo build --bin http_proxy         # HTTP proxy (if Axum conflicts resolved)

# Run tests
cargo test
```

### Environment Variables Summary

**Windows (set persistently):**
```powershell
[Environment]::SetEnvironmentVariable('PROTOC', 'C:\protoc\bin\protoc.exe', 'User')
[Environment]::SetEnvironmentVariable('LIBCLANG_PATH', 'C:\Program Files\LLVM\bin', 'User')
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';C:\protoc\bin', 'User')
```

**Linux/macOS:**
```bash
export PROTOC=/usr/bin/protoc  # or appropriate path
export LIBCLANG_PATH=/usr/lib  # or appropriate path
```

### Quick Setup Script (Windows)

Create `setup_build.ps1`:
```powershell
# Install protoc
New-Item -ItemType Directory -Path "C:\protoc" -Force
Invoke-WebRequest -Uri "https://github.com/protocolbuffers/protobuf/releases/download/v28.3/protoc-28.3-win64.zip" -OutFile "C:\protoc\protoc.zip"
Expand-Archive -Path "C:\protoc\protoc.zip" -DestinationPath "C:\protoc" -Force

# Install LLVM
winget install LLVM.LLVM

# Set environment variables
[Environment]::SetEnvironmentVariable('PROTOC', 'C:\protoc\bin\protoc.exe', 'User')
[Environment]::SetEnvironmentVariable('LIBCLANG_PATH', 'C:\Program Files\LLVM\bin', 'User')
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';C:\protoc\bin', 'User')

# Verify
protoc --version
Test-Path "C:\Program Files\LLVM\bin\libclang.dll"
```

### Troubleshooting Tips

1. **Restart your terminal/IDE** after setting environment variables
2. **Check paths** - ensure protoc.exe and libclang.dll exist at the specified locations
3. **Use absolute paths** in environment variables
4. **Verify permissions** - ensure you can execute the binaries
5. **Check for antivirus interference** - some security software blocks downloads or execution
6. **Use `cargo clean`** before rebuilding after fixes

### Related Documentation

- [QUICKSTART.md](QUICKSTART.md) - General setup instructions
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture
- [USAGE.md](USAGE.md) - Usage examples