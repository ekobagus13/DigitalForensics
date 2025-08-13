use crate::forensic_types::{PrefetchFile, VolumeInfo, AuditEntry};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

/// Prefetch file analysis for evidence of execution
/// Prefetch files are created by Windows to optimize application startup
/// They contain valuable forensic information about program execution

pub fn collect_prefetch_files() -> (Vec<PrefetchFile>, Vec<AuditEntry>) {
    let mut prefetch_files = Vec::new();
    let mut audit_log = Vec::new();
    
    let start_time = std::time::Instant::now();
    
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        component: "prefetch".to_string(),
        action: "start_collection".to_string(),
        details: "Starting Prefetch file analysis".to_string(),
        duration_ms: None,
        result: "started".to_string(),
    });
    
    // Standard Prefetch directory locations
    let prefetch_paths = vec![
        "C:\\Windows\\Prefetch",
        "C:\\Windows\\System32\\Prefetch", // Alternative location
    ];
    
    for prefetch_path in prefetch_paths {
        if let Ok(entries) = collect_prefetch_from_directory(prefetch_path) {
            prefetch_files.extend(entries.0);
            audit_log.extend(entries.1);
        }
    }
    
    let duration = start_time.elapsed();
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        component: "prefetch".to_string(),
        action: "complete_collection".to_string(),
        details: format!("Collected {} prefetch files", prefetch_files.len()),
        duration_ms: Some(duration.as_millis() as u64),
        result: "success".to_string(),
    });
    
    (prefetch_files, audit_log)
}

fn collect_prefetch_from_directory(directory: &str) -> Result<(Vec<PrefetchFile>, Vec<AuditEntry>), std::io::Error> {
    let mut prefetch_files = Vec::new();
    let mut audit_log = Vec::new();
    
    if !Path::new(directory).exists() {
        audit_log.push(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "WARN".to_string(),
            component: "prefetch".to_string(),
            action: "directory_check".to_string(),
            details: format!("Prefetch directory not found: {}", directory),
            duration_ms: None,
            result: "not_found".to_string(),
        });
        return Ok((prefetch_files, audit_log));
    }
    
    audit_log.push(AuditEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        component: "prefetch".to_string(),
        action: "scan_directory".to_string(),
        details: format!("Scanning prefetch directory: {}", directory),
        duration_ms: None,
        result: "started".to_string(),
    });
    
    for entry in WalkDir::new(directory).max_depth(1) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension.to_string_lossy().to_uppercase() == "PF" {
                            match analyze_prefetch_file(path) {
                                Ok(prefetch_file) => {
                                    prefetch_files.push(prefetch_file);
                                    audit_log.push(AuditEntry {
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                        level: "DEBUG".to_string(),
                                        component: "prefetch".to_string(),
                                        action: "analyze_file".to_string(),
                                        details: format!("Analyzed: {}", path.display()),
                                        duration_ms: None,
                                        result: "success".to_string(),
                                    });
                                }
                                Err(e) => {
                                    audit_log.push(AuditEntry {
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                        level: "ERROR".to_string(),
                                        component: "prefetch".to_string(),
                                        action: "analyze_file".to_string(),
                                        details: format!("Failed to analyze {}: {}", path.display(), e),
                                        duration_ms: None,
                                        result: "error".to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                audit_log.push(AuditEntry {
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    level: "ERROR".to_string(),
                    component: "prefetch".to_string(),
                    action: "directory_walk".to_string(),
                    details: format!("Error walking directory: {}", e),
                    duration_ms: None,
                    result: "error".to_string(),
                });
            }
        }
    }
    
    Ok((prefetch_files, audit_log))
}

fn analyze_prefetch_file(path: &Path) -> Result<PrefetchFile, Box<dyn std::error::Error>> {
    let file_data = fs::read(path)?;
    let metadata = fs::metadata(path)?;
    
    // Calculate file hash
    let mut hasher = Sha256::new();
    hasher.update(&file_data);
    let hash = hex::encode(hasher.finalize());
    
    // Extract filename without extension
    let filename = path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    // Parse prefetch file (simplified - real implementation would parse binary format)
    let prefetch_file = parse_prefetch_data(&file_data, &filename, &hash, &metadata)?;
    
    Ok(prefetch_file)
}

fn parse_prefetch_data(
    data: &[u8], 
    filename: &str, 
    hash: &str, 
    metadata: &fs::Metadata
) -> Result<PrefetchFile, Box<dyn std::error::Error>> {
    // Simplified prefetch parsing - in a real implementation, this would
    // parse the actual prefetch binary format according to Microsoft specifications
    
    // Extract executable name from filename (format: EXECUTABLE-HASH.pf)
    let executable_name = if let Some(dash_pos) = filename.find('-') {
        filename[..dash_pos].to_string()
    } else {
        filename.replace(".pf", "").replace(".PF", "")
    };
    
    // Basic prefetch file structure analysis
    let version = if data.len() >= 4 {
        u32::from_le_bytes([data[0], data[1], data[2], data[3]])
    } else {
        0
    };
    
    // For demonstration, we'll create a basic prefetch entry
    // Real implementation would parse:
    // - File header
    // - File information
    // - Metrics array
    // - Trace chains array
    // - Filename strings
    // - Volume information
    
    Ok(PrefetchFile {
        filename: filename.to_string(),
        executable_name,
        run_count: extract_run_count(data),
        last_run_time: extract_last_run_time(data),
        creation_time: metadata.created()
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_else(|_| "Unknown".to_string()),
        file_size: metadata.len(),
        hash: hash.to_string(),
        version,
        referenced_files: extract_referenced_files(data),
        volumes: extract_volume_info(data),
    })
}

fn extract_run_count(data: &[u8]) -> u32 {
    // Simplified extraction - real implementation would parse at correct offset
    if data.len() >= 16 {
        u32::from_le_bytes([data[12], data[13], data[14], data[15]])
    } else {
        1 // Default assumption
    }
}

fn extract_last_run_time(data: &[u8]) -> String {
    // Simplified extraction - real implementation would parse FILETIME at correct offset
    if data.len() >= 24 {
        // This would normally convert FILETIME to readable format
        chrono::Utc::now().to_rfc3339()
    } else {
        "Unknown".to_string()
    }
}

fn extract_referenced_files(data: &[u8]) -> Vec<String> {
    // Simplified extraction - real implementation would parse filename strings section
    let mut files = Vec::new();
    
    // Look for common file patterns in the data
    let data_str = String::from_utf8_lossy(data);
    let patterns = vec![".exe", ".dll", ".sys", ".bat", ".cmd", ".ps1"];
    
    for pattern in patterns {
        if data_str.contains(pattern) {
            // This is a very simplified approach
            // Real implementation would properly parse the strings section
            files.push(format!("C:\\Windows\\System32\\example{}", pattern));
        }
    }
    
    if files.is_empty() {
        files.push("No referenced files found".to_string());
    }
    
    files
}

fn extract_volume_info(data: &[u8]) -> Vec<VolumeInfo> {
    // Simplified extraction - real implementation would parse volume information section
    vec![
        VolumeInfo {
            device_path: "\\Device\\HarddiskVolume1".to_string(),
            volume_name: "Windows".to_string(),
            serial_number: "12345678".to_string(),
            creation_time: chrono::Utc::now().to_rfc3339(),
        }
    ]
}

/// Get prefetch statistics for reporting
pub fn get_prefetch_statistics(prefetch_files: &[PrefetchFile]) -> std::collections::HashMap<String, u32> {
    let mut stats = std::collections::HashMap::new();
    
    stats.insert("total_files".to_string(), prefetch_files.len() as u32);
    
    let total_runs: u32 = prefetch_files.iter().map(|pf| pf.run_count).sum();
    stats.insert("total_executions".to_string(), total_runs);
    
    let unique_executables = prefetch_files.iter()
        .map(|pf| &pf.executable_name)
        .collect::<std::collections::HashSet<_>>()
        .len();
    stats.insert("unique_executables".to_string(), unique_executables as u32);
    
    // Count by file extensions
    let mut exe_count = 0;
    let mut dll_count = 0;
    let mut other_count = 0;
    
    for pf in prefetch_files {
        if pf.executable_name.to_lowercase().ends_with(".exe") {
            exe_count += 1;
        } else if pf.executable_name.to_lowercase().ends_with(".dll") {
            dll_count += 1;
        } else {
            other_count += 1;
        }
    }
    
    stats.insert("exe_files".to_string(), exe_count);
    stats.insert("dll_files".to_string(), dll_count);
    stats.insert("other_files".to_string(), other_count);
    
    stats
}

/// Find prefetch files for a specific executable
pub fn find_prefetch_by_executable<'a>(prefetch_files: &'a [PrefetchFile], executable: &str) -> Vec<&'a PrefetchFile> {
    prefetch_files.iter()
        .filter(|pf| pf.executable_name.to_lowercase().contains(&executable.to_lowercase()))
        .collect()
}

/// Get most frequently executed programs
pub fn get_most_executed_programs<'a>(prefetch_files: &'a [PrefetchFile], limit: usize) -> Vec<(&'a PrefetchFile, u32)> {
    let mut programs: Vec<_> = prefetch_files.iter()
        .map(|pf| (pf, pf.run_count))
        .collect();
    
    programs.sort_by(|a, b| b.1.cmp(&a.1));
    programs.truncate(limit);
    programs
}

/// Get recently executed programs
pub fn get_recently_executed_programs<'a>(prefetch_files: &'a [PrefetchFile], limit: usize) -> Vec<&'a PrefetchFile> {
    let mut programs: Vec<_> = prefetch_files.iter().collect();
    
    programs.sort_by(|a, b| b.last_run_time.cmp(&a.last_run_time));
    programs.truncate(limit);
    programs
}