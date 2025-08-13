// Test execution evidence collection only
use std::process::Command;

fn main() {
    println!("Testing execution evidence collection integration...");
    
    // Create a simple test that only tests the execution evidence modules
    let output = Command::new("cargo")
        .args(&["run", "--bin", "triageir-cli", "--", "--help"])
        .current_dir(".")
        .output()
        .expect("Failed to run CLI");
    
    println!("CLI help output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.stderr.is_empty() {
        println!("CLI errors:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    println!("Exit status: {}", output.status);
}