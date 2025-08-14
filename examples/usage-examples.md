# TriageIR Usage Examples

This document provides practical examples of using TriageIR for various digital forensics and incident response scenarios.

## Table of Contents

1. [Basic Usage Examples](#basic-usage-examples)
2. [Incident Response Scenarios](#incident-response-scenarios)
3. [Forensic Analysis Workflows](#forensic-analysis-workflows)
4. [System Monitoring Examples](#system-monitoring-examples)
5. [Automation and Scripting](#automation-and-scripting)
6. [Advanced Use Cases](#advanced-use-cases)

## Basic Usage Examples

### Quick System Triage

```bash
# Basic scan with all default settings
triageir-cli.exe --output basic_triage.json --verbose

# Fast scan for immediate threat assessment
triageir-cli.exe --skip-hashes --skip-events --output quick_assessment.json

# Comprehensive scan with maximum detail
triageir-cli.exe --max-events 10000 --verbose --output comprehensive_scan.json
```

### Targeted Data Collection

```bash
# Collect only running processes and network connections
triageir-cli.exe --only processes,network --output network_analysis.json

# Focus on persistence mechanisms
triageir-cli.exe --only persistence --output persistence_check.json

# Execution evidence analysis
triageir-cli.exe --only execution,processes --output execution_timeline.json

# System information and logged-on users
triageir-cli.exe --only system --output system_status.json
```

## Incident Response Scenarios

### Scenario 1: Suspected Malware Infection

**Objective**: Quickly identify potentially malicious processes and network connections

```bash
# Phase 1: Immediate threat assessment (< 30 seconds)
triageir-cli.exe --only processes,network --skip-hashes --output ir_malware_phase1.json

# Phase 2: Persistence check (< 60 seconds)
triageir-cli.exe --only persistence --output ir_malware_phase2.json

# Phase 3: Execution evidence (if time permits)
triageir-cli.exe --only execution --output ir_malware_phase3.json

# Phase 4: Full collection for analysis
triageir-cli.exe --max-events 5000 --verbose --output ir_malware_full.json
```

**Analysis Focus**:
- Unusual process names or locations
- Unexpected network connections to external IPs
- Suspicious persistence mechanisms
- Recent execution evidence

### Scenario 2: Data Exfiltration Investigation

**Objective**: Identify processes and connections involved in data theft

```bash
# Network-focused collection
triageir-cli.exe --only processes,network,events --max-events 2000 --output data_exfil.json

# Include system information for timeline
triageir-cli.exe --only system,processes,network,events --max-events 5000 --output data_exfil_timeline.json
```

**Analysis Focus**:
- Large outbound network connections
- Processes accessing sensitive file locations
- Event logs showing file access patterns
- Unusual network protocols or destinations

### Scenario 3: Insider Threat Investigation

**Objective**: Document user activity and system access

```bash
# User activity focused collection
triageir-cli.exe --only system,processes,events --max-events 10000 --output insider_threat.json

# Include execution evidence for program usage
triageir-cli.exe --only system,processes,execution,events --max-events 15000 --output insider_timeline.json
```

**Analysis Focus**:
- Logged-on users and session times
- Processes run by specific users
- Event logs showing logon/logoff patterns
- Execution evidence of data access tools

## Forensic Analysis Workflows

### Complete Evidence Collection

```bash
# Full forensic collection with maximum detail
triageir-cli.exe --verbose --max-events 50000 --output forensic_complete.json

# Verify collection completeness
type forensic_complete.json | findstr "collection_log" | findstr "ERROR"
```

### Timeline Analysis

```bash
# Collect execution evidence for timeline reconstruction
triageir-cli.exe --only execution,events --max-events 20000 --output timeline_data.json

# Include process information for context
triageir-cli.exe --only execution,processes,events --max-events 20000 --output timeline_complete.json
```

### Hash-based Analysis

```bash
# Collect process hashes for malware identification
triageir-cli.exe --only processes --output process_hashes.json

# Skip hashes for speed, collect later if needed
triageir-cli.exe --skip-hashes --output quick_processes.json
```

## System Monitoring Examples

### Daily System Health Check

```bash
# Balanced daily collection
triageir-cli.exe --skip-hashes --max-events 500 --output daily_check_%DATE%.json

# Quick process monitoring
triageir-cli.exe --only processes --skip-hashes --output process_monitor_%DATE%.json
```

### Weekly Comprehensive Scan

```bash
# Weekly full system scan
triageir-cli.exe --max-events 5000 --verbose --output weekly_scan_%DATE%.json

# Compare with baseline
fc baseline_scan.json weekly_scan_%DATE%.json > weekly_changes.txt
```

### Continuous Monitoring Setup

```bash
# Hourly process snapshots
triageir-cli.exe --only processes --skip-hashes --output hourly_processes_%TIME%.json

# Daily network monitoring
triageir-cli.exe --only network,persistence --output daily_network_%DATE%.json
```

## Automation and Scripting

### Batch Script for Automated Collection

```batch
@echo off
REM Automated TriageIR Collection Script

set TIMESTAMP=%DATE:~-4,4%%DATE:~-10,2%%DATE:~-7,2%_%TIME:~0,2%%TIME:~3,2%%TIME:~6,2%
set TIMESTAMP=%TIMESTAMP: =0%
set OUTPUT_DIR=C:\TriageIR\Results
set OUTPUT_FILE=%OUTPUT_DIR%\triage_%COMPUTERNAME%_%TIMESTAMP%.json

echo Starting TriageIR collection at %DATE% %TIME%
echo Output: %OUTPUT_FILE%

REM Create output directory if it doesn't exist
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

REM Run collection
triageir-cli.exe --verbose --max-events 2000 --output "%OUTPUT_FILE%"

if %ERRORLEVEL% equ 0 (
    echo Collection completed successfully
    echo File size: 
    for %%A in ("%OUTPUT_FILE%") do echo %%~zA bytes
) else (
    echo Collection failed with error code %ERRORLEVEL%
)

echo Collection finished at %DATE% %TIME%
```

### PowerShell Script for Advanced Automation

```powershell
# Advanced TriageIR Automation Script

param(
    [string]$OutputPath = "C:\TriageIR\Results",
    [string]$ConfigFile = "triageir-config.json",
    [switch]$QuickScan,
    [switch]$FullScan
)

# Load configuration
if (Test-Path $ConfigFile) {
    $config = Get-Content $ConfigFile | ConvertFrom-Json
} else {
    $config = @{
        MaxEvents = 2000
        SkipHashes = $false
        Verbose = $true
    }
}

# Determine scan type
if ($QuickScan) {
    $args = @("--skip-hashes", "--skip-events")
    $scanType = "quick"
} elseif ($FullScan) {
    $args = @("--max-events", "10000")
    $scanType = "full"
} else {
    $args = @("--max-events", $config.MaxEvents)
    $scanType = "standard"
}

# Add verbose flag if configured
if ($config.Verbose) {
    $args += "--verbose"
}

# Generate output filename
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$hostname = $env:COMPUTERNAME
$outputFile = Join-Path $OutputPath "triage_${hostname}_${scanType}_${timestamp}.json"

# Ensure output directory exists
if (!(Test-Path $OutputPath)) {
    New-Item -ItemType Directory -Path $OutputPath -Force
}

# Run collection
Write-Host "Starting TriageIR $scanType scan..."
Write-Host "Output: $outputFile"

$args += @("--output", $outputFile)
$process = Start-Process -FilePath "triageir-cli.exe" -ArgumentList $args -Wait -PassThru

# Check results
if ($process.ExitCode -eq 0) {
    $fileSize = (Get-Item $outputFile).Length
    Write-Host "Collection completed successfully"
    Write-Host "File size: $fileSize bytes"
    
    # Optional: Upload to central location
    # Copy-Item $outputFile "\\server\share\triageir\"
} else {
    Write-Error "Collection failed with exit code $($process.ExitCode)"
}
```

### Scheduled Task Setup

```batch
REM Create scheduled task for daily collection
schtasks /create /tn "TriageIR Daily Scan" /tr "C:\Tools\TriageIR\daily-scan.bat" /sc daily /st 02:00 /ru SYSTEM

REM Create scheduled task for weekly full scan
schtasks /create /tn "TriageIR Weekly Full Scan" /tr "C:\Tools\TriageIR\weekly-scan.bat" /sc weekly /d SUN /st 01:00 /ru SYSTEM
```

## Advanced Use Cases

### Multi-System Deployment

```batch
REM Deploy to multiple systems via PsExec
for /f %%i in (computers.txt) do (
    echo Collecting from %%i
    psexec \\%%i -c triageir-cli.exe --output \\server\share\%%i_triage.json --verbose
)
```

### Custom Collection Profiles

```bash
# Malware analysis profile
triageir-cli.exe --only processes,network,persistence,execution --max-events 1000 --output malware_profile.json

# Network security profile
triageir-cli.exe --only network,processes,events --max-events 3000 --output network_profile.json

# User activity profile
triageir-cli.exe --only system,processes,events,execution --max-events 5000 --output user_activity_profile.json

# Performance monitoring profile
triageir-cli.exe --only system,processes --skip-hashes --output performance_profile.json
```

### Integration with SIEM Systems

```powershell
# Convert TriageIR output for SIEM ingestion
$triageData = Get-Content "triage_results.json" | ConvertFrom-Json

# Extract key indicators for SIEM
$indicators = @{
    Timestamp = $triageData.scan_metadata.scan_start_utc
    Hostname = $triageData.scan_metadata.hostname
    ProcessCount = $triageData.artifacts.running_processes.Count
    NetworkConnections = $triageData.artifacts.network_connections.Count
    PersistenceMechanisms = $triageData.artifacts.persistence_mechanisms.Count
    Errors = ($triageData.collection_log | Where-Object { $_.level -eq "ERROR" }).Count
}

# Send to SIEM (example with REST API)
$json = $indicators | ConvertTo-Json
Invoke-RestMethod -Uri "https://siem.company.com/api/events" -Method POST -Body $json -ContentType "application/json"
```

### Comparison and Baseline Analysis

```bash
# Create baseline
triageir-cli.exe --output baseline.json --verbose

# Regular comparison scans
triageir-cli.exe --output current.json --verbose

# Compare results (requires custom script)
python compare_scans.py baseline.json current.json --output changes.json
```

### Memory-Constrained Environments

```bash
# Minimal resource usage
triageir-cli.exe --only system,processes --skip-hashes --skip-events --output minimal.json

# Staged collection to reduce memory usage
triageir-cli.exe --only system --output stage1.json
triageir-cli.exe --only processes --skip-hashes --output stage2.json
triageir-cli.exe --only network --output stage3.json
```

## Output Analysis Examples

### PowerShell Analysis Scripts

```powershell
# Load and analyze TriageIR results
$results = Get-Content "triage_results.json" | ConvertFrom-Json

# Find suspicious processes
$suspiciousProcesses = $results.artifacts.running_processes | Where-Object {
    $_.executable_path -notlike "C:\Windows\*" -and
    $_.executable_path -notlike "C:\Program Files*" -and
    $_.name -notlike "*.exe"
}

# Find external network connections
$externalConnections = $results.artifacts.network_connections | Where-Object {
    $_.remote_address -notlike "127.*" -and
    $_.remote_address -notlike "192.168.*" -and
    $_.remote_address -notlike "10.*" -and
    $_.state -eq "ESTABLISHED"
}

# Find unusual persistence mechanisms
$unusualPersistence = $results.artifacts.persistence_mechanisms | Where-Object {
    $_.source -notlike "*Microsoft*" -and
    $_.source -notlike "*Windows*"
}

# Generate summary report
Write-Host "=== TriageIR Analysis Summary ==="
Write-Host "Scan Time: $($results.scan_metadata.scan_start_utc)"
Write-Host "Hostname: $($results.scan_metadata.hostname)"
Write-Host "Total Processes: $($results.artifacts.running_processes.Count)"
Write-Host "Suspicious Processes: $($suspiciousProcesses.Count)"
Write-Host "External Connections: $($externalConnections.Count)"
Write-Host "Unusual Persistence: $($unusualPersistence.Count)"
```

### Python Analysis Example

```python
import json
import datetime
from collections import Counter

# Load TriageIR results
with open('triage_results.json', 'r') as f:
    results = json.load(f)

# Analyze process patterns
process_names = [p['name'] for p in results['artifacts']['running_processes']]
process_counts = Counter(process_names)

print("Top 10 Most Common Processes:")
for name, count in process_counts.most_common(10):
    print(f"  {name}: {count}")

# Analyze network connections
connections = results['artifacts']['network_connections']
external_ips = set()
for conn in connections:
    remote_ip = conn['remote_address'].split(':')[0]
    if not (remote_ip.startswith('127.') or 
            remote_ip.startswith('192.168.') or 
            remote_ip.startswith('10.')):
        external_ips.add(remote_ip)

print(f"\nExternal IP Addresses: {len(external_ips)}")
for ip in sorted(external_ips):
    print(f"  {ip}")

# Timeline analysis
scan_time = datetime.datetime.fromisoformat(
    results['scan_metadata']['scan_start_utc'].replace('Z', '+00:00')
)
print(f"\nScan performed at: {scan_time}")
print(f"System uptime: {results['artifacts']['system_info']['uptime_secs']} seconds")
```

---

These examples demonstrate the flexibility and power of TriageIR for various digital forensics and incident response scenarios. Adapt these examples to your specific environment and requirements.