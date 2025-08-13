#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::logger::Logger;
    use crate::types::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_complete_cli_workflow() {
        let logger = Logger::new(false);
        
        // Create temporary output file
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_output.json");
        
        let args = Args {
            format: "json".to_string(),
            output: Some(output_path.to_string_lossy().to_string()),
            password: None,
            verbose: false,
        };
        
        // Run the complete collection workflow
        let result = run_collection(&args, &logger);
        
        // Should complete successfully
        assert!(result.is_ok(), "Collection should complete successfully: {:?}", result);
        
        // Output file should exist
        assert!(output_path.exists(), "Output file should be created");
        
        // Output should be valid JSON
        let json_content = fs::read_to_string(&output_path).unwrap();
        let parsed: ScanResults = serde_json::from_str(&json_content).unwrap();
        
        // Validate the structure
        assert!(parsed.validate().is_ok());
        assert!(!parsed.scan_metadata.scan_id.is_empty());
        assert!(!parsed.scan_metadata.hostname.is_empty());
        assert!(parsed.scan_metadata.scan_duration_ms > 0);
        
        // Should have some log entries
        assert!(!parsed.collection_log.is_empty());
        
        // Should have collected some artifacts (at least current process)
        assert!(parsed.artifacts.total_artifact_count() > 0);
    }

    #[test]
    fn test_json_output_to_stdout() {
        let logger = Logger::new(false);
        
        let args = Args {
            format: "json".to_string(),
            output: None, // Output to stdout
            password: None,
            verbose: false,
        };
        
        // This test would normally capture stdout, but for simplicity
        // we'll just test that the function completes without error
        let result = run_collection(&args, &logger);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verbose_logging() {
        let logger = Logger::new(true);
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("verbose_output.json");
        
        let args = Args {
            format: "json".to_string(),
            output: Some(output_path.to_string_lossy().to_string()),
            password: None,
            verbose: true,
        };
        
        let result = run_collection(&args, &logger);
        assert!(result.is_ok());
        
        // Should have generated log entries
        let summary = logger.get_summary();
        assert!(summary.total_count > 0);
        assert!(summary.info_count > 0);
    }

    #[test]
    fn test_scan_metadata_generation() {
        let hostname = "TEST-HOST".to_string();
        let os_version = "Windows 10".to_string();
        let scan_results = ScanResults::new(hostname.clone(), os_version.clone());
        
        // Check metadata is properly initialized
        assert_eq!(scan_results.scan_metadata.hostname, hostname);
        assert_eq!(scan_results.scan_metadata.os_version, os_version);
        assert!(!scan_results.scan_metadata.scan_id.is_empty());
        assert!(!scan_results.scan_metadata.scan_start_utc.is_empty());
        assert_eq!(scan_results.scan_metadata.cli_version, env!("CARGO_PKG_VERSION"));
        
        // UUID should be valid
        assert!(uuid::Uuid::parse_str(&scan_results.scan_metadata.scan_id).is_ok());
        
        // Timestamp should be valid ISO 8601
        assert!(chrono::DateTime::parse_from_rfc3339(&scan_results.scan_metadata.scan_start_utc).is_ok());
    }

    #[test]
    fn test_artifact_collection_integration() {
        let logger = Logger::new(false);
        
        // Test individual collection functions work together
        let (system_info, _) = crate::system_info::collect_system_info();
        let (processes, _) = crate::processes::collect_processes();
        let (connections, _) = crate::network::collect_network_connections();
        let (mechanisms, _) = crate::persistence::collect_persistence_mechanisms();
        let (event_logs, _) = crate::event_logs::collect_event_logs();
        
        // Create integrated scan results
        let mut scan_results = ScanResults::new("TEST".to_string(), "Windows".to_string());
        scan_results.artifacts.system_info = system_info;
        scan_results.artifacts.running_processes = processes;
        scan_results.artifacts.network_connections = connections;
        scan_results.artifacts.persistence_mechanisms = mechanisms;
        scan_results.artifacts.event_logs = event_logs;
        
        // Should validate successfully
        assert!(scan_results.validate().is_ok());
        
        // Should serialize to JSON successfully
        let json = serde_json::to_string_pretty(&scan_results);
        assert!(json.is_ok());
        
        // Should deserialize back successfully
        let deserialized: Result<ScanResults, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_error_handling_integration() {
        let logger = Logger::new(false);
        
        // Test with invalid output path (should fail gracefully)
        let args = Args {
            format: "json".to_string(),
            output: Some("/invalid/path/that/does/not/exist/output.json".to_string()),
            password: None,
            verbose: false,
        };
        
        let result = run_collection(&args, &logger);
        
        // Should fail with appropriate error
        assert!(result.is_err());
        
        if let Err(error) = result {
            // Should be a system API error
            assert_eq!(error.kind, crate::logger::error_handling::ErrorKind::SystemApiError);
            assert!(error.message.contains("Failed to write to file"));
        }
    }

    #[test]
    fn test_command_line_argument_parsing() {
        use clap::Parser;
        
        // Test default arguments
        let args = Args::parse_from(&["triageir-cli"]);
        assert_eq!(args.format, "json");
        assert_eq!(args.output, None);
        assert_eq!(args.password, None);
        assert_eq!(args.verbose, false);
        
        // Test with all arguments
        let args = Args::parse_from(&[
            "triageir-cli",
            "--format", "json",
            "--output", "test.json",
            "--password", "secret",
            "--verbose"
        ]);
        assert_eq!(args.format, "json");
        assert_eq!(args.output, Some("test.json".to_string()));
        assert_eq!(args.password, Some("secret".to_string()));
        assert_eq!(args.verbose, true);
    }

    #[test]
    fn test_json_schema_compliance() {
        let logger = Logger::new(false);
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("schema_test.json");
        
        let args = Args {
            format: "json".to_string(),
            output: Some(output_path.to_string_lossy().to_string()),
            password: None,
            verbose: false,
        };
        
        let result = run_collection(&args, &logger);
        assert!(result.is_ok());
        
        // Read and parse the JSON output
        let json_content = fs::read_to_string(&output_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_content).unwrap();
        
        // Verify required top-level fields exist
        assert!(parsed.get("scan_metadata").is_some());
        assert!(parsed.get("artifacts").is_some());
        assert!(parsed.get("collection_log").is_some());
        
        // Verify scan_metadata structure
        let metadata = parsed.get("scan_metadata").unwrap();
        assert!(metadata.get("scan_id").is_some());
        assert!(metadata.get("scan_start_utc").is_some());
        assert!(metadata.get("scan_duration_ms").is_some());
        assert!(metadata.get("hostname").is_some());
        assert!(metadata.get("os_version").is_some());
        assert!(metadata.get("cli_version").is_some());
        
        // Verify artifacts structure
        let artifacts = parsed.get("artifacts").unwrap();
        assert!(artifacts.get("system_info").is_some());
        assert!(artifacts.get("running_processes").is_some());
        assert!(artifacts.get("network_connections").is_some());
        assert!(artifacts.get("persistence_mechanisms").is_some());
        assert!(artifacts.get("event_logs").is_some());
        
        // Verify collection_log is an array
        let collection_log = parsed.get("collection_log").unwrap();
        assert!(collection_log.is_array());
    }

    #[test]
    fn test_performance_characteristics() {
        use std::time::Instant;
        
        let logger = Logger::new(false);
        
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("perf_test.json");
        
        let args = Args {
            format: "json".to_string(),
            output: Some(output_path.to_string_lossy().to_string()),
            password: None,
            verbose: false,
        };
        
        let start_time = Instant::now();
        let result = run_collection(&args, &logger);
        let duration = start_time.elapsed();
        
        assert!(result.is_ok());
        
        // Collection should complete within reasonable time (30 seconds)
        assert!(duration.as_secs() < 30, "Collection took too long: {:?}", duration);
        
        // Output file should not be excessively large (< 10MB for basic scan)
        let file_size = fs::metadata(&output_path).unwrap().len();
        assert!(file_size < 10 * 1024 * 1024, "Output file too large: {} bytes", file_size);
    }
}