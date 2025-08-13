use crate::types::{EventLogs, EventLogEntry, LogEntry};

#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::System::EventLog::*,
    Win32::Foundation::*,
};

use std::collections::HashMap;

/// Collect Windows Event Log entries from Security and System logs
pub fn collect_event_logs() -> (EventLogs, Vec<LogEntry>) {
    let mut logs = Vec::new();
    logs.push(LogEntry::info("Starting event log collection"));
    
    let mut event_logs = EventLogs::default();
    
    // Collect Security event log entries
    match collect_security_events() {
        Ok(security_events) => {
            let count = security_events.len();
            event_logs.security = security_events;
            logs.push(LogEntry::info(&format!("Collected {} Security log entries", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect Security log entries: {}", e)));
        }
    }
    
    // Collect System event log entries
    match collect_system_events() {
        Ok(system_events) => {
            let count = system_events.len();
            event_logs.system = system_events;
            logs.push(LogEntry::info(&format!("Collected {} System log entries", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect System log entries: {}", e)));
        }
    }
    
    // Collect Application event log entries
    match collect_application_events() {
        Ok(application_events) => {
            let count = application_events.len();
            event_logs.application = application_events;
            logs.push(LogEntry::info(&format!("Collected {} Application log entries", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect Application log entries: {}", e)));
        }
    }
    
    let total_events = event_logs.total_entries();
    logs.push(LogEntry::info(&format!("Total event log entries collected: {}", total_events)));
    logs.push(LogEntry::info("Event log collection completed"));
    
    (event_logs, logs)
}

/// Collect Security event log entries
#[cfg(windows)]
fn collect_security_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    collect_events_from_log("Security", get_security_event_filter())
}

/// Collect System event log entries
#[cfg(windows)]
fn collect_system_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    collect_events_from_log("System", get_system_event_filter())
}

/// Collect Application event log entries
#[cfg(windows)]
fn collect_application_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    collect_events_from_log("Application", get_application_event_filter())
}

/// Collect events from a specific Windows Event Log
#[cfg(windows)]
fn collect_events_from_log(log_name: &str, event_filter: HashMap<u32, &str>) -> std::result::Result<Vec<EventLogEntry>, String> {
    let mut events = Vec::new();
    
    unsafe {
        // Open the event log
        let log_name_wide: Vec<u16> = log_name.encode_utf16().chain(std::iter::once(0)).collect();
        let h_event_log = match OpenEventLogW(None, PCWSTR(log_name_wide.as_ptr())) {
            Ok(handle) => handle,
            Err(_) => return Err(format!("Failed to open {} event log", log_name)),
        };
        
        if h_event_log.is_invalid() {
            return Err(format!("Failed to open {} event log", log_name));
        }
        
        // Get the number of records
        let mut num_records = 0u32;
        let mut oldest_record = 0u32;
        
        if GetNumberOfEventLogRecords(h_event_log, &mut num_records).is_err() ||
           GetOldestEventLogRecord(h_event_log, &mut oldest_record).is_err() {
            let _ = CloseEventLog(h_event_log);
            return Err("Failed to get event log information".to_string());
        }
        
        // Limit the number of events to collect (most recent 1000)
        let max_events = 1000;
        let start_record = if num_records > max_events {
            oldest_record + num_records - max_events
        } else {
            oldest_record
        };
        
        // Read events
        let mut buffer = vec![0u8; 65536]; // 64KB buffer
        let mut bytes_read = 0u32;
        let mut bytes_needed = 0u32;
        
        for record_num in start_record..(start_record + std::cmp::min(num_records, max_events)) {
            if ReadEventLogW(
                h_event_log,
                READ_EVENT_LOG_READ_FLAGS(0x0002 | 0x0004), // EVENTLOG_SEEK_READ | EVENTLOG_FORWARDS_READ
                record_num,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut bytes_read,
                &mut bytes_needed,
            ).is_ok() {
                // Parse the event record
                if let Ok(event) = parse_event_record(&buffer[..bytes_read as usize], &event_filter) {
                    events.push(event);
                }
            }
        }
        
        let _ = CloseEventLog(h_event_log);
    }
    
    // Sort events by timestamp (most recent first)
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    Ok(events)
}

/// Parse an event log record
#[cfg(windows)]
fn parse_event_record(buffer: &[u8], event_filter: &HashMap<u32, &str>) -> std::result::Result<EventLogEntry, std::io::Error> {
    if buffer.len() < std::mem::size_of::<EVENTLOGRECORD>() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Buffer too small for event record"));
    }
    
    unsafe {
        let record = &*(buffer.as_ptr() as *const EVENTLOGRECORD);
        
        // Only collect events we're interested in
        if !event_filter.contains_key(&record.EventID) {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Event not in filter"));
        }
        
        // Convert timestamp
        let timestamp = convert_event_timestamp(record.TimeGenerated);
        
        // Get event level
        let level = match record.EventType {
            EVENTLOG_ERROR_TYPE => "Error",
            EVENTLOG_WARNING_TYPE => "Warning", 
            EVENTLOG_INFORMATION_TYPE => "Information",
            EVENTLOG_AUDIT_SUCCESS => "Audit Success",
            EVENTLOG_AUDIT_FAILURE => "Audit Failure",
            _ => "Unknown",
        }.to_string();
        
        // Extract message (simplified - in full implementation would resolve message strings)
        let message = event_filter.get(&record.EventID)
            .unwrap_or(&"Unknown event")
            .to_string();
        
        Ok(EventLogEntry::new(
            record.EventID,
            level,
            timestamp,
            message,
        ))
    }
}

/// Convert Windows event timestamp to ISO 8601 string
#[cfg(windows)]
fn convert_event_timestamp(timestamp: u32) -> String {
    // Windows event log timestamps are seconds since January 1, 1970 (Unix epoch)
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now());
    datetime.to_rfc3339()
}

/// Get filter for Security event log (important event IDs)
fn get_security_event_filter() -> HashMap<u32, &'static str> {
    let mut filter = HashMap::new();
    
    // Logon events
    filter.insert(4624, "An account was successfully logged on");
    filter.insert(4625, "An account failed to log on");
    filter.insert(4634, "An account was logged off");
    filter.insert(4647, "User initiated logoff");
    filter.insert(4648, "A logon was attempted using explicit credentials");
    
    // Account management
    filter.insert(4720, "A user account was created");
    filter.insert(4722, "A user account was enabled");
    filter.insert(4723, "An attempt was made to change an account's password");
    filter.insert(4724, "An attempt was made to reset an account's password");
    filter.insert(4725, "A user account was disabled");
    filter.insert(4726, "A user account was deleted");
    filter.insert(4738, "A user account was changed");
    
    // Privilege escalation
    filter.insert(4672, "Special privileges assigned to new logon");
    filter.insert(4673, "A privileged service was called");
    filter.insert(4674, "An operation was attempted on a privileged object");
    
    // Process and object access
    filter.insert(4688, "A new process has been created");
    filter.insert(4689, "A process has exited");
    filter.insert(4656, "A handle to an object was requested");
    filter.insert(4658, "The handle to an object was closed");
    
    // System events
    filter.insert(4608, "Windows is starting up");
    filter.insert(4609, "Windows is shutting down");
    filter.insert(4616, "The system time was changed");
    
    filter
}

/// Get filter for System event log (important event IDs)
fn get_system_event_filter() -> HashMap<u32, &'static str> {
    let mut filter = HashMap::new();
    
    // System startup/shutdown
    filter.insert(6005, "The Event log service was started");
    filter.insert(6006, "The Event log service was stopped");
    filter.insert(6008, "The previous system shutdown was unexpected");
    filter.insert(6009, "Microsoft Windows version information");
    filter.insert(6013, "The system uptime");
    
    // Service events
    filter.insert(7034, "A service terminated unexpectedly");
    filter.insert(7035, "A service was successfully sent a start or stop control");
    filter.insert(7036, "A service entered the running or stopped state");
    filter.insert(7040, "The start type of a service was changed");
    
    // System errors
    filter.insert(1001, "Windows Error Reporting");
    filter.insert(1000, "Application Error");
    filter.insert(1002, "Application Hang");
    
    // Hardware events
    filter.insert(41, "The system has rebooted without cleanly shutting down first");
    
    // Driver events
    filter.insert(219, "Driver installation");
    filter.insert(220, "Driver installation failure");
    
    filter
}

/// Get filter for Application event log (important event IDs)
fn get_application_event_filter() -> HashMap<u32, &'static str> {
    let mut filter = HashMap::new();
    
    // Application errors and crashes
    filter.insert(1000, "Application Error");
    filter.insert(1001, "Windows Error Reporting");
    filter.insert(1002, "Application Hang");
    filter.insert(1026, "Application popup");
    
    // .NET Framework events
    filter.insert(1023, ".NET Runtime Error");
    filter.insert(1026, ".NET Runtime");
    
    // Windows Installer events
    filter.insert(1033, "Windows Installer reconfigured a product");
    filter.insert(1034, "Windows Installer removed a product");
    filter.insert(11707, "Installation completed successfully");
    filter.insert(11708, "Installation failed");
    filter.insert(11724, "Package removal completed successfully");
    
    // Application installation/uninstallation
    filter.insert(2, "The application-specific permission settings do not grant Local Activation permission");
    filter.insert(10016, "The application-specific permission settings do not grant Local Activation permission");
    
    // Security-related application events
    filter.insert(4625, "An account failed to log on");
    filter.insert(4648, "A logon was attempted using explicit credentials");
    
    filter
}

/// Fallback implementation for non-Windows platforms
#[cfg(not(windows))]
fn collect_security_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    Ok(Vec::new()) // Return empty vector on non-Windows platforms
}

#[cfg(not(windows))]
fn collect_system_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    Ok(Vec::new()) // Return empty vector on non-Windows platforms
}

#[cfg(not(windows))]
fn collect_application_events() -> std::result::Result<Vec<EventLogEntry>, String> {
    Ok(Vec::new()) // Return empty vector on non-Windows platforms
}

/// Filter events by event ID
pub fn filter_events_by_id(events: &[EventLogEntry], event_id: u32) -> Vec<&EventLogEntry> {
    events.iter().filter(|e| e.event_id == event_id).collect()
}

/// Filter events by level
pub fn filter_events_by_level<'a>(events: &'a [EventLogEntry], level: &str) -> Vec<&'a EventLogEntry> {
    events.iter().filter(|e| e.level == level).collect()
}

/// Get events within a time range
pub fn filter_events_by_time_range<'a>(events: &'a [EventLogEntry], start_time: &str, end_time: &str) -> Vec<&'a EventLogEntry> {
    events.iter().filter(|e| {
        e.timestamp >= start_time.to_string() && e.timestamp <= end_time.to_string()
    }).collect()
}

/// Get most recent events
pub fn get_recent_events(events: &[EventLogEntry], count: usize) -> Vec<&EventLogEntry> {
    let mut sorted_events: Vec<&EventLogEntry> = events.iter().collect();
    sorted_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    sorted_events.into_iter().take(count).collect()
}

/// Find logon events
pub fn find_logon_events(security_events: &[EventLogEntry]) -> Vec<&EventLogEntry> {
    let logon_event_ids = vec![4624, 4625, 4634, 4647, 4648];
    security_events.iter().filter(|e| logon_event_ids.contains(&e.event_id)).collect()
}

/// Find process creation events
pub fn find_process_events(security_events: &[EventLogEntry]) -> Vec<&EventLogEntry> {
    let process_event_ids = vec![4688, 4689];
    security_events.iter().filter(|e| process_event_ids.contains(&e.event_id)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_event_logs() {
        let (event_logs, logs) = collect_event_logs();
        
        // Should have log entries
        assert!(!logs.is_empty());
        
        // Should have start and completion messages
        assert!(logs.iter().any(|log| log.message.contains("Starting event log collection")));
        assert!(logs.iter().any(|log| log.message.contains("completed")));
        
        // Event logs structure should be initialized
        assert!(event_logs.total_entries() >= 0);
    }

    #[test]
    fn test_get_security_event_filter() {
        let filter = get_security_event_filter();
        
        // Should contain important security events
        assert!(filter.contains_key(&4624)); // Successful logon
        assert!(filter.contains_key(&4625)); // Failed logon
        assert!(filter.contains_key(&4688)); // Process creation
        assert!(filter.contains_key(&4720)); // User account created
        
        // Should have meaningful descriptions
        assert_eq!(filter.get(&4624), Some(&"An account was successfully logged on"));
    }

    #[test]
    fn test_get_system_event_filter() {
        let filter = get_system_event_filter();
        
        // Should contain important system events
        assert!(filter.contains_key(&6005)); // Event log service started
        assert!(filter.contains_key(&6006)); // Event log service stopped
        assert!(filter.contains_key(&7034)); // Service terminated unexpectedly
        assert!(filter.contains_key(&1001)); // Windows Error Reporting
        
        // Should have meaningful descriptions
        assert_eq!(filter.get(&6005), Some(&"The Event log service was started"));
    }

    #[test]
    fn test_get_application_event_filter() {
        let filter = get_application_event_filter();
        
        // Should contain important application events
        assert!(filter.contains_key(&1000)); // Application Error
        assert!(filter.contains_key(&1001)); // Windows Error Reporting
        assert!(filter.contains_key(&1002)); // Application Hang
        assert!(filter.contains_key(&1023)); // .NET Runtime Error
        assert!(filter.contains_key(&11707)); // Installation completed successfully
        
        // Should have meaningful descriptions
        assert_eq!(filter.get(&1000), Some(&"Application Error"));
        assert_eq!(filter.get(&11707), Some(&"Installation completed successfully"));
    }

    #[test]
    fn test_filter_events_by_id() {
        let events = vec![
            EventLogEntry::new(4624, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Logon".to_string()),
            EventLogEntry::new(4625, "Warning".to_string(), "2023-01-01T00:01:00Z".to_string(), "Failed logon".to_string()),
            EventLogEntry::new(4624, "Information".to_string(), "2023-01-01T00:02:00Z".to_string(), "Another logon".to_string()),
        ];
        
        let logon_events = filter_events_by_id(&events, 4624);
        assert_eq!(logon_events.len(), 2);
        
        let failed_logon_events = filter_events_by_id(&events, 4625);
        assert_eq!(failed_logon_events.len(), 1);
    }

    #[test]
    fn test_filter_events_by_level() {
        let events = vec![
            EventLogEntry::new(1001, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Info event".to_string()),
            EventLogEntry::new(1002, "Warning".to_string(), "2023-01-01T00:01:00Z".to_string(), "Warning event".to_string()),
            EventLogEntry::new(1003, "Error".to_string(), "2023-01-01T00:02:00Z".to_string(), "Error event".to_string()),
        ];
        
        let info_events = filter_events_by_level(&events, "Information");
        assert_eq!(info_events.len(), 1);
        
        let warning_events = filter_events_by_level(&events, "Warning");
        assert_eq!(warning_events.len(), 1);
        
        let error_events = filter_events_by_level(&events, "Error");
        assert_eq!(error_events.len(), 1);
    }

    #[test]
    fn test_find_logon_events() {
        let events = vec![
            EventLogEntry::new(4624, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Successful logon".to_string()),
            EventLogEntry::new(4625, "Warning".to_string(), "2023-01-01T00:01:00Z".to_string(), "Failed logon".to_string()),
            EventLogEntry::new(4688, "Information".to_string(), "2023-01-01T00:02:00Z".to_string(), "Process created".to_string()),
            EventLogEntry::new(4634, "Information".to_string(), "2023-01-01T00:03:00Z".to_string(), "Logoff".to_string()),
        ];
        
        let logon_events = find_logon_events(&events);
        assert_eq!(logon_events.len(), 3); // 4624, 4625, 4634
    }

    #[test]
    fn test_find_process_events() {
        let events = vec![
            EventLogEntry::new(4624, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Logon".to_string()),
            EventLogEntry::new(4688, "Information".to_string(), "2023-01-01T00:01:00Z".to_string(), "Process created".to_string()),
            EventLogEntry::new(4689, "Information".to_string(), "2023-01-01T00:02:00Z".to_string(), "Process exited".to_string()),
        ];
        
        let process_events = find_process_events(&events);
        assert_eq!(process_events.len(), 2); // 4688, 4689
    }

    #[test]
    fn test_get_recent_events() {
        let events = vec![
            EventLogEntry::new(1001, "Information".to_string(), "2023-01-01T00:00:00Z".to_string(), "Event 1".to_string()),
            EventLogEntry::new(1002, "Information".to_string(), "2023-01-01T00:02:00Z".to_string(), "Event 2".to_string()),
            EventLogEntry::new(1003, "Information".to_string(), "2023-01-01T00:01:00Z".to_string(), "Event 3".to_string()),
        ];
        
        let recent = get_recent_events(&events, 2);
        assert_eq!(recent.len(), 2);
        // Should be sorted by timestamp (most recent first)
        assert_eq!(recent[0].event_id, 1002); // 00:02:00
        assert_eq!(recent[1].event_id, 1003); // 00:01:00
    }
}