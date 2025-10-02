//! Integration tests for go-judge Rust FFI bindings
//!
//! Note: These tests require the libgojudge.so library to be built and available.
//! Run: CGO_ENABLED=1 go build -buildmode=c-shared -o libgojudge.so ./cmd/go-judge-ffi

use go_judge_ffi_rust::*;

#[test]
#[ignore] // Ignore by default as it requires the FFI library to be built
fn test_init_parameter_serialization() {
    let params = InitParameter {
        parallelism: Some(4),
        dir: Some("/tmp/test-judge".to_string()),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&params).unwrap();
    assert!(json.contains("parallelism"));
    assert!(json.contains("4"));
}

#[test]
#[ignore] // Ignore by default as it requires the FFI library to be built
fn test_request_serialization() {
    let request = Request {
        request_id: Some("test-1".to_string()),
        cmd: vec![Cmd {
            args: vec!["/bin/echo".to_string(), "test".to_string()],
            cpu_limit: Some(1_000_000_000),
            memory_limit: Some(32 * 1024 * 1024),
            ..Default::default()
        }],
        ..Default::default()
    };
    
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("test-1"));
    assert!(json.contains("/bin/echo"));
}

#[test]
#[ignore] // Ignore by default as it requires the FFI library to be built
fn test_status_serialization() {
    let status = Status::Accepted;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"Accepted\"");
    
    let status = Status::MemoryLimitExceeded;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"Memory Limit Exceeded\"");
}

#[test]
#[ignore] // Ignore by default as it requires the FFI library and proper setup
fn test_basic_initialization() {
    let mut judge = GoJudge::new();
    let params = InitParameter {
        parallelism: Some(2),
        dir: Some("/tmp/rust-test-judge".to_string()),
        ..Default::default()
    };
    
    // This will fail if the library is not built or not in the path
    match judge.init(&params) {
        Ok(_) => println!("Initialization successful"),
        Err(e) => panic!("Initialization failed: {}", e),
    }
}

#[test]
#[ignore] // Ignore by default as it requires the FFI library and proper setup
fn test_file_operations() {
    let mut judge = GoJudge::new();
    judge.init(&InitParameter {
        dir: Some("/tmp/rust-test-judge-files".to_string()),
        ..Default::default()
    }).expect("Failed to initialize");
    
    // Add a file
    let content = b"Hello from Rust test!";
    let file_id = judge.file_add(content, "test.txt")
        .expect("Failed to add file");
    
    // Retrieve the file
    let retrieved = judge.file_get(&file_id)
        .expect("Failed to get file");
    assert_eq!(retrieved, content);
    
    // List files
    let files = judge.file_list().expect("Failed to list files");
    assert!(files.iter().any(|(id, _)| id == &file_id));
    
    // Delete the file
    judge.file_delete(&file_id).expect("Failed to delete file");
    
    // Verify it's deleted
    match judge.file_get(&file_id) {
        Err(e) => assert!(e.contains("does not exist")),
        Ok(_) => panic!("File should have been deleted"),
    }
}
