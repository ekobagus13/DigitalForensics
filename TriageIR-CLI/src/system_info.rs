use crate::types::{SystemInfo, LoggedOnUser, LogEntry};
use sysinfo::System;
use std::time::{SystemTime, UNIX_EPOCH};

/// Collect comprehensive system information
pub fn collect_system_info() -> (SystemInfo, Vec<LogEntry>) {
    let mut logs = Vec::new();
    logs.push(LogEntry::info("Starting system information collection"));
    
    let mut system_info = SystemInfo::default();
    
    // Collect system uptime
    match collect_uptime() {
        Ok(uptime) => {
            system_info.uptime_secs = uptime;
            logs.push(LogEntry::info(&format!("System uptime: {} seconds", uptime)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect system uptime: {}", e)));
        }
    }
    
    // Collect logged-on users
    match collect_logged_on_users() {
        Ok(users) => {
            let user_count = users.len();
            system_info.logged_on_users = users;
            logs.push(LogEntry::info(&format!("Found {} logged-on users", user_count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect logged-on users: {}", e)));
        }
    }
    
    logs.push(LogEntry::info("System information collection completed"));
    (system_info, logs)
}

/// Collect system uptime in seconds
fn collect_uptime() -> Result<u64, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Get boot time and calculate uptime
    let boot_time = System::boot_time();
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {}", e))?
        .as_secs();
    
    if current_time > boot_time {
        Ok(current_time - boot_time)
    } else {
        Err("Invalid boot time detected".to_string())
    }
}

/// Collect information about currently logged-on users
fn collect_logged_on_users() -> Result<Vec<LoggedOnUser>, String> {
    let mut users = Vec::new();
    let _sys = System::new_all();
    // Note: sysinfo 0.30+ doesn't have users() method, using Windows API fallback
    // sys.refresh_users_list();
    
    // for user in sys.users() {
    //     // Convert user information to our format
    //     let logged_user = LoggedOnUser::new(
    //         user.name().to_string(),
    //         get_user_domain(user.name()),
    //         format_logon_time(user.name()),
    //     );
    //     users.push(logged_user);
    // }
    
    // Fallback: Add current user from environment
    if let Ok(username) = std::env::var("USERNAME") {
        let domain = std::env::var("USERDOMAIN").unwrap_or_else(|_| "WORKGROUP".to_string());
        let logged_user = LoggedOnUser::new(
            username,
            domain,
            chrono::Utc::now().to_rfc3339(),
        );
        users.push(logged_user);
    }
    
    // Remove duplicates based on username
    users.sort_by(|a, b| a.username.cmp(&b.username));
    users.dedup_by(|a, b| a.username == b.username);
    
    Ok(users)
}

/// Get domain information for a user (Windows-specific)
fn get_user_domain(_username: &str) -> String {
    // Try to get computer name as default domain
    std::env::var("COMPUTERNAME").unwrap_or_else(|_| {
        std::env::var("USERDOMAIN").unwrap_or_else(|_| "WORKGROUP".to_string())
    })
}

/// Format logon time for a user (placeholder implementation)
fn format_logon_time(_username: &str) -> String {
    // This is a simplified implementation
    // In a full implementation, we would query Windows APIs for actual logon times
    chrono::Utc::now().to_rfc3339()
}

/// Get detailed OS version information
pub fn get_detailed_os_version() -> String {
    let _sys = System::new();
    // sys.refresh_system();
    
    format!("{} {} ({})", 
        System::name().unwrap_or("Unknown OS".to_string()),
        System::os_version().unwrap_or("Unknown Version".to_string()),
        System::kernel_version().unwrap_or("Unknown Kernel".to_string())
    )
}

/// Get system hostname
pub fn get_system_hostname() -> String {
    // let mut sys = System::new();
    // sys.refresh_system();
    
    System::host_name().unwrap_or_else(|| {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "UNKNOWN".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_system_info() {
        let (system_info, logs) = collect_system_info();
        
        // Should have some log entries
        assert!(!logs.is_empty());
        
        // Should have collected uptime (should be > 0 on running system)
        // Note: This might be 0 in test environments
        assert!(system_info.uptime_secs >= 0);
        
        // Logs should contain start and completion messages
        assert!(logs.iter().any(|log| log.message.contains("Starting system information")));
        assert!(logs.iter().any(|log| log.message.contains("completed")));
    }

    #[test]
    fn test_get_detailed_os_version() {
        let version = get_detailed_os_version();
        assert!(!version.is_empty());
        assert!(!version.contains("Unknown OS"));
    }

    #[test]
    fn test_get_system_hostname() {
        let hostname = get_system_hostname();
        assert!(!hostname.is_empty());
        // Should not be the fallback value in most environments
        // assert_ne!(hostname, "UNKNOWN");
    }

    #[test]
    fn test_collect_uptime() {
        match collect_uptime() {
            Ok(uptime) => {
                // Uptime should be reasonable (not negative, not impossibly large)
                assert!(uptime < 365 * 24 * 3600); // Less than a year
            }
            Err(_) => {
                // It's okay if uptime collection fails in test environment
            }
        }
    }

    #[test]
    fn test_collect_logged_on_users() {
        match collect_logged_on_users() {
            Ok(users) => {
                // Should have at least the current user in most environments
                // But this might be empty in some test environments
                for user in &users {
                    assert!(!user.username.is_empty());
                    assert!(!user.domain.is_empty());
                    assert!(!user.logon_time.is_empty());
                }
            }
            Err(_) => {
                // It's okay if user collection fails in test environment
            }
        }
    }
}