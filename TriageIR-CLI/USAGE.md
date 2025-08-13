# TriageIR CLI Usage Guide

## Overview

TriageIR CLI is a forensically sound digital forensics triage tool designed for rapid evidence collection on Windows systems. It collects system information, running processes, network connections, persistence mechanisms, and Windows Event Log entries.

## Prerequisites

- Windows operating system (Windows 7 or later)
- Administrative privileges (recommended for complete data collection)
- No external dependencies required (static executable)

## Basic Usage

### Quick Start

```cmd
# Basic scan with output to stdout
triageir-cli.exe

# Scan with verbose output to file
triageir-cli.exe --verbose --output scan_results.json

# Fast scan without process hashes
triageir-cli.exe --skip-hashes --output quick_scan.json
```

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--format` | | Output format (currently only json) | json |
| `--output` | `-o` | Output file path (stdout if not specified) | - |
| `--verbose` | `-v` | Enable verbose logging to stderr | false |
| `--skip-hashes` | | Skip process hash calculation (faster) | false |
| `--skip-events` | | Skip event log collection entirely | false |
| `--max-events` | | Limit number of event log entries | 1000 |
| `--only` | | Collect only specific artifact types | all |
| `--password` | | Reserved for future encrypted output | - |
| `--help` | `-h` | Show help information | - |
| `--version` | `-V` | Show version information | - |

### Artifact Types

When using `--only`, you can specify one or more of these artifact types (comma-separated):

- **system** - System information (uptime, logged-on users, OS version)
- **processes** - Running processes with metadata and SHA-256 hashes
- **network** - Active network connections with owning processes
- **persistence** - Autostart mechanisms (registry keys, services, startup folders)
- **events** - Windows Event Log entries (Security and System logs)

## Usage Examples

### Basic Collection

```cmd
# Collect all artifacts with default settings
triageir-cli.exe --output full_scan.json

# Verbose collection with progress information
triageir-cli.exe --verbose --output verbose_scan.json
```

### Performance Optimized Collection

```cmd
# Fast collection without process hashes and event logs
triageir-cli.exe --skip-hashes --skip-events --output fast_scan.json

# Limit event log entries for faster collection
triageir-cli.exe --max-events 100 --output limited_events.json
```

### Targeted Collection

```cmd
# Collect only running processes and network connections
triageir-cli.exe --only processes,network --output network_analysis.json

# Collect only persistence mechanisms
triageir-cli.exe --only persistence --output persistence_check.json

# Collect system info and processes without hashes
triageir-cli.exe --only system,processes --skip-hashes --output system_processes.json
```

### Incident Response Scenarios

```cmd
# Quick triage for active threats
triageir-cli.exe --only processes,network,persistence --skip-hashes --output threat_triage.json

# Comprehensive forensic collection
triageir-cli.exe --verbose --max-events 5000 --output forensic_collection.json

# Memory-efficient collection for resource-constrained systems
triageir-cli.exe --skip-events --skip-hashes --only system,processes --output minimal_scan.json
```

## Output Format

The tool outputs JSON data conforming to a structured schema:

```json
{
  "scan_metadata": {
    "scan_id": "uuid-v4",
    "scan_start_utc": "2023-01-01T00:00:00Z",
    "scan_duration_ms": 1234,
    "hostname": "COMPUTER-NAME",
    "os_version": "Windows 10 Pro",
    "cli_version": "0.1.0"
  },
  "artifacts": {
    "system_info": { ... },
    "running_processes": [ ... ],
    "network_connections": [ ... ],
    "persistence_mechanisms": [ ... ],
    "event_logs": { ... }
  },
  "collection_log": [ ... ]
}
```

## Best Practices

### For Incident Response

1. **Run with administrative privileges** for complete data access
2. **Use verbose mode** (`--verbose`) to monitor collection progress
3. **Save output to file** for analysis and documentation
4. **Consider performance options** (`--skip-hashes`, `--skip-events`) for time-critical situations

### For Forensic Analysis

1. **Collect all artifacts** with default settings for comprehensive analysis
2. **Document the collection** by saving verbose output
3. **Verify data integrity** by checking the collection log for errors
4. **Preserve timestamps** - all timestamps are in UTC ISO 8601 format

### For Automated Deployment

1. **Use specific output paths** to avoid conflicts
2. **Handle exit codes** (0 = success, 1 = non-fatal errors, 2 = fatal errors)
3. **Parse collection logs** to identify any collection issues
4. **Consider resource constraints** when deploying to multiple systems

## Troubleshooting

### Common Issues

**Access Denied Errors**
- Run as Administrator for complete data collection
- Some artifacts may be unavailable to standard users

**Large Output Files**
- Use `--skip-events` or `--max-events` to reduce file size
- Use `--only` to collect specific artifact types

**Slow Collection**
- Use `--skip-hashes` to skip process hash calculation
- Use `--skip-events` to skip event log collection
- Use `--only` to collect only needed artifacts

**JSON Parsing Errors**
- Ensure output file is completely written before parsing
- Check collection log for serialization errors

### Exit Codes

- **0** - Success, collection completed without errors
- **1** - Non-fatal errors occurred, but collection completed
- **2** - Fatal errors occurred, collection may be incomplete

### Log Levels

When using `--verbose`, log messages are categorized as:
- **INFO** - Normal operation messages
- **WARN** - Non-fatal issues that don't stop collection
- **ERROR** - Fatal issues that may affect collection completeness

## Security Considerations

1. **Administrative Privileges** - The tool requires admin rights for complete data access
2. **Data Sensitivity** - Output contains sensitive system information
3. **Network Isolation** - Tool does not make network connections
4. **File Permissions** - Secure output files appropriately
5. **Audit Trail** - All operations are logged with timestamps

## Integration

### Scripting

```batch
@echo off
echo Running TriageIR collection...
triageir-cli.exe --verbose --output "%COMPUTERNAME%_triage.json"
if %ERRORLEVEL% EQU 0 (
    echo Collection completed successfully
) else (
    echo Collection completed with errors (exit code %ERRORLEVEL%)
)
```

### PowerShell

```powershell
$outputFile = "$env:COMPUTERNAME`_triage_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
$process = Start-Process -FilePath "triageir-cli.exe" -ArgumentList "--verbose", "--output", $outputFile -Wait -PassThru
if ($process.ExitCode -eq 0) {
    Write-Host "Collection completed successfully: $outputFile"
} else {
    Write-Warning "Collection completed with errors (exit code $($process.ExitCode))"
}
```

## Support

For issues, questions, or feature requests, please refer to the project documentation or contact the development team.