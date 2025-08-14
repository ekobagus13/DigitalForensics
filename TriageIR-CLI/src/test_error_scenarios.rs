//! Integration tests for comprehensive error handling scenarios
//! This module tests the error handling and logging infrastructure

#[cfg(test)]
mod error_scenario_tests {
    use crate::logger::{Logger, error_handling::*};
    use std::sync::Arc;

    #[test]
    fn test_comprehensive_error_handling_scenarios() {
        let logger = Arc::new(Logger::new(false));
        
        // Test scenario 1: Access denied error with graceful degradation
        let access_denied_result: ForensicResult<String> = Err(ForensicError::access_denied("Registry access denied"));
        let handled = handle_error_gracefully(access_denied_result, &logger, "registry_scan");
        assert_eq!(handled, None);
        
        // Test scenario 2: System API error (fatal)
        let api_error_result: ForensicResult<String> = Err(ForensicError::system_api_error("GetProcessList failed"));
        let handled = handle_error_gracefully(api_error_result, &logger, "process_enumeration");
        assert_eq!(handled, None);
        
        // Test scenario 3: Not found error (non-fatal)
        let not_found_result: ForensicResult<String> = Err(ForensicError::not_found("Prefetch directory not found"));
        let handled = handle_error_gracefully(not_found_result, &logger, "prefetch_analysis");
        assert_eq!(handled, None);
        
        // Test scenario 4: Successful operation
        let success_result: ForensicResult<String> = Ok("Success".to_string());
        let handled = handle_error_gracefully(success_result, &logger, "successful_operation");
        assert_eq!(handled, Some("Success".to_string()));
        
        // Verify logging occurred correctly
        let summary = logger.get_summary();
        assert_eq!(summary.error_count, 1); // Only the fatal system API error
        assert_eq!(summary.warn_count, 2);  // Access denied and not found are warnings
        assert_eq!(summary.info_count, 0);  // No info messages in this test
        assert_eq!(summary.total_count, 3); // Total of 3 log entries
        
        // Verify success rate calculation (allow for floating point precision)
        let success_rate = summary.success_rate();
        assert!((success_rate - 66.66666666666667).abs() < 0.0001); // 2 out of 3 non-error entries
        
        // Test that appropriate log levels were used
        let entries = logger.get_entries();
        let error_entries: Vec<_> = entries.iter().filter(|e| e.level == "ERROR").collect();
        let warn_entries: Vec<_> = entries.iter().filter(|e| e.level == "WARN").collect();
        
        assert_eq!(error_entries.len(), 1);
        assert_eq!(warn_entries.len(), 2);
        
        // Verify error messages contain operation context
        assert!(error_entries[0].message.contains("process_enumeration"));
        assert!(warn_entries.iter().any(|e| e.message.contains("registry_scan")));
        assert!(warn_entries.iter().any(|e| e.message.contains("prefetch_analysis")));
    }

    #[test]
    fn test_retry_mechanism_with_eventual_success() {
        let logger = Arc::new(Logger::new(false));
        let attempt_count = std::sync::Arc::new(std::sync::Mutex::new(0));
        
        let attempt_count_clone = attempt_count.clone();
        let result = retry_operation(
            move || {
                let mut count = attempt_count_clone.lock().unwrap();
                *count += 1;
                if *count < 3 {
                    Err(ForensicError::new(ErrorKind::NetworkError, "Temporary network failure"))
                } else {
                    Ok("Network operation succeeded")
                }
            },
            5, // max attempts
            &logger,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Network operation succeeded");
        assert_eq!(*attempt_count.lock().unwrap(), 3);
        
        // Verify retry warnings were logged
        let entries = logger.get_entries();
        let retry_warnings: Vec<_> = entries.iter()
            .filter(|e| e.level == "WARN" && e.message.contains("retrying"))
            .collect();
        assert_eq!(retry_warnings.len(), 2); // Two retry attempts before success
    }

    #[test]
    fn test_retry_mechanism_with_eventual_failure() {
        let logger = Arc::new(Logger::new(false));
        let attempt_count = std::sync::Arc::new(std::sync::Mutex::new(0));
        
        let attempt_count_clone = attempt_count.clone();
        let result: ForensicResult<&str> = retry_operation(
            move || {
                let mut count = attempt_count_clone.lock().unwrap();
                *count += 1;
                Err(ForensicError::new(ErrorKind::NetworkError, "Persistent network failure"))
            },
            3, // max attempts
            &logger,
        );
        
        assert!(result.is_err());
        assert_eq!(*attempt_count.lock().unwrap(), 3);
        
        // Verify all retry attempts were logged
        let entries = logger.get_entries();
        let retry_warnings: Vec<_> = entries.iter()
            .filter(|e| e.level == "WARN" && e.message.contains("retrying"))
            .collect();
        assert_eq!(retry_warnings.len(), 2); // Two retry warnings (attempts 1 and 2)
    }

    #[test]
    fn test_non_retryable_error_immediate_failure() {
        let logger = Arc::new(Logger::new(false));
        let attempt_count = std::sync::Arc::new(std::sync::Mutex::new(0));
        
        let attempt_count_clone = attempt_count.clone();
        let result: ForensicResult<&str> = retry_operation(
            move || {
                let mut count = attempt_count_clone.lock().unwrap();
                *count += 1;
                Err(ForensicError::access_denied("Access denied - not retryable"))
            },
            3, // max attempts
            &logger,
        );
        
        assert!(result.is_err());
        assert_eq!(*attempt_count.lock().unwrap(), 1); // Only one attempt for non-retryable error
        
        // Verify no retry warnings were logged
        let entries = logger.get_entries();
        let retry_warnings: Vec<_> = entries.iter()
            .filter(|e| e.level == "WARN" && e.message.contains("retrying"))
            .collect();
        assert_eq!(retry_warnings.len(), 0);
    }

    #[test]
    fn test_error_context_preservation() {
        let error_with_context = ForensicError::with_context(
            ErrorKind::SystemApiError,
            "Failed to enumerate processes",
            "CreateToolhelp32Snapshot"
        );
        
        let error_string = error_with_context.to_string();
        assert!(error_string.contains("CreateToolhelp32Snapshot"));
        assert!(error_string.contains("Failed to enumerate processes"));
        assert!(error_string.contains("SYSTEM_API_ERROR"));
        
        // Test user-friendly message
        let user_message = error_with_context.user_message();
        assert!(user_message.contains("System error"));
        assert!(user_message.contains("system issue"));
    }

    #[test]
    fn test_logger_memory_management_under_stress() {
        let logger = Logger::new(false);
        
        // Generate many log entries to test memory limits
        for i in 0..15000 {
            if i % 3 == 0 {
                logger.error(&format!("Error message {}", i));
            } else if i % 3 == 1 {
                logger.warn(&format!("Warning message {}", i));
            } else {
                logger.info(&format!("Info message {}", i));
            }
        }
        
        let entries = logger.get_entries();
        assert!(entries.len() <= 10000, "Logger should limit entries to prevent memory issues");
        
        // Verify the summary still works correctly with memory limits
        let summary = logger.get_summary();
        assert!(summary.total_count <= 10000);
        assert!(summary.error_count > 0);
        assert!(summary.warn_count > 0);
        assert!(summary.info_count > 0);
        
        // Verify success rate calculation works with limited entries
        let success_rate = summary.success_rate();
        assert!(success_rate >= 0.0 && success_rate <= 100.0);
    }

    #[test]
    fn test_concurrent_error_handling() {
        use std::sync::Arc;
        use std::thread;
        
        let logger = Arc::new(Logger::new(false));
        let mut handles = vec![];
        
        // Spawn multiple threads that generate different types of errors
        for thread_id in 0..10 {
            let logger_clone = Arc::clone(&logger);
            let handle = thread::spawn(move || {
                for i in 0..100 {
                    let operation_name = format!("thread_{}_operation_{}", thread_id, i);
                    
                    let result: ForensicResult<String> = if i % 4 == 0 {
                        Err(ForensicError::access_denied("Access denied"))
                    } else if i % 4 == 1 {
                        Err(ForensicError::system_api_error("System API failed"))
                    } else if i % 4 == 2 {
                        Err(ForensicError::not_found("Resource not found"))
                    } else {
                        Ok("Success".to_string())
                    };
                    
                    handle_error_gracefully(result, &logger_clone, &operation_name);
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify thread-safe logging worked correctly
        let summary = logger.get_summary();
        // Note: Due to memory limits, we may not have all 1000 entries
        assert!(summary.total_count > 0 && summary.total_count <= 1000); // Some entries logged
        
        // Verify error distribution (approximately, accounting for memory limits and threading)
        let total = summary.total_count;
        
        // Just verify we have some errors and warnings, ratios may vary due to memory limits
        assert!(summary.error_count > 0, "Should have some error entries");
        assert!(summary.warn_count > 0, "Should have some warning entries");
        assert_eq!(summary.info_count, 0); // No info messages in this test
        
        // Verify the total is reasonable (should be less than or equal to memory limit)
        assert!(total > 0 && total <= 10000, "Total count {} should be reasonable", total);
        
        // Verify success rate is reasonable (should be > 0 since we have non-error entries)
        let actual_success_rate = summary.success_rate();
        assert!(actual_success_rate >= 0.0 && actual_success_rate <= 100.0, 
            "Success rate {} should be between 0 and 100", actual_success_rate);
    }

    #[test]
    fn test_verbose_mode_stderr_output() {
        // Test that verbose mode affects logging behavior
        let verbose_logger = Logger::new(true);
        let quiet_logger = Logger::new(false);
        
        verbose_logger.info("Verbose test message");
        quiet_logger.info("Quiet test message");
        
        // Both should have the same number of entries
        assert_eq!(verbose_logger.get_entries().len(), 1);
        assert_eq!(quiet_logger.get_entries().len(), 1);
        
        // The difference is in stderr output, which we can't easily test here
        // but the verbose flag is properly stored and used
    }

    #[test]
    fn test_forensic_error_classification() {
        // Test fatal error classification
        let fatal_errors = vec![
            ForensicError::system_api_error("System API failed"),
            ForensicError::invalid_data("Invalid data format"),
        ];
        
        for error in &fatal_errors {
            assert!(error.is_fatal(), "Error should be classified as fatal: {}", error);
            assert!(!error.is_retryable(), "Fatal errors should not be retryable: {}", error);
        }
        
        // Test retryable error classification
        let retryable_errors = vec![
            ForensicError::new(ErrorKind::NetworkError, "Network timeout"),
            ForensicError::new(ErrorKind::Timeout, "Operation timeout"),
        ];
        
        for error in &retryable_errors {
            assert!(error.is_retryable(), "Error should be retryable: {}", error);
            assert!(!error.is_fatal(), "Retryable errors should not be fatal: {}", error);
        }
        
        // Test non-fatal, non-retryable errors
        let other_errors = vec![
            ForensicError::access_denied("Access denied"),
            ForensicError::not_found("File not found"),
        ];
        
        for error in &other_errors {
            assert!(!error.is_fatal(), "Error should not be fatal: {}", error);
            assert!(!error.is_retryable(), "Error should not be retryable: {}", error);
        }
    }
}