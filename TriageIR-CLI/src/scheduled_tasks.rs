use crate::forensic_types::{ScheduledTask, TaskTrigger, TaskAction, AuditEntry};
use std::process::Command;
use std::collections::HashMap;
use regex::Regex;

/// Scheduled Tasks analysis for persistence mechanisms
/// Scheduled tasks are a common persistence mechanism used by both legitimate software and malware

pub fn collect_scheduled_tasks() -> (Vec<ScheduledTask>, Vec<AuditEntry>) {
    let mut scheduled_tasks = Vec::new();
    let mut audit_log = Vec::new();
    
    let start_time = std::time::Instant::now();
    
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        component: "scheduled_tasks".to_string(),
        action: "start_collection".to_string(),
        details: "Starting scheduled tasks analysis".to_string(),
        duration_ms: None,
        result: "started".to_string(),
    });
    
    // Use schtasks.exe to enumerate all scheduled tasks
    match collect_tasks_via_schtasks() {
        Ok((tasks, logs)) => {
            scheduled_tasks.extend(tasks);
            audit_log.extend(logs);
        }
        Err(e) => {
            audit_log.push(AuditEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "ERROR".to_string(),
                component: "scheduled_tasks".to_string(),
                action: "schtasks_execution".to_string(),
                details: format!("Failed to execute schtasks: {}", e),
                duration_ms: None,
                result: "error".to_string(),
            });
        }
    }
    
    // Also try to collect from registry (backup method)
    match collect_tasks_from_registry() {
        Ok((tasks, logs)) => {
            // Merge with existing tasks, avoiding duplicates
            for task in tasks {
                if !scheduled_tasks.iter().any(|t| t.name == task.name && t.path == task.path) {
                    scheduled_tasks.push(task);
                }
            }
            audit_log.extend(logs);
        }
        Err(e) => {
            audit_log.push(AuditEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "WARN".to_string(),
                component: "scheduled_tasks".to_string(),
                action: "registry_collection".to_string(),
                details: format!("Registry collection failed: {}", e),
                duration_ms: None,
                result: "error".to_string(),
            });
        }
    }
    
    let duration = start_time.elapsed();
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        component: "scheduled_tasks".to_string(),
        action: "complete_collection".to_string(),
        details: format!("Collected {} scheduled tasks", scheduled_tasks.len()),
        duration_ms: Some(duration.as_millis() as u64),
        result: "success".to_string(),
    });
    
    (scheduled_tasks, audit_log)
}

fn collect_tasks_via_schtasks() -> Result<(Vec<ScheduledTask>, Vec<AuditEntry>), Box<dyn std::error::Error>> {
    let mut scheduled_tasks = Vec::new();
    let mut audit_log = Vec::new();
    
    // First, get list of all tasks
    let output = Command::new("schtasks")
        .args(&["/query", "/fo", "csv", "/v"])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("schtasks failed with exit code: {}", output.status).into());
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    
    if lines.is_empty() {
        return Ok((scheduled_tasks, audit_log));
    }
    
    // Parse CSV header to understand column positions
    let header = lines[0];
    let columns: Vec<&str> = parse_csv_line(header);
    
    // Find column indices
    let task_name_idx = find_column_index(&columns, "TaskName");
    let next_run_idx = find_column_index(&columns, "Next Run Time");
    let status_idx = find_column_index(&columns, "Status");
    let last_run_idx = find_column_index(&columns, "Last Run Time");
    let run_as_user_idx = find_column_index(&columns, "Run As User");
    let task_to_run_idx = find_column_index(&columns, "Task To Run");
    
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "DEBUG".to_string(),
        component: "scheduled_tasks".to_string(),
        action: "parse_csv".to_string(),
        details: format!("Parsing {} task entries", lines.len() - 1),
        duration_ms: None,
        result: "started".to_string(),
    });
    
    // Parse each task line
    for (line_num, line) in lines.iter().skip(1).enumerate() {
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
        
        // Get detailed information for this specific task
        match get_task_details(&task_path) {
            Ok(detailed_task) => {
                scheduled_tasks.push(detailed_task);
                audit_log.push(AuditEntry {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    level: "DEBUG".to_string(),
                    component: "scheduled_tasks".to_string(),
                    action: "parse_task".to_string(),
                    details: format!("Parsed task: {}", task_name),
                    duration_ms: None,
                    result: "success".to_string(),
                });
            }
            Err(e) => {
                // Create basic task from CSV data
                let basic_task = ScheduledTask {
                    name: extract_task_name(task_name),
                    path: task_path,
                    state: get_field(&fields, status_idx).unwrap_or("Unknown").to_string(),
                    last_run_time: get_field(&fields, last_run_idx).unwrap_or("Never").to_string(),
                    next_run_time: get_field(&fields, next_run_idx).unwrap_or("Never").to_string(),
                    run_as_user: get_field(&fields, run_as_user_idx).unwrap_or("Unknown").to_string(),
                    command: get_field(&fields, task_to_run_idx).unwrap_or("Unknown").to_string(),
                    arguments: String::new(),
                    working_directory: String::new(),
                    triggers: vec![],
                    actions: vec![],
                    creation_date: "Unknown".to_string(),
                    author: "Unknown".to_string(),
                    description: "Basic task info from CSV".to_string(),
                };
                
                scheduled_tasks.push(basic_task);
                
                audit_log.push(AuditEntry {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    level: "WARN".to_string(),
                    component: "scheduled_tasks".to_string(),
                    action: "parse_task".to_string(),
                    details: format!("Failed to get details for {}: {}", task_name, e),
                    duration_ms: None,
                    result: "partial".to_string(),
                });
            }
        }
    }
    
    Ok((scheduled_tasks, audit_log))
}

fn get_task_details(task_path: &str) -> Result<ScheduledTask, Box<dyn std::error::Error>> {
    // Get detailed XML information for the task
    let output = Command::new("schtasks")
        .args(&["/query", "/tn", task_path, "/xml"])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Failed to get task details for {}", task_path).into());
    }
    
    let xml_content = String::from_utf8_lossy(&output.stdout);
    parse_task_xml(&xml_content, task_path)
}

fn parse_task_xml(xml_content: &str, task_path: &str) -> Result<ScheduledTask, Box<dyn std::error::Error>> {
    // Simplified XML parsing - in production, use a proper XML parser
    let task_name = extract_task_name(task_path);
    
    // Extract basic information using regex patterns
    let author = extract_xml_value(xml_content, "Author").unwrap_or("Unknown".to_string());
    let description = extract_xml_value(xml_content, "Description").unwrap_or("No description".to_string());
    let creation_date = extract_xml_value(xml_content, "Date").unwrap_or("Unknown".to_string());
    
    // Extract command and arguments
    let command = extract_xml_value(xml_content, "Command").unwrap_or("Unknown".to_string());
    let arguments = extract_xml_value(xml_content, "Arguments").unwrap_or(String::new());
    let working_directory = extract_xml_value(xml_content, "WorkingDirectory").unwrap_or(String::new());
    
    // Extract user context
    let run_as_user = extract_xml_value(xml_content, "UserId")
        .or_else(|| extract_xml_value(xml_content, "Principal"))
        .unwrap_or("Unknown".to_string());
    
    // Parse triggers (simplified)
    let triggers = parse_task_triggers(xml_content);
    
    // Parse actions (simplified)
    let actions = parse_task_actions(xml_content);
    
    Ok(ScheduledTask {
        name: task_name,
        path: task_path.to_string(),
        state: "Unknown".to_string(), // Would need additional query
        last_run_time: "Unknown".to_string(),
        next_run_time: "Unknown".to_string(),
        run_as_user,
        command,
        arguments,
        working_directory,
        triggers,
        actions,
        creation_date,
        author,
        description,
    })
}

fn extract_xml_value(xml_content: &str, tag_name: &str) -> Option<String> {
    let pattern = format!(r"<{}>(.*?)</{}>", tag_name, tag_name);
    let re = Regex::new(&pattern).ok()?;
    
    re.captures(xml_content)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

fn parse_task_triggers(xml_content: &str) -> Vec<TaskTrigger> {
    let mut triggers = Vec::new();
    
    // Look for common trigger patterns
    if xml_content.contains("<TimeTrigger>") {
        triggers.push(TaskTrigger {
            trigger_type: "Time".to_string(),
            start_boundary: extract_xml_value(xml_content, "StartBoundary").unwrap_or("Unknown".to_string()),
            end_boundary: extract_xml_value(xml_content, "EndBoundary").unwrap_or("None".to_string()),
            enabled: !xml_content.contains("<Enabled>false</Enabled>"),
            repetition_interval: extract_xml_value(xml_content, "Interval").unwrap_or("None".to_string()),
            repetition_duration: extract_xml_value(xml_content, "Duration").unwrap_or("None".to_string()),
        });
    }
    
    if xml_content.contains("<LogonTrigger>") {
        triggers.push(TaskTrigger {
            trigger_type: "Logon".to_string(),
            start_boundary: "On logon".to_string(),
            end_boundary: "None".to_string(),
            enabled: !xml_content.contains("<Enabled>false</Enabled>"),
            repetition_interval: "None".to_string(),
            repetition_duration: "None".to_string(),
        });
    }
    
    if xml_content.contains("<BootTrigger>") {
        triggers.push(TaskTrigger {
            trigger_type: "Boot".to_string(),
            start_boundary: "On boot".to_string(),
            end_boundary: "None".to_string(),
            enabled: !xml_content.contains("<Enabled>false</Enabled>"),
            repetition_interval: "None".to_string(),
            repetition_duration: "None".to_string(),
        });
    }
    
    if triggers.is_empty() {
        triggers.push(TaskTrigger {
            trigger_type: "Unknown".to_string(),
            start_boundary: "Unknown".to_string(),
            end_boundary: "Unknown".to_string(),
            enabled: true,
            repetition_interval: "Unknown".to_string(),
            repetition_duration: "Unknown".to_string(),
        });
    }
    
    triggers
}

fn parse_task_actions(xml_content: &str) -> Vec<TaskAction> {
    let mut actions = Vec::new();
    
    if xml_content.contains("<Exec>") {
        actions.push(TaskAction {
            action_type: "Execute".to_string(),
            path: extract_xml_value(xml_content, "Command").unwrap_or("Unknown".to_string()),
            arguments: extract_xml_value(xml_content, "Arguments").unwrap_or(String::new()),
            working_directory: extract_xml_value(xml_content, "WorkingDirectory").unwrap_or(String::new()),
        });
    }
    
    if actions.is_empty() {
        actions.push(TaskAction {
            action_type: "Unknown".to_string(),
            path: "Unknown".to_string(),
            arguments: String::new(),
            working_directory: String::new(),
        });
    }
    
    actions
}

fn collect_tasks_from_registry() -> Result<(Vec<ScheduledTask>, Vec<AuditEntry>), Box<dyn std::error::Error>> {
    let mut scheduled_tasks = Vec::new();
    let mut audit_log = Vec::new();
    
    // Registry locations for scheduled tasks
    let task_registry_paths = vec![
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\TaskCache\\Tree",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Schedule\\TaskCache\\Tasks",
    ];
    
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "DEBUG".to_string(),
        component: "scheduled_tasks".to_string(),
        action: "registry_scan".to_string(),
        details: "Scanning registry for scheduled tasks".to_string(),
        duration_ms: None,
        result: "started".to_string(),
    });
    
    // This is a simplified implementation
    // Real implementation would parse the binary task data from registry
    
    Ok((scheduled_tasks, audit_log))
}

// Helper functions
fn parse_csv_line(line: &str) -> Vec<&str> {
    // Simple CSV parsing - in production, use a proper CSV parser
    line.split(',').map(|s| s.trim_matches('"').trim()).collect()
}

fn find_column_index(columns: &[&str], column_name: &str) -> Option<usize> {
    columns.iter().position(|&col| col.eq_ignore_ascii_case(column_name))
}

fn get_field(fields: &[&str], index: Option<usize>) -> Option<&str> {
    index.and_then(|i| fields.get(i)).copied()
}

fn extract_task_name(task_path: &str) -> String {
    task_path.split('\\').last().unwrap_or(task_path).to_string()
}

/// Get scheduled task statistics for reporting
pub fn get_scheduled_task_statistics(tasks: &[ScheduledTask]) -> HashMap<String, u32> {
    let mut stats = HashMap::new();
    
    stats.insert("total_tasks".to_string(), tasks.len() as u32);
    
    let enabled_count = tasks.iter()
        .filter(|task| task.state.eq_ignore_ascii_case("Ready") || task.state.eq_ignore_ascii_case("Running"))
        .count();
    stats.insert("enabled_tasks".to_string(), enabled_count as u32);
    
    let disabled_count = tasks.len() - enabled_count;
    stats.insert("disabled_tasks".to_string(), disabled_count as u32);
    
    // Count by trigger types
    let mut logon_triggers = 0;
    let mut boot_triggers = 0;
    let mut time_triggers = 0;
    
    for task in tasks {
        for trigger in &task.triggers {
            match trigger.trigger_type.as_str() {
                "Logon" => logon_triggers += 1,
                "Boot" => boot_triggers += 1,
                "Time" => time_triggers += 1,
                _ => {}
            }
        }
    }
    
    stats.insert("logon_triggers".to_string(), logon_triggers);
    stats.insert("boot_triggers".to_string(), boot_triggers);
    stats.insert("time_triggers".to_string(), time_triggers);
    
    // Count by authors
    let microsoft_tasks = tasks.iter()
        .filter(|task| task.author.contains("Microsoft"))
        .count();
    stats.insert("microsoft_tasks".to_string(), microsoft_tasks as u32);
    
    let third_party_tasks = tasks.len() - microsoft_tasks;
    stats.insert("third_party_tasks".to_string(), third_party_tasks as u32);
    
    stats
}

/// Find suspicious scheduled tasks
pub fn find_suspicious_tasks(tasks: &[ScheduledTask]) -> Vec<&ScheduledTask> {
    tasks.iter()
        .filter(|task| is_suspicious_task(task))
        .collect()
}

fn is_suspicious_task(task: &ScheduledTask) -> bool {
    let suspicious_indicators = vec![
        // Suspicious paths
        task.command.contains("\\Temp\\"),
        task.command.contains("\\AppData\\"),
        task.command.contains("powershell"),
        task.command.contains("cmd.exe"),
        task.command.contains("wscript"),
        task.command.contains("cscript"),
        
        // Suspicious names
        task.name.len() == 1, // Single character names
        task.name.contains("update") && !task.author.contains("Microsoft"),
        task.name.contains("system") && !task.author.contains("Microsoft"),
        
        // Suspicious triggers
        task.triggers.iter().any(|t| t.trigger_type == "Logon"),
        task.triggers.iter().any(|t| t.trigger_type == "Boot"),
        
        // Suspicious users
        task.run_as_user.contains("SYSTEM"),
        task.run_as_user.contains("Administrator"),
        
        // Non-Microsoft tasks with system-level access
        !task.author.contains("Microsoft") && task.run_as_user.contains("SYSTEM"),
    ];
    
    suspicious_indicators.iter().any(|&indicator| indicator)
}

/// Get tasks that run at startup
pub fn get_startup_tasks(tasks: &[ScheduledTask]) -> Vec<&ScheduledTask> {
    tasks.iter()
        .filter(|task| {
            task.triggers.iter().any(|trigger| {
                trigger.trigger_type == "Boot" || trigger.trigger_type == "Logon"
            })
        })
        .collect()
}

/// Find tasks by command pattern
pub fn find_tasks_by_command(tasks: &[ScheduledTask], pattern: &str) -> Vec<&ScheduledTask> {
    tasks.iter()
        .filter(|task| task.command.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}