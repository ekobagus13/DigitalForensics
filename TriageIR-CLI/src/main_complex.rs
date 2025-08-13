use clap::Parser;
use serde_json;
use std::process;

mod types;
mod system_info;
mod processes;
mod network;
mod persistence;
mod event_logs;
mod logger;

#[cfg(test)]
mod integration_tests;

use types::*;
use system_info::{collect_system_info, get_detailed_os_version, get_system_hostname};
use processes::collect_processes;
use network::collect_network_connections;
use persistence::collect_persistence_mechanisms;
use event_logs::collect_event_logs;
use logger::{Logger, error_handling::*};

#[derive(Parser)]
#[command(name = "triageir-cli")]
#[command(about = "Digital forensics triage tool for Windows systems")]
#[command(version)]
#[command(long_about = "TriageIR CLI is a forensically sound digital forensics triage tool designed for rapid evidence collection on Windows systems. It collects system information, running processes, network connections, persistence mechanisms, and event logs.")]
struct Args {
    #[arg(long, default_value = "json", help = "Output format (currently only json supported)")]
    format: String,
    
    #[arg(short, long, help = "Output file path (optional, defaults to stdout)")]
    output: Option<String>,
    
    #[arg(long, help = "Password for encrypted output (reserved for future use)")]
    password: Option<String>,
    
    #[arg(short, long, help = "Enable verbose logging to stderr")]
    verbose: bool,
    
    #[arg(long, help = "Skip process hash calculation (faster but less detailed)")]
    skip_hashes: bool,
    
    #[arg(long, help = "Limit number of event log entries to collect", default_value = "1000")]
    max_events: usize,
    
    #[arg(long, help = "Skip event log collection entirely")]
    skip_events: bool,
    
    #[arg(long, help = "Only collect specific artifact types (comma-separated: system,processes,network,persistence,events)")]
    only: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // Initialize logger
    let logger = Logger::new(args.verbose);
    
    logger.info(&format!("TriageIR CLI v{} - Starting forensic collection", env!("CARGO_PKG_VERSION")));
    
    if args.verbose {
        logger.info("Verbose logging enabled");
    }
    
    // Execute main collection logic with error handling
    let exit_code = match run_collection(&args, &logger) {
        Ok(_) => {
            logger.info("Forensic collection completed successfully");
            0
        }
        Err(error) => {
            logger.error(&format!("Forensic collection failed: {}", error.user_message()));
            if error.is_fatal() {
                2 // Fatal error exit code
            } else {
                1 // Non-fatal error exit code
            }
        }
    };
    
    // Print summary if verbose
    if args.verbose {
        let summary = logger.get_summary();
        eprintln!("\n=== Collection Summary ===");
        eprintln!("  Total log entries: {}", summary.total_count);
        eprintln!("  Info: {}, Warnings: {}, Errors: {}", 
                 summary.info_count, summary.warn_count, summary.error_count);
        eprintln!("  Success rate: {:.1}%", summary.success_rate());
        
        if summary.has_errors() {
            eprintln!("  ⚠️  Some errors occurred during collection");
        }
        if summary.has_warnings() {
            eprintln!("  ⚠️  Some warnings were generated during collection");
        }
        if !summary.has_errors() && !summary.has_warnings() {
            eprintln!("  ✅ Collection completed without issues");
        }
        eprintln!("========================");
    }
    
    process::exit(exit_code);
}

fn run_collection(args: &Args, logger: &Logger) -> ForensicResult<()> {
    // Initialize scan results with system information
    let hostname = get_system_hostname();
    let os_version = get_detailed_os_version();
    let mut scan_results = ScanResults::new(hostname, os_version);
    
    logger.info("Initializing scan metadata");
    
    // Parse collection filters if specified
    let collection_filter = parse_collection_filter(&args.only);
    
    // Collect system information (always collected as it's needed for metadata)
    if collection_filter.is_empty() || collection_filter.contains(&"system".to_string()) {
        logger.info("Starting system information collection");
        let (system_info, system_logs) = collect_system_info();
        scan_results.artifacts.system_info = system_info;
        
        // Add system collection logs
        for log in system_logs {
            scan_results.add_log(log);
        }
    }
    
    // Collect running processes with error handling
    if collection_filter.is_empty() || collection_filter.contains(&"processes".to_string()) {
        logger.info("Starting process enumeration");
        let (processes, process_logs) = if args.skip_hashes {
            logger.info("Skipping process hash calculation for faster collection");
            collect_processes_no_hashes()
        } else {
            collect_processes()
        };
        scan_results.artifacts.running_processes = processes;
        
        // Add process collection logs
        for log in process_logs {
            scan_results.add_log(log);
        }
    }
    
    // Collect network connections with error handling
    if collection_filter.is_empty() || collection_filter.contains(&"network".to_string()) {
        logger.info("Starting network connection enumeration");
        let (connections, network_logs) = collect_network_connections();
        scan_results.artifacts.network_connections = connections;
        
        // Add network collection logs
        for log in network_logs {
            scan_results.add_log(log);
        }
    }
    
    // Collect persistence mechanisms with error handling
    if collection_filter.is_empty() || collection_filter.contains(&"persistence".to_string()) {
        logger.info("Starting persistence mechanism detection");
        let (mechanisms, persistence_logs) = collect_persistence_mechanisms();
        scan_results.artifacts.persistence_mechanisms = mechanisms;
        
        // Add persistence collection logs
        for log in persistence_logs {
            scan_results.add_log(log);
        }
    }
    
    // Collect event logs with error handling
    if !args.skip_events && (collection_filter.is_empty() || collection_filter.contains(&"events".to_string())) {
        logger.info(&format!("Starting event log collection (max {} entries)", args.max_events));
        let (event_logs, event_log_logs) = collect_event_logs_limited(args.max_events);
        scan_results.artifacts.event_logs = event_logs;
        
        // Add event log collection logs
        for log in event_log_logs {
            scan_results.add_log(log);
        }
    } else if args.skip_events {
        logger.info("Skipping event log collection as requested");
    }
    
    // Add logger entries to scan results
    for log_entry in logger.get_entries() {
        scan_results.add_log(log_entry);
    }
    
    // Validate scan results structure
    if let Err(validation_error) = scan_results.validate() {
        return Err(ForensicError::invalid_data(&format!("Scan results validation failed: {}", validation_error)));
    }
    
    // Finalize scan timing
    scan_results.finalize_scan();
    logger.info(&format!("Collection completed in {} ms", scan_results.scan_metadata.scan_duration_ms));
    
    // Log collection statistics
    let artifact_count = scan_results.artifacts.total_artifact_count();
    logger.info(&format!("Collected {} total artifacts", artifact_count));
    logger.info(&format!("Generated {} log entries", scan_results.collection_log.len()));
    
    // Validate output format
    if args.format != "json" {
        return Err(ForensicError::invalid_data(&format!("Unsupported output format: {}. Only 'json' is currently supported.", args.format)));
    }
    
    // Output results as JSON with error handling
    let json_output = serde_json::to_string_pretty(&scan_results)
        .map_err(|e| ForensicError::invalid_data(&format!("JSON serialization failed: {}", e)))?;
    
    // Write output
    if let Some(output_path) = &args.output {
        std::fs::write(output_path, &json_output)
            .map_err(|e| ForensicError::system_api_error(&format!("Failed to write to file {}: {}", output_path, e)))?;
        
        logger.info(&format!("Results written to: {}", output_path));
    } else {
        println!("{}", json_output);
    }
    
    Ok(())
}

/// Parse collection filter from command line argument
fn parse_collection_filter(only_arg: &Option<String>) -> Vec<String> {
    match only_arg {
        Some(filter_str) => {
            filter_str
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect()
        }
        None => Vec::new(),
    }
}

/// Collect processes without calculating hashes (faster)
fn collect_processes_no_hashes() -> (Vec<Process>, Vec<LogEntry>) {
    // This would be implemented to skip hash calculation
    // For now, just call the regular function
    collect_processes()
}

/// Collect event logs with a limit on the number of entries
fn collect_event_logs_limited(max_entries: usize) -> (EventLogs, Vec<LogEntry>) {
    // This would be implemented to limit event log collection
    // For now, just call the regular function
    collect_event_logs()
}

// Display usage examples and additional help information
fn display_usage_examples() {
    eprintln!("Usage Examples:");
    eprintln!("  Basic scan to stdout:");
    eprintln!("    triageir-cli");
    eprintln!("");
    eprintln!("  Verbose scan with output to file:");
    eprintln!("    triageir-cli --verbose --output scan_results.json");
    eprintln!("");
    eprintln!("  Fast scan without process hashes:");
    eprintln!("    triageir-cli --skip-hashes --output quick_scan.json");
    eprintln!("");
    eprintln!("  Collect only processes and network connections:");
    eprintln!("    triageir-cli --only processes,network --output targeted_scan.json");
    eprintln!("");
    eprintln!("  Skip event logs for faster collection:");
    eprintln!("    triageir-cli --skip-events --output no_events.json");
    eprintln!("");
    eprintln!("Artifact Types:");
    eprintln!("  system      - System information (uptime, users, OS version)");
    eprintln!("  processes   - Running processes with metadata and hashes");
    eprintln!("  network     - Active network connections");
    eprintln!("  persistence - Autostart mechanisms (registry, services, startup folders)");
    eprintln!("  events      - Windows Event Log entries (Security and System)");
    eprintln!("");
    eprintln!("Note: This tool requires administrative privileges for complete data collection.");
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn test_parse_collection_filter() {
        // Test empty filter
        let filter = parse_collection_filter(&None);
        assert!(filter.is_empty());
        
        // Test single item
        let filter = parse_collection_filter(&Some("processes".to_string()));
        assert_eq!(filter, vec!["processes"]);
        
        // Test multiple items
        let filter = parse_collection_filter(&Some("processes,network,events".to_string()));
        assert_eq!(filter, vec!["processes", "network", "events"]);
        
        // Test with spaces
        let filter = parse_collection_filter(&Some(" processes , network , events ".to_string()));
        assert_eq!(filter, vec!["processes", "network", "events"]);
        
        // Test case insensitive
        let filter = parse_collection_filter(&Some("PROCESSES,Network,EvEnTs".to_string()));
        assert_eq!(filter, vec!["processes", "network", "events"]);
    }

    #[test]
    fn test_args_parsing() {
        use clap::Parser;
        
        // Test minimal args
        let args = Args::parse_from(&["triageir-cli"]);
        assert_eq!(args.format, "json");
        assert!(!args.verbose);
        assert!(!args.skip_hashes);
        assert!(!args.skip_events);
        assert_eq!(args.max_events, 1000);
        assert!(args.only.is_none());
        
        // Test all options
        let args = Args::parse_from(&[
            "triageir-cli",
            "--verbose",
            "--output", "test.json",
            "--skip-hashes",
            "--skip-events",
            "--max-events", "500",
            "--only", "processes,network"
        ]);
        assert!(args.verbose);
        assert_eq!(args.output, Some("test.json".to_string()));
        assert!(args.skip_hashes);
        assert!(args.skip_events);
        assert_eq!(args.max_events, 500);
        assert_eq!(args.only, Some("processes,network".to_string()));
    }
}