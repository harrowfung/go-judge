use std::env;

fn main() {
    // Tell cargo to look for the libgojudge library in the current directory
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", dir);

    // Also check common library paths
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");

    // Link to the gojudge library
    println!("cargo:rustc-link-lib=dylib=gojudge");

    // Rebuild if the library changes
    println!("cargo:rerun-if-changed=libgojudge.so");
    println!("cargo:rerun-if-changed=libgojudge.h");
}
