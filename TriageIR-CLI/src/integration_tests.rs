#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::process::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_with_logging() {
        // Create a temporary output file
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_path = temp_file.path().to_str().unwrap();

        // Run the CLI with verbose logging
        let output = Command::new("cargo")
            .args(&["run", "--", "--verbose", "--output", temp_path])
            .current_dir(".")
            .output()
            .expect("Failed to execute CLI");

        // Verify the command executed successfully
        if !output.status.success() {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            panic!("CLI execution failed");
        }

        // Verify output file was created
        assert!(std::path::Path::new(temp_path).exists(), "Output file should exist");

        // Read and parse the JSON output
        let json_content = std::fs::read_to_string(temp_path)
            .expect("Failed to read output file");
        
        let parsed: serde_json::Value = serde_json::from_str(&json_content)
            .expect("Output should be valid JSON");

        // Verify required fields exist
        assert!(parsed["scan_metadata"].is_object(), "scan_metadata should exist");
        assert!(parsed["artifacts"].is_object(), "artifacts should exist");
        assert!(parsed["collection_log"].is_array(), "collection_log should exist");

        // Verify collection log contains entries
        let collection_log = parsed["collection_log"].as_array().unwrap();
        assert!(!collection_log.is_empty(), "Collection log should not be empty");

        // Verify log entries have required fields
        for log_entry in collection_log {
            assert!(log_entry["timestamp"].is_string(), "Log entry should have timestamp");
            assert!(log_entry["level"].is_string(), "Log entry should have level");
            assert!(log_entry["message"].is_string(), "Log entry should have message");
        }

        // Verify scan metadata includes collection summary
        let scan_metadata = &parsed["scan_metadata"];
        assert!(scan_metadata["collection_summary"].is_object(), "Should include collection summary");
        
        let collection_summary = &scan_metadata["collection_summary"];
        assert!(collection_summary["total_logs"].is_number(), "Should include total log count");
        assert!(collection_summary["error_count"].is_number(), "Should include error count");
        assert!(collection_summary["warning_count"].is_number(), "Should include warning count");
        assert!(collection_summary["success_rate"].is_number(), "Should include success rate");
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