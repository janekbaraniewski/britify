use std::env;
use std::process::Command;

fn main() {
    // Let's try tae fetch th' latest Git tag, shall we?
    // It'll gie us th' current version o' th' code, or "0.0.0" if somethin's amiss.
    let output = Command::new("git")
        .args(&["describe", "--tags"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "0.0.0".to_string());

    let version = output.trim();

    // Print th' version information, laddie, sae Cargo can pick it up.
    println!("cargo:rustc-env=VERSION={}", version);
}
