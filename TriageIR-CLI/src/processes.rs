use crate::types::{Process, LogEntry};
use sysinfo::{System, Pid};
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

/// Collect information about all running processes
pub fn collect_processes() -> (Vec<Process>, Vec<LogEntry>) {
    let mut logs = Vec::new();
    logs.push(LogEntry::info("Starting process enumeration"));
    
    let mut processes = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_processes();
    
    let total_processes = sys.processes().len();
    logs.push(LogEntry::info(&format!("Found {} running processes", total_processes)));
    
    let mut successful_collections = 0;
    let mut hash_calculation_errors = 0;
    
    for (pid, process) in sys.processes() {
        match collect_single_process(*pid, process) {
            Ok(mut proc_info) => {
                // Calculate SHA-256 hash of executable if path is available
                if proc_info.has_executable_path() {
                    match calculate_file_hash(&proc_info.executable_path) {
                        Ok(hash) => {
                            proc_info.sha256_hash = hash;
                        }
                        Err(e) => {
                            hash_calculation_errors += 1;
                            proc_info.sha256_hash = "ERROR".to_string();
                            if hash_calculation_errors <= 5 { // Limit error logging
                                logs.push(LogEntry::warn(&format!("Failed to calculate hash for {}: {}", proc_info.executable_path, e)));
                            }
                        }
                    }
                } else {
                    proc_info.sha256_hash = "N/A".to_string();
                }
                
                // For now, just add a placeholder for loaded modules
                // TODO: Implement Windows API-based module enumeration in next iteration
                proc_info.loaded_modules = Vec::new();
                
                processes.push(proc_info);
                successful_collections += 1;
            }
            Err(e) => {
                logs.push(LogEntry::warn(&format!("Failed to collect process info for PID {}: {}", pid, e)));
            }
        }
    }
    
    logs.push(LogEntry::info(&format!("Successfully collected {} process details", successful_collections)));
    
    if hash_calculation_errors > 0 {
        logs.push(LogEntry::warn(&format!("Failed to calculate hashes for {} processes", hash_calculation_errors)));
    }
    
    // Sort processes by PID for consistent output
    processes.sort_by(|a, b| a.pid.cmp(&b.pid));
    
    logs.push(LogEntry::info("Process enumeration completed"));
    (processes, logs)
}

/// Collect information about a single process
fn collect_single_process(pid: Pid, process: &sysinfo::Process) -> std::result::Result<Process, String> {
    let pid_u32 = pid.as_u32();
    let parent_pid = process.parent().map(|p| p.as_u32()).unwrap_or(0);
    
    let name = process.name().to_string();
    
    // Get command line - handle potential access issues
    let command_line = process.cmd().join(" ");
    
    // Get executable path
    let executable_path = process.exe()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "N/A".to_string());
    
    Ok(Process::new(
        pid_u32,
        parent_pid,
        name,
        command_line,
        executable_path,
    ))
}

/// Calculate SHA-256 hash of a file
fn calculate_file_hash(file_path: &str) -> std::result::Result<String, String> {
    if file_path == "N/A" || file_path.is_empty() {
        return Err("Invalid file path".to_string());
    }
    
    let path = Path::new(file_path);
    if !path.exists() {
        return Err("File does not exist".to_string());
    }
    
    // Read file contents
    let file_contents = fs::read(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&file_contents);
    let result = hasher.finalize();
    
    Ok(hex::encode(result))
}

/// Get process tree information (parent-child relationships)
pub fn build_process_tree(processes: &[Process]) -> Vec<(u32, Vec<u32>)> {
    let mut tree = Vec::new();
    
    for process in processes {
        let children: Vec<u32> = processes
            .iter()
            .filter(|p| p.parent_pid == process.pid)
            .map(|p| p.pid)
            .collect();
        
        if !children.is_empty() {
            tree.push((process.pid, children));
        }
    }
    
    tree
}

/// Find processes by name (case-insensitive)
pub fn find_processes_by_name<'a>(processes: &'a [Process], name: &str) -> Vec<&'a Process> {
    let name_lower = name.to_lowercase();
    processes
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&name_lower))
        .collect()
}

/// Find processes with external network connections
pub fn find_processes_with_network_activity<'a>(processes: &'a [Process], network_pids: &[u32]) -> Vec<&'a Process> {
    processes
        .iter()
        .filter(|p| network_pids.contains(&p.pid))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_collect_processes() {
        let (processes, logs) = collect_processes();
        
        // Should have some processes (at least the current process)
        assert!(!processes.is_empty());
        
        // Should have log entries
        assert!(!logs.is_empty());
        
        // Should have start and completion messages
        assert!(logs.iter().any(|log| log.message.contains("Starting process enumeration")));
        assert!(logs.iter().any(|log| log.message.contains("completed")));
        
        // Processes should be sorted by PID
        for i in 1..processes.len() {
            assert!(processes[i-1].pid <= processes[i].pid);
        }
        
        // Each process should have basic information
        for process in &processes {
            assert!(process.pid > 0);
            assert!(!process.name.is_empty());
            // Should have empty modules list for now
            assert_eq!(process.loaded_modules.len(), 0);
        }
    }

    #[test]
    fn test_calculate_file_hash() {
        // Create a temporary file for testing
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, World!").unwrap();
        
        let file_path_str = file_path.to_string_lossy().to_string();
        let hash = calculate_file_hash(&file_path_str).unwrap();
        
        // Should be a valid SHA-256 hash (64 hex characters)
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        
        // Test with non-existent file
        let result = calculate_file_hash("non_existent_file.txt");
        assert!(result.is_err());
        
        // Test with invalid path
        let result = calculate_file_hash("N/A");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_process_tree() {
        let processes = vec![
            Process::new(1, 0, "init".to_string(), "init".to_string(), "/sbin/init".to_string()),
            Process::new(2, 1, "child1".to_string(), "child1".to_string(), "/bin/child1".to_string()),
            Process::new(3, 1, "child2".to_string(), "child2".to_string(), "/bin/child2".to_string()),
            Process::new(4, 2, "grandchild".to_string(), "grandchild".to_string(), "/bin/grandchild".to_string()),
        ];
        
        let tree = build_process_tree(&processes);
        
        // Should have entries for processes with children
        assert!(!tree.is_empty());
        
        // Process 1 should have children 2 and 3
        let process_1_children = tree.iter().find(|(pid, _)| *pid == 1).map(|(_, children)| children);
        assert!(process_1_children.is_some());
        let children = process_1_children.unwrap();
        assert!(children.contains(&2));
        assert!(children.contains(&3));
    }

    #[test]
    fn test_find_processes_by_name() {
        let processes = vec![
            Process::new(1, 0, "notepad.exe".to_string(), "notepad.exe".to_string(), "C:\\Windows\\notepad.exe".to_string()),
            Process::new(2, 0, "chrome.exe".to_string(), "chrome.exe".to_string(), "C:\\Program Files\\Google\\Chrome\\chrome.exe".to_string()),
            Process::new(3, 0, "firefox.exe".to_string(), "firefox.exe".to_string(), "C:\\Program Files\\Mozilla Firefox\\firefox.exe".to_string()),
        ];
        
        let found = find_processes_by_name(&processes, "chrome");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "chrome.exe");
        
        let found = find_processes_by_name(&processes, "NOTEPAD");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "notepad.exe");
        
        let found = find_processes_by_name(&processes, "nonexistent");
        assert_eq!(found.len(), 0);
    }

    #[test]
    fn test_find_processes_with_network_activity() {
        let processes = vec![
            Process::new(1, 0, "notepad.exe".to_string(), "notepad.exe".to_string(), "C:\\Windows\\notepad.exe".to_string()),
            Process::new(2, 0, "chrome.exe".to_string(), "chrome.exe".to_string(), "C:\\Program Files\\Google\\Chrome\\chrome.exe".to_string()),
            Process::new(3, 0, "firefox.exe".to_string(), "firefox.exe".to_string(), "C:\\Program Files\\Mozilla Firefox\\firefox.exe".to_string()),
        ];
        
        let network_pids = vec![2, 3];
        let found = find_processes_with_network_activity(&processes, &network_pids);
        
        assert_eq!(found.len(), 2);
        assert!(found.iter().any(|p| p.pid == 2));
        assert!(found.iter().any(|p| p.pid == 3));
    }

    #[test]
    fn test_process_module_system_detection() {
        use crate::types::ProcessModule;
        
        let system_module = ProcessModule::new(
            "kernel32.dll".to_string(),
            "C:\\Windows\\System32\\kernel32.dll".to_string(),
            "0x7FF800000000".to_string(),
            1024000,
            "10.0.19041.1".to_string(),
        );
        assert!(system_module.is_system_module());
        
        let user_module = ProcessModule::new(
            "custom.dll".to_string(),
            "C:\\Program Files\\MyApp\\custom.dll".to_string(),
            "0x7FF900000000".to_string(),
            512000,
            "1.0.0.0".to_string(),
        );
        assert!(!user_module.is_system_module());
    }
}