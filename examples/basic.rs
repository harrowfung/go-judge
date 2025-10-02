//! Basic example of using go-judge-ffi-rust
//!
//! This example demonstrates:
//! 1. Initializing the sandbox
//! 2. Adding a file to the file store
//! 3. Executing a simple command
//! 4. Retrieving the results

use go_judge_ffi_rust::{Cmd, CmdFile, GoJudge, InitParameter, Request};
use std::collections::HashMap;

fn main() {
    println!("=== go-judge Rust FFI Example ===\n");

    // Create a new GoJudge instance
    let mut judge = GoJudge::new();
    println!("✓ Created GoJudge instance");

    // Initialize with default parameters
    let init_params = InitParameter {
        parallelism: Some(4),
        dir: Some("/tmp/go-judge-rust".to_string()),
        ..Default::default()
    };

    match judge.init(&init_params) {
        Ok(_) => println!("✓ Initialized sandbox environment"),
        Err(e) => {
            eprintln!("✗ Failed to initialize: {}", e);
            return;
        }
    }

    // Example 1: Simple echo command
    println!("\n--- Example 1: Simple Echo ---");
    let request = Request {
        request_id: Some("example-1".to_string()),
        cmd: vec![Cmd {
            args: vec!["/bin/echo".to_string(), "Hello from Rust!".to_string()],
            cpu_limit: Some(1_000_000_000),       // 1 second
            memory_limit: Some(32 * 1024 * 1024), // 32 MB
            ..Default::default()
        }],
        ..Default::default()
    };

    match judge.exec(&request) {
        Ok(response) => {
            println!("Request ID: {:?}", response.request_id);
            for (i, result) in response.results.iter().enumerate() {
                println!("Result {}: {:?}", i, result.status);
                println!("  Exit status: {}", result.exit_status);
                println!("  Time: {} ns", result.time);
                println!("  Memory: {} bytes", result.memory);
                if let Some(error) = &result.error {
                    println!("  Error: {}", error);
                }
            }
        }
        Err(e) => eprintln!("✗ Execution failed: {}", e),
    }

    // Example 2: Execute a program with file input/output
    println!("\n--- Example 2: File Operations ---");

    // Add a test file
    let test_content = b"Hello, World!\nThis is a test file.\n";
    match judge.file_add(test_content, "input.txt") {
        Ok(file_id) => {
            println!("✓ Added file with ID: {}", file_id);

            // Create a request that uses the cached file
            let mut copy_in = HashMap::new();
            copy_in.insert(
                "/tmp/input.txt".to_string(),
                CmdFile {
                    file_id: Some(file_id.clone()),
                    ..Default::default()
                },
            );

            let request = Request {
                request_id: Some("example-2".to_string()),
                cmd: vec![Cmd {
                    args: vec!["/bin/cat".to_string(), "/tmp/input.txt".to_string()],
                    cpu_limit: Some(1_000_000_000),
                    memory_limit: Some(32 * 1024 * 1024),
                    copy_in: Some(copy_in),
                    copy_out: Some(vec!["stdout".to_string()]),
                    ..Default::default()
                }],
                ..Default::default()
            };

            match judge.exec(&request) {
                Ok(response) => {
                    for result in &response.results {
                        println!("Status: {:?}", result.status);
                        if let Some(files) = &result.files {
                            for (name, content) in files {
                                println!("Output file '{}': {}", name, content);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("✗ Execution failed: {}", e),
            }

            // Retrieve the file
            match judge.file_get(&file_id) {
                Ok(content) => {
                    println!("✓ Retrieved file: {} bytes", content.len());
                    println!("  Content: {}", String::from_utf8_lossy(&content));
                }
                Err(e) => eprintln!("✗ Failed to get file: {}", e),
            }

            // Delete the file
            match judge.file_delete(&file_id) {
                Ok(_) => println!("✓ Deleted file"),
                Err(e) => eprintln!("✗ Failed to delete file: {}", e),
            }
        }
        Err(e) => eprintln!("✗ Failed to add file: {}", e),
    }

    // Example 3: List files in the store
    println!("\n--- Example 3: List Files ---");
    match judge.file_list() {
        Ok(files) => {
            println!("Files in store: {}", files.len());
            for (id, name) in files {
                println!("  {} -> {}", id, name);
            }
        }
        Err(e) => eprintln!("✗ Failed to list files: {}", e),
    }

    println!("\n=== Example Complete ===");
}
