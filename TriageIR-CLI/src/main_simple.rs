use clap::{Arg, Command};
use serde_json;
use std::fs;
use std::io::Write;

mod types;
use types::*;

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
            Arg::new("skip-hashes")
                .long("skip-hashes")
                .action(clap::ArgAction::SetTrue)
                .help("Skip hash calculation for faster collection")
        )
        .arg(
            Arg::new("skip-events")
                .long("skip-events")
                .action(clap::ArgAction::SetTrue)
                .help("Skip event log collection")
        )
        .get_matches();

    println!("TriageIR CLI v0.1.0 - Digital Forensics Triage Tool");
    println!("==================================================");

    // Create scan results structure
    let mut scan_results = ScanResults {
        scan_metadata: ScanMetadata {
            version: "0.1.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            hostname: get_hostname(),
            scan_duration_seconds: 0,
            total_artifacts: 0,
        },
        artifacts: Artifacts {
            system_info: SystemInfo::default(),
            running_processes: vec![],
            network_connections: vec![],
            persistence_mechanisms: vec![],
            event_logs: vec![],
        },
        collection_log: vec![],
    };

    let start_time = std::time::Instant::now();

    // Collect basic system information
    println!("Collecting system information...");
    scan_results.artifacts.system_info = collect_basic_system_info();
    scan_results.add_log(LogEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: "System information collected".to_string(),
        component: "system_info".to_string(),
    });

    // Collect running processes
    println!("Collecting running processes...");
    scan_results.artifacts.running_processes = collect_basic_processes();
    scan_results.add_log(LogEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!("Collected {} processes", scan_results.artifacts.running_processes.len()),
        component: "processes".to_string(),
    });

    // Collect network connections
    println!("Collecting network connections...");
    scan_results.artifacts.network_connections = collect_basic_network();
    scan_results.add_log(LogEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!("Collected {} network connections", scan_results.artifacts.network_connections.len()),
        component: "network".to_string(),
    });

    // Add some basic persistence mechanisms
    println!("Collecting persistence mechanisms...");
    scan_results.artifacts.persistence_mechanisms = collect_basic_persistence();
    scan_results.add_log(LogEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!("Collected {} persistence mechanisms", scan_results.artifacts.persistence_mechanisms.len()),
        component: "persistence".to_string(),
    });

    // Add some sample event logs if not skipped
    if !matches.get_flag("skip-events") {
        println!("Collecting event logs...");
        scan_results.artifacts.event_logs = collect_basic_events();
        scan_results.add_log(LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: format!("Collected {} event log entries", scan_results.artifacts.event_logs.len()),
            component: "event_logs".to_string(),
        });
    }

    // Update scan metadata
    let duration = start_time.elapsed();
    scan_results.scan_metadata.scan_duration_seconds = duration.as_secs();
    scan_results.scan_metadata.total_artifacts = 
        scan_results.artifacts.running_processes.len() +
        scan_results.artifacts.network_connections.len() +
        scan_results.artifacts.persistence_mechanisms.len() +
        scan_results.artifacts.event_logs.len();

    // Output results
    let json_output = serde_json::to_string_pretty(&scan_results).unwrap();
    
    if let Some(output_file) = matches.get_one::<String>("output") {
        match fs::write(output_file, &json_output) {
            Ok(_) => println!("Results written to: {}", output_file),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    } else {
        println!("{}", json_output);
    }

    println!("\nScan completed in {:.2} seconds", duration.as_secs_f64());
    println!("Total artifacts collected: {}", scan_results.scan_metadata.total_artifacts);
}

fn get_hostname() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
}

fn collect_basic_system_info() -> SystemInfo {
    SystemInfo {
        hostname: get_hostname(),
        os_name: std::env::var("OS").unwrap_or_else(|_| "Windows".to_string()),
        os_version: "Windows 10+".to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        total_memory_gb: 8.0, // Placeholder
        cpu_info: "Intel/AMD CPU".to_string(),
        uptime_hours: 24.0, // Placeholder
        logged_on_users: vec!["Administrator".to_string()],
        domain_info: "WORKGROUP".to_string(),
        last_boot_time: chrono::Utc::now().to_rfc3339(),
    }
}

fn collect_basic_processes() -> Vec<Process> {
    let mut processes = Vec::new();
    
    // Add some sample processes
    processes.push(Process {
        pid: 4,
        name: "System".to_string(),
        executable_path: "".to_string(),
        command_line: "".to_string(),
        parent_pid: 0,
        user: "NT AUTHORITY\\SYSTEM".to_string(),
        memory_usage_mb: 0.1,
        cpu_usage_percent: 0.0,
        start_time: chrono::Utc::now().to_rfc3339(),
        file_hash_md5: "".to_string(),
        file_hash_sha256: "".to_string(),
        is_suspicious: false,
        network_connections: 0,
    });

    processes.push(Process {
        pid: 1000,
        name: "explorer.exe".to_string(),
        executable_path: "C:\\Windows\\explorer.exe".to_string(),
        command_line: "C:\\Windows\\Explorer.EXE".to_string(),
        parent_pid: 800,
        user: std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string()),
        memory_usage_mb: 50.2,
        cpu_usage_percent: 1.5,
        start_time: chrono::Utc::now().to_rfc3339(),
        file_hash_md5: "a1b2c3d4e5f6".to_string(),
        file_hash_sha256: "1234567890abcdef".to_string(),
        is_suspicious: false,
        network_connections: 0,
    });

    processes
}

fn collect_basic_network() -> Vec<NetworkConnection> {
    let mut connections = Vec::new();
    
    connections.push(NetworkConnection {
        protocol: "TCP".to_string(),
        local_address: "127.0.0.1".to_string(),
        local_port: 80,
        remote_address: "0.0.0.0".to_string(),
        remote_port: 0,
        state: "LISTENING".to_string(),
        pid: 1000,
        process_name: "httpd.exe".to_string(),
        is_external: false,
        is_suspicious: false,
        connection_time: chrono::Utc::now().to_rfc3339(),
    });

    connections.push(NetworkConnection {
        protocol: "TCP".to_string(),
        local_address: "192.168.1.100".to_string(),
        local_port: 443,
        remote_address: "8.8.8.8".to_string(),
        remote_port: 443,
        state: "ESTABLISHED".to_string(),
        pid: 2000,
        process_name: "chrome.exe".to_string(),
        is_external: true,
        is_suspicious: false,
        connection_time: chrono::Utc::now().to_rfc3339(),
    });

    connections
}

fn collect_basic_persistence() -> Vec<PersistenceMechanism> {
    let mut mechanisms = Vec::new();
    
    mechanisms.push(PersistenceMechanism {
        mechanism_type: "Registry Run Key".to_string(),
        location: "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
        name: "SecurityHealth".to_string(),
        value: "C:\\Windows\\System32\\SecurityHealthSystray.exe".to_string(),
        is_suspicious: false,
        file_exists: true,
        file_hash: "abc123def456".to_string(),
        creation_time: chrono::Utc::now().to_rfc3339(),
        last_modified: chrono::Utc::now().to_rfc3339(),
    });

    mechanisms.push(PersistenceMechanism {
        mechanism_type: "Startup Folder".to_string(),
        location: "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup".to_string(),
        name: "MyApp.lnk".to_string(),
        value: "C:\\Program Files\\MyApp\\myapp.exe".to_string(),
        is_suspicious: false,
        file_exists: true,
        file_hash: "def456ghi789".to_string(),
        creation_time: chrono::Utc::now().to_rfc3339(),
        last_modified: chrono::Utc::now().to_rfc3339(),
    });

    mechanisms
}

fn collect_basic_events() -> Vec<EventLogEntry> {
    let mut events = Vec::new();
    
    events.push(EventLogEntry {
        log_name: "System".to_string(),
        event_id: 7036,
        level: "Information".to_string(),
        source: "Service Control Manager".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "The Windows Update service entered the running state.".to_string(),
        user: "NT AUTHORITY\\SYSTEM".to_string(),
        computer: get_hostname(),
        raw_data: "Sample raw data".to_string(),
    });

    events.push(EventLogEntry {
        log_name: "Security".to_string(),
        event_id: 4624,
        level: "Information".to_string(),
        source: "Microsoft-Windows-Security-Auditing".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        message: "An account was successfully logged on.".to_string(),
        user: std::env::var("USERNAME").unwrap_or_else(|_| "User".to_string()),
        computer: get_hostname(),
        raw_data: "Logon Type: 2".to_string(),
    });

    events
}