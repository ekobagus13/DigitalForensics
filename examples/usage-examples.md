# TriageIR Usage Examples

This document provides practical examples of using TriageIR for various digital forensics and incident response scenarios.

## Table of Contents

1. [Basic Usage Examples](#basic-usage-examples)
2. [Incident Response Scenarios](#incident-response-scenarios)
3. [Forensic Analysis Workflows](#forensic-analysis-workflows)
4. [System Monitoring Examples](#system-monitoring-examples)
5. [Automation and Scripting](#automation-and-scripting)
6. [Advanced Use Cases](#advanced-use-cases)
7. [Integration Examples](#integration-examples)
8. [Troubleshooting Examples](#troubleshooting-examples)

## Basic Usage Examples

### Quick System Triage

```cmd
# Basic scan with all default settings
triageir-cli.exe --output basic_triage.json --verbose

# Fast scan for immediate threat assessment
triageir-cli.exe --skip-hashes --skip-events --output quick_assessment.json

# Comprehensive scan with maximum detail
triageir-cli.exe --max-events 10000 --verbose --output comprehensive_scan.json

# Minimal resource usage scan
triageir-cli.exe --skip-hashes --max-events 500 --output minimal_scan.json
```

### Targeted Data Collection

```cmd
# Collect only running processes and network connections
triageir-cli.exe --only processes,network --output network_analysis.json

# Focus on persistence mechanisms
triageir-cli.exe --only persistence --output persistence_check.json

# Execution evidence analysis
triageir-cli.exe --only execution,processes --output execution_timeline.json

# System information and logged-on users
triageir-cli.exe --only system --output system_status.json

# Event logs only (security focus)
triageir-cli.exe --only events --max-events 2000 --output security_events.json
```

### Output Management

```cmd
# Save to specific directory with timestamp
triageir-cli.exe --output "C:\Evidence\scan_%DATE:~-4,4%%DATE:~-10,2%%DATE:~-7,2%_%TIME:~0,2%%TIME:~3,2%.json"

# Pipe output to analysis tool
triageir-cli.exe | python analyze_results.py

# Save with case number
triageir-cli.exe --output "CASE-2024-001_initial_triage.json" --verbose
```

## Incident Response Scenarios

### Scenario 1: Suspected Malware Infection

**Objective**: Quickly identify potentially malicious processes and network connections

**Command**:
```cmd
triageir-cli.exe --only processes,network,persistence --skip-hashes --output malware_triage.json --verbose
```

**Analysis Focus**:
- Processes with unusual names or locations
- Network connections to external IPs
- Unknown persistence mechanisms
- Processes without digital signatures

**Follow-up Actions**:
```cmd
# If suspicious processes found, get detailed execution evidence
triageir-cli.exe --only execution --output execution_evidence.json

# Get comprehensive event logs for timeline
triageir-cli.exe --only events --max-events 5000 --output incident_events.json
```

### Scenario 2: Data Exfiltration Investigation

**Objective**: Identify processes with network activity and potential data theft

**Command**:
```cmd
triageir-cli.exe --only processes,network --output data_exfil_check.json --verbose
```

**Analysis Script** (PowerShell):
```powershell
# Load and analyze results
$results = Get-Content "data_exfil_check.json" | ConvertFrom-Json

# Find processes with external network connections
$external_connections = $results.artifacts.network_connections | Where-Object {
    $_.remote_address -notmatch "^(127\.|192\.168\.|10\.|172\.(1[6-9]|2[0-9]|3[01])\.|::1|fe80:)"
}

# Map to processes
foreach ($conn in $external_connections) {
    $process = $results.artifacts.running_processes | Where-Object { $_.pid -eq $conn.owning_pid }
    Write-Host "External connection: $($conn.remote_address) by $($process.name) ($($process.pid))"
}
```

### Scenario 3: Insider Threat Assessment

**Objective**: Monitor user activity and system changes

**Command**:
```cmd
triageir-cli.exe --only system,events,execution --max-events 3000 --output insider_threat.json --verbose
```

**Key Indicators**:
- Unusual logon times or patterns
- Execution of administrative tools
- Access to sensitive directories
- USB device usage events

### Scenario 4: Ransomware Response

**Objective**: Rapid assessment of ransomware impact

**Command**:
```cmd
triageir-cli.exe --skip-events --output ransomware_assessment.json --verbose
```

**Immediate Analysis**:
- Identify encryption processes
- Check for ransom note processes
- Assess system persistence changes
- Document current system state

## Forensic Analysis Workflows

### Complete Evidence Collection

**Phase 1: Initial Triage**
```cmd
triageir-cli.exe --only system,processes --skip-hashes --output phase1_triage.json --verbose
```

**Phase 2: Detailed Analysis**
```cmd
triageir-cli.exe --max-events 10000 --output phase2_detailed.json --verbose
```

**Phase 3: Execution Timeline**
```cmd
triageir-cli.exe --only execution,events --max-events 5000 --output phase3_timeline.json --verbose
```

### Memory Analysis Preparation

```cmd
# Collect process information for memory analysis correlation
triageir-cli.exe --only processes --output memory_correlation.json --verbose

# Get network connections for memory analysis context
triageir-cli.exe --only network --output network_context.json
```

### Timeline Analysis

```cmd
# Collect execution evidence with events for timeline
triageir-cli.exe --only execution,events --max-events 8000 --output timeline_data.json --verbose
```

**Timeline Analysis Script** (Python):
```python
import json
from datetime import datetime

# Load results
with open('timeline_data.json') as f:
    data = json.load(f)

# Create timeline from execution evidence
timeline = []

# Add prefetch entries
for pf in data['artifacts']['execution_evidence']['prefetch_files']:
    timeline.append({
        'timestamp': pf['last_run_time'],
        'type': 'Execution',
        'description': f"Executed: {pf['executable_name']} (count: {pf['run_count']})"
    })

# Add event log entries
for event in data['artifacts']['event_logs']['security']:
    timeline.append({
        'timestamp': event['timestamp'],
        'type': 'Event',
        'description': f"Event {event['event_id']}: {event['message'][:100]}..."
    })

# Sort by timestamp
timeline.sort(key=lambda x: x['timestamp'])

# Output timeline
for entry in timeline[-20:]:  # Last 20 events
    print(f"{entry['timestamp']} - {entry['type']}: {entry['description']}")
```

## System Monitoring Examples

### Daily System Health Check

**Automated Script** (batch):
```cmd
@echo off
set DATE_STAMP=%DATE:~-4,4%%DATE:~-10,2%%DATE:~-7,2%
triageir-cli.exe --skip-hashes --max-events 1000 --output "daily_check_%DATE_STAMP%.json" --verbose

# Check for changes in process count
python compare_daily_scans.py
```

### Baseline Creation

```cmd
# Create system baseline
triageir-cli.exe --output baseline_system.json --verbose

# Weekly comparison
triageir-cli.exe --output weekly_check.json --verbose
python compare_with_baseline.py baseline_system.json weekly_check.json
```

### Performance Monitoring

```cmd
# Monitor system performance impact
triageir-cli.exe --only system,processes --output performance_check.json --verbose
```

## Automation and Scripting

### PowerShell Integration

```powershell
# TriageIR PowerShell wrapper function
function Invoke-TriageIR {
    param(
        [string]$OutputPath = "triageir_$(Get-Date -Format 'yyyyMMdd_HHmmss').json",
        [string[]]$ArtifactTypes = @(),
        [switch]$SkipHashes,
        [int]$MaxEvents = 1000,
        [switch]$Verbose
    )
    
    $args = @("--output", $OutputPath)
    
    if ($ArtifactTypes.Count -gt 0) {
        $args += "--only", ($ArtifactTypes -join ",")
    }
    
    if ($SkipHashes) { $args += "--skip-hashes" }
    if ($MaxEvents -ne 1000) { $args += "--max-events", $MaxEvents }
    if ($Verbose) { $args += "--verbose" }
    
    & "triageir-cli.exe" @args
    
    if ($LASTEXITCODE -eq 0) {
        return Get-Content $OutputPath | ConvertFrom-Json
    } else {
        throw "TriageIR execution failed with exit code $LASTEXITCODE"
    }
}

# Usage examples
$results = Invoke-TriageIR -ArtifactTypes @("processes", "network") -SkipHashes -Verbose
$suspicious_processes = $results.artifacts.running_processes | Where-Object { $_.name -match "temp|tmp" }
```

### Python Integration

```python
import subprocess
import json
import sys
from datetime import datetime

class TriageIR:
    def __init__(self, cli_path="triageir-cli.exe"):
        self.cli_path = cli_path
    
    def scan(self, output_file=None, artifact_types=None, skip_hashes=False, 
             max_events=1000, verbose=False):
        """Run TriageIR scan and return results"""
        
        args = [self.cli_path]
        
        if output_file:
            args.extend(["--output", output_file])
        
        if artifact_types:
            args.extend(["--only", ",".join(artifact_types)])
        
        if skip_hashes:
            args.append("--skip-hashes")
        
        if max_events != 1000:
            args.extend(["--max-events", str(max_events)])
        
        if verbose:
            args.append("--verbose")
        
        try:
            result = subprocess.run(args, capture_output=True, text=True, check=True)
            
            if output_file:
                with open(output_file, 'r') as f:
                    return json.load(f)
            else:
                return json.loads(result.stdout)
                
        except subprocess.CalledProcessError as e:
            print(f"TriageIR failed: {e.stderr}", file=sys.stderr)
            raise

# Usage examples
triage = TriageIR()

# Quick malware check
results = triage.scan(
    artifact_types=["processes", "network", "persistence"],
    skip_hashes=True,
    verbose=True
)

# Analyze results
suspicious_processes = [
    p for p in results['artifacts']['running_processes']
    if 'temp' in p['executable_path'].lower() or 'appdata' in p['executable_path'].lower()
]

print(f"Found {len(suspicious_processes)} potentially suspicious processes")
```

### Scheduled Task Integration

**Windows Task Scheduler XML**:
```xml
<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2">
  <Triggers>
    <CalendarTrigger>
      <StartBoundary>2024-01-01T02:00:00</StartBoundary>
      <ScheduleByDay>
        <DaysInterval>1</DaysInterval>
      </ScheduleByDay>
    </CalendarTrigger>
  </Triggers>
  <Actions>
    <Exec>
      <Command>C:\Program Files\TriageIR\CLI\triageir-cli.exe</Command>
      <Arguments>--output "C:\Logs\daily_triage.json" --skip-hashes --max-events 1000</Arguments>
    </Exec>
  </Actions>
</Task>
```

## Advanced Use Cases

### Multi-System Collection

**Network Deployment Script**:
```cmd
@echo off
set SYSTEMS=SERVER01 SERVER02 WORKSTATION01 WORKSTATION02

for %%S in (%SYSTEMS%) do (
    echo Collecting from %%S...
    psexec \\%%S -c triageir-cli.exe --output "\\EVIDENCE-SERVER\Collections\%%S_triage.json" --verbose
)
```

### SIEM Integration

**Splunk Integration**:
```python
import json
import requests

def send_to_splunk(triageir_results, splunk_url, auth_token):
    """Send TriageIR results to Splunk"""
    
    # Extract key indicators
    events = []
    
    # Process suspicious processes
    for process in triageir_results['artifacts']['running_processes']:
        if not process.get('sha256_hash'):  # Unsigned processes
            events.append({
                'timestamp': triageir_results['scan_metadata']['scan_start_utc'],
                'source': 'triageir',
                'sourcetype': 'triageir:process',
                'event': {
                    'type': 'suspicious_process',
                    'pid': process['pid'],
                    'name': process['name'],
                    'path': process['executable_path'],
                    'command_line': process['command_line']
                }
            })
    
    # Send to Splunk HEC
    headers = {
        'Authorization': f'Splunk {auth_token}',
        'Content-Type': 'application/json'
    }
    
    for event in events:
        response = requests.post(f"{splunk_url}/services/collector", 
                               headers=headers, 
                               json=event)
        print(f"Sent event: {response.status_code}")
```

### Custom Analysis Framework

```python
class TriageIRAnalyzer:
    def __init__(self, results_file):
        with open(results_file) as f:
            self.results = json.load(f)
    
    def find_suspicious_processes(self):
        """Identify potentially suspicious processes"""
        suspicious = []
        
        for process in self.results['artifacts']['running_processes']:
            score = 0
            reasons = []
            
            # Check for unsigned executables
            if not process.get('sha256_hash'):
                score += 3
                reasons.append("Unsigned executable")
            
            # Check for unusual locations
            path = process['executable_path'].lower()
            if any(loc in path for loc in ['temp', 'appdata', 'downloads']):
                score += 2
                reasons.append("Unusual location")
            
            # Check for suspicious names
            name = process['name'].lower()
            if any(sus in name for sus in ['temp', 'tmp', 'update', 'install']):
                score += 1
                reasons.append("Suspicious name")
            
            if score >= 3:
                suspicious.append({
                    'process': process,
                    'score': score,
                    'reasons': reasons
                })
        
        return sorted(suspicious, key=lambda x: x['score'], reverse=True)
    
    def analyze_network_connections(self):
        """Analyze network connections for anomalies"""
        external_connections = []
        
        for conn in self.results['artifacts']['network_connections']:
            remote_ip = conn['remote_address'].split(':')[0]
            
            # Check if external (not private IP ranges)
            if not any(remote_ip.startswith(prefix) for prefix in 
                      ['127.', '192.168.', '10.', '172.16.', '172.17.', 
                       '172.18.', '172.19.', '172.20.', '172.21.', '172.22.',
                       '172.23.', '172.24.', '172.25.', '172.26.', '172.27.',
                       '172.28.', '172.29.', '172.30.', '172.31.']):
                
                # Find owning process
                process = next((p for p in self.results['artifacts']['running_processes'] 
                              if p['pid'] == conn['owning_pid']), None)
                
                external_connections.append({
                    'connection': conn,
                    'process': process
                })
        
        return external_connections
    
    def generate_report(self):
        """Generate analysis report"""
        report = {
            'scan_info': self.results['scan_metadata'],
            'suspicious_processes': self.find_suspicious_processes(),
            'external_connections': self.analyze_network_connections(),
            'summary': {
                'total_processes': len(self.results['artifacts']['running_processes']),
                'total_connections': len(self.results['artifacts']['network_connections']),
                'persistence_mechanisms': len(self.results['artifacts']['persistence_mechanisms'])
            }
        }
        
        return report

# Usage
analyzer = TriageIRAnalyzer('scan_results.json')
report = analyzer.generate_report()

print(f"Analysis Report for {report['scan_info']['hostname']}")
print(f"Suspicious processes: {len(report['suspicious_processes'])}")
print(f"External connections: {len(report['external_connections'])}")
```

## Integration Examples

### Elastic Stack Integration

```python
from elasticsearch import Elasticsearch

def index_triageir_results(results, es_client, index_name):
    """Index TriageIR results in Elasticsearch"""
    
    # Index processes
    for process in results['artifacts']['running_processes']:
        doc = {
            '@timestamp': results['scan_metadata']['scan_start_utc'],
            'hostname': results['scan_metadata']['hostname'],
            'artifact_type': 'process',
            'process': process
        }
        es_client.index(index=index_name, body=doc)
    
    # Index network connections
    for conn in results['artifacts']['network_connections']:
        doc = {
            '@timestamp': results['scan_metadata']['scan_start_utc'],
            'hostname': results['scan_metadata']['hostname'],
            'artifact_type': 'network_connection',
            'connection': conn
        }
        es_client.index(index=index_name, body=doc)

# Usage
es = Elasticsearch(['localhost:9200'])
with open('scan_results.json') as f:
    results = json.load(f)

index_triageir_results(results, es, 'triageir-artifacts')
```

### Jupyter Notebook Analysis

```python
import pandas as pd
import matplotlib.pyplot as plt

# Load TriageIR results
with open('scan_results.json') as f:
    results = json.load(f)

# Convert processes to DataFrame
processes_df = pd.DataFrame(results['artifacts']['running_processes'])

# Analyze process memory usage
plt.figure(figsize=(12, 6))
plt.subplot(1, 2, 1)
processes_df['memory_usage'].hist(bins=50)
plt.title('Process Memory Usage Distribution')
plt.xlabel('Memory Usage (bytes)')

# Analyze process start times
plt.subplot(1, 2, 2)
processes_df['start_time'] = pd.to_datetime(processes_df['start_time'])
processes_df.set_index('start_time').resample('H').size().plot()
plt.title('Process Start Times by Hour')
plt.xlabel('Time')
plt.ylabel('Number of Processes Started')

plt.tight_layout()
plt.show()

# Find processes without hashes (potentially suspicious)
unsigned_processes = processes_df[processes_df['sha256_hash'].isna()]
print(f"Found {len(unsigned_processes)} unsigned processes:")
print(unsigned_processes[['name', 'executable_path']].to_string())
```

## Troubleshooting Examples

### Permission Issues

```cmd
# Test with minimal permissions
triageir-cli.exe --only system --output permission_test.json

# Check what failed
type permission_test.json | find "collection_log"
```

### Performance Issues

```cmd
# Minimal resource scan
triageir-cli.exe --skip-hashes --skip-events --only system,processes --output minimal.json

# Monitor resource usage during scan
powershell -command "Get-Process triageir-cli | Select-Object CPU,WorkingSet,VirtualMemorySize"
```

### Large System Optimization

```cmd
# Staged collection for large systems
triageir-cli.exe --only system,processes --output stage1.json
triageir-cli.exe --only network,persistence --output stage2.json
triageir-cli.exe --only events --max-events 2000 --output stage3.json
triageir-cli.exe --only execution --output stage4.json
```

### Error Diagnosis

```cmd
# Verbose output with error details
triageir-cli.exe --verbose --output debug_scan.json 2> error_log.txt

# Check collection log for issues
python -c "import json; data=json.load(open('debug_scan.json')); [print(f'{log[\"level\"]}: {log[\"message\"]}') for log in data['collection_log'] if log['level'] in ['ERROR', 'WARN']]"
```

---

**Note**: All examples assume TriageIR CLI is in PATH or current directory. Adjust paths as needed for your installation.

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