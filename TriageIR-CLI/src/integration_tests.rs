#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::process::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_complete_workflow_with_file_output() {
        // Create a temporary output file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Run the CLI with verbose logging and file output
        let output = Command::new("cargo")
            .args(&["run", "--", "--verbose", "--output", temp_path, "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        // Verify the command executed successfully
        if !output.status.success() {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            panic!("CLI execution failed with exit code: {}", output.status);
        }

        // Verify output file was created
        assert!(std::path::Path::new(temp_path).exists(), "Output file should exist");

        // Read and parse the JSON output
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        
        let parsed: serde_json::Value = serde_json::from_str(&json_content)
            .expect("Output should be valid JSON");

        // Verify required top-level fields exist
        assert!(parsed["scan_metadata"].is_object(), "scan_metadata should exist");
        assert!(parsed["artifacts"].is_object(), "artifacts should exist");
        assert!(parsed["collection_log"].is_array(), "collection_log should exist");

        // Verify scan metadata structure
        let scan_metadata = &parsed["scan_metadata"];
        assert!(scan_metadata["scan_id"].is_string(), "Should have scan_id");
        assert!(scan_metadata["scan_start_utc"].is_string(), "Should have scan_start_utc");
        assert!(scan_metadata["scan_duration_ms"].is_number(), "Should have scan_duration_ms");
        assert!(scan_metadata["hostname"].is_string(), "Should have hostname");
        assert!(scan_metadata["os_version"].is_string(), "Should have os_version");
        assert!(scan_metadata["cli_version"].is_string(), "Should have cli_version");
        assert!(scan_metadata["total_artifacts"].is_number(), "Should have total_artifacts");
        
        // Verify collection summary exists
        assert!(scan_metadata["collection_summary"].is_object(), "Should include collection summary");
        let collection_summary = &scan_metadata["collection_summary"];
        assert!(collection_summary["total_logs"].is_number(), "Should include total log count");
        assert!(collection_summary["error_count"].is_number(), "Should include error count");
        assert!(collection_summary["warning_count"].is_number(), "Should include warning count");
        assert!(collection_summary["success_rate"].is_number(), "Should include success rate");

        // Verify artifacts structure
        let artifacts = &parsed["artifacts"];
        assert!(artifacts["system_info"].is_object(), "Should have system_info");
        assert!(artifacts["running_processes"].is_array(), "Should have running_processes");
        assert!(artifacts["network_connections"].is_array(), "Should have network_connections");
        assert!(artifacts["persistence_mechanisms"].is_array(), "Should have persistence_mechanisms");
        assert!(artifacts["event_logs"].is_object(), "Should have event_logs");
        assert!(artifacts["execution_evidence"].is_object(), "Should have execution_evidence");

        // Verify execution evidence structure
        let execution_evidence = &artifacts["execution_evidence"];
        assert!(execution_evidence["prefetch_files"].is_array(), "Should have prefetch_files");
        assert!(execution_evidence["shimcache_entries"].is_array(), "Should have shimcache_entries");

        // Verify event logs structure
        let event_logs = &artifacts["event_logs"];
        assert!(event_logs["security"].is_array(), "Should have security events");
        assert!(event_logs["system"].is_array(), "Should have system events");
        assert!(event_logs["application"].is_array(), "Should have application events");

        // Verify collection log contains entries
        let collection_log = parsed["collection_log"].as_array().unwrap();
        assert!(!collection_log.is_empty(), "Collection log should not be empty");

        // Verify log entries have required fields
        for log_entry in collection_log {
            assert!(log_entry["timestamp"].is_string(), "Log entry should have timestamp");
            assert!(log_entry["level"].is_string(), "Log entry should have level");
            assert!(log_entry["message"].is_string(), "Log entry should have message");
        }

        // Verify that we collected some artifacts
        let total_artifacts = scan_metadata["total_artifacts"].as_u64().unwrap();
        assert!(total_artifacts > 0, "Should have collected some artifacts");
    }

    #[test]
    fn test_cli_stdout_output() {
        // Run the CLI with stdout output (no --output flag)
        let output = Command::new("cargo")
            .args(&["run", "--", "--format", "json"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        // Verify the command executed successfully
        if !output.status.success() {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            panic!("CLI execution failed with exit code: {}", output.status);
        }

        // Parse the JSON output from stdout
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let parsed: serde_json::Value = serde_json::from_str(&stdout_str)
            .expect("Stdout should contain valid JSON");

        // Verify basic structure
        assert!(parsed["scan_metadata"].is_object(), "scan_metadata should exist");
        assert!(parsed["artifacts"].is_object(), "artifacts should exist");
        assert!(parsed["collection_log"].is_array(), "collection_log should exist");
    }

    #[test]
    fn test_cli_invalid_format() {
        // Run the CLI with invalid format
        let output = Command::new("cargo")
            .args(&["run", "--", "--format", "xml"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        // Should fail with exit code 2 (clap's default for invalid arguments)
        assert!(!output.status.success(), "Should fail with invalid format");
        assert_eq!(output.status.code(), Some(2), "Should exit with code 2");
        
        // Verify error message mentions invalid value
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("invalid value 'xml'"), "Should show invalid value error");
    }

    #[test]
    fn test_error_handling_in_collection() {
        // This test verifies that the application handles errors gracefully
        // and continues collection even when some operations fail
        
        let logger = Logger::new(false);
        
        // Simulate various error conditions that might occur during collection
        let test_operations = vec![
            ("system_info", simulate_system_info_collection(&logger)),
            ("process_enum", simulate_process_enumeration(&logger)),
            ("network_scan", simulate_network_scan(&logger)),
        ];
        
        let mut successful_ops = 0;
        for (op_name, result) in test_operations {
            if handle_error_gracefully(result, &logger, op_name).is_some() {
                successful_ops += 1;
            }
        }
        
        // Verify that at least some operations succeeded
        assert!(successful_ops > 0, "At least some operations should succeed");
        
        // Verify logging captured the operations
        let summary = logger.get_summary();
        assert!(summary.total_count > 0, "Should have logged operations");
    }

    #[test]
    fn test_cli_command_line_arguments() {
        // Test various command line argument combinations
        
        // Test help flag
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        
        assert!(output.status.success(), "Help should succeed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("forensically sound command-line tool"), "Should show description");
        assert!(stdout.contains("--verbose"), "Should show verbose option");
        assert!(stdout.contains("--output"), "Should show output option");
        assert!(stdout.contains("--format"), "Should show format option");

        // Test version flag
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");
        
        assert!(output.status.success(), "Version should succeed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("triageir-cli"), "Should show program name");
    }

    #[test]
    fn test_scan_metadata_generation() {
        // Test that scan metadata is properly generated
        let hostname = "TEST-HOST".to_string();
        let os_version = "Windows 10".to_string();
        let mut scan_results = ScanResults::new(hostname.clone(), os_version.clone());
        
        // Verify initial metadata
        assert_eq!(scan_results.scan_metadata.hostname, hostname);
        assert_eq!(scan_results.scan_metadata.os_version, os_version);
        assert!(!scan_results.scan_metadata.scan_id.is_empty());
        assert!(!scan_results.scan_metadata.scan_start_utc.is_empty());
        assert_eq!(scan_results.scan_metadata.scan_duration_ms, 0);
        
        // Test UUID format
        assert!(uuid::Uuid::parse_str(&scan_results.scan_metadata.scan_id).is_ok(), 
                "Scan ID should be valid UUID");
        
        // Test timestamp format
        assert!(chrono::DateTime::parse_from_rfc3339(&scan_results.scan_metadata.scan_start_utc).is_ok(),
                "Scan start time should be valid ISO 8601");
        
        // Test finalization
        std::thread::sleep(std::time::Duration::from_millis(10)); // Small delay
        scan_results.finalize_scan();
        assert!(scan_results.scan_metadata.scan_duration_ms > 0, "Duration should be updated");
    }

    #[test]
    fn test_json_schema_compliance() {
        // Test that the JSON output complies with the expected schema
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Run CLI to generate output
        let output = Command::new("cargo")
            .args(&["run", "--", "--output", temp_path])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        assert!(output.status.success(), "CLI should succeed");

        // Read and parse JSON
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        let parsed: serde_json::Value = serde_json::from_str(&json_content)
            .expect("Should be valid JSON");

        // Test schema compliance for all major sections
        validate_scan_metadata_schema(&parsed["scan_metadata"]);
        validate_artifacts_schema(&parsed["artifacts"]);
        validate_collection_log_schema(&parsed["collection_log"]);
    }

    // Helper functions for testing error scenarios
    fn simulate_system_info_collection(logger: &Logger) -> ForensicResult<String> {
        logger.info("Simulating system info collection");
        Ok("System info collected".to_string())
    }

    fn simulate_process_enumeration(logger: &Logger) -> ForensicResult<String> {
        logger.warn("Simulating process enumeration with warnings");
        // Simulate a non-fatal error
        Err(ForensicError::access_denied("Some processes inaccessible"))
    }

    fn simulate_network_scan(logger: &Logger) -> ForensicResult<String> {
        logger.info("Simulating network scan");
        Ok("Network connections collected".to_string())
    }

    // Schema validation helper functions
    fn validate_scan_metadata_schema(metadata: &serde_json::Value) {
        assert!(metadata.is_object(), "Metadata should be object");
        assert!(metadata["scan_id"].is_string(), "Should have scan_id");
        assert!(metadata["scan_start_utc"].is_string(), "Should have scan_start_utc");
        assert!(metadata["scan_duration_ms"].is_number(), "Should have scan_duration_ms");
        assert!(metadata["hostname"].is_string(), "Should have hostname");
        assert!(metadata["os_version"].is_string(), "Should have os_version");
        assert!(metadata["cli_version"].is_string(), "Should have cli_version");
        assert!(metadata["total_artifacts"].is_number(), "Should have total_artifacts");
        assert!(metadata["collection_summary"].is_object(), "Should have collection_summary");
    }

    fn validate_artifacts_schema(artifacts: &serde_json::Value) {
        assert!(artifacts.is_object(), "Artifacts should be object");
        assert!(artifacts["system_info"].is_object(), "Should have system_info");
        assert!(artifacts["running_processes"].is_array(), "Should have running_processes");
        assert!(artifacts["network_connections"].is_array(), "Should have network_connections");
        assert!(artifacts["persistence_mechanisms"].is_array(), "Should have persistence_mechanisms");
        assert!(artifacts["event_logs"].is_object(), "Should have event_logs");
        assert!(artifacts["execution_evidence"].is_object(), "Should have execution_evidence");
        
        // Validate execution evidence structure
        let exec_evidence = &artifacts["execution_evidence"];
        assert!(exec_evidence["prefetch_files"].is_array(), "Should have prefetch_files");
        assert!(exec_evidence["shimcache_entries"].is_array(), "Should have shimcache_entries");
        
        // Validate event logs structure
        let event_logs = &artifacts["event_logs"];
        assert!(event_logs["security"].is_array(), "Should have security events");
        assert!(event_logs["system"].is_array(), "Should have system events");
        assert!(event_logs["application"].is_array(), "Should have application events");
    }

    fn validate_collection_log_schema(collection_log: &serde_json::Value) {
        assert!(collection_log.is_array(), "Collection log should be array");
        
        if let Some(logs) = collection_log.as_array() {
            for log_entry in logs {
                assert!(log_entry["timestamp"].is_string(), "Log should have timestamp");
                assert!(log_entry["level"].is_string(), "Log should have level");
                assert!(log_entry["message"].is_string(), "Log should have message");
            }
        }
    }

    #[test]
    fn test_json_output_format() {
        // Test that the JSON output conforms to the expected schema
        let logger = Logger::new(false);
        
        // Create a minimal scan result
        let mut scan_results = ScanResults::new("TEST-HOST".to_string(), "Windows 10".to_string());
        scan_results.add_log(LogEntry::info("Test log entry"));
        scan_results.finalize_scan();
        
        // Serialize to JSON
        let json_output = serde_json::to_string_pretty(&scan_results)
            .expect("Should serialize successfully");
        
        // Parse back to verify structure
        let parsed: serde_json::Value = serde_json::from_str(&json_output)
            .expect("Should parse back successfully");
        
        // Verify required top-level fields
        assert!(parsed["scan_metadata"].is_object());
        assert!(parsed["artifacts"].is_object());
        assert!(parsed["collection_log"].is_array());
        
        // Verify scan metadata fields
        let metadata = &parsed["scan_metadata"];
        assert!(metadata["scan_id"].is_string());
        assert!(metadata["scan_start_utc"].is_string());
        assert!(metadata["scan_duration_ms"].is_number());
        assert!(metadata["hostname"].is_string());
        assert!(metadata["os_version"].is_string());
        assert!(metadata["cli_version"].is_string());
    }

    #[test]
    fn test_verbose_mode_behavior() {
        // Test that verbose mode affects logging behavior appropriately
        let verbose_logger = Logger::new(true);
        let quiet_logger = Logger::new(false);
        
        verbose_logger.info("Verbose test message");
        quiet_logger.info("Quiet test message");
        
        // Both should log the message internally
        assert_eq!(verbose_logger.get_entries().len(), 1);
        assert_eq!(quiet_logger.get_entries().len(), 1);
        
        // The difference is in stderr output, which we can't easily test here
        // but the verbose flag should be properly passed through the system
    }
}

use crate::logger::{Logger, error_handling::{ForensicResult, ForensicError, handle_error_gracefully}};
use crate::types::{ScanResults, LogEntry};