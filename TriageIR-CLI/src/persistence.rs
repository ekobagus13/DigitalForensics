use crate::types::{PersistenceMechanism, PersistenceType, LogEntry};
use winreg::enums::*;
use winreg::RegKey;
use std::path::Path;
use std::fs;
use std::process::Command;
use regex::Regex;

/// Collect all persistence mechanisms found on the system
pub fn collect_persistence_mechanisms() -> (Vec<PersistenceMechanism>, Vec<LogEntry>) {
    let mut logs = Vec::new();
    logs.push(LogEntry::info("Starting persistence mechanism detection"));
    
    let mut mechanisms = Vec::new();
    
    // Collect Registry Run keys
    match collect_registry_run_keys() {
        Ok(run_keys) => {
            let count = run_keys.len();
            mechanisms.extend(run_keys);
            logs.push(LogEntry::info(&format!("Found {} Registry Run key entries", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect Registry Run keys: {}", e)));
        }
    }
    
    // Collect Startup folder entries
    match collect_startup_folder_entries() {
        Ok(startup_entries) => {
            let count = startup_entries.len();
            mechanisms.extend(startup_entries);
            logs.push(LogEntry::info(&format!("Found {} Startup folder entries", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect Startup folder entries: {}", e)));
        }
    }
    
    // Collect Windows Services (basic detection)
    match collect_service_persistence() {
        Ok(services) => {
            let count = services.len();
            mechanisms.extend(services);
            logs.push(LogEntry::info(&format!("Found {} potentially suspicious services", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect service information: {}", e)));
        }
    }
    
    // Collect Scheduled Tasks via Windows Task Scheduler API
    match collect_scheduled_tasks() {
        Ok(tasks) => {
            let count = tasks.len();
            mechanisms.extend(tasks);
            logs.push(LogEntry::info(&format!("Found {} scheduled tasks", count)));
        }
        Err(e) => {
            logs.push(LogEntry::warn(&format!("Failed to collect scheduled tasks: {}", e)));
        }
    }
    
    // Sort mechanisms by type and name for consistent output
    mechanisms.sort_by(|a, b| {
        a.mechanism_type.cmp(&b.mechanism_type)
            .then_with(|| a.name.cmp(&b.name))
    });
    
    let total_mechanisms = mechanisms.len();
    logs.push(LogEntry::info(&format!("Total persistence mechanisms found: {}", total_mechanisms)));
    logs.push(LogEntry::info("Persistence mechanism detection completed"));
    
    (mechanisms, logs)
}

/// Collect Registry Run key entries
fn collect_registry_run_keys() -> Result<Vec<PersistenceMechanism>, String> {
    let mut mechanisms = Vec::new();
    
    // Common Registry Run key locations
    let run_key_paths = vec![
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce"),
        (HKEY_CURRENT_USER, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"),
        (HKEY_CURRENT_USER, r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\RunOnce"),
    ];
    
    for (hive, path) in run_key_paths {
        match RegKey::predef(hive).open_subkey(path) {
            Ok(key) => {
                for value_name in key.enum_values().filter_map(|v| v.ok()) {
                    let name = value_name.0;
                    match key.get_value::<String, _>(&name) {
                        Ok(command) => {
                            let source = format!("{}\\{}", 
                                hive_to_string(hive), 
                                path
                            );
                            
                            mechanisms.push(PersistenceMechanism::new(
                                PersistenceType::RegistryRunKey.as_str().to_string(),
                                name,
                                command,
                                source,
                            ));
                        }
                        Err(_) => {
                            // Skip values that can't be read as strings
                        }
                    }
                }
            }
            Err(_) => {
                // Key doesn't exist or can't be accessed - this is normal
            }
        }
    }
    
    Ok(mechanisms)
}

/// Collect Startup folder entries
fn collect_startup_folder_entries() -> Result<Vec<PersistenceMechanism>, String> {
    let mut mechanisms = Vec::new();
    
    // Common startup folder locations
    let startup_paths = vec![
        get_startup_folder_path("common"),
        get_startup_folder_path("user"),
    ];
    
    for startup_path in startup_paths {
        if let Some(path) = startup_path {
            if Path::new(&path).exists() {
                match fs::read_dir(&path) {
                    Ok(entries) => {
                        for entry in entries.filter_map(|e| e.ok()) {
                            let file_path = entry.path();
                            if let Some(file_name) = file_path.file_name() {
                                let name = file_name.to_string_lossy().to_string();
                                let command = file_path.to_string_lossy().to_string();
                                
                                mechanisms.push(PersistenceMechanism::new(
                                    PersistenceType::StartupFolder.as_str().to_string(),
                                    name,
                                    command,
                                    path.clone(),
                                ));
                            }
                        }
                    }
                    Err(_) => {
                        // Can't read directory - skip
                    }
                }
            }
        }
    }
    
    Ok(mechanisms)
}

/// Collect potentially suspicious Windows Services
fn collect_service_persistence() -> Result<Vec<PersistenceMechanism>, String> {
    let mut mechanisms = Vec::new();
    
    // Access Services registry key
    let services_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"SYSTEM\CurrentControlSet\Services")
        .map_err(|e| format!("Failed to open Services registry key: {}", e))?;
    
    for service_name in services_key.enum_keys().filter_map(|k| k.ok()) {
        if let Ok(service_key) = services_key.open_subkey(&service_name) {
            // Check if this is a user-mode service with an executable
            if let Ok(image_path) = service_key.get_value::<String, _>("ImagePath") {
                // Look for potentially suspicious characteristics
                if is_potentially_suspicious_service(&service_name, &image_path) {
                    let source = format!(r"HKLM\SYSTEM\CurrentControlSet\Services\{}", service_name);
                    
                    mechanisms.push(PersistenceMechanism::new(
                        PersistenceType::Service.as_str().to_string(),
                        service_name,
                        image_path,
                        source,
                    ));
                }
            }
        }
    }
    
    Ok(mechanisms)
}

/// Check if a service might be suspicious (basic heuristics)
fn is_potentially_suspicious_service(name: &str, image_path: &str) -> bool {
    let name_lower = name.to_lowercase();
    let path_lower = image_path.to_lowercase();
    
    // Skip well-known Windows services
    let known_safe_services = vec![
        "wuauserv", "bits", "eventlog", "winmgmt", "schedule", "themes",
        "audiosrv", "browser", "dhcp", "dnscache", "lanmanserver",
        "lanmanworkstation", "netlogon", "nla", "policyagent", "rpcss",
        "samss", "seclogon", "sens", "sharedaccess", "shellhwdetection",
        "spooler", "srservice", "ssdpsrv", "stisvc", "tapisrv", "termservice",
        "w32time", "winhttp", "wmi", "wsearch", "wuauserv"
    ];
    
    if known_safe_services.iter().any(|&safe| name_lower.contains(safe)) {
        return false;
    }
    
    // Look for suspicious characteristics
    let suspicious_paths = vec![
        "temp", "tmp", "appdata", "downloads", "desktop", "documents",
        "public", "users\\public", "programdata"
    ];
    
    let suspicious_extensions = vec![
        ".bat", ".cmd", ".ps1", ".vbs", ".js", ".jar", ".scr"
    ];
    
    // Check for suspicious paths
    if suspicious_paths.iter().any(|&susp| path_lower.contains(susp)) {
        return true;
    }
    
    // Check for suspicious file extensions
    if suspicious_extensions.iter().any(|&ext| path_lower.ends_with(ext)) {
        return true;
    }
    
    // Check for executables not in standard system directories
    if !path_lower.contains("system32") && 
       !path_lower.contains("syswow64") && 
       !path_lower.contains("program files") &&
       !path_lower.contains("windows\\") {
        return true;
    }
    
    false
}

/// Get startup folder path
fn get_startup_folder_path(folder_type: &str) -> Option<String> {
    match folder_type {
        "common" => {
            // All Users startup folder
            std::env::var("ALLUSERSPROFILE").ok()
                .map(|path| format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", path))
        }
        "user" => {
            // Current user startup folder
            std::env::var("APPDATA").ok()
                .map(|path| format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", path))
        }
        _ => None,
    }
}

/// Convert registry hive to string representation
fn hive_to_string(hive: HKEY) -> &'static str {
    match hive {
        HKEY_LOCAL_MACHINE => "HKLM",
        HKEY_CURRENT_USER => "HKCU",
        HKEY_CLASSES_ROOT => "HKCR",
        HKEY_USERS => "HKU",
        HKEY_CURRENT_CONFIG => "HKCC",
        _ => "UNKNOWN",
    }
}

/// Filter mechanisms by type
pub fn filter_mechanisms_by_type(mechanisms: &[PersistenceMechanism], mechanism_type: &str) -> Vec<&PersistenceMechanism> {
    mechanisms.iter().filter(|m| m.mechanism_type == mechanism_type).collect()
}

/// Find mechanisms with suspicious characteristics
pub fn find_suspicious_mechanisms(mechanisms: &[PersistenceMechanism]) -> Vec<&PersistenceMechanism> {
    mechanisms.iter().filter(|m| is_mechanism_suspicious(m)).collect()
}

/// Collect scheduled tasks via Windows Task Scheduler API
fn collect_scheduled_tasks() -> Result<Vec<PersistenceMechanism>, String> {
    let mut mechanisms = Vec::new();
    
    // Use schtasks.exe to enumerate all scheduled tasks
    match Command::new("schtasks")
        .args(&["/query", "/fo", "csv", "/v"])
        .output() {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                
                if lines.len() > 1 {
                    // Parse CSV header to understand column positions
                    let header = lines[0];
                    let columns: Vec<&str> = parse_csv_line(header);
                    
                    // Find column indices
                    let task_name_idx = find_column_index(&columns, "TaskName");
                    let status_idx = find_column_index(&columns, "Status");
                    let run_as_user_idx = find_column_index(&columns, "Run As User");
                    let task_to_run_idx = find_column_index(&columns, "Task To Run");
                    
                    // Parse each task line
                    for line in lines.iter().skip(1) {
                        if line.trim().is_empty() {
                            continue;
                        }
                        
                        let fields = parse_csv_line(line);
                        
                        let task_name = get_field(&fields, task_name_idx).unwrap_or("Unknown");
                        let task_path = if task_name.starts_with('\\') {
                            task_name.to_string()
                        } else {
                            format!("\\{}", task_name)
                        };
                        
                        let status = get_field(&fields, status_idx).unwrap_or("Unknown");
                        let run_as_user = get_field(&fields, run_as_user_idx).unwrap_or("Unknown");
                        let command = get_field(&fields, task_to_run_idx).unwrap_or("Unknown");
                        
                        // Only include enabled/ready tasks or suspicious ones
                        if status.eq_ignore_ascii_case("Ready") || 
                           status.eq_ignore_ascii_case("Running") ||
                           is_suspicious_task_command(command) {
                            
                            let clean_name = extract_task_name(&task_path);
                            let source = format!("Task Scheduler: {}", task_path);
                            
                            mechanisms.push(PersistenceMechanism::new(
                                PersistenceType::ScheduledTask.as_str().to_string(),
                                clean_name,
                                format!("{} (User: {})", command, run_as_user),
                                source,
                            ));
                        }
                    }
                }
            } else {
                return Err("schtasks command failed".to_string());
            }
        }
        Err(e) => {
            return Err(format!("Failed to execute schtasks: {}", e));
        }
    }
    
    Ok(mechanisms)
}

/// Check if a scheduled task command appears suspicious
fn is_suspicious_task_command(command: &str) -> bool {
    let command_lower = command.to_lowercase();
    
    let suspicious_patterns = vec![
        "powershell", "cmd.exe", "wscript", "cscript", "regsvr32",
        "rundll32", "mshta", "bitsadmin", "certutil", "temp\\", "tmp\\",
        "appdata\\", "downloads\\", "public\\", ".bat", ".cmd", ".ps1", ".vbs", ".js"
    ];
    
    suspicious_patterns.iter().any(|&pattern| command_lower.contains(pattern))
}

/// Parse CSV line (simple implementation)
fn parse_csv_line(line: &str) -> Vec<&str> {
    // Simple CSV parsing - handles quoted fields
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                fields.push(current_field.trim());
                current_field = String::new();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }
    
    // Add the last field
    fields.push(current_field.trim());
    
    // Convert to Vec<&str> by leaking the strings (for simplicity in this context)
    fields.into_iter().map(|s| Box::leak(s.into_boxed_str()) as &str).collect()
}

/// Find column index by name
fn find_column_index(columns: &[&str], column_name: &str) -> Option<usize> {
    columns.iter().position(|&col| col.eq_ignore_ascii_case(column_name))
}

/// Get field by index
fn get_field(fields: &[&str], index: Option<usize>) -> Option<&str> {
    index.and_then(|i| fields.get(i)).copied()
}

/// Extract task name from path
fn extract_task_name(task_path: &str) -> String {
    task_path.split('\\').last().unwrap_or(task_path).to_string()
}

/// Check if a persistence mechanism appears suspicious
fn is_mechanism_suspicious(mechanism: &PersistenceMechanism) -> bool {
    let command_lower = mechanism.command.to_lowercase();
    
    // Suspicious file locations
    let suspicious_locations = vec![
        "temp", "tmp", "appdata\\local\\temp", "downloads", "desktop",
        "documents", "public", "users\\public", "programdata"
    ];
    
    // Suspicious file extensions
    let suspicious_extensions = vec![
        ".bat", ".cmd", ".ps1", ".vbs", ".js", ".jar", ".scr", ".pif"
    ];
    
    // Suspicious command patterns
    let suspicious_patterns = vec![
        "powershell", "cmd.exe", "wscript", "cscript", "regsvr32",
        "rundll32", "mshta", "bitsadmin", "certutil"
    ];
    
    // Check for suspicious locations
    if suspicious_locations.iter().any(|&loc| command_lower.contains(loc)) {
        return true;
    }
    
    // Check for suspicious extensions
    if suspicious_extensions.iter().any(|&ext| command_lower.contains(ext)) {
        return true;
    }
    
    // Check for suspicious command patterns
    if suspicious_patterns.iter().any(|&pattern| command_lower.contains(pattern)) {
        return true;
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_persistence_mechanisms() {
        let (mechanisms, logs) = collect_persistence_mechanisms();
        
        // Should have log entries
        assert!(!logs.is_empty());
        
        // Should have start and completion messages
        assert!(logs.iter().any(|log| log.message.contains("Starting persistence mechanism")));
        assert!(logs.iter().any(|log| log.message.contains("completed")));
        
        // Mechanisms should be sorted
        for i in 1..mechanisms.len() {
            let prev = &mechanisms[i-1];
            let curr = &mechanisms[i];
            assert!(prev.mechanism_type <= curr.mechanism_type);
            if prev.mechanism_type == curr.mechanism_type {
                assert!(prev.name <= curr.name);
            }
        }
    }

    #[test]
    fn test_is_potentially_suspicious_service() {
        // Known safe service should not be flagged
        assert!(!is_potentially_suspicious_service("wuauserv", "C:\\Windows\\System32\\svchost.exe"));
        
        // Service in temp directory should be flagged
        assert!(is_potentially_suspicious_service("malware", "C:\\Temp\\malware.exe"));
        
        // Service with suspicious extension should be flagged
        assert!(is_potentially_suspicious_service("script", "C:\\Windows\\script.bat"));
        
        // Service outside standard directories should be flagged
        assert!(is_potentially_suspicious_service("custom", "C:\\CustomApp\\service.exe"));
    }

    #[test]
    fn test_is_mechanism_suspicious() {
        let suspicious_mechanism = PersistenceMechanism::new(
            "Registry Run Key".to_string(),
            "malware".to_string(),
            "C:\\Temp\\malware.exe".to_string(),
            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
        );
        assert!(is_mechanism_suspicious(&suspicious_mechanism));
        
        let normal_mechanism = PersistenceMechanism::new(
            "Registry Run Key".to_string(),
            "Adobe Updater".to_string(),
            "C:\\Program Files\\Adobe\\Updater\\updater.exe".to_string(),
            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
        );
        assert!(!is_mechanism_suspicious(&normal_mechanism));
        
        let powershell_mechanism = PersistenceMechanism::new(
            "Registry Run Key".to_string(),
            "script".to_string(),
            "powershell.exe -ExecutionPolicy Bypass -File C:\\script.ps1".to_string(),
            "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
        );
        assert!(is_mechanism_suspicious(&powershell_mechanism));
    }

    #[test]
    fn test_filter_mechanisms_by_type() {
        let mechanisms = vec![
            PersistenceMechanism::new("Registry Run Key".to_string(), "app1".to_string(), "cmd1".to_string(), "source1".to_string()),
            PersistenceMechanism::new("Startup Folder".to_string(), "app2".to_string(), "cmd2".to_string(), "source2".to_string()),
            PersistenceMechanism::new("Registry Run Key".to_string(), "app3".to_string(), "cmd3".to_string(), "source3".to_string()),
        ];
        
        let run_keys = filter_mechanisms_by_type(&mechanisms, "Registry Run Key");
        assert_eq!(run_keys.len(), 2);
        
        let startup = filter_mechanisms_by_type(&mechanisms, "Startup Folder");
        assert_eq!(startup.len(), 1);
    }

    #[test]
    fn test_hive_to_string() {
        assert_eq!(hive_to_string(HKEY_LOCAL_MACHINE), "HKLM");
        assert_eq!(hive_to_string(HKEY_CURRENT_USER), "HKCU");
        assert_eq!(hive_to_string(HKEY_CLASSES_ROOT), "HKCR");
    }

    #[test]
    fn test_is_suspicious_task_command() {
        // Suspicious commands should be flagged
        assert!(is_suspicious_task_command("powershell.exe -ExecutionPolicy Bypass"));
        assert!(is_suspicious_task_command("cmd.exe /c malware.bat"));
        assert!(is_suspicious_task_command("C:\\Temp\\malware.exe"));
        assert!(is_suspicious_task_command("wscript.exe script.vbs"));
        
        // Normal commands should not be flagged
        assert!(!is_suspicious_task_command("C:\\Program Files\\Adobe\\Updater\\updater.exe"));
        assert!(!is_suspicious_task_command("C:\\Windows\\System32\\svchost.exe"));
    }

    #[test]
    fn test_parse_csv_line() {
        let line = r#""Task Name","Status","Run As User""#;
        let fields = parse_csv_line(line);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0], "Task Name");
        assert_eq!(fields[1], "Status");
        assert_eq!(fields[2], "Run As User");
        
        let line_with_commas = r#""Adobe Acrobat Update Task","Ready","NT AUTHORITY\SYSTEM""#;
        let fields = parse_csv_line(line_with_commas);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0], "Adobe Acrobat Update Task");
        assert_eq!(fields[1], "Ready");
        assert_eq!(fields[2], "NT AUTHORITY\\SYSTEM");
    }

    #[test]
    fn test_find_column_index() {
        let columns = vec!["TaskName", "Status", "Run As User"];
        assert_eq!(find_column_index(&columns, "TaskName"), Some(0));
        assert_eq!(find_column_index(&columns, "Status"), Some(1));
        assert_eq!(find_column_index(&columns, "Run As User"), Some(2));
        assert_eq!(find_column_index(&columns, "NonExistent"), None);
    }

    #[test]
    fn test_extract_task_name() {
        assert_eq!(extract_task_name("\\Microsoft\\Windows\\UpdateOrchestrator\\Schedule Scan"), "Schedule Scan");
        assert_eq!(extract_task_name("\\Adobe Updater Task"), "Adobe Updater Task");
        assert_eq!(extract_task_name("SimpleTask"), "SimpleTask");
    }
}