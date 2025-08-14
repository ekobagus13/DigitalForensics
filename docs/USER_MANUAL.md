# TriageIR User Manual

## Table of Contents

1. [Introduction](#introduction)
2. [System Requirements](#system-requirements)
3. [Installation](#installation)
4. [Getting Started](#getting-started)
5. [CLI User Guide](#cli-user-guide)
6. [GUI User Guide](#gui-user-guide)
7. [Understanding Results](#understanding-results)
8. [Troubleshooting](#troubleshooting)
9. [Best Practices](#best-practices)
10. [Legal and Compliance](#legal-and-compliance)

## Introduction

TriageIR is a professional digital forensics triage tool designed for rapid evidence collection on Windows systems. It consists of two components:

- **TriageIR-CLI**: A high-performance Rust-based command-line engine for forensic data collection
- **TriageIR-GUI**: An intuitive Electron-based graphical interface for data visualization and analysis

### Key Features

- **Forensically Sound**: Read-only system access with comprehensive audit trails
- **High Performance**: Sub-second to minute-level collection times
- **Comprehensive Coverage**: System info, processes, network, persistence, event logs, execution evidence
- **Professional Output**: Structured JSON data with HTML report generation
- **Minimal Impact**: Designed to minimize observer effect on target systems

### Use Cases

- Digital forensics investigations
- Incident response activities
- Security auditing and compliance
- Malware analysis and threat hunting
- System administration and monitoring

## System Requirements

### Minimum Requirements

- **Operating System**: Windows 10 or Windows Server 2016 (or later)
- **RAM**: 4 GB minimum, 8 GB recommended
- **Disk Space**: 100 MB for installation, additional space for output files
- **Processor**: x64 compatible processor
- **Permissions**: Standard user (Administrator recommended for complete data access)

### Recommended Requirements

- **Operating System**: Windows 11 or Windows Server 2022
- **RAM**: 16 GB or more
- **Disk Space**: 1 GB free space for large collections
- **Processor**: Multi-core x64 processor
- **Permissions**: Administrator privileges

### Software Dependencies

- **CLI**: No external dependencies (static executable)
- **GUI**: Bundled with all required components
- **Optional**: Python 3.x for output validation scripts

## Installation

### Option 1: Download Pre-built Binaries

1. Download the latest release from the official repository
2. Extract the archive to your preferred location (e.g., `C:\Tools\TriageIR\`)
3. Verify the installation by running the CLI tool

```cmd
cd C:\Tools\TriageIR\
TriageIR-CLI\triageir-cli.exe --version
```

### Option 2: Install via Installer Package

1. Download the TriageIR installer (`TriageIR-Setup.exe`)
2. Run the installer as Administrator
3. Follow the installation wizard
4. Launch TriageIR from the Start Menu or Desktop shortcut

### Option 3: Portable Installation

1. Download the portable package (`TriageIR-Portable.zip`)
2. Extract to a USB drive or network location
3. Run directly without installation

### Verification

After installation, verify both components:

```cmd
# Test CLI
triageir-cli.exe --help

# Test GUI (if installed)
# Launch from Start Menu or run triageir-gui.exe
```

## Getting Started

### Quick Start with CLI

1. Open Command Prompt as Administrator
2. Navigate to the TriageIR-CLI directory
3. Run a basic scan:

```cmd
triageir-cli.exe --output my_first_scan.json --verbose
```

4. View the results in the generated JSON file

### Quick Start with GUI

1. Launch TriageIR GUI from Start Menu or Desktop
2. Click "Quick Scan" to run a standard forensic collection
3. Wait for the scan to complete
4. Browse results in the organized tabs
5. Export reports as needed

### First Scan Checklist

- [ ] Run as Administrator for complete data access
- [ ] Ensure sufficient disk space for output files
- [ ] Document the scan purpose and scope
- [ ] Save output files with descriptive names
- [ ] Review collection logs for any errors

## CLI User Guide

### Basic Commands

```cmd
# Basic scan to stdout
triageir-cli.exe

# Scan with output to file
triageir-cli.exe --output scan_results.json

# Verbose scan with progress information
triageir-cli.exe --verbose --output detailed_scan.json

# Fast scan without process hashes
triageir-cli.exe --skip-hashes --output quick_scan.json
```

### Command Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `--output`, `-o` | Output file path | `--output results.json` |
| `--verbose`, `-v` | Enable verbose logging | `--verbose` |
| `--skip-hashes` | Skip process hash calculation | `--skip-hashes` |
| `--skip-events` | Skip event log collection | `--skip-events` |
| `--max-events` | Limit event log entries | `--max-events 1000` |
| `--only` | Collect specific artifacts | `--only processes,network` |

### Artifact Types

- **system**: System information (uptime, users, OS version)
- **processes**: Running processes with metadata
- **network**: Active network connections
- **persistence**: Autostart mechanisms
- **events**: Windows Event Log entries
- **execution**: Prefetch and Shimcache evidence

### Common Use Cases

#### Incident Response
```cmd
# Quick threat assessment
triageir-cli.exe --only processes,network,persistence --skip-hashes --output ir_triage.json

# Comprehensive incident response
triageir-cli.exe --verbose --max-events 5000 --output ir_comprehensive.json
```

#### Forensic Analysis
```cmd
# Complete evidence collection
triageir-cli.exe --verbose --max-events 10000 --output forensic_complete.json

# Execution evidence focus
triageir-cli.exe --only execution,processes --output execution_analysis.json
```

#### System Monitoring
```cmd
# Daily system check
triageir-cli.exe --skip-hashes --max-events 500 --output daily_check.json

# Process monitoring
triageir-cli.exe --only processes --skip-hashes --output process_monitor.json
```

## GUI User Guide

### Main Interface

The TriageIR GUI provides an intuitive interface with the following sections:

1. **Scan Configuration Panel**: Configure scan parameters
2. **Progress Display**: Real-time scan progress and status
3. **Results Viewer**: Tabbed interface for different artifact types
4. **Export Options**: Save and report generation functions

### Running a Scan

1. **Quick Scan**: Click "Quick Scan" for standard collection
2. **Custom Scan**: Configure specific parameters (future enhancement)
3. **Monitor Progress**: Watch real-time progress updates
4. **View Results**: Browse collected artifacts when complete

### Viewing Results

#### System Information Tab
- System uptime and basic information
- Logged-on users and session details
- Operating system version and configuration

#### Processes Tab
- Running processes with PID, name, and command line
- Process executable paths and SHA-256 hashes
- Parent-child process relationships
- Loaded DLL information

#### Network Tab
- Active TCP and UDP connections
- Local and remote addresses
- Connection states and owning processes
- Listening ports and services

#### Persistence Tab
- Registry Run keys and autostart entries
- Scheduled tasks and services
- Startup folder contents
- Persistence mechanism categorization

#### Event Logs Tab
- Security event log entries
- System event log entries
- Filterable by event ID, level, and timestamp
- Detailed event messages and metadata

#### Execution Evidence Tab
- Windows Prefetch file analysis
- Application Compatibility Cache (Shimcache) entries
- Execution counts and timestamps
- File path and execution evidence

### Export and Reporting

1. **Save Results**: Export raw JSON data
2. **Generate Report**: Create formatted HTML reports
3. **Custom Reports**: Select specific artifact types
4. **Print Reports**: Print-friendly report formatting

## Understanding Results

### JSON Output Structure

```json
{
  "scan_metadata": {
    "scan_id": "unique-identifier",
    "scan_start_utc": "timestamp",
    "scan_duration_ms": 1234,
    "hostname": "computer-name",
    "os_version": "Windows version",
    "cli_version": "tool-version"
  },
  "artifacts": {
    "system_info": { ... },
    "running_processes": [ ... ],
    "network_connections": [ ... ],
    "persistence_mechanisms": [ ... ],
    "event_logs": { ... },
    "execution_evidence": { ... }
  },
  "collection_log": [ ... ]
}
```

### Key Data Points

#### Process Analysis
- **PID/PPID**: Process and parent process identifiers
- **Command Line**: Full command line used to start process
- **SHA-256 Hash**: Cryptographic hash of executable file
- **Loaded DLLs**: Dynamic libraries loaded by the process

#### Network Connections
- **Protocol**: TCP or UDP
- **Local/Remote Addresses**: Connection endpoints
- **State**: Connection status (ESTABLISHED, LISTENING, etc.)
- **Owning PID**: Process that owns the connection

#### Persistence Mechanisms
- **Type**: Category of persistence (Registry, Scheduled Task, etc.)
- **Name**: Identifier or name of the mechanism
- **Command**: Command or executable being persisted
- **Source**: Location where persistence is configured

#### Execution Evidence
- **Prefetch Files**: Evidence of program execution with counts and timestamps
- **Shimcache Entries**: Application Compatibility Cache with execution flags
- **File Paths**: Full paths to executed programs
- **Timestamps**: Last execution and modification times

### Interpreting Results

#### Normal vs. Suspicious Indicators

**Normal Indicators:**
- Standard Windows processes (explorer.exe, svchost.exe, etc.)
- Expected network connections (Windows Update, domain authentication)
- Standard persistence mechanisms (Windows services, legitimate software)

**Suspicious Indicators:**
- Processes with unusual names or locations
- Unexpected network connections to external IPs
- Persistence mechanisms in unusual locations
- Execution evidence for unknown or suspicious programs

#### Analysis Workflow

1. **System Overview**: Review system information and uptime
2. **Process Analysis**: Identify unusual or suspicious processes
3. **Network Assessment**: Check for unexpected connections
4. **Persistence Review**: Examine autostart mechanisms
5. **Event Correlation**: Cross-reference with event log entries
6. **Execution Timeline**: Analyze program execution evidence

## Troubleshooting

### Common Issues

#### Access Denied Errors
**Problem**: Tool cannot access certain system resources
**Solution**: 
- Run as Administrator
- Check Windows security policies
- Verify user permissions

#### Large Output Files
**Problem**: Output files are too large
**Solution**:
- Use `--max-events` to limit event log entries
- Use `--only` to collect specific artifacts
- Use `--skip-events` for faster collection

#### Slow Performance
**Problem**: Collection takes too long
**Solution**:
- Use `--skip-hashes` to skip hash calculation
- Limit event log collection with `--max-events`
- Use targeted collection with `--only`

#### GUI Not Starting
**Problem**: GUI application fails to launch
**Solution**:
- Check system requirements
- Verify CLI executable location
- Run from command line to see error messages

### Error Codes

- **Exit Code 0**: Success, no errors
- **Exit Code 1**: Non-fatal errors, collection may be incomplete
- **Exit Code 2**: Fatal errors, collection failed

### Getting Help

1. Check the collection log for detailed error messages
2. Run with `--verbose` for additional diagnostic information
3. Verify system requirements and permissions
4. Consult the troubleshooting section of this manual
5. Contact support with detailed error information

## Best Practices

### For Incident Response

1. **Document Everything**: Record scan parameters, timestamps, and purpose
2. **Preserve Evidence**: Save original output files without modification
3. **Chain of Custody**: Maintain proper documentation of evidence handling
4. **Time Sensitivity**: Use performance options for time-critical situations
5. **Verification**: Check collection logs for completeness

### For Forensic Analysis

1. **Comprehensive Collection**: Use default settings for complete evidence
2. **Hash Verification**: Include process hashes for integrity verification
3. **Timeline Analysis**: Correlate execution evidence with event logs
4. **Documentation**: Generate detailed reports for case files
5. **Validation**: Verify data integrity and completeness

### For System Monitoring

1. **Regular Collection**: Establish baseline and monitor changes
2. **Automated Deployment**: Use scripting for consistent collection
3. **Resource Management**: Balance completeness with system impact
4. **Trend Analysis**: Compare results over time for anomaly detection
5. **Alert Thresholds**: Define criteria for suspicious activity

### Security Considerations

1. **Data Protection**: Secure output files with appropriate permissions
2. **Network Isolation**: Tool operates offline, no network transmission
3. **Audit Trail**: Maintain logs of all tool usage
4. **Access Control**: Limit tool access to authorized personnel
5. **Data Retention**: Follow organizational policies for evidence retention

## Legal and Compliance

### Legal Considerations

1. **Authorization**: Ensure proper authorization before running scans
2. **Jurisdiction**: Understand legal requirements in your jurisdiction
3. **Privacy**: Respect privacy laws and organizational policies
4. **Documentation**: Maintain proper documentation for legal proceedings
5. **Chain of Custody**: Follow established procedures for evidence handling

### Compliance Standards

- **NIST Cybersecurity Framework**: Supports detection and response activities
- **ISO 27001**: Assists with security monitoring and incident management
- **GDPR**: Consider privacy implications when collecting personal data
- **Industry Standards**: Adapt usage to specific industry requirements

### Best Practices for Legal Use

1. **Written Authorization**: Obtain written permission before scanning systems
2. **Scope Definition**: Clearly define the scope and purpose of collection
3. **Data Minimization**: Collect only necessary data for the investigation
4. **Secure Storage**: Store collected data securely with access controls
5. **Retention Policies**: Follow organizational data retention policies

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**Applies to**: TriageIR v1.0.0 and later