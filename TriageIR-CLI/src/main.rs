use clap::{Arg, Command};
use serde_json::json;
use std::fs;
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
        .get_matches();

    println!("TriageIR CLI v0.1.0 - Digital Forensics Triage Tool");
    println!("==================================================");

    let start_time = std::time::Instant::now();
    let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let username = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());

    if matches.get_flag("verbose") {
        println!("Starting forensic data collection...");
        println!("Target system: {}", hostname);
        println!("Current user: {}", username);
    }

    // Initialize system information collector
    let mut sys = System::new_all();
    sys.refresh_all();

    // Collect real system information
    let system_info = collect_system_info(&sys, &hostname, &username);
    
    // Collect real running processes with enhanced DLL enumeration
    let (processes_data, process_logs) = processes::collect_processes();
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
    
    // Collect network connections using the network module
    let (network_connections_data, network_logs) = network::collect_network_connections();
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
    
    // Collect persistence mechanisms using the persistence module
    let (persistence_mechanisms_data, persistence_logs) = persistence::collect_persistence_mechanisms();
    let persistence_mechanisms = persistence_mechanisms_data.into_iter().map(|p| {
        json!({
            "type": p.mechanism_type,
            "name": p.name,
            "command": p.command,
            "source": p.source
        })
    }).collect::<Vec<_>>();
    
    // Collect event logs using the event_logs module
    let (event_logs_data, event_logs_collection_logs) = event_logs::collect_event_logs();
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
    
    // Collect execution evidence using prefetch and shimcache modules
    let (prefetch_files_data, prefetch_logs) = prefetch::collect_prefetch_files();
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
    
    let (shimcache_entries_data, shimcache_logs) = shimcache::collect_shimcache_entries();
    let shimcache_entries = shimcache_entries_data.into_iter().map(|sc| {
        json!({
            "path": sc.path,
            "last_modified": sc.last_modified,
            "file_size": sc.file_size,
            "last_update": sc.last_update,
            "execution_flag": sc.execution_flag
        })
    }).collect::<Vec<_>>();
    
    let total_artifacts = processes.len() + network_connections.len() + persistence_mechanisms.len() + total_event_entries + prefetch_files.len() + shimcache_entries.len();
    
    if matches.get_flag("verbose") {
        println!("✓ System information collected");
        println!("✓ Running processes enumerated ({} processes)", processes.len());
        println!("✓ Network connections analyzed ({} connections)", network_connections.len());
        println!("✓ Persistence mechanisms detected ({} mechanisms)", persistence_mechanisms.len());
        println!("✓ Event logs collected ({} entries)", total_event_entries);
        println!("✓ Prefetch files analyzed ({} files)", prefetch_files.len());
        println!("✓ Shimcache entries collected ({} entries)", shimcache_entries.len());
    }

    let duration = start_time.elapsed();
    
    // Create comprehensive scan results
    let scan_results = json!({
        "scan_metadata": {
            "version": "0.1.0",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "hostname": hostname,
            "scan_duration_seconds": duration.as_secs_f64(),
            "total_artifacts": total_artifacts
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
        "collection_log": create_collection_log(process_logs, network_logs, persistence_logs, event_logs_collection_logs, prefetch_logs, shimcache_logs)
    });

    if matches.get_flag("verbose") {
        println!("Scan completed in {:.2} seconds", duration.as_secs_f64());
    }

    // Output results
    let json_output = serde_json::to_string_pretty(&scan_results).unwrap();
    
    if let Some(output_file) = matches.get_one::<String>("output") {
        match fs::write(output_file, &json_output) {
            Ok(_) => {
                println!("✓ Results written to: {}", output_file);
                if matches.get_flag("verbose") {
                    println!("File size: {} bytes", json_output.len());
                }
            },
            Err(e) => {
                eprintln!("✗ Error writing to file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", json_output);
    }

    if !matches.get_flag("verbose") {
        println!("\nScan completed successfully in {:.2} seconds", duration.as_secs_f64());
        println!("Total artifacts collected: {}", total_artifacts);
    }
}

fn collect_system_info(sys: &System, hostname: &str, username: &str) -> serde_json::Value {
    let boot_time = System::boot_time();
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() - boot_time;
    
    json!({
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
    })
}





fn create_collection_log(process_logs: Vec<types::LogEntry>, network_logs: Vec<types::LogEntry>, persistence_logs: Vec<types::LogEntry>, event_logs_logs: Vec<types::LogEntry>, prefetch_logs: Vec<forensic_types::AuditEntry>, shimcache_logs: Vec<forensic_types::AuditEntry>) -> Vec<serde_json::Value> {
    let mut all_logs = Vec::new();
    
    // Add process collection logs
    for log in process_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "message": log.message
        }));
    }
    
    // Add network collection logs
    for log in network_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "message": log.message
        }));
    }
    
    // Add persistence collection logs
    for log in persistence_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "message": log.message
        }));
    }
    
    // Add event logs collection logs
    for log in event_logs_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "message": log.message
        }));
    }
    
    // Add prefetch collection logs
    for log in prefetch_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "component": log.component,
            "action": log.action,
            "details": log.details,
            "duration_ms": log.duration_ms,
            "result": log.result
        }));
    }
    
    // Add shimcache collection logs
    for log in shimcache_logs {
        all_logs.push(json!({
            "timestamp": log.timestamp,
            "level": log.level,
            "component": log.component,
            "action": log.action,
            "details": log.details,
            "duration_ms": log.duration_ms,
            "result": log.result
        }));
    }
    
    // Add other collection logs
    all_logs.push(json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": "INFO",
        "message": "System information collected"
    }));
    
    all_logs
}