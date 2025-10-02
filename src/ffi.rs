//! Low-level FFI bindings to the go-judge C library
//!
//! This module contains raw unsafe bindings to the C functions exported by libgojudge.

use std::os::raw::c_char;

#[link(name = "gojudge")]
extern "C" {
    /// Initialize the sandbox environment
    ///
    /// # Safety
    /// The pointer must be a valid null-terminated C string containing JSON
    pub fn Init(i: *mut c_char) -> i32;

    /// Execute a command inside the container runner
    ///
    /// # Safety
    /// The pointer must be a valid null-terminated C string containing JSON.
    /// The returned pointer must be freed by the caller using `libc::free`.
    pub fn Exec(e: *mut c_char) -> *mut c_char;

    /// Get the list of files in the file store
    ///
    /// # Safety
    /// The ids and names pointers will be allocated by the function.
    /// Both the 2D arrays and individual strings must be freed by the caller.
    pub fn FileList(ids: *mut *mut *mut c_char, names: *mut *mut *mut c_char) -> usize;

    /// Add a file to the file store
    ///
    /// # Safety
    /// content must be a valid pointer to a buffer of size contentLen.
    /// name must be a valid null-terminated C string.
    /// The returned pointer must be freed by the caller using `libc::free`.
    pub fn FileAdd(content: *mut c_char, contentLen: i32, name: *mut c_char) -> *mut c_char;

    /// Get a file from the file store by ID
    ///
    /// # Safety
    /// e must be a valid null-terminated C string.
    /// out will be allocated by the function and must be freed by the caller.
    /// Returns the length of the file (>= 0), or an error code (< 0):
    /// - -1: File does not exist
    /// - -2: Internal error
    pub fn FileGet(e: *mut c_char, out: *mut *mut c_char) -> i32;

    /// Delete a file from the file store by ID
    ///
    /// # Safety
    /// e must be a valid null-terminated C string.
    /// Returns 0 if failed, non-zero if successful.
    pub fn FileDelete(e: *mut c_char) -> i32;
}
