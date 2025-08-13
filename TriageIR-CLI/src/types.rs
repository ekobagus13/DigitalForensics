use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root structure containing all scan results and metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanResults {
    pub scan_metadata: ScanMetadata,
    pub artifacts: Artifacts,
    pub collection_log: Vec<LogEntry>,
}

impl ScanResults {
    /// Create a new ScanResults with initialized metadata
    pub fn new(hostname: String, os_version: String) -> Self {
        let scan_id = uuid::Uuid::new_v4().to_string();
        let scan_start = chrono::Utc::now();
        
        ScanResults {
            scan_metadata: ScanMetadata {
                scan_id,
                scan_start_utc: scan_start.to_rfc3339(),
                scan_duration_ms: 0,
                hostname,
                os_version,
                cli_version: env!("CARGO_PKG_VERSION").to_string(),
            },
            artifacts: Artifacts::default(),
            collection_log: Vec::new(),
        }
    }
    
    /// Update scan duration based on start time
    pub fn finalize_scan(&mut self) {
        if let Ok(start_time) = chrono::DateTime::parse_from_rfc3339(&self.scan_metadata.scan_start_utc) {
            let duration = chrono::Utc::now().signed_duration_since(start_time.with_timezone(&chrono::Utc));
            self.scan_metadata.scan_duration_ms = duration.num_milliseconds().max(0) as u64;
        }
    }
    
    /// Add a log entry to the collection log
    pub fn add_log(&mut self, entry: LogEntry) {
        self.collection_log.push(entry);
    }
    
    /// Validate the scan results structure
    pub fn validate(&self) -> Result<(), String> {
        if self.scan_metadata.scan_id.is_empty() {
            return Err("Scan ID cannot be empty".to_string());
        }
        
        if self.scan_metadata.hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }
        
        // Validate UUID format
        if uuid::Uuid::parse_str(&self.scan_metadata.scan_id).is_err() {
            return Err("Invalid UUID format for scan_id".to_string());
        }
        
        // Validate timestamp format
        if chrono::DateTime::parse_from_rfc3339(&self.scan_metadata.scan_start_utc).is_err() {
            return Err("Invalid timestamp format for scan_start_utc".to_string());
        }
        
        Ok(())
    }
}

/// Metadata about the forensic scan execution
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanMetadata {
    /// Unique identifier for this scan (UUID v4)
    pub scan_id: String,
    /// ISO 8601 timestamp when scan started
    pub scan_start_utc: String,
    /// Total scan duration in milliseconds
    pub scan_duration_ms: u64,
    /// Target system hostname
    pub hostname: String,
    /// Operating system version information
    pub os_version: String,
    /// CLI tool version
    pub cli_version: String,
}

/// Container for all collected forensic artifacts
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Artifacts {
    pub system_info: SystemInfo,
    pub running_processes: Vec<Process>,
    pub network_connections: Vec<NetworkConnection>,
    pub persistence_mechanisms: Vec<PersistenceMechanism>,
    pub event_logs: EventLogs,
}

impl Artifacts {
    /// Get total count of all collected artifacts
    pub fn total_artifact_count(&self) -> usize {
        self.running_processes.len() +
        self.network_connections.len() +
        self.persistence_mechanisms.len() +
        self.event_logs.security.len() +
        self.event_logs.system.len() +
        self.system_info.logged_on_users.len()
    }
}

/// System information and current state
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemInfo {
    /// System uptime in seconds
    pub uptime_secs: u64,
    /// Currently logged-on users
    pub logged_on_users: Vec<LoggedOnUser>,
}

/// Information about a logged-on user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedOnUser {
    /// Username
    pub username: String,
    /// Domain or computer name
    pub domain: String,
    /// Logon timestamp (ISO 8601)
    pub logon_time: String,
}

impl LoggedOnUser {
    pub fn new(username: String, domain: String, logon_time: String) -> Self {
        LoggedOnUser {
            username,
            domain,
            logon_time,
        }
    }
}

/// Information about a running process
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Process {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub parent_pid: u32,
    /// Process name
    pub name: String,
    /// Full command line
    pub command_line: String,
    /// Path to executable file
    pub executable_path: String,
    /// SHA-256 hash of executable
    pub sha256_hash: String,
}

impl Process {
    pub fn new(pid: u32, parent_pid: u32, name: String, command_line: String, executable_path: String) -> Self {
        Process {
            pid,
            parent_pid,
            name,
            command_line,
            executable_path,
            sha256_hash: String::new(), // Will be calculated separately
        }
    }
    
    /// Check if this process has a valid executable path
    pub fn has_executable_path(&self) -> bool {
        !self.executable_path.is_empty() && self.executable_path != "N/A"
    }
}

/// Network connection information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConnection {
    /// Protocol type (TCP or UDP)
    pub protocol: String,
    /// Local address and port
    pub local_address: String,
    /// Remote address and port
    pub remote_address: String,
    /// Connection state (for TCP)
    pub state: String,
    /// Process ID that owns this connection
    pub owning_pid: u32,
}

impl NetworkConnection {
    pub fn new(protocol: String, local_address: String, remote_address: String, state: String, owning_pid: u32) -> Self {
        NetworkConnection {
            protocol,
            local_address,
            remote_address,
            state,
            owning_pid,
        }
    }
    
    /// Check if this is an external connection (not localhost)
    pub fn is_external(&self) -> bool {
        !self.remote_address.starts_with("127.0.0.1") &&
        !self.remote_address.starts_with("::1") &&
        !self.remote_address.starts_with("0.0.0.0") &&
        self.remote_address != "*:*"
    }
}

/// Persistence mechanism found on the system
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersistenceMechanism {
    /// Type of persistence mechanism
    #[serde(rename = "type")]
    pub mechanism_type: String,
    /// Name or identifier
    pub name: String,
    /// Command or executable path
    pub command: String,
    /// Source location (registry key, task path, etc.)
    pub source: String,
}

impl PersistenceMechanism {
    pub fn new(mechanism_type: String, name: String, command: String, source: String) -> Self {
        PersistenceMechanism {
            mechanism_type,
            name,
            command,
            source,
        }
    }
}

/// Common persistence mechanism types
pub enum PersistenceType {
    RegistryRunKey,
    ScheduledTask,
    Service,
    StartupFolder,
    WMIEventConsumer,
}

impl PersistenceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PersistenceType::RegistryRunKey => "Registry Run Key",
            PersistenceType::ScheduledTask => "Scheduled Task",
            PersistenceType::Service => "Windows Service",
            PersistenceType::StartupFolder => "Startup Folder",
            PersistenceType::WMIEventConsumer => "WMI Event Consumer",
        }
    }
}

/// Windows Event Log collections
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EventLogs {
    /// Security event log entries
    pub security: Vec<EventLogEntry>,
    /// System event log entries
    pub system: Vec<EventLogEntry>,
}

impl EventLogs {
    /// Get total count of all event log entries
    pub fn total_entries(&self) -> usize {
        self.security.len() + self.system.len()
    }
}

/// Individual event log entry
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventLogEntry {
    /// Windows Event ID
    pub event_id: u32,
    /// Event level (Information, Warning, Error, etc.)
    pub level: String,
    /// Event timestamp (ISO 8601)
    pub timestamp: String,
    /// Event message/description
    pub message: String,
}

impl EventLogEntry {
    pub fn new(event_id: u32, level: String, timestamp: String, message: String) -> Self {
        EventLogEntry {
            event_id,
            level,
            timestamp,
            message,
        }
    }
}

/// Collection log entry for tracking scan progress and issues
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    /// Timestamp when log entry was created (ISO 8601)
    pub timestamp: String,
    /// Log level (INFO, WARN, ERROR)
    pub level: String,
    /// Log message
    pub message: String,
}

impl LogEntry {
    /// Create a new log entry with current timestamp
    pub fn new(level: &str, message: &str) -> Self {
        LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            message: message.to_string(),
        }
    }
    
    /// Create an INFO level log entry
    pub fn info(message: &str) -> Self {
        Self::new("INFO", message)
    }
    
    /// Create a WARN level log entry
    pub fn warn(message: &str) -> Self {
        Self::new("WARN", message)
    }
    
    /// Create an ERROR level log entry
    pub fn error(message: &str) -> Self {
        Self::new("ERROR", message)
    }
    
    /// Create a log entry with formatted message
    pub fn info_fmt(message: &str, args: &[&str]) -> Self {
        let formatted = args.iter().enumerate().fold(message.to_string(), |acc, (i, arg)| {
            acc.replace(&format!("{{{}}}", i), arg)
        });
        Self::info(&formatted)
    }
}

/// Log levels for collection logging
#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}
#
[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_scan_results_creation() {
        let results = ScanResults::new("TEST-HOST".to_string(), "Windows 10".to_string());
        
        assert_eq!(results.scan_metadata.hostname, "TEST-HOST");
        assert_eq!(results.scan_metadata.os_version, "Windows 10");
        assert!(!results.scan_metadata.scan_id.is_empty());
        assert!(!results.scan_metadata.scan_start_utc.is_empty());
        assert_eq!(results.scan_metadata.scan_duration_ms, 0);
    }

    #[test]
    fn test_scan_results_validation() {
        let results = ScanResults::new("TEST-HOST".to_string(), "Windows 10".to_string());
        assert!(results.validate().is_ok());
        
        let mut invalid_results = results.clone();
        invalid_results.scan_metadata.scan_id = "invalid-uuid".to_string();
        assert!(invalid_results.validate().is_err());
    }

    #[test]
    fn test_json_serialization() {
        let results = ScanResults::new("TEST-HOST".to_string(), "Windows 10".to_string());
        
        // Test serialization
        let json = serde_json::to_string(&results).expect("Failed to serialize");
        assert!(!json.is_empty());
        
        // Test deserialization
        let deserialized: ScanResults = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.scan_metadata.hostname, results.scan_metadata.hostname);
    }

    #[test]
    fn test_log_entry_creation() {
        let log = LogEntry::info("Test message");
        assert_eq!(log.level, "INFO");
        assert_eq!(log.message, "Test message");
        assert!(!log.timestamp.is_empty());
        
        let warn_log = LogEntry::warn("Warning message");
        assert_eq!(warn_log.level, "WARN");
        
        let error_log = LogEntry::error("Error message");
        assert_eq!(error_log.level, "ERROR");
    }

    #[test]
    fn test_process_creation() {
        let process = Process::new(1234, 5678, "test.exe".to_string(), "test.exe --arg".to_string(), "C:\\test.exe".to_string());
        
        assert_eq!(process.pid, 1234);
        assert_eq!(process.parent_pid, 5678);
        assert_eq!(process.name, "test.exe");
        assert!(process.has_executable_path());
    }

    #[test]
    fn test_network_connection_external_detection() {
        let external_conn = NetworkConnection::new(
            "TCP".to_string(),
            "192.168.1.100:12345".to_string(),
            "8.8.8.8:80".to_string(),
            "ESTABLISHED".to_string(),
            1234
        );
        assert!(external_conn.is_external());
        
        let local_conn = NetworkConnection::new(
            "TCP".to_string(),
            "127.0.0.1:12345".to_string(),
            "127.0.0.1:80".to_string(),
            "ESTABLISHED".to_string(),
            1234
        );
        assert!(!local_conn.is_external());
    }

    #[test]
    fn test_persistence_mechanism_creation() {
        let mechanism = PersistenceMechanism::new(
            PersistenceType::RegistryRunKey.as_str().to_string(),
            "TestApp".to_string(),
            "C:\\test.exe".to_string(),
            "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string()
        );
        
        assert_eq!(mechanism.mechanism_type, "Registry Run Key");
        assert_eq!(mechanism.name, "TestApp");
    }

    #[test]
    fn test_artifacts_count() {
        let mut artifacts = Artifacts::default();
        assert_eq!(artifacts.total_artifact_count(), 0);
        
        artifacts.running_processes.push(Process::new(1, 0, "test".to_string(), "test".to_string(), "test".to_string()));
        artifacts.network_connections.push(NetworkConnection::new("TCP".to_string(), "local".to_string(), "remote".to_string(), "ESTABLISHED".to_string(), 1));
        
        assert_eq!(artifacts.total_artifact_count(), 2);
    }

    #[test]
    fn test_event_logs_count() {
        let mut event_logs = EventLogs::default();
        assert_eq!(event_logs.total_entries(), 0);
        
        event_logs.security.push(EventLogEntry::new(4624, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Logon".to_string()));
        event_logs.system.push(EventLogEntry::new(1001, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "System".to_string()));
        
        assert_eq!(event_logs.total_entries(), 2);
    }
}