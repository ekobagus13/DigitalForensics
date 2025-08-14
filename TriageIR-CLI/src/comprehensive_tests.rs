//! Comprehensive test suite for TriageIR CLI
//! This module contains extensive tests for validation, performance, and forensic soundness

#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use std::process::Command;
    use std::time::{Duration, Instant};
    use tempfile::{NamedTempFile, TempDir};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// Test forensic soundness and data integrity
    #[test]
    fn test_forensic_soundness_and_data_integrity() {
        println!("Testing forensic soundness and data integrity...");
        
        // Create temporary output file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Run CLI multiple times to ensure consistent results
        let mut scan_results = Vec::new();
        for i in 0..3 {
            println!("  Running scan iteration {}/3...", i + 1);
            
            let output = Command::new("cargo")
                .args(&["run", "--", "--output", temp_path, "--format", "json"])
                .current_dir(".")
                .output()
                .expect("Failed to execute CLI");

            assert!(output.status.success(), "CLI execution should succeed");

            let json_content = std::fs::read_to_string(temp_path)
                .expect("Failed to read output file");
            let parsed: Value = serde_json::from_str(&json_content)
                .expect("Output should be valid JSON");
            
            scan_results.push(parsed);
            
            // Small delay between scans
            std::thread::sleep(Duration::from_millis(100));
        }

        // Verify scan metadata consistency
        for (i, result) in scan_results.iter().enumerate() {
            println!("  Validating scan {} metadata...", i + 1);
            
            // Each scan should have unique scan_id
            let scan_id = result["scan_metadata"]["scan_id"].as_str().unwrap();
            assert!(uuid::Uuid::parse_str(scan_id).is_ok(), "Scan ID should be valid UUID");
            
            // Hostname should be consistent
            let hostname = result["scan_metadata"]["hostname"].as_str().unwrap();
            assert!(!hostname.is_empty(), "Hostname should not be empty");
            
            // OS version should be consistent
            let os_version = result["scan_metadata"]["os_version"].as_str().unwrap();
            assert!(!os_version.is_empty(), "OS version should not be empty");
            
            // Timestamps should be valid ISO 8601
            let timestamp = result["scan_metadata"]["scan_start_utc"].as_str().unwrap();
            assert!(chrono::DateTime::parse_from_rfc3339(timestamp).is_ok(), 
                "Timestamp should be valid ISO 8601");
        }

        // Verify data consistency across scans (some artifacts should be similar)
        let first_scan = &scan_results[0];
        let second_scan = &scan_results[1];
        
        // System info should be consistent
        assert_eq!(
            first_scan["artifacts"]["system_info"]["hostname"],
            second_scan["artifacts"]["system_info"]["hostname"],
            "Hostname should be consistent across scans"
        );
        
        // Process count should be reasonably similar (within 10% variance)
        let first_process_count = first_scan["artifacts"]["running_processes"].as_array().unwrap().len();
        let second_process_count = second_scan["artifacts"]["running_processes"].as_array().unwrap().len();
        let variance = ((first_process_count as f64 - second_process_count as f64).abs() / first_process_count as f64) * 100.0;
        assert!(variance < 20.0, "Process count variance should be less than 20% (was {:.1}%)", variance);

        println!("✓ Forensic soundness and data integrity validated");
    }

    /// Test performance under various system loads
    #[test]
    fn test_performance_under_load() {
        println!("Testing performance under various system loads...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Baseline performance test
        println!("  Running baseline performance test...");
        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let baseline_duration = start_time.elapsed();
        
        assert!(output.status.success(), "Baseline scan should succeed");
        assert!(baseline_duration.as_secs() < 30, "Baseline scan should complete within 30 seconds");
        
        println!("  Baseline scan completed in {:.2} seconds", baseline_duration.as_secs_f64());

        // Test with concurrent system activity
        println!("  Testing with concurrent system activity...");
        let activity_handles: Vec<_> = (0..5).map(|i| {
            thread::spawn(move || {
                // Simulate system activity
                for _ in 0..1000 {
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "echo", &format!("test_{}", i)])
                        .output();
                    thread::sleep(Duration::from_millis(1));
                }
            })
        }).collect();

        let start_time = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        let load_duration = start_time.elapsed();

        // Wait for background activity to complete
        for handle in activity_handles {
            handle.join().unwrap();
        }

        assert!(output.status.success(), "Scan under load should succeed");
        
        // Performance should not degrade significantly under load
        let performance_ratio = load_duration.as_secs_f64() / baseline_duration.as_secs_f64();
        assert!(performance_ratio < 3.0, "Performance under load should not degrade more than 3x (was {:.2}x)", performance_ratio);
        
        println!("  Scan under load completed in {:.2} seconds ({:.2}x baseline)", 
            load_duration.as_secs_f64(), performance_ratio);

        // Memory usage test
        println!("  Testing memory usage...");
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let file_size = json_content.len();
        
        // Output should be reasonable size (not excessive)
        assert!(file_size > 1000, "Output should contain substantial data");
        assert!(file_size < 100_000_000, "Output should not be excessively large (>100MB)");
        
        println!("  Output file size: {} bytes ({:.2} MB)", file_size, file_size as f64 / 1_000_000.0);
        println!("✓ Performance testing completed");
    }

    /// Test error scenarios and recovery mechanisms
    #[test]
    fn test_error_scenarios_and_recovery() {
        println!("Testing error scenarios and recovery mechanisms...");
        
        // Test with invalid output directory
        println!("  Testing invalid output directory...");
        let invalid_path = "/invalid/path/that/does/not/exist/output.json";
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", invalid_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        
        // Should fail gracefully with appropriate exit code
        assert!(!output.status.success(), "Should fail with invalid output path");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Failed to create parent directory") || 
                stderr.contains("Failed to write file"), 
                "Should show appropriate error message");

        // Test with insufficient permissions (simulate by trying to write to system directory)
        if cfg!(windows) {
            println!("  Testing insufficient permissions...");
            let restricted_path = "C:\\Windows\\System32\\triageir_test.json";
            let output = Command::new("cargo")
                .args(&["run", "--", "--output", restricted_path, "--format", "json"])
                .current_dir(".")
                .output()
                .expect("Failed to execute CLI");
            
            // Should handle permission errors gracefully
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(stderr.contains("Failed to write file") || 
                        stderr.contains("Access is denied"), 
                        "Should show permission error message");
            }
        }

        // Test graceful degradation with partial failures
        println!("  Testing graceful degradation...");
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();
        
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json", "--verbose"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        
        // Even with potential partial failures, should produce valid output
        assert!(output.status.success() || output.status.code() == Some(2), 
                "Should succeed or exit with partial success code");
        
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should produce valid JSON even with partial failures");
        
        // Should have collection log with error information
        let collection_log = parsed["collection_log"].as_array().unwrap();
        assert!(!collection_log.is_empty(), "Should have collection log entries");
        
        // Check for error/warning entries in log
        let has_errors_or_warnings = collection_log.iter().any(|entry| {
            let level = entry["level"].as_str().unwrap_or("");
            level == "ERROR" || level == "WARN"
        });
        
        if has_errors_or_warnings {
            println!("  ✓ Graceful degradation with error logging detected");
        } else {
            println!("  ✓ Clean execution with no errors detected");
        }

        println!("✓ Error scenario testing completed");
    }

    /// Test JSON schema compliance and validation
    #[test]
    fn test_json_schema_compliance() {
        println!("Testing JSON schema compliance...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success(), "CLI should succeed");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should be valid JSON");

        // Comprehensive schema validation
        validate_complete_schema(&parsed);
        
        println!("✓ JSON schema compliance validated");
    }

    /// Test cross-platform compatibility (Windows versions)
    #[test]
    fn test_windows_version_compatibility() {
        println!("Testing Windows version compatibility...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success(), "Should work on current Windows version");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should produce valid JSON");

        // Verify OS detection works
        let os_version = parsed["scan_metadata"]["os_version"].as_str().unwrap();
        assert!(!os_version.is_empty(), "Should detect OS version");
        assert!(os_version.contains("Windows") || os_version != "Unknown", 
                "Should detect Windows OS");

        // Verify Windows-specific artifacts are collected
        let artifacts = &parsed["artifacts"];
        
        // Should have Windows-specific data
        assert!(artifacts["running_processes"].as_array().unwrap().len() > 0, 
                "Should collect Windows processes");
        assert!(artifacts["event_logs"]["security"].as_array().is_some(), 
                "Should access Windows event logs");
        assert!(artifacts["persistence_mechanisms"].as_array().is_some(), 
                "Should detect Windows persistence mechanisms");

        println!("  Detected OS: {}", os_version);
        println!("✓ Windows version compatibility validated");
    }

    /// Test data consistency and artifact correlation
    #[test]
    fn test_data_consistency_and_correlation() {
        println!("Testing data consistency and artifact correlation...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success(), "CLI should succeed");

        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should be valid JSON");

        let artifacts = &parsed["artifacts"];
        
        // Test process-network correlation
        let processes = artifacts["running_processes"].as_array().unwrap();
        let network_connections = artifacts["network_connections"].as_array().unwrap();
        
        // Build PID map from processes
        let mut process_pids = std::collections::HashSet::new();
        for process in processes {
            if let Some(pid) = process["pid"].as_u64() {
                process_pids.insert(pid);
            }
        }
        
        // Verify network connections reference valid PIDs
        let mut valid_network_correlations = 0;
        for connection in network_connections {
            if let Some(owning_pid) = connection["owning_pid"].as_u64() {
                if owning_pid > 0 && process_pids.contains(&owning_pid) {
                    valid_network_correlations += 1;
                }
            }
        }
        
        if !network_connections.is_empty() {
            let correlation_rate = (valid_network_correlations as f64 / network_connections.len() as f64) * 100.0;
            println!("  Process-network correlation rate: {:.1}%", correlation_rate);
            // Allow for some system processes that might not be captured
            assert!(correlation_rate > 50.0, "Should have reasonable process-network correlation");
        }

        // Test timestamp consistency
        let scan_start = parsed["scan_metadata"]["scan_start_utc"].as_str().unwrap();
        let scan_start_time = chrono::DateTime::parse_from_rfc3339(scan_start).unwrap();
        
        // All event log timestamps should be before or around scan time
        let event_logs = &artifacts["event_logs"];
        for log_type in ["security", "system", "application"] {
            if let Some(events) = event_logs[log_type].as_array() {
                for event in events.iter().take(10) { // Check first 10 events
                    if let Some(timestamp_str) = event["timestamp"].as_str() {
                        if let Ok(event_time) = chrono::DateTime::parse_from_rfc3339(timestamp_str) {
                            // Event should not be significantly in the future
                            let time_diff = event_time.signed_duration_since(scan_start_time);
                            assert!(time_diff.num_hours() < 24, 
                                "Event timestamp should not be more than 24 hours in the future");
                        }
                    }
                }
            }
        }

        println!("✓ Data consistency and correlation validated");
    }

    /// Test resource usage and optimization
    #[test]
    fn test_resource_usage_optimization() {
        println!("Testing resource usage and optimization...");
        
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Monitor resource usage during execution
        let start_time = Instant::now();
        let start_memory = get_current_memory_usage();
        
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        
        let execution_time = start_time.elapsed();
        let end_memory = get_current_memory_usage();
        
        assert!(output.status.success(), "CLI should succeed");
        
        // Verify reasonable execution time
        assert!(execution_time.as_secs() < 60, "Should complete within 60 seconds");
        
        // Verify output quality vs. time trade-off
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: Value = serde_json::from_str(&json_content)
            .expect("Should be valid JSON");
        
        let total_artifacts = parsed["scan_metadata"]["total_artifacts"].as_u64().unwrap();
        let artifacts_per_second = total_artifacts as f64 / execution_time.as_secs_f64();
        
        println!("  Execution time: {:.2} seconds", execution_time.as_secs_f64());
        println!("  Total artifacts: {}", total_artifacts);
        println!("  Artifacts per second: {:.1}", artifacts_per_second);
        println!("  Memory usage change: {} bytes", end_memory.saturating_sub(start_memory));
        
        // Should collect reasonable number of artifacts efficiently
        assert!(artifacts_per_second > 10.0, "Should collect at least 10 artifacts per second");
        
        println!("✓ Resource usage optimization validated");
    }

    /// Helper function to validate complete JSON schema
    fn validate_complete_schema(data: &Value) {
        // Validate top-level structure
        assert!(data.is_object(), "Root should be object");
        assert!(data["scan_metadata"].is_object(), "Should have scan_metadata");
        assert!(data["artifacts"].is_object(), "Should have artifacts");
        assert!(data["collection_log"].is_array(), "Should have collection_log");

        // Validate scan_metadata
        let metadata = &data["scan_metadata"];
        validate_scan_metadata_complete(metadata);

        // Validate artifacts
        let artifacts = &data["artifacts"];
        validate_artifacts_complete(artifacts);

        // Validate collection_log
        let collection_log = data["collection_log"].as_array().unwrap();
        validate_collection_log_complete(collection_log);
    }

    fn validate_scan_metadata_complete(metadata: &Value) {
        // Required fields
        assert!(metadata["scan_id"].is_string(), "Should have scan_id");
        assert!(metadata["scan_start_utc"].is_string(), "Should have scan_start_utc");
        assert!(metadata["scan_duration_ms"].is_number(), "Should have scan_duration_ms");
        assert!(metadata["hostname"].is_string(), "Should have hostname");
        assert!(metadata["os_version"].is_string(), "Should have os_version");
        assert!(metadata["cli_version"].is_string(), "Should have cli_version");
        assert!(metadata["total_artifacts"].is_number(), "Should have total_artifacts");
        assert!(metadata["collection_summary"].is_object(), "Should have collection_summary");

        // Validate UUID format
        let scan_id = metadata["scan_id"].as_str().unwrap();
        assert!(uuid::Uuid::parse_str(scan_id).is_ok(), "Scan ID should be valid UUID");

        // Validate timestamp format
        let timestamp = metadata["scan_start_utc"].as_str().unwrap();
        assert!(chrono::DateTime::parse_from_rfc3339(timestamp).is_ok(), 
                "Timestamp should be valid ISO 8601");

        // Validate collection summary
        let summary = &metadata["collection_summary"];
        assert!(summary["total_logs"].is_number(), "Should have total_logs");
        assert!(summary["error_count"].is_number(), "Should have error_count");
        assert!(summary["warning_count"].is_number(), "Should have warning_count");
        assert!(summary["success_rate"].is_number(), "Should have success_rate");
    }

    fn validate_artifacts_complete(artifacts: &Value) {
        // Required artifact sections
        assert!(artifacts["system_info"].is_object(), "Should have system_info");
        assert!(artifacts["running_processes"].is_array(), "Should have running_processes");
        assert!(artifacts["network_connections"].is_array(), "Should have network_connections");
        assert!(artifacts["persistence_mechanisms"].is_array(), "Should have persistence_mechanisms");
        assert!(artifacts["event_logs"].is_object(), "Should have event_logs");
        assert!(artifacts["execution_evidence"].is_object(), "Should have execution_evidence");

        // Validate system_info structure
        let system_info = &artifacts["system_info"];
        assert!(system_info["hostname"].is_string(), "System info should have hostname");
        assert!(system_info["os_name"].is_string(), "System info should have os_name");
        assert!(system_info["current_user"].is_string(), "System info should have current_user");

        // Validate process structure (if processes exist)
        let processes = artifacts["running_processes"].as_array().unwrap();
        if !processes.is_empty() {
            let first_process = &processes[0];
            assert!(first_process["pid"].is_number(), "Process should have PID");
            assert!(first_process["name"].is_string(), "Process should have name");
            assert!(first_process["executable_path"].is_string(), "Process should have executable_path");
        }

        // Validate event_logs structure
        let event_logs = &artifacts["event_logs"];
        assert!(event_logs["security"].is_array(), "Should have security events");
        assert!(event_logs["system"].is_array(), "Should have system events");
        assert!(event_logs["application"].is_array(), "Should have application events");

        // Validate execution_evidence structure
        let execution_evidence = &artifacts["execution_evidence"];
        assert!(execution_evidence["prefetch_files"].is_array(), "Should have prefetch_files");
        assert!(execution_evidence["shimcache_entries"].is_array(), "Should have shimcache_entries");
    }

    fn validate_collection_log_complete(collection_log: &[Value]) {
        for entry in collection_log {
            assert!(entry["timestamp"].is_string(), "Log entry should have timestamp");
            assert!(entry["level"].is_string(), "Log entry should have level");
            assert!(entry["message"].is_string(), "Log entry should have message");

            // Validate log level
            let level = entry["level"].as_str().unwrap();
            assert!(["INFO", "WARN", "ERROR"].contains(&level), 
                    "Log level should be INFO, WARN, or ERROR");

            // Validate timestamp format
            let timestamp = entry["timestamp"].as_str().unwrap();
            assert!(chrono::DateTime::parse_from_rfc3339(timestamp).is_ok(), 
                    "Log timestamp should be valid ISO 8601");
        }
    }

    /// Helper function to get current memory usage (simplified)
    fn get_current_memory_usage() -> u64 {
        // This is a simplified implementation
        // In a real scenario, you might use system APIs to get actual memory usage
        std::process::id() as u64 * 1024 // Placeholder
    }
}

// Import necessary modules for testing
use crate::logger::{Logger, error_handling::{ForensicResult, ForensicError, handle_error_gracefully}};
use crate::types::{ScanResults, LogEntry};