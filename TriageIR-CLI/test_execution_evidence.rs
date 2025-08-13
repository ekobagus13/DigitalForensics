// Simple test for execution evidence collection
use std::process::Command;

fn main() {
    println!("Testing execution evidence collection...");
    
    // Test prefetch collection
    println!("Testing prefetch collection...");
    match Command::new("cargo")
        .args(&["test", "--lib", "prefetch", "--", "--nocapture"])
        .current_dir(".")
        .output()
    {
        Ok(output) => {
            println!("Prefetch test output: {}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("Prefetch test errors: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => println!("Failed to run prefetch test: {}", e),
    }
    
    // Test shimcache collection
    println!("Testing shimcache collection...");
    match Command::new("cargo")
        .args(&["test", "--lib", "shimcache", "--", "--nocapture"])
        .current_dir(".")
        .output()
    {
        Ok(output) => {
            println!("Shimcache test output: {}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("Shimcache test errors: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => println!("Failed to run shimcache test: {}", e),
    }
}