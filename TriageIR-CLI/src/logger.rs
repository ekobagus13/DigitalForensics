use crate::types::{LogEntry, LogLevel};
use std::sync::Mutex;
use std::collections::VecDeque;

/// Global logger for collecting all log entries during scan execution
pub struct Logger {
    entries: Mutex<VecDeque<LogEntry>>,
    verbose: bool,
    max_entries: usize,
}

impl Logger {
    /// Create a new logger instance
    pub fn new(verbose: bool) -> Self {
        Logger {
            entries: Mutex::new(VecDeque::new()),
            verbose,
            max_entries: 10000, // Limit memory usage
        }
    }
    
    /// Log an info message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    /// Log a warning message
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    
    /// Log an error message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
    /// Log a message with specified level
    pub fn log(&self, level: LogLevel, message: &str) {
        let entry = LogEntry::new(level.as_str(), message);
        
        // Print to stderr if verbose mode is enabled
        if self.verbose {
            eprintln!("[{}] {}: {}", entry.timestamp, entry.level, entry.message);
        }
        
        // Add to internal log collection
        if let Ok(mut entries) = self.entries.lock() {
            // Maintain maximum number of entries to prevent memory issues
            if entries.len() >= self.max_entries {
                entries.pop_front();
            }
            entries.push_back(entry);
        }
    }
    
    /// Log with formatted message
    pub fn log_fmt(&self, level: LogLevel, format: &str, args: &[&dyn std::fmt::Display]) {
        let message = format_message(format, args);
        self.log(level, &message);
    }
    
    /// Get all collected log entries
    pub fn get_entries(&self) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get entries by level
    pub fn get_entries_by_level(&self, level: LogLevel) -> Vec<LogEntry> {
        if let Ok(entries) = self.entries.lock() {
            entries.iter()
                .filter(|entry| entry.level == level.as_str())
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get count of entries by level
    pub fn get_count_by_level(&self, level: LogLevel) -> usize {
        if let Ok(entries) = self.entries.lock() {
            entries.iter()
                .filter(|entry| entry.level == level.as_str())
                .count()
        } else {
            0
        }
    }
    
    /// Clear all log entries
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }
    
    /// Get summary of log levels
    pub fn get_summary(&self) -> LogSummary {
        if let Ok(entries) = self.entries.lock() {
            let mut summary = LogSummary::default();
            
            for entry in entries.iter() {
                match entry.level.as_str() {
                    "INFO" => summary.info_count += 1,
                    "WARN" => summary.warn_count += 1,
                    "ERROR" => summary.error_count += 1,
                    _ => summary.other_count += 1,
                }
            }
            
            summary.total_count = entries.len();
            summary
        } else {
            LogSummary::default()
        }
    }
}

/// Summary of log entries by level
#[derive(Debug, Default, Clone)]
pub struct LogSummary {
    pub total_count: usize,
    pub info_count: usize,
    pub warn_count: usize,
    pub error_count: usize,
    pub other_count: usize,
}

impl LogSummary {
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
    
    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.warn_count > 0
    }
    
    /// Get success rate (percentage of non-error entries)
    pub fn success_rate(&self) -> f64 {
        if self.total_count == 0 {
            100.0
        } else {
            ((self.total_count - self.error_count) as f64 / self.total_count as f64) * 100.0
        }
    }
}

/// Format a message with arguments
fn format_message(format: &str, args: &[&dyn std::fmt::Display]) -> String {
    let mut result = format.to_string();
    
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, &arg.to_string());
    }
    
    result
}

/// Error handling utilities
pub mod error_handling {
    use super::*;
    
    /// Result type for forensic operations
    pub type ForensicResult<T> = Result<T, ForensicError>;
    
    /// Custom error type for forensic operations
    #[derive(Debug, Clone)]
    pub struct ForensicError {
        pub kind: ErrorKind,
        pub message: String,
        pub context: Option<String>,
    }
    
    /// Types of forensic errors
    #[derive(Debug, Clone, PartialEq)]
    pub enum ErrorKind {
        /// Access denied or insufficient privileges
        AccessDenied,
        /// System API call failed
        SystemApiError,
        /// File or resource not found
        NotFound,
        /// Invalid data or format
        InvalidData,
        /// Network or communication error
        NetworkError,
        /// Timeout occurred
        Timeout,
        /// Unknown or unexpected error
        Unknown,
    }
    
    impl ForensicError {
        /// Create a new forensic error
        pub fn new(kind: ErrorKind, message: &str) -> Self {
            ForensicError {
                kind,
                message: message.to_string(),
                context: None,
            }
        }
        
        /// Create a new forensic error with context
        pub fn with_context(kind: ErrorKind, message: &str, context: &str) -> Self {
            ForensicError {
                kind,
                message: message.to_string(),
                context: Some(context.to_string()),
            }
        }
        
        /// Create an access denied error
        pub fn access_denied(message: &str) -> Self {
            Self::new(ErrorKind::AccessDenied, message)
        }
        
        /// Create a system API error
        pub fn system_api_error(message: &str) -> Self {
            Self::new(ErrorKind::SystemApiError, message)
        }
        
        /// Create a not found error
        pub fn not_found(message: &str) -> Self {
            Self::new(ErrorKind::NotFound, message)
        }
        
        /// Create an invalid data error
        pub fn invalid_data(message: &str) -> Self {
            Self::new(ErrorKind::InvalidData, message)
        }
        
        /// Check if this error should be treated as fatal
        pub fn is_fatal(&self) -> bool {
            matches!(self.kind, ErrorKind::SystemApiError | ErrorKind::InvalidData)
        }
        
        /// Check if this error can be retried
        pub fn is_retryable(&self) -> bool {
            matches!(self.kind, ErrorKind::NetworkError | ErrorKind::Timeout)
        }
        
        /// Get a user-friendly error message
        pub fn user_message(&self) -> String {
            match self.kind {
                ErrorKind::AccessDenied => {
                    format!("Access denied: {}. Try running as administrator.", self.message)
                }
                ErrorKind::SystemApiError => {
                    format!("System error: {}. This may indicate a system issue.", self.message)
                }
                ErrorKind::NotFound => {
                    format!("Resource not found: {}. This is normal for some systems.", self.message)
                }
                ErrorKind::InvalidData => {
                    format!("Invalid data encountered: {}. Data may be corrupted.", self.message)
                }
                ErrorKind::NetworkError => {
                    format!("Network error: {}. Check network connectivity.", self.message)
                }
                ErrorKind::Timeout => {
                    format!("Operation timed out: {}. System may be under heavy load.", self.message)
                }
                ErrorKind::Unknown => {
                    format!("Unknown error: {}.", self.message)
                }
            }
        }
    }
    
    impl std::fmt::Display for ForensicError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(context) = &self.context {
                write!(f, "{} ({}): {}", self.kind.as_str(), context, self.message)
            } else {
                write!(f, "{}: {}", self.kind.as_str(), self.message)
            }
        }
    }
    
    impl std::error::Error for ForensicError {}
    
    impl ErrorKind {
        pub fn as_str(&self) -> &'static str {
            match self {
                ErrorKind::AccessDenied => "ACCESS_DENIED",
                ErrorKind::SystemApiError => "SYSTEM_API_ERROR",
                ErrorKind::NotFound => "NOT_FOUND",
                ErrorKind::InvalidData => "INVALID_DATA",
                ErrorKind::NetworkError => "NETWORK_ERROR",
                ErrorKind::Timeout => "TIMEOUT",
                ErrorKind::Unknown => "UNKNOWN",
            }
        }
    }
    
    /// Retry mechanism for operations that might fail temporarily
    pub fn retry_operation<T, F>(
        operation: F,
        max_attempts: usize,
        logger: &Logger,
    ) -> ForensicResult<T>
    where
        F: Fn() -> ForensicResult<T>,
    {
        let mut last_error = None;
        
        for attempt in 1..=max_attempts {
            match operation() {
                Ok(result) => {
                    if attempt > 1 {
                        logger.info(&format!("Operation succeeded on attempt {}", attempt));
                    }
                    return Ok(result);
                }
                Err(error) => {
                    if error.is_retryable() && attempt < max_attempts {
                        logger.warn(&format!("Attempt {} failed, retrying: {}", attempt, error));
                        std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
                        last_error = Some(error);
                    } else {
                        return Err(error);
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| ForensicError::new(ErrorKind::Unknown, "All retry attempts failed")))
    }
    
    /// Handle errors gracefully and continue operation
    pub fn handle_error_gracefully<T>(
        result: ForensicResult<T>,
        logger: &Logger,
        operation_name: &str,
    ) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                if error.is_fatal() {
                    logger.error(&format!("Fatal error in {}: {}", operation_name, error.user_message()));
                } else {
                    logger.warn(&format!("Non-fatal error in {}: {}", operation_name, error.user_message()));
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::error_handling::*;

    #[test]
    fn test_logger_basic_functionality() {
        let logger = Logger::new(false);
        
        logger.info("Test info message");
        logger.warn("Test warning message");
        logger.error("Test error message");
        
        let entries = logger.get_entries();
        assert_eq!(entries.len(), 3);
        
        assert_eq!(entries[0].level, "INFO");
        assert_eq!(entries[0].message, "Test info message");
        
        assert_eq!(entries[1].level, "WARN");
        assert_eq!(entries[1].message, "Test warning message");
        
        assert_eq!(entries[2].level, "ERROR");
        assert_eq!(entries[2].message, "Test error message");
    }

    #[test]
    fn test_logger_summary() {
        let logger = Logger::new(false);
        
        logger.info("Info 1");
        logger.info("Info 2");
        logger.warn("Warning 1");
        logger.error("Error 1");
        
        let summary = logger.get_summary();
        assert_eq!(summary.total_count, 4);
        assert_eq!(summary.info_count, 2);
        assert_eq!(summary.warn_count, 1);
        assert_eq!(summary.error_count, 1);
        
        assert!(summary.has_errors());
        assert!(summary.has_warnings());
        assert_eq!(summary.success_rate(), 75.0); // 3 out of 4 non-error
    }

    #[test]
    fn test_logger_filtering() {
        let logger = Logger::new(false);
        
        logger.info("Info message");
        logger.warn("Warning message");
        logger.error("Error message");
        
        let info_entries = logger.get_entries_by_level(LogLevel::Info);
        assert_eq!(info_entries.len(), 1);
        assert_eq!(info_entries[0].message, "Info message");
        
        let error_count = logger.get_count_by_level(LogLevel::Error);
        assert_eq!(error_count, 1);
    }

    #[test]
    fn test_forensic_error_creation() {
        let error = ForensicError::access_denied("Cannot access registry key");
        assert_eq!(error.kind, ErrorKind::AccessDenied);
        assert_eq!(error.message, "Cannot access registry key");
        
        let error_with_context = ForensicError::with_context(
            ErrorKind::SystemApiError,
            "API call failed",
            "GetProcessList"
        );
        assert_eq!(error_with_context.context, Some("GetProcessList".to_string()));
    }

    #[test]
    fn test_error_classification() {
        let access_error = ForensicError::access_denied("Access denied");
        assert!(!access_error.is_fatal());
        assert!(!access_error.is_retryable());
        
        let system_error = ForensicError::system_api_error("System API failed");
        assert!(system_error.is_fatal());
        assert!(!system_error.is_retryable());
        
        let network_error = ForensicError::new(ErrorKind::NetworkError, "Connection failed");
        assert!(!network_error.is_fatal());
        assert!(network_error.is_retryable());
    }

    #[test]
    fn test_user_friendly_messages() {
        let access_error = ForensicError::access_denied("Registry access failed");
        let user_msg = access_error.user_message();
        assert!(user_msg.contains("Try running as administrator"));
        
        let not_found_error = ForensicError::not_found("File not found");
        let user_msg = not_found_error.user_message();
        assert!(user_msg.contains("This is normal for some systems"));
    }

    #[test]
    fn test_format_message() {
        let formatted = format_message("Found {0} processes with {1} connections", &[&42, &"TCP"]);
        assert_eq!(formatted, "Found 42 processes with TCP connections");
    }

    #[test]
    fn test_retry_operation() {
        let logger = Logger::new(false);
        let mut attempt_count = 0;
        
        let result = retry_operation(
            || {
                attempt_count += 1;
                if attempt_count < 3 {
                    Err(ForensicError::new(ErrorKind::NetworkError, "Temporary failure"))
                } else {
                    Ok("Success")
                }
            },
            3,
            &logger,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempt_count, 3);
    }

    #[test]
    fn test_handle_error_gracefully() {
        let logger = Logger::new(false);
        
        // Test successful operation
        let success_result: ForensicResult<i32> = Ok(42);
        let handled = handle_error_gracefully(success_result, &logger, "test_operation");
        assert_eq!(handled, Some(42));
        
        // Test non-fatal error
        let error_result: ForensicResult<i32> = Err(ForensicError::access_denied("Access denied"));
        let handled = handle_error_gracefully(error_result, &logger, "test_operation");
        assert_eq!(handled, None);
        
        // Should have logged the error
        let entries = logger.get_entries();
        assert!(entries.iter().any(|e| e.level == "WARN" && e.message.contains("test_operation")));
    }
}