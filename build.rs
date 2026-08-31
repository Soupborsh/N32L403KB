use std::env;
use std::fs::copy;

fn main() {
    let is_app = env::var_os("CARGO_FEATURE_APP").is_some();
    if is_app {
        copy("memory_app.x", "memory.x").unwrap();
    } else {
        copy("memory_bare.x", "memory.x").unwrap();
    }
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=memory_app.x");
    println!("cargo::rerun-if-changed=memory_bare.x");
}
