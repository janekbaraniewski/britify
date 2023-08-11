use std::env;
use std::process::Command;

fn main() {
    // Try to fetch the latest Git tag
    let output = Command::new("git")
        .args(&["describe", "--tags"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "0.0.0".to_string());

    let version = output.trim();

    // Print the version information for Cargo to pick up
    println!("cargo:rustc-env=VERSION={}", version);
}
