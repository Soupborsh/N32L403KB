use std::env;
use std::fs::copy;

fn main() {
    let is_app = env::var_os("CARGO_FEATURE_APP").is_some();
    let in_ram = env::var_os("CARGO_FEATURE_IN_RAM").is_some();

    if in_ram {
        copy("memory_ram.x", "memory.x").unwrap();
        println!("cargo:rustc-link-arg-bins=-Tlink_ram.x");
        println!("cargo:rerun-if-changed=link_ram.x");
    } else {
        if is_app {
            copy("memory_app.x", "memory.x").unwrap();
        } else {
            copy("memory_bare.x", "memory.x").unwrap();
        }
        println!("cargo:rustc-link-arg-bins=-Tlink.x");
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=memory_app.x");
    println!("cargo::rerun-if-changed=memory_bare.x");
    println!("cargo::rerun-if-changed=memory_ram.x");
}
