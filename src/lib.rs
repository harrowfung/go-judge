//! Rust FFI bindings for go-judge sandbox
//!
//! This crate provides safe Rust bindings to the go-judge FFI library,
//! which enables running programs in a restricted sandbox environment.
//!
//! # Example
//!
//! ```no_run
//! use go_judge_ffi_rust::{GoJudge, InitParameter, Request, Cmd};
//!
//! // Initialize the sandbox
//! let mut judge = GoJudge::new();
//! let init_params = InitParameter {
//!     parallelism: 4,
//!     ..Default::default()
//! };
//! judge.init(&init_params).expect("Failed to initialize");
//!
//! // Execute a command
//! let request = Request {
//!     request_id: "test-1".to_string(),
//!     cmd: vec![Cmd {
//!         args: vec!["/bin/echo".to_string(), "Hello, World!".to_string()],
//!         ..Default::default()
//!     }],
//!     ..Default::default()
//! };
//!
//! let response = judge.exec(&request).expect("Failed to execute");
//! println!("Response: {:?}", response);
//! ```

pub mod ffi;
pub mod types;

pub use types::*;

use std::ffi::{CStr, CString};
use std::ptr;

/// Main struct for interacting with the go-judge FFI library
pub struct GoJudge {
    initialized: bool,
}

impl GoJudge {
    /// Create a new GoJudge instance
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Initialize the sandbox environment
    pub fn init(&mut self, params: &InitParameter) -> Result<(), String> {
        let json = serde_json::to_string(params)
            .map_err(|e| format!("Failed to serialize init parameters: {}", e))?;

        let c_json = CString::new(json).map_err(|e| format!("Failed to create CString: {}", e))?;

        let result = unsafe { ffi::Init(c_json.as_ptr() as *mut i8) };

        if result == 0 {
            self.initialized = true;
            Ok(())
        } else {
            Err(format!("Init failed with code: {}", result))
        }
    }

    /// Execute a command in the sandbox
    pub fn exec(&self, request: &Request) -> Result<Response, String> {
        if !self.initialized {
            return Err("GoJudge not initialized".to_string());
        }

        let json = serde_json::to_string(request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let c_json = CString::new(json).map_err(|e| format!("Failed to create CString: {}", e))?;

        let result_ptr = unsafe { ffi::Exec(c_json.as_ptr() as *mut i8) };

        if result_ptr.is_null() {
            return Err("Exec returned null".to_string());
        }

        let c_str = unsafe { CStr::from_ptr(result_ptr) };
        let result_json = c_str
            .to_str()
            .map_err(|e| format!("Failed to convert result to string: {}", e))?;

        let response: Response = serde_json::from_str(result_json)
            .map_err(|e| format!("Failed to deserialize response: {}", e))?;

        // Free the C string
        unsafe { libc::free(result_ptr as *mut libc::c_void) };

        Ok(response)
    }

    /// List all files in the file store
    pub fn file_list(&self) -> Result<Vec<(String, String)>, String> {
        if !self.initialized {
            return Err("GoJudge not initialized".to_string());
        }

        let mut ids_ptr: *mut *mut i8 = ptr::null_mut();
        let mut names_ptr: *mut *mut i8 = ptr::null_mut();

        let count = unsafe { ffi::FileList(&mut ids_ptr, &mut names_ptr) };

        let mut files = Vec::new();

        for i in 0..count {
            unsafe {
                let id_ptr = *ids_ptr.add(i);
                let name_ptr = *names_ptr.add(i);

                let id = CStr::from_ptr(id_ptr).to_string_lossy().into_owned();
                let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();

                files.push((id, name));

                // Free the individual strings
                libc::free(id_ptr as *mut libc::c_void);
                libc::free(name_ptr as *mut libc::c_void);
            }
        }

        // Free the arrays
        unsafe {
            libc::free(ids_ptr as *mut libc::c_void);
            libc::free(names_ptr as *mut libc::c_void);
        }

        Ok(files)
    }

    /// Add a file to the file store
    pub fn file_add(&self, content: &[u8], name: &str) -> Result<String, String> {
        if !self.initialized {
            return Err("GoJudge not initialized".to_string());
        }

        let c_name = CString::new(name).map_err(|e| format!("Failed to create CString: {}", e))?;

        let file_id_ptr = unsafe {
            ffi::FileAdd(
                content.as_ptr() as *mut i8,
                content.len() as i32,
                c_name.as_ptr() as *mut i8,
            )
        };

        if file_id_ptr.is_null() {
            return Err("FileAdd returned null".to_string());
        }

        let c_str = unsafe { CStr::from_ptr(file_id_ptr) };
        let file_id = c_str.to_string_lossy().into_owned();

        // Free the C string
        unsafe { libc::free(file_id_ptr as *mut libc::c_void) };

        Ok(file_id)
    }

    /// Get a file from the file store by ID
    pub fn file_get(&self, file_id: &str) -> Result<Vec<u8>, String> {
        if !self.initialized {
            return Err("GoJudge not initialized".to_string());
        }

        let c_file_id =
            CString::new(file_id).map_err(|e| format!("Failed to create CString: {}", e))?;

        let mut out_ptr: *mut i8 = ptr::null_mut();

        let len = unsafe { ffi::FileGet(c_file_id.as_ptr() as *mut i8, &mut out_ptr) };

        if len < 0 {
            return Err(match len {
                -1 => "File does not exist".to_string(),
                -2 => "Internal error".to_string(),
                _ => format!("Unknown error: {}", len),
            });
        }

        let content =
            unsafe { std::slice::from_raw_parts(out_ptr as *const u8, len as usize).to_vec() };

        // Free the buffer
        unsafe { libc::free(out_ptr as *mut libc::c_void) };

        Ok(content)
    }

    /// Delete a file from the file store by ID
    pub fn file_delete(&self, file_id: &str) -> Result<(), String> {
        if !self.initialized {
            return Err("GoJudge not initialized".to_string());
        }

        let c_file_id =
            CString::new(file_id).map_err(|e| format!("Failed to create CString: {}", e))?;

        let result = unsafe { ffi::FileDelete(c_file_id.as_ptr() as *mut i8) };

        if result == 0 {
            Err("Failed to delete file".to_string())
        } else {
            Ok(())
        }
    }
}

impl Default for GoJudge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_parameter_serialization() {
        let params = InitParameter {
            parallelism: Some(4),
            ..Default::default()
        };
        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("parallelism"));
    }
}
