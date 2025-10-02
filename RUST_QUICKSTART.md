# Quick Start Guide - Rust FFI Bindings

Get started with go-judge Rust bindings in 5 minutes!

## Prerequisites

- Go 1.25+ (for building the FFI library)
- Rust 1.70+ (for the Rust bindings)
- Linux system (tested on Ubuntu/Debian)
- Root/sudo access (for sandbox setup)

## Step 1: Build the FFI Library

```bash
# Clone the repository
git clone https://github.com/harrowfung/go-judge
cd go-judge

# Build the C FFI library
CGO_ENABLED=1 go build -buildmode=c-shared -o libgojudge.so ./cmd/go-judge-ffi

# This creates:
# - libgojudge.so (shared library)
# - libgojudge.h (C header)
```

## Step 2: Install the Library (Optional)

To make the library available system-wide:

```bash
# Copy to system library directory
sudo cp libgojudge.so /usr/local/lib/
sudo ldconfig

# Or, set LD_LIBRARY_PATH for current session
export LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH
```

## Step 3: Build the Rust Bindings

```bash
# Check that everything compiles
cargo check

# Run unit tests
cargo test --lib

# Build the library
cargo build --release
```

## Step 4: Run an Example

```bash
# Run the basic example
LD_LIBRARY_PATH=. cargo run --example basic

# Expected output:
# === go-judge Rust FFI Example ===
# ✓ Created GoJudge instance
# ✓ Initialized sandbox environment
# ...
```

## Step 5: Use in Your Project

### Add to Cargo.toml

```toml
[dependencies]
go-judge-ffi-rust = { path = "/path/to/go-judge" }
```

### Basic Usage

```rust
use go_judge_ffi_rust::{GoJudge, InitParameter, Request, Cmd};

fn main() -> Result<(), String> {
    // 1. Create and initialize
    let mut judge = GoJudge::new();
    judge.init(&InitParameter::default())?;

    // 2. Create a request
    let request = Request {
        cmd: vec![Cmd {
            args: vec!["/bin/echo".to_string(), "Hello!".to_string()],
            cpu_limit: Some(1_000_000_000),      // 1 second
            memory_limit: Some(32 * 1024 * 1024), // 32 MB
            ..Default::default()
        }],
        ..Default::default()
    };

    // 3. Execute and get results
    let response = judge.exec(&request)?;
    
    println!("Status: {:?}", response.results[0].status);
    println!("Time: {} ns", response.results[0].time);
    
    Ok(())
}
```

## Common Tasks

### Execute a Simple Command

```rust
let request = Request {
    cmd: vec![Cmd {
        args: vec!["/bin/ls".to_string(), "-la".to_string()],
        ..Default::default()
    }],
    ..Default::default()
};
```

### Set Resource Limits

```rust
let cmd = Cmd {
    args: vec!["/usr/bin/python3".to_string(), "script.py".to_string()],
    cpu_limit: Some(2_000_000_000),      // 2 seconds CPU
    memory_limit: Some(256 * 1024 * 1024), // 256 MB RAM
    proc_limit: Some(10),                 // Max 10 processes
    ..Default::default()
};
```

### Work with Files

```rust
// Add a file
let file_id = judge.file_add(b"Hello, World!", "input.txt")?;

// Use in a command
let mut copy_in = HashMap::new();
copy_in.insert(
    "/tmp/input.txt".to_string(),
    CmdFile {
        file_id: Some(file_id.clone()),
        ..Default::default()
    },
);

let request = Request {
    cmd: vec![Cmd {
        args: vec!["/bin/cat".to_string(), "/tmp/input.txt".to_string()],
        copy_in: Some(copy_in),
        copy_out: Some(vec!["stdout".to_string()]),
        ..Default::default()
    }],
    ..Default::default()
};

let response = judge.exec(&request)?;

// Access output
if let Some(files) = &response.results[0].files {
    if let Some(stdout) = files.get("stdout") {
        println!("Output: {}", stdout);
    }
}
```

### Handle Different Status Types

```rust
use go_judge_ffi_rust::Status;

match response.results[0].status {
    Status::Accepted => {
        println!("Program succeeded!");
    }
    Status::TimeLimitExceeded => {
        println!("Program took too long");
    }
    Status::MemoryLimitExceeded => {
        println!("Program used too much memory");
    }
    Status::NonzeroExitStatus => {
        println!("Program failed with exit code: {}", 
                 response.results[0].exit_status);
    }
    _ => {
        println!("Other status: {:?}", response.results[0].status);
    }
}
```

## Troubleshooting

### "cannot open shared object file"

The library can't be found. Solutions:

```bash
# Option 1: Set LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/path/to/go-judge:$LD_LIBRARY_PATH

# Option 2: Install system-wide
sudo cp libgojudge.so /usr/local/lib/
sudo ldconfig

# Option 3: Use rpath (in build.rs or rustc flags)
```

### "Init failed with code: -1"

Initialization failed. Check:
- You have root/sudo privileges (required for cgroups)
- Cgroup filesystem is mounted at `/sys/fs/cgroup`
- Init parameters are valid JSON

### Compilation Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

## Development Workflow

### Using Make

```bash
# Build everything
make -f Makefile.rust all

# Just build FFI library
make -f Makefile.rust ffi

# Run example
make -f Makefile.rust example

# Run tests
make -f Makefile.rust test

# Clean up
make -f Makefile.rust clean
```

### Testing

```bash
# Unit tests (no FFI required)
cargo test --lib

# Integration tests (requires FFI library)
LD_LIBRARY_PATH=. cargo test -- --ignored

# Specific test
cargo test test_init_parameter_serialization
```

### Documentation

```bash
# Generate and open docs
cargo doc --open --no-deps
```

## Next Steps

1. Read the [Architecture Guide](RUST_ARCHITECTURE.md) to understand internals
2. Check out the [full README](RUST_README.md) for complete API reference
3. Explore the [examples](examples/) directory for more use cases
4. Read the [go-judge documentation](https://docs.goj.ac) for advanced features

## Need Help?

- Check the [issues](https://github.com/harrowfung/go-judge/issues)
- Read the [go-judge documentation](https://docs.goj.ac)
- Look at the examples in the `examples/` directory

## Contributing

Found a bug or want to add a feature? Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request
