use clap::{Arg, Command};
use serde_json::json;
use std::fs;
use std::sync::Arc;
use sysinfo::System;

mod types;
mod processes;
mod system_info;
mod network;
mod persistence;
mod event_logs;
mod logger;
mod prefetch;
mod shimcache;
mod forensic_types;

#[cfg(test)]
mod integration_tests;

use logger::{Logger, error_handling::{ForensicResult, ForensicError, ErrorKind, handle_error_gracefully}};
use types::{ScanResults, LogEntry};

fn main() {
    let matches = Command::new("triageir-cli")
        .version("0.1.0")
        .about("Digital Forensics Triage Tool")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for results (JSON format)")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose output")
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .default_value("json")
                .help("Output format (json)")
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let logger = Arc::new(Logger::new(verbose));
    
    // Initialize scan results with proper error handling
    let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let mut scan_results = ScanResults::new(hostname.clone(), os_version);
    
    logger.info("TriageIR CLI v0.1.0 - Digital Forensics Triage Tool started");
    logger.info(&format!("Target system: {}", hostname));
    logger.info(&format!("Current user: {}", std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())));
    logger.info(&format!("Verbose mode: {}", verbose));

    if verbose {
        println!("TriageIR CLI v0.1.0 - Digital Forensics Triage Tool");
        println!("==================================================");
        println!("Starting forensic data collection...");
        println!("Target system: {}", hostname);
        println!("Current user: {}", std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string()));
    }

    let start_time = std::time::Instant::now();

    // Initialize system information collector with error handling
    let system_info = match collect_system_info_safe(&logger) {
        Some(info) => {
            logger.info("System information collected successfully");
            info
        }
        None => {
            logger.error("Failed to collect system information, using defaults");
            json!({
                "hostname": hostname,
                "os_name": "Unknown",
                "os_version": "Unknown",
                "architecture": std::env::consts::ARCH,
                "current_user": "Unknown",
                "uptime_hours": 0.0,
                "last_boot_time": chrono::Utc::now().to_rfc3339(),
                "total_memory": 0,
                "used_memory": 0,
                "cpu_count": 0
            })
        }
    };
    
    // Collect running processes with comprehensive error handling
    logger.info("Starting process enumeration");
    let (processes_data, process_logs) = processes::collect_processes();
    
    // Add process logs to main logger
    for log in &process_logs {
        scan_results.add_log(log.clone());
    }
    
    let processes = processes_data.into_iter().map(|p| {
        json!({
            "pid": p.pid,
            "parent_pid": p.parent_pid,
            "name": p.name,
            "command_line": p.command_line,
            "executable_path": p.executable_path,
            "sha256_hash": p.sha256_hash,
            "loaded_modules": p.loaded_modules.into_iter().map(|m| {
                json!({
                    "name": m.name,
                    "file_path": m.file_path,
                    "base_address": m.base_address,
                    "size": m.size,
                    "version": m.version,
                    "is_system_module": m.is_system_module()
                })
            }).collect::<Vec<_>>()
        })
    }).collect::<Vec<_>>();
    
    logger.info(&format!("Process enumeration completed: {} processes collected", processes.len()));
    
    // Collect network connections with error handling
    logger.info("Starting network connection enumeration");
    let (network_connections_data, network_logs) = network::collect_network_connections();
    
    // Add network logs to main logger
    for log in &network_logs {
        scan_results.add_log(log.clone());
    }
    
    let network_connections = network_connections_data.into_iter().map(|conn| {
        json!({
            "protocol": conn.protocol,
            "local_address": conn.local_address,
            "remote_address": conn.remote_address,
            "state": conn.state,
            "owning_pid": conn.owning_pid,
            "is_external": conn.is_external()
        })
    }).collect::<Vec<_>>();
    
    logger.info(&format!("Network enumeration completed: {} connections collected", network_connections.len()));
    
    // Collect persistence mechanisms with error handling
    logger.info("Starting persistence mechanism detection");
    let (persistence_mechanisms_data, persistence_logs) = persistence::collect_persistence_mechanisms();
    
    // Add persistence logs to main logger
    for log in &persistence_logs {
        scan_results.add_log(log.clone());
    }
    
    let persistence_mechanisms = persistence_mechanisms_data.into_iter().map(|p| {
        json!({
            "type": p.mechanism_type,
            "name": p.name,
            "command": p.command,
            "source": p.source
        })
    }).collect::<Vec<_>>();
    
    logger.info(&format!("Persistence detection completed: {} mechanisms found", persistence_mechanisms.len()));
    
    // Collect event logs with error handling
    logger.info("Starting event log collection");
    let (event_logs_data, event_logs_collection_logs) = event_logs::collect_event_logs();
    
    // Add event log collection logs to main logger
    for log in &event_logs_collection_logs {
        scan_results.add_log(log.clone());
    }
    
    let total_event_entries = event_logs_data.total_entries();
    let event_logs = json!({
        "security": event_logs_data.security.into_iter().map(|e| {
            json!({
                "event_id": e.event_id,
                "level": e.level,
                "timestamp": e.timestamp,
                "message": e.message
            })
        }).collect::<Vec<_>>(),
        "system": event_logs_data.system.into_iter().map(|e| {
            json!({
                "event_id": e.event_id,
                "level": e.level,
                "timestamp": e.timestamp,
                "message": e.message
            })
        }).collect::<Vec<_>>(),
        "application": event_logs_data.application.into_iter().map(|e| {
            json!({
                "event_id": e.event_id,
                "level": e.level,
                "timestamp": e.timestamp,
                "message": e.message
            })
        }).collect::<Vec<_>>()
    });
    
    logger.info(&format!("Event log collection completed: {} entries collected", total_event_entries));
    
    // Collect execution evidence with error handling
    logger.info("Starting execution evidence collection");
    let (prefetch_files_data, prefetch_logs) = prefetch::collect_prefetch_files();
    
    // Convert forensic audit entries to log entries
    for audit_entry in &prefetch_logs {
        let log_entry = LogEntry::new(&audit_entry.level, &format!("[{}] {}: {} ({}ms)", 
            audit_entry.component, audit_entry.action, audit_entry.details, audit_entry.duration_ms));
        scan_results.add_log(log_entry);
    }
    
    let prefetch_files = prefetch_files_data.into_iter().map(|pf| {
        json!({
            "filename": pf.filename,
            "executable_name": pf.executable_name,
            "run_count": pf.run_count,
            "last_run_time": pf.last_run_time,
            "creation_time": pf.creation_time,
            "file_size": pf.file_size,
            "hash": pf.hash,
            "version": pf.version,
            "referenced_files": pf.referenced_files,
            "volumes": pf.volumes.into_iter().map(|v| {
                json!({
                    "device_path": v.device_path,
                    "volume_name": v.volume_name,
                    "serial_number": v.serial_number,
                    "creation_time": v.creation_time
                })
            }).collect::<Vec<_>>()
        })
    }).collect::<Vec<_>>();
    
    logger.info(&format!("Prefetch analysis completed: {} files analyzed", prefetch_files.len()));
    
    let (shimcache_entries_data, shimcache_logs) = shimcache::collect_shimcache_entries();
    
    // Convert forensic audit entries to log entries
    for audit_entry in &shimcache_logs {
        let log_entry = LogEntry::new(&audit_entry.level, &format!("[{}] {}: {} ({}ms)", 
            audit_entry.component, audit_entry.action, audit_entry.details, audit_entry.duration_ms));
        scan_results.add_log(log_entry);
    }
    
    let shimcache_entries = shimcache_entries_data.into_iter().map(|sc| {
        json!({
            "path": sc.path,
            "last_modified": sc.last_modified,
            "file_size": sc.file_size,
            "last_update": sc.last_update,
            "execution_flag": sc.execution_flag
        })
    }).collect::<Vec<_>>();
    
    logger.info(&format!("Shimcache analysis completed: {} entries collected", shimcache_entries.len()));
    
    let total_artifacts = processes.len() + network_connections.len() + persistence_mechanisms.len() + total_event_entries + prefetch_files.len() + shimcache_entries.len();
    
    let duration = start_time.elapsed();
    logger.info(&format!("Scan completed in {:.2} seconds", duration.as_secs_f64()));
    logger.info(&format!("Total artifacts collected: {}", total_artifacts));
    
    // Get logger summary for final reporting
    let log_summary = logger.get_summary();
    logger.info(&format!("Collection summary - Total logs: {}, Errors: {}, Warnings: {}, Success rate: {:.1}%", 
        log_summary.total_count, log_summary.error_count, log_summary.warn_count, log_summary.success_rate()));
    
    if verbose {
        println!("✓ System information collected");
        println!("✓ Running processes enumerated ({} processes)", processes.len());
        println!("✓ Network connections analyzed ({} connections)", network_connections.len());
        println!("✓ Persistence mechanisms detected ({} mechanisms)", persistence_mechanisms.len());
        println!("✓ Event logs collected ({} entries)", total_event_entries);
        println!("✓ Prefetch files analyzed ({} files)", prefetch_files.len());
        println!("✓ Shimcache entries collected ({} entries)", shimcache_entries.len());
        
        if log_summary.has_errors() {
            println!("⚠ {} errors encountered during collection", log_summary.error_count);
        }
        if log_summary.has_warnings() {
            println!("⚠ {} warnings generated during collection", log_summary.warn_count);
        }
    }
    
    // Finalize scan results with proper metadata
    scan_results.finalize_scan();
    
    // Add all logger entries to the scan results
    for entry in logger.get_entries() {
        scan_results.add_log(entry);
    }
    
    // Update scan results with collected artifacts
    scan_results.artifacts.system_info.uptime_secs = 0; // Will be updated by system_info collection
    
    // Create comprehensive scan results JSON
    let final_scan_results = json!({
        "scan_metadata": {
            "scan_id": scan_results.scan_metadata.scan_id,
            "scan_start_utc": scan_results.scan_metadata.scan_start_utc,
            "scan_duration_ms": duration.as_millis() as u64,
            "hostname": hostname,
            "os_version": scan_results.scan_metadata.os_version,
            "cli_version": scan_results.scan_metadata.cli_version,
            "total_artifacts": total_artifacts,
            "collection_summary": {
                "total_logs": log_summary.total_count,
                "error_count": log_summary.error_count,
                "warning_count": log_summary.warn_count,
                "success_rate": log_summary.success_rate()
            }
        },
        "artifacts": {
            "system_info": system_info,
            "running_processes": processes,
            "network_connections": network_connections,
            "persistence_mechanisms": persistence_mechanisms,
            "event_logs": event_logs,
            "execution_evidence": {
                "prefetch_files": prefetch_files,
                "shimcache_entries": shimcache_entries
            }
        },
        "collection_log": scan_results.collection_log.into_iter().map(|log| {
            json!({
                "timestamp": log.timestamp,
                "level": log.level,
                "message": log.message
            })
        }).collect::<Vec<_>>()
    });

    // Output results with comprehensive error handling
    match serde_json::to_string_pretty(&final_scan_results) {
        Ok(json_output) => {
            if let Some(output_file) = matches.get_one::<String>("output") {
                match write_output_file(output_file, &json_output, &logger) {
                    Ok(_) => {
                        logger.info(&format!("Results written to file: {}", output_file));
                        if verbose {
                            println!("✓ Results written to: {}", output_file);
                            println!("File size: {} bytes", json_output.len());
                        }
                    }
                    Err(e) => {
                        logger.error(&format!("Failed to write output file: {}", e));
                        eprintln!("✗ Error writing to file: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("{}", json_output);
            }
        }
        Err(e) => {
            logger.error(&format!("Failed to serialize scan results: {}", e));
            eprintln!("✗ Error serializing results: {}", e);
            std::process::exit(1);
        }
    }

    // Final status reporting
    if verbose {
        println!("\nScan completed in {:.2} seconds", duration.as_secs_f64());
        println!("Total artifacts collected: {}", total_artifacts);
        println!("Collection logs: {} entries ({} errors, {} warnings)", 
            log_summary.total_count, log_summary.error_count, log_summary.warn_count);
    } else {
        println!("\nScan completed successfully in {:.2} seconds", duration.as_secs_f64());
        println!("Total artifacts collected: {}", total_artifacts);
        if log_summary.has_errors() || log_summary.has_warnings() {
            println!("Collection completed with {} errors and {} warnings", 
                log_summary.error_count, log_summary.warn_count);
        }
    }
    
    // Exit with appropriate code based on collection success
    if log_summary.error_count > 0 {
        std::process::exit(2); // Partial success with errors
    }
}

/// Collect system information with comprehensive error handling
fn collect_system_info_safe(logger: &Logger) -> Option<serde_json::Value> {
    let operation = || -> ForensicResult<serde_json::Value> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let hostname = std::env::var("COMPUTERNAME")
            .map_err(|_| ForensicError::system_api_error("Failed to get hostname"))?;
        let username = std::env::var("USERNAME")
            .map_err(|_| ForensicError::system_api_error("Failed to get username"))?;
        
        let boot_time = System::boot_time();
        let uptime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| ForensicError::system_api_error("Failed to calculate uptime"))?
            .as_secs() - boot_time;
        
        Ok(json!({
            "hostname": hostname,
            "os_name": System::name().unwrap_or_else(|| "Windows_NT".to_string()),
            "os_version": System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            "architecture": std::env::consts::ARCH,
            "current_user": username,
            "uptime_hours": (uptime as f64) / 3600.0,
            "last_boot_time": chrono::DateTime::from_timestamp(boot_time as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now())
                .to_rfc3339(),
            "total_memory": sys.total_memory(),
            "used_memory": sys.used_memory(),
            "cpu_count": sys.cpus().len()
        }))
    };
    
    handle_error_gracefully(operation(), logger, "system_info_collection")
}

/// Write output file with proper error handling and logging
fn write_output_file(output_file: &str, content: &str, logger: &Logger) -> ForensicResult<()> {
    logger.info(&format!("Writing output to file: {}", output_file));
    
    // Validate file path
    let path = std::path::Path::new(output_file);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            logger.warn(&format!("Parent directory does not exist: {:?}", parent));
            return Err(ForensicError::not_found("Parent directory does not exist"));
        }
    }
    
    // Write file with error handling
    fs::write(output_file, content)
        .map_err(|e| ForensicError::system_api_error(&format!("Failed to write file: {}", e)))?;
    
    // Verify file was written correctly
    let written_size = fs::metadata(output_file)
        .map_err(|e| ForensicError::system_api_error(&format!("Failed to verify file: {}", e)))?
        .len();
    
    if written_size != content.len() as u64 {
        return Err(ForensicError::invalid_data("File size mismatch after write"));
    }
    
    logger.info(&format!("Successfully wrote {} bytes to {}", written_size, output_file));
    Ok(())
}





