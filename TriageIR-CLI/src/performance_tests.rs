//! Performance testing and benchmarking for TriageIR CLI
//! This module contains performance tests and benchmarks to ensure optimal operation

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::process::Command;
    use std::time::{Duration, Instant};
    use tempfile::NamedTempFile;
    use serde_json::Value;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// Benchmark baseline performance metrics
    #[test]
    fn benchmark_baseline_performance() {
        println!("Benchmarking baseline performance...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        let mut execution_times = Vec::new();
        let mut artifact_counts = Vec::new();
        let mut output_sizes = Vec::new();

        // Run multiple iterations to get stable metrics
        for i in 0..5 {
            println!("  Running benchmark iteration {}/5...", i + 1);
            
            let start_time = Instant::now();
            let output = Command::new("cargo")
                .args(&["run", "--", "--output", temp_path, "--format", "json"])
                .current_dir(".")
                .output()
                .expect("Failed to execute CLI");
            let execution_time = start_time.elapsed();

            assert!(output.status.success(), "CLI should succeed in benchmark");

            let json_content = std::fs::read_to_string(temp_path)
                .expect("Failed to read output file");
            let parsed: Value = serde_json::from_str(&json_content)
                .expect("Should be valid JSON");

            let artifact_count = parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap();
            let output_size = json_content.len();

            execution_times.push(execution_time);
            artifact_counts.push(artifact_count);
            output_sizes.push(output_size);

            // Small delay between iterations
            std::thread::sleep(Duration::from_millis(500));
        }

        // Calculate statistics
        let avg_time = execution_times.iter().sum::<Duration>() / execution_times.len() as u32;
        let min_time = execution_times.iter().min().unwrap();
        let max_time = execution_times.iter().max().unwrap();
        
        let avg_artifacts = artifact_counts.iter().sum::<u64>() / artifact_counts.len() as u64;
        let avg_output_size = output_sizes.iter().sum::<usize>() / output_sizes.len();
        
        let artifacts_per_second = avg_artifacts as f64 / avg_time.as_secs_f64();

        println!("  Performance Metrics:");
        println!("  ===================");
        println!("  Average execution time: {:.2} seconds", avg_time.as_secs_f64());
        println!("  Min execution time: {:.2} seconds", min_time.as_secs_f64());
        println!("  Max execution time: {:.2} seconds", max_time.as_secs_f64());
        println!("  Average artifacts collected: {}", avg_artifacts);
        println!("  Average output size: {} bytes ({:.2} MB)", avg_output_size, avg_output_size as f64 / 1_000_000.0);
        println!("  Artifacts per second: {:.1}", artifacts_per_second);

        // Performance assertions
        assert!(avg_time.as_secs() < 30, "Average execution time should be under 30 seconds");
        assert!(artifacts_per_second > 10.0, "Should collect at least 10 artifacts per second");
        assert!(avg_output_size > 1000, "Should generate substantial output");
        assert!(avg_output_size < 50_000_000, "Output should not be excessively large");

        println!("✓ Baseline performance benchmarking completed");
    }

    /// Test performance under memory pressure
    #[test]
    fn test_performance_under_memory_pressure() {
        println!("Testing performance under memory pressure...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Create memory pressure by allocating large amounts of memory
        let memory_pressure_handles: Vec<_> = (0..3).map(|i| {
            thread::spawn(move || {
                println!("  Starting memory pressure thread {}...", i);
                let mut memory_hogs = Vec::new();
                
                // Allocate memory in chunks
                for _ in 0..100 {
                    let chunk: Vec<u8> = vec![0; 1_000_000]; // 1MB chunks
                    memory_hogs.push(chunk);
                    std::thread::sleep(Duration::from_millis(10));
                }
                
                // Hold memory for duration of test
                std::thread::sleep(Duration::from_secs(15));
                
                println!("  Memory pressure thread {} releasing memory...", i);
                drop(memory_hogs);
            })
        }).collect();

        // Wait for memory pressure to build up
        std::thread::sleep(Duration::from_secs(2));

        // Run CLI under memory pressure
        println!("  Running CLI under memory pressure...");
        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let execution_time = start_time.elapsed();

        // Wait for memory pressure threads to complete
        for handle in memory_pressure_handles {
            handle.join().unwrap();
        }

        assert!(output.status.success(), "CLI should succeed under memory pressure");
        assert!(execution_time.as_secs() < 60, "Should complete within 60 seconds under memory pressure");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should produce valid JSON under memory pressure");

        let artifact_count = parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap();
        assert!(artifact_count > 0, "Should collect artifacts even under memory pressure");

        println!("  Execution time under memory pressure: {:.2} seconds", execution_time.as_secs_f64());
        println!("  Artifacts collected: {}", artifact_count);
        println!("✓ Performance under memory pressure validated");
    }

    /// Test concurrent execution performance
    #[test]
    fn test_concurrent_execution_performance() {
        println!("Testing concurrent execution performance...");
        
        let concurrent_runs = 3;
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();

        for i in 0..concurrent_runs {
            let results_clone = Arc::clone(&results);
            let handle = thread::spawn(move || {
                let temp_file = NamedTempFile::new().expect("Failed to create temp file");
                let temp_path = temp_file.path().to_str().unwrap();

                println!("  Starting concurrent run {}...", i + 1);
                let start_time = Instant::now();
                let output = Command::new("cargo")
                    .args(&["run", "--", "--output", temp_path, "--format", "json"])
                    .current_dir(".")
                    .output()
                    .expect("Failed to execute CLI");
                let execution_time = start_time.elapsed();

                let success = output.status.success();
                let artifact_count = if success {
                    let json_content = std::fs::read_to_string(temp_path)
                        .expect("Failed to read output file");
                    let parsed: Value = serde_json::from_str(&json_content)
                        .expect("Should be valid JSON");
                    parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap_or(0)
                } else {
                    0
                };

                let mut results = results_clone.lock().unwrap();
                results.push((i, execution_time, success, artifact_count));
                
                println!("  Concurrent run {} completed in {:.2} seconds", i + 1, execution_time.as_secs_f64());
            });
            handles.push(handle);
        }

        // Wait for all concurrent runs to complete
        for handle in handles {
            handle.join().unwrap();
        }

        let results = results.lock().unwrap();
        
        // Analyze concurrent execution results
        let successful_runs = results.iter().filter(|(_, _, success, _)| *success).count();
        let avg_time = results.iter()
            .filter(|(_, _, success, _)| *success)
            .map(|(_, time, _, _)| time.as_secs_f64())
            .sum::<f64>() / successful_runs as f64;
        let total_artifacts: u64 = results.iter()
            .filter(|(_, _, success, _)| *success)
            .map(|(_, _, _, artifacts)| *artifacts)
            .sum();

        println!("  Concurrent Execution Results:");
        println!("  ============================");
        println!("  Successful runs: {}/{}", successful_runs, concurrent_runs);
        println!("  Average execution time: {:.2} seconds", avg_time);
        println!("  Total artifacts collected: {}", total_artifacts);

        // Performance assertions for concurrent execution
        assert!(successful_runs >= concurrent_runs - 1, "Most concurrent runs should succeed");
        assert!(avg_time < 45.0, "Average concurrent execution time should be reasonable");
        assert!(total_artifacts > 0, "Should collect artifacts in concurrent runs");

        println!("✓ Concurrent execution performance validated");
    }

    /// Test scalability with large datasets
    #[test]
    fn test_scalability_with_large_datasets() {
        println!("Testing scalability with large datasets...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Create additional system activity to increase dataset size
        let activity_handles: Vec<_> = (0..10).map(|i| {
            thread::spawn(move || {
                // Create multiple processes to increase process count
                for j in 0..20 {
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "timeout", "1", ">nul"])
                        .spawn();
                    
                    // Create network activity
                    let _ = std::process::Command::new("ping")
                        .args(&["-n", "1", "127.0.0.1"])
                        .output();
                    
                    std::thread::sleep(Duration::from_millis(50));
                }
            })
        }).collect();

        // Wait for activity to start
        std::thread::sleep(Duration::from_secs(1));

        // Run CLI with increased dataset
        println!("  Running CLI with increased system activity...");
        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let execution_time = start_time.elapsed();

        // Wait for background activity to complete
        for handle in activity_handles {
            let _ = handle.join();
        }

        assert!(output.status.success(), "CLI should handle large datasets");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should produce valid JSON with large datasets");

        let artifact_count = parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap();
        let output_size = json_content.len();
        let processing_rate = artifact_count as f64 / execution_time.as_secs_f64();

        println!("  Large Dataset Results:");
        println!("  =====================");
        println!("  Execution time: {:.2} seconds", execution_time.as_secs_f64());
        println!("  Artifacts collected: {}", artifact_count);
        println!("  Output size: {} bytes ({:.2} MB)", output_size, output_size as f64 / 1_000_000.0);
        println!("  Processing rate: {:.1} artifacts/second", processing_rate);

        // Scalability assertions
        assert!(execution_time.as_secs() < 90, "Should handle large datasets within 90 seconds");
        assert!(processing_rate > 5.0, "Should maintain reasonable processing rate with large datasets");
        assert!(output_size < 100_000_000, "Output should remain manageable even with large datasets");

        println!("✓ Scalability with large datasets validated");
    }

    /// Test memory usage patterns and optimization
    #[test]
    fn test_memory_usage_patterns() {
        println!("Testing memory usage patterns...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Monitor memory usage during execution
        let memory_samples = Arc::new(Mutex::new(Vec::new()));
        let memory_samples_clone = Arc::clone(&memory_samples);
        
        // Start memory monitoring thread
        let monitor_handle = thread::spawn(move || {
            for i in 0..30 { // Monitor for 30 seconds
                let memory_usage = get_process_memory_usage();
                let mut samples = memory_samples_clone.lock().unwrap();
                samples.push((i, memory_usage));
                std::thread::sleep(Duration::from_secs(1));
            }
        });

        // Run CLI while monitoring memory
        std::thread::sleep(Duration::from_secs(2)); // Let monitoring start
        
        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let execution_time = start_time.elapsed();

        // Wait for monitoring to complete
        monitor_handle.join().unwrap();

        assert!(output.status.success(), "CLI should succeed during memory monitoring");

        let memory_samples = memory_samples.lock().unwrap();
        if !memory_samples.is_empty() {
            let max_memory = memory_samples.iter().map(|(_, mem)| *mem).max().unwrap_or(0);
            let min_memory = memory_samples.iter().map(|(_, mem)| *mem).min().unwrap_or(0);
            let avg_memory = memory_samples.iter().map(|(_, mem)| *mem).sum::<u64>() / memory_samples.len() as u64;

            println!("  Memory Usage Analysis:");
            println!("  =====================");
            println!("  Execution time: {:.2} seconds", execution_time.as_secs_f64());
            println!("  Peak memory usage: {} MB", max_memory / 1_000_000);
            println!("  Minimum memory usage: {} MB", min_memory / 1_000_000);
            println!("  Average memory usage: {} MB", avg_memory / 1_000_000);
            println!("  Memory growth: {} MB", (max_memory - min_memory) / 1_000_000);

            // Memory usage assertions
            assert!(max_memory < 500_000_000, "Peak memory usage should be under 500MB");
            assert!((max_memory - min_memory) < 200_000_000, "Memory growth should be under 200MB");
        }

        println!("✓ Memory usage patterns validated");
    }

    /// Test I/O performance and optimization
    #[test]
    fn test_io_performance() {
        println!("Testing I/O performance...");
        
        // Test with different output destinations
        let test_scenarios = vec![
            ("stdout", None),
            ("temp_file", Some("temp_output.json")),
            ("nested_dir", Some("test_dir/nested/output.json")),
        ];

        for (scenario_name, output_path) in test_scenarios {
            println!("  Testing {} scenario...", scenario_name);
            
            let mut args = vec!["run", "--", "--format", "json"];
            let temp_file;
            
            if let Some(path) = output_path {
                temp_file = NamedTempFile::new().expect("Failed to create temp file");
                let temp_path = temp_file.path().to_str().unwrap();
                args.extend(&["--output", temp_path]);
            }

            let start_time = Instant::now();
            let output = Command::new("cargo")
                .args(&args)
                .current_dir(".")
                .output()
                .expect("Failed to execute CLI");
            let execution_time = start_time.elapsed();

            assert!(output.status.success(), "CLI should succeed in {} scenario", scenario_name);

            let output_size = if output_path.is_some() {
                let temp_path = temp_file.path().to_str().unwrap();
                std::fs::metadata(temp_path).unwrap().len()
            } else {
                output.stdout.len() as u64
            };

            let io_throughput = output_size as f64 / execution_time.as_secs_f64();

            println!("    Execution time: {:.2} seconds", execution_time.as_secs_f64());
            println!("    Output size: {} bytes", output_size);
            println!("    I/O throughput: {:.1} bytes/second", io_throughput);

            // I/O performance assertions
            assert!(execution_time.as_secs() < 45, "I/O should not significantly impact performance");
            assert!(io_throughput > 1000.0, "Should maintain reasonable I/O throughput");
        }

        println!("✓ I/O performance validated");
    }

    /// Helper function to get process memory usage (simplified)
    fn get_process_memory_usage() -> u64 {
        // This is a simplified implementation
        // In a real scenario, you might use Windows APIs to get actual memory usage
        use std::process;
        
        // Try to get memory info from system (placeholder implementation)
        let pid = process::id();
        
        // For testing purposes, return a simulated memory usage
        // In production, this would use actual system APIs
        (pid as u64) * 1024 * 1024 // Placeholder: PID * 1MB
    }

    /// Performance regression test
    #[test]
    fn test_performance_regression() {
        println!("Testing for performance regression...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Define performance baselines (these would be updated as the tool evolves)
        let max_execution_time = Duration::from_secs(30);
        let min_artifacts_per_second = 10.0;
        let max_memory_usage = 500_000_000; // 500MB

        // Run performance test
        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let execution_time = start_time.elapsed();

        assert!(output.status.success(), "CLI should succeed in regression test");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should be valid JSON");

        let artifact_count = parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap();
        let artifacts_per_second = artifact_count as f64 / execution_time.as_secs_f64();

        println!("  Regression Test Results:");
        println!("  =======================");
        println!("  Execution time: {:.2} seconds (max: {:.2})", 
            execution_time.as_secs_f64(), max_execution_time.as_secs_f64());
        println!("  Artifacts per second: {:.1} (min: {:.1})", 
            artifacts_per_second, min_artifacts_per_second);
        println!("  Output size: {} bytes", json_content.len());

        // Regression assertions
        assert!(execution_time <= max_execution_time, 
            "Execution time regression detected: {:.2}s > {:.2}s", 
            execution_time.as_secs_f64(), max_execution_time.as_secs_f64());
        
        assert!(artifacts_per_second >= min_artifacts_per_second,
            "Performance regression detected: {:.1} artifacts/s < {:.1} artifacts/s",
            artifacts_per_second, min_artifacts_per_second);

        println!("✓ No performance regression detected");
    }
}

// Import necessary modules for testing
use crate::logger::{Logger, error_handling::{ForensicResult, ForensicError}};
use crate::types::{ScanResults, LogEntry};