# go-judge Rust FFI Bindings

Rust bindings for the [go-judge](https://github.com/criyle/go-judge) sandbox environment. This crate provides safe Rust wrappers around the C FFI interface exported by go-judge.

## Overview

go-judge is a fast, simple, and secure sandbox for running untrusted programs. These Rust bindings allow you to use go-judge from Rust applications with a safe, idiomatic API.

## Features

- ✅ Safe Rust wrappers around the C FFI
- ✅ Full type safety with serde serialization
- ✅ Support for all go-judge FFI operations:
  - Initialize sandbox environment
  - Execute commands with resource limits
  - File store operations (add, get, delete, list)
- ✅ Comprehensive examples

## Prerequisites

Before using this crate, you need to build the go-judge FFI library:

```bash
# Build the FFI library
cd /path/to/go-judge
CGO_ENABLED=1 go build -buildmode=c-shared -o libgojudge.so ./cmd/go-judge-ffi
```

This will generate:
- `libgojudge.so` - The shared library
- `libgojudge.h` - The C header file

## Building

Add this to your `Cargo.toml`:

```toml
[dependencies]
go-judge-ffi-rust = { path = "/path/to/go-judge" }
```

Make sure the `libgojudge.so` is in your library path:

```bash
export LD_LIBRARY_PATH=/path/to/go-judge:$LD_LIBRARY_PATH
```

Or copy it to a system library directory:

```bash
sudo cp libgojudge.so /usr/local/lib/
sudo ldconfig
```

## Usage

### Basic Example

```rust
use go_judge_ffi_rust::{GoJudge, InitParameter, Request, Cmd};

fn main() {
    // Create and initialize the sandbox
    let mut judge = GoJudge::new();
    let init_params = InitParameter {
        parallelism: Some(4),
        ..Default::default()
    };
    judge.init(&init_params).expect("Failed to initialize");

    // Execute a command
    let request = Request {
        request_id: Some("test-1".to_string()),
        cmd: vec![Cmd {
            args: vec!["/bin/echo".to_string(), "Hello, World!".to_string()],
            cpu_limit: Some(1_000_000_000),  // 1 second
            memory_limit: Some(32 * 1024 * 1024),  // 32 MB
            ..Default::default()
        }],
        ..Default::default()
    };

    let response = judge.exec(&request).expect("Failed to execute");
    println!("Status: {:?}", response.results[0].status);
}
```

### File Operations

```rust
use go_judge_ffi_rust::GoJudge;

let mut judge = GoJudge::new();
judge.init(&Default::default()).unwrap();

// Add a file
let file_id = judge.file_add(b"Hello, World!", "test.txt")
    .expect("Failed to add file");

// Get the file
let content = judge.file_get(&file_id)
    .expect("Failed to get file");
println!("File content: {}", String::from_utf8_lossy(&content));

// Delete the file
judge.file_delete(&file_id).expect("Failed to delete file");

// List all files
let files = judge.file_list().expect("Failed to list files");
for (id, name) in files {
    println!("File: {} ({})", name, id);
}
```

### Resource Limits

```rust
use go_judge_ffi_rust::{Request, Cmd};

let request = Request {
    cmd: vec![Cmd {
        args: vec!["/usr/bin/python3".to_string(), "solution.py".to_string()],
        // Resource limits
        cpu_limit: Some(2_000_000_000),  // 2 seconds CPU time
        clock_limit: Some(3_000_000_000),  // 3 seconds wall time
        memory_limit: Some(256 * 1024 * 1024),  // 256 MB
        stack_limit: Some(32 * 1024 * 1024),  // 32 MB stack
        proc_limit: Some(10),  // Max 10 processes
        ..Default::default()
    }],
    ..Default::default()
};
```

## Examples

See the `examples/` directory for complete working examples:

```bash
# Run the basic example (requires libgojudge.so to be built)
cargo run --example basic
```

## API Reference

### GoJudge

The main struct for interacting with the sandbox:

- `new()` - Create a new instance
- `init(params: &InitParameter)` - Initialize the sandbox
- `exec(request: &Request)` - Execute commands
- `file_add(content: &[u8], name: &str)` - Add a file to the file store
- `file_get(file_id: &str)` - Get a file from the file store
- `file_delete(file_id: &str)` - Delete a file from the file store
- `file_list()` - List all files in the file store

### Types

#### InitParameter

Configuration for initializing the sandbox:
- `parallelism` - Number of parallel workers
- `dir` - Directory for file storage
- `tmpfs_param` - tmpfs mount parameters
- `mount_conf` - Path to mount configuration
- And more...

#### Request

A request to execute one or more commands:
- `request_id` - Optional identifier
- `cmd` - Vector of commands to execute
- `pipe_mapping` - Optional pipe mappings between commands

#### Cmd

A single command with resource limits:
- `args` - Command arguments
- `env` - Environment variables
- `cpu_limit` - CPU time limit (nanoseconds)
- `memory_limit` - Memory limit (bytes)
- `copy_in` - Files to copy into the container
- `copy_out` - Files to copy out of the container
- And more...

#### Response

Response from command execution:
- `request_id` - Request identifier
- `results` - Vector of execution results
- `error` - Error message if request failed

#### Result

Result of a single command execution:
- `status` - Execution status (Accepted, MemoryLimitExceeded, etc.)
- `exit_status` - Exit code
- `time` - CPU time used (nanoseconds)
- `memory` - Memory used (bytes)
- `run_time` - Wall clock time (nanoseconds)
- `files` - Output files
- `file_ids` - Cached file IDs

## Status Types

Possible execution statuses:
- `Accepted` - Normal exit with status 0
- `MemoryLimitExceeded` - Memory limit exceeded
- `TimeLimitExceeded` - Time limit exceeded
- `OutputLimitExceeded` - Output limit exceeded
- `FileError` - File operation error
- `NonzeroExitStatus` - Non-zero exit status
- `Signalled` - Killed by signal
- `DangerousSyscall` - Dangerous system call detected
- `InternalError` - Internal error

## Safety

The FFI bindings use `unsafe` code to interface with the C library. However, the public API is safe and handles memory management automatically:

- C strings are properly converted to/from Rust strings
- Memory allocated by C is freed using `libc::free`
- All null pointer checks are performed
- Error handling is done through `Result` types

## Platform Support

Currently supports:
- Linux x86_64

Note: go-judge has experimental support for Windows and macOS, but the Rust bindings have only been tested on Linux.

## License

MIT License - Same as go-judge

## Contributing

Contributions are welcome! Please ensure:
1. Code is properly formatted with `cargo fmt`
2. All tests pass with `cargo test`
3. Examples run successfully
4. Documentation is updated

## Links

- [go-judge repository](https://github.com/criyle/go-judge)
- [go-judge documentation](https://docs.goj.ac)
