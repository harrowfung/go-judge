//! Example of using go-judge-ffi-rust to test a compiler/interpreter
//!
//! This example demonstrates running a C program and capturing its output.

use go_judge_ffi_rust::{Cmd, CmdFile, GoJudge, InitParameter, Request};
use std::collections::HashMap;

fn main() {
    println!("=== Compiler Test Example ===\n");

    // Initialize the judge
    let mut judge = GoJudge::new();
    let init_params = InitParameter {
        parallelism: Some(2),
        dir: Some("/tmp/go-judge-compiler".to_string()),
        ..Default::default()
    };

    match judge.init(&init_params) {
        Ok(_) => println!("✓ Sandbox initialized"),
        Err(e) => {
            eprintln!("✗ Failed to initialize: {}", e);
            return;
        }
    }

    // Example C program
    let c_program = r#"
#include <stdio.h>

int main() {
    printf("Hello from C!\n");
    return 0;
}
"#;

    // Add the source file
    let source_file_id = match judge.file_add(c_program.as_bytes(), "program.c") {
        Ok(id) => {
            println!("✓ Added source file with ID: {}", id);
            id
        }
        Err(e) => {
            eprintln!("✗ Failed to add source file: {}", e);
            return;
        }
    };

    // Step 1: Compile the C program
    println!("\n--- Step 1: Compiling C Program ---");

    let mut compile_copy_in = HashMap::new();
    compile_copy_in.insert(
        "/w/program.c".to_string(),
        CmdFile {
            file_id: Some(source_file_id.clone()),
            ..Default::default()
        },
    );

    let compile_request = Request {
        request_id: Some("compile".to_string()),
        cmd: vec![Cmd {
            args: vec![
                "/usr/bin/gcc".to_string(),
                "-o".to_string(),
                "/w/program".to_string(),
                "/w/program.c".to_string(),
            ],
            env: Some(vec!["PATH=/usr/bin:/bin".to_string()]),
            cpu_limit: Some(5_000_000_000),        // 5 seconds
            memory_limit: Some(256 * 1024 * 1024), // 256 MB
            copy_in: Some(compile_copy_in),
            copy_out_cached: Some(vec!["program".to_string()]),
            ..Default::default()
        }],
        ..Default::default()
    };

    let compile_result = match judge.exec(&compile_request) {
        Ok(response) => {
            if response.results.is_empty() {
                eprintln!("✗ No compile results returned");
                return;
            }
            println!("Compile status: {:?}", response.results[0].status);
            println!("Compile time: {} ns", response.results[0].time);
            println!("Memory used: {} bytes", response.results[0].memory);
            response
        }
        Err(e) => {
            eprintln!("✗ Compilation failed: {}", e);
            return;
        }
    };

    // Get the compiled binary file ID
    let binary_file_id = match &compile_result.results[0].file_ids {
        Some(ids) => {
            if let Some(id) = ids.get("program") {
                println!("✓ Compiled binary file ID: {}", id);
                id.clone()
            } else {
                eprintln!("✗ Compiled binary not found in outputs");
                return;
            }
        }
        None => {
            eprintln!("✗ No file IDs returned from compilation");
            return;
        }
    };

    // Step 2: Run the compiled program
    println!("\n--- Step 2: Running Compiled Program ---");

    let mut run_copy_in = HashMap::new();
    run_copy_in.insert(
        "/w/program".to_string(),
        CmdFile {
            file_id: Some(binary_file_id.clone()),
            ..Default::default()
        },
    );

    let run_request = Request {
        request_id: Some("run".to_string()),
        cmd: vec![Cmd {
            args: vec!["/w/program".to_string()],
            env: Some(vec!["PATH=/usr/bin:/bin".to_string()]),
            cpu_limit: Some(1_000_000_000),       // 1 second
            memory_limit: Some(32 * 1024 * 1024), // 32 MB
            copy_in: Some(run_copy_in),
            copy_out: Some(vec!["stdout".to_string(), "stderr".to_string()]),
            ..Default::default()
        }],
        ..Default::default()
    };

    match judge.exec(&run_request) {
        Ok(response) => {
            if response.results.is_empty() {
                eprintln!("✗ No run results returned");
                return;
            }

            let result = &response.results[0];
            println!("Run status: {:?}", result.status);
            println!("Exit code: {}", result.exit_status);
            println!("Run time: {} ns", result.time);
            println!("Memory used: {} bytes", result.memory);

            if let Some(files) = &result.files {
                if let Some(stdout) = files.get("stdout") {
                    println!("\nProgram output (stdout):");
                    println!("{}", stdout);
                }
                if let Some(stderr) = files.get("stderr") {
                    if !stderr.is_empty() {
                        println!("\nProgram errors (stderr):");
                        println!("{}", stderr);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Execution failed: {}", e);
        }
    }

    // Clean up
    println!("\n--- Cleanup ---");
    let _ = judge.file_delete(&source_file_id);
    let _ = judge.file_delete(&binary_file_id);
    println!("✓ Cleaned up files");

    println!("\n=== Example Complete ===");
}
