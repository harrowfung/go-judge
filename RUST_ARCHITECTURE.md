# Rust FFI Bindings Architecture

This document describes the architecture and design decisions for the Rust FFI bindings to go-judge.

## Overview

The Rust bindings provide a safe, idiomatic interface to the go-judge C FFI library. The implementation is split into several layers:

```
┌─────────────────────────────────────────┐
│    Application Code (User)              │
├─────────────────────────────────────────┤
│    Safe Rust API (lib.rs)               │
│    - GoJudge struct                     │
│    - Result types                       │
│    - Error handling                     │
├─────────────────────────────────────────┤
│    Type System (types.rs)               │
│    - Serializable types                 │
│    - Serde integration                  │
├─────────────────────────────────────────┤
│    Raw FFI Bindings (ffi.rs)            │
│    - extern "C" declarations            │
│    - Unsafe interface                   │
├─────────────────────────────────────────┤
│    C FFI Library (libgojudge.so)        │
│    - Go runtime                         │
│    - Sandbox implementation             │
└─────────────────────────────────────────┘
```

## Module Structure

### `src/ffi.rs` - Raw FFI Bindings

This module contains the low-level unsafe bindings to the C library:

```rust
extern "C" {
    pub fn Init(i: *mut c_char) -> i32;
    pub fn Exec(e: *mut c_char) -> *mut c_char;
    // ... more functions
}
```

**Design Decisions:**
- All functions are marked `unsafe` to reflect their nature
- Parameters use raw C types (`*mut c_char`, `i32`, etc.)
- No automatic memory management - callers must handle `malloc`/`free`
- Well-documented with safety contracts

### `src/types.rs` - Type System

Defines Rust types that correspond to the Go/JSON types used by go-judge:

**Key Types:**
- `InitParameter` - Sandbox initialization configuration
- `Request` / `Response` - Execution request/response
- `Cmd` - Command with resource limits
- `CmdResult` - Execution result
- `Status` - Execution status enum
- `CmdFile` - File specification

**Design Decisions:**
- All types derive `Serialize` and `Deserialize` from serde
- Use of `Option<T>` for optional fields
- Snake case field names with `#[serde(rename_all = "camelCase")]`
- Named `CmdResult` instead of `Result` to avoid conflict with std::result::Result
- Comprehensive documentation on each field

### `src/lib.rs` - Safe API

Provides a safe, ergonomic Rust interface:

**Main Structure:**
```rust
pub struct GoJudge {
    initialized: bool,
}
```

**Key Methods:**
- `init(&mut self, params: &InitParameter)` - Initialize sandbox
- `exec(&self, request: &Request)` - Execute commands
- `file_add/get/delete/list()` - File operations

**Design Decisions:**

1. **State Tracking**: The `initialized` field prevents operations before initialization
2. **Memory Safety**: All C strings are properly converted and freed
3. **Error Handling**: Returns `Result<T, String>` for all fallible operations
4. **Resource Management**: Automatically frees C-allocated memory
5. **Type Safety**: Strong typing prevents invalid states

## Memory Management

The Rust bindings carefully manage memory across the FFI boundary:

### C → Rust (Receiving Data)

When receiving data from C:

```rust
let result_ptr = unsafe { ffi::Exec(c_json.as_ptr() as *mut i8) };
// ... use the pointer ...
unsafe { libc::free(result_ptr as *mut libc::c_void) }; // Free when done
```

**Pattern:**
1. Call C function to get pointer
2. Convert to Rust type
3. Free the C memory using `libc::free`

### Rust → C (Sending Data)

When sending data to C:

```rust
let c_json = CString::new(json)?;
unsafe { ffi::Init(c_json.as_ptr() as *mut i8) };
// CString automatically freed when dropped
```

**Pattern:**
1. Create `CString` from Rust string
2. Pass pointer to C
3. Let `CString` drop naturally

### Arrays and Complex Types

For arrays (e.g., in `FileList`):

```rust
for i in 0..count {
    unsafe {
        let id_ptr = *ids_ptr.add(i);
        // ... use the string ...
        libc::free(id_ptr as *mut libc::c_void); // Free each string
    }
}
// Free the array itself
unsafe {
    libc::free(ids_ptr as *mut libc::c_void);
    libc::free(names_ptr as *mut libc::c_void);
}
```

**Pattern:**
1. Iterate through elements, freeing each
2. Free the array container

## Error Handling

The bindings use Rust's `Result` type for error handling:

```rust
pub fn exec(&self, request: &Request) -> Result<Response, String>
```

**Error Types:**
- Serialization errors (from `serde_json`)
- FFI errors (null pointers, invalid data)
- Initialization errors (not initialized)
- File operation errors (not found, internal error)

**Design Decisions:**
- Use `String` for error messages (simple, flexible)
- Check initialization state before operations
- Validate null pointers from C
- Provide descriptive error messages

## Serialization

All data exchange with the C library uses JSON:

```
Rust Type → JSON String → C char* → Go → Process
```

**Benefits:**
- Language-agnostic data format
- Easy to debug (human-readable)
- Matches existing go-judge REST API
- Leverages serde ecosystem

**Trade-offs:**
- Slight performance overhead vs. binary format
- Acceptable for typical use cases (running programs takes >> serialization time)

## Safety Guarantees

The safe API provides several guarantees:

1. **No Undefined Behavior**: All unsafe code is encapsulated
2. **Memory Safety**: All allocations are tracked and freed
3. **Type Safety**: Cannot mix up types or create invalid states
4. **Thread Safety**: Each `GoJudge` instance is not `Sync` (single-threaded use)

## Testing Strategy

Tests are organized into layers:

### Unit Tests (`src/lib.rs`)
- Test type serialization
- Test basic functionality
- No FFI dependencies

### Integration Tests (`tests/`)
- Test actual FFI operations
- Marked with `#[ignore]` to skip by default
- Require `libgojudge.so` to be built

### Examples (`examples/`)
- Demonstrate real-world usage
- Serve as integration tests
- Show best practices

## Building and Linking

### Build Process

1. **Go FFI Library**: Built with CGo in c-shared mode
   ```bash
   CGO_ENABLED=1 go build -buildmode=c-shared -o libgojudge.so ./cmd/go-judge-ffi
   ```

2. **Rust Bindings**: Standard cargo build
   ```bash
   cargo build
   ```

3. **Linking**: `build.rs` configures library search paths

### Runtime Linking

The Rust bindings dynamically link to `libgojudge.so`:

```
Rust binary → libgojudge.so → Go runtime
```

**Requirements:**
- `libgojudge.so` must be in `LD_LIBRARY_PATH` or system library path
- Go runtime is embedded in the `.so` file
- No additional Go installation needed at runtime

## Performance Considerations

1. **JSON Overhead**: Minimal for typical workloads
2. **FFI Boundary**: Each call crosses into Go runtime
3. **Memory Copies**: JSON strings are copied across boundary
4. **Go Runtime**: Garbage collector runs in background

**Optimization Opportunities:**
- Batch operations when possible
- Reuse file IDs to avoid re-uploading
- Use `copy_out_cached` for frequently accessed files

## Future Improvements

Potential enhancements:

1. **Async Support**: Add async/await interface
2. **Builder Pattern**: Make constructing requests more ergonomic
3. **Type-safe File References**: Newtype wrappers for file IDs
4. **Custom Error Types**: Enum instead of String for better matching
5. **Callback Support**: If go-judge adds streaming callbacks
6. **Binary Protocol**: Alternative to JSON for performance

## Contributing

When modifying the bindings:

1. Keep unsafe code minimal and well-documented
2. Add tests for new functionality
3. Update examples for new features
4. Run `cargo fmt` and `cargo clippy`
5. Ensure documentation is complete

## References

- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [go-judge Documentation](https://docs.goj.ac)
- [serde JSON](https://github.com/serde-rs/json)
