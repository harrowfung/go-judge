//! Type definitions for go-judge FFI
//!
//! This module contains Rust types that correspond to the Go types used in go-judge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for initializing the sandbox environment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitParameter {
    /// Path to the container init binary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinit_path: Option<String>,

    /// Number of parallel workers (default: 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,

    /// tmpfs parameters (default: "size=16m,nr_inodes=4k")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs_param: Option<String>,

    /// Directory for file storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,

    /// Enable network sharing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_share: Option<bool>,

    /// Mount configuration file path (default: "mount.yaml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_conf: Option<String>,

    /// Source prefix for file paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_prefix: Option<String>,

    /// Cgroup prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_prefix: Option<String>,

    /// CPU set for container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,

    /// Starting credential ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_start: Option<i32>,

    /// Enable CPU rate limiting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cpu_rate: Option<bool>,

    /// CPU CFS period (in nanoseconds, default: 100ms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_period: Option<u64>,

    /// Disable fallback mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fallback: Option<bool>,
}

impl Default for InitParameter {
    fn default() -> Self {
        Self {
            cinit_path: None,
            parallelism: Some(4),
            tmpfs_param: Some("size=16m,nr_inodes=4k".to_string()),
            dir: None,
            net_share: None,
            mount_conf: Some("mount.yaml".to_string()),
            src_prefix: None,
            cgroup_prefix: None,
            cpuset: None,
            cred_start: None,
            enable_cpu_rate: None,
            cpu_cfs_period: Some(100_000_000), // 100ms in nanoseconds
            no_fallback: None,
        }
    }
}

/// File from multiple sources (local, memory, cached, or pipe)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CmdFile {
    /// Path to local file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,

    /// File content in memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// ID of cached file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// Name for pipe collector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Max size for pipe collector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,

    /// Symlink target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlink: Option<String>,

    /// Stream input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_in: Option<bool>,

    /// Stream output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_out: Option<bool>,

    /// Is pipe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe: Option<bool>,
}

/// Command definition with resource limits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cmd {
    /// Command arguments
    pub args: Vec<String>,

    /// Environment variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    /// File descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<CmdFile>>,

    /// CPU time limit in nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_limit: Option<u64>,

    /// Real CPU time limit in nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_cpu_limit: Option<u64>,

    /// Clock time limit in nanoseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_limit: Option<u64>,

    /// Memory limit in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<u64>,

    /// Stack limit in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_limit: Option<u64>,

    /// Process limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proc_limit: Option<u64>,

    /// CPU rate limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_rate_limit: Option<u64>,

    /// CPU set limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_set_limit: Option<String>,

    /// Files to copy into the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_in: Option<HashMap<String, CmdFile>>,

    /// Files to copy out of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_out: Option<Vec<String>>,

    /// Files to copy out and cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_out_cached: Option<Vec<String>>,

    /// Maximum size for copy out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_out_max: Option<u64>,

    /// Directory to copy out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_out_dir: Option<String>,

    /// Enable TTY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,

    /// Strict memory limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_memory_limit: Option<bool>,

    /// Data segment limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_segment_limit: Option<bool>,

    /// Address space limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_space_limit: Option<bool>,
}

/// Pipe index for a file descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeIndex {
    /// Command index
    pub index: i32,
    /// File descriptor
    pub fd: i32,
}

/// Pipe mapping between commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeMap {
    /// Input pipe
    #[serde(rename = "in")]
    pub in_pipe: PipeIndex,
    /// Output pipe
    pub out: PipeIndex,
    /// Pipe name
    pub name: String,
    /// Maximum pipe size
    pub max: i64,
    /// Proxy mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<bool>,
}

/// Request to execute commands
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Commands to execute
    pub cmd: Vec<Cmd>,

    /// Pipe mappings between commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe_mapping: Option<Vec<PipeMap>>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    /// Program exited normally with status code 0
    Accepted,
    /// Memory limit exceeded
    #[serde(rename = "Memory Limit Exceeded")]
    MemoryLimitExceeded,
    /// Time limit exceeded
    #[serde(rename = "Time Limit Exceeded")]
    TimeLimitExceeded,
    /// Output limit exceeded
    #[serde(rename = "Output Limit Exceeded")]
    OutputLimitExceeded,
    /// File error
    #[serde(rename = "File Error")]
    FileError,
    /// Non-zero exit status
    #[serde(rename = "Nonzero Exit Status")]
    NonzeroExitStatus,
    /// Signalled (killed by signal)
    Signalled,
    /// Dangerous system call
    #[serde(rename = "Dangerous Syscall")]
    DangerousSyscall,
    /// Internal error
    #[serde(rename = "Internal Error")]
    InternalError,
}

/// File error information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileError {
    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,
    /// Error message
    pub message: String,
    /// File name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Result of a command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmdResult {
    /// Execution status
    pub status: Status,

    /// Exit status code
    pub exit_status: i32,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// CPU time used in nanoseconds
    pub time: u64,

    /// Memory used in bytes
    pub memory: u64,

    /// Wall clock time in nanoseconds
    pub run_time: u64,

    /// Peak process count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proc_peak: Option<u64>,

    /// Output files (name -> content as base64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, String>>,

    /// Cached file IDs (name -> file ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<HashMap<String, String>>,

    /// File errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_error: Option<Vec<FileError>>,
}

/// Response from command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Results for each command
    pub results: Vec<CmdResult>,

    /// Error message if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
