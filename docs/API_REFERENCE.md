# TriageIR API Reference

## Table of Contents

1. [CLI Command Line Interface](#cli-command-line-interface)
2. [JSON Output Schema](#json-output-schema)
3. [Data Structures](#data-structures)
4. [GUI IPC API](#gui-ipc-api)
5. [Error Codes and Messages](#error-codes-and-messages)
6. [Configuration Options](#configuration-options)
7. [Extension Points](#extension-points)

## CLI Command Line Interface

### Synopsis

```bash
triageir-cli [OPTIONS]
```

### Options

#### Output Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--output` | `-o` | `<FILE>` | stdout | Output file path for JSON results |
| `--format` | | `<FORMAT>` | json | Output format (currently only 'json' supported) |
| `--verbose` | `-v` | flag | false | Enable verbose logging to stderr |

#### Collection Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--skip-hashes` | flag | false | Skip SHA-256 hash calculation for processes |
| `--skip-events` | flag | false | Skip Windows Event Log collection entirely |
| `--max-events` | `<NUM>` | 1000 | Maximum number of event log entries to collect |
| `--only` | `<TYPES>` | all | Comma-separated list of artifact types to collect |

#### Utility Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help information and exit |
| `--version` | `-V` | Show version information and exit |

### Artifact Types

When using `--only`, specify one or more of these types (comma-separated):

- **system**: System information (uptime, users, OS version)
- **processes**: Running processes with metadata and hashes
- **network**: Active network connections with owning processes
- **persistence**: Autostart mechanisms and persistence methods
- **events**: Windows Event Log entries (Security and System)
- **execution**: Execution evidence (Prefetch and Shimcache)

### Usage Examples

#### Basic Collection
```bash
# Collect all artifacts to stdout
triageir-cli

# Collect all artifacts to file
triageir-cli --output scan_results.json

# Verbose collection with progress
triageir-cli --verbose --output detailed_scan.json
```

#### Performance Optimized
```bash
# Fast collection without hashes
triageir-cli --skip-hashes --output fast_scan.json

# Minimal collection
triageir-cli --skip-hashes --skip-events --output minimal.json

# Limited event logs
triageir-cli --max-events 100 --output limited.json
```

#### Targeted Collection
```bash
# Process and network only
triageir-cli --only processes,network --output network_analysis.json

# Persistence mechanisms only
triageir-cli --only persistence --output persistence_check.json

# Execution evidence focus
triageir-cli --only execution,processes --output execution_analysis.json
```

### Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Collection completed without errors |
| 1 | Warning | Non-fatal errors occurred, collection may be incomplete |
| 2 | Error | Fatal errors occurred, collection failed |

## JSON Output Schema

### Root Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["scan_metadata", "artifacts", "collection_log"],
  "properties": {
    "scan_metadata": { "$ref": "#/definitions/ScanMetadata" },
    "artifacts": { "$ref": "#/definitions/Artifacts" },
    "collection_log": {
      "type": "array",
      "items": { "$ref": "#/definitions/LogEntry" }
    }
  }
}
```

### Schema Definitions

#### ScanMetadata
```json
{
  "ScanMetadata": {
    "type": "object",
    "required": ["scan_id", "scan_start_utc", "scan_duration_ms", "hostname", "os_version", "cli_version"],
    "properties": {
      "scan_id": {
        "type": "string",
        "format": "uuid",
        "description": "Unique identifier for this scan (UUID v4)"
      },
      "scan_start_utc": {
        "type": "string",
        "format": "date-time",
        "description": "Scan start time in UTC (ISO 8601)"
      },
      "scan_duration_ms": {
        "type": "integer",
        "minimum": 0,
        "description": "Total scan duration in milliseconds"
      },
      "hostname": {
        "type": "string",
        "description": "Target system hostname"
      },
      "os_version": {
        "type": "string",
        "description": "Operating system version string"
      },
      "cli_version": {
        "type": "string",
        "description": "TriageIR CLI version used for collection"
      }
    }
  }
}
```

#### Artifacts
```json
{
  "Artifacts": {
    "type": "object",
    "required": ["system_info", "running_processes", "network_connections", "persistence_mechanisms", "event_logs", "execution_evidence"],
    "properties": {
      "system_info": { "$ref": "#/definitions/SystemInfo" },
      "running_processes": {
        "type": "array",
        "items": { "$ref": "#/definitions/Process" }
      },
      "network_connections": {
        "type": "array",
        "items": { "$ref": "#/definitions/NetworkConnection" }
      },
      "persistence_mechanisms": {
        "type": "array",
        "items": { "$ref": "#/definitions/PersistenceMechanism" }
      },
      "event_logs": { "$ref": "#/definitions/EventLogs" },
      "execution_evidence": { "$ref": "#/definitions/ExecutionEvidence" }
    }
  }
}
```

## Data Structures

### SystemInfo

```json
{
  "SystemInfo": {
    "type": "object",
    "required": ["uptime_secs", "logged_on_users", "hostname", "os_version", "architecture", "total_memory", "available_memory"],
    "properties": {
      "uptime_secs": {
        "type": "integer",
        "minimum": 0,
        "description": "System uptime in seconds"
      },
      "logged_on_users": {
        "type": "array",
        "items": { "$ref": "#/definitions/LoggedOnUser" },
        "description": "Currently logged-on users"
      },
      "hostname": {
        "type": "string",
        "description": "System hostname"
      },
      "os_version": {
        "type": "string",
        "description": "Operating system version"
      },
      "architecture": {
        "type": "string",
        "enum": ["x86", "x64", "ARM", "ARM64"],
        "description": "System architecture"
      },
      "total_memory": {
        "type": "integer",
        "minimum": 0,
        "description": "Total system memory in bytes"
      },
      "available_memory": {
        "type": "integer",
        "minimum": 0,
        "description": "Available system memory in bytes"
      }
    }
  }
}
```

### Process

```json
{
  "Process": {
    "type": "object",
    "required": ["pid", "parent_pid", "name", "command_line", "executable_path", "start_time"],
    "properties": {
      "pid": {
        "type": "integer",
        "minimum": 0,
        "description": "Process identifier"
      },
      "parent_pid": {
        "type": "integer",
        "minimum": 0,
        "description": "Parent process identifier"
      },
      "name": {
        "type": "string",
        "description": "Process name"
      },
      "command_line": {
        "type": "string",
        "description": "Full command line used to start the process"
      },
      "executable_path": {
        "type": "string",
        "description": "Full path to the process executable"
      },
      "sha256_hash": {
        "type": ["string", "null"],
        "pattern": "^[a-fA-F0-9]{64}$",
        "description": "SHA-256 hash of the executable file"
      },
      "start_time": {
        "type": "string",
        "format": "date-time",
        "description": "Process start time in UTC (ISO 8601)"
      },
      "memory_usage": {
        "type": "integer",
        "minimum": 0,
        "description": "Process memory usage in bytes"
      },
      "cpu_usage": {
        "type": "number",
        "minimum": 0,
        "maximum": 100,
        "description": "CPU usage percentage"
      },
      "loaded_dlls": {
        "type": "array",
        "items": { "$ref": "#/definitions/LoadedDll" },
        "description": "Dynamic libraries loaded by the process"
      }
    }
  }
}
```

### NetworkConnection

```json
{
  "NetworkConnection": {
    "type": "object",
    "required": ["protocol", "local_address", "remote_address", "state", "owning_pid"],
    "properties": {
      "protocol": {
        "type": "string",
        "enum": ["TCP", "UDP"],
        "description": "Network protocol"
      },
      "local_address": {
        "type": "string",
        "description": "Local address and port (format: IP:PORT)"
      },
      "remote_address": {
        "type": "string",
        "description": "Remote address and port (format: IP:PORT)"
      },
      "state": {
        "type": "string",
        "enum": ["LISTENING", "ESTABLISHED", "TIME_WAIT", "CLOSE_WAIT", "FIN_WAIT1", "FIN_WAIT2", "SYN_SENT", "SYN_RECV", "LAST_ACK", "CLOSED"],
        "description": "Connection state"
      },
      "owning_pid": {
        "type": "integer",
        "minimum": 0,
        "description": "Process ID that owns this connection"
      }
    }
  }
}
```

### PersistenceMechanism

```json
{
  "PersistenceMechanism": {
    "type": "object",
    "required": ["type", "name", "command", "source"],
    "properties": {
      "type": {
        "type": "string",
        "enum": ["Registry Run Key", "Scheduled Task", "Service", "Startup Folder", "WMI Event", "DLL Hijacking"],
        "description": "Type of persistence mechanism"
      },
      "name": {
        "type": "string",
        "description": "Name or identifier of the persistence mechanism"
      },
      "command": {
        "type": "string",
        "description": "Command or executable being persisted"
      },
      "source": {
        "type": "string",
        "description": "Source location (registry key, file path, etc.)"
      },
      "enabled": {
        "type": "boolean",
        "description": "Whether the persistence mechanism is currently enabled"
      },
      "last_modified": {
        "type": ["string", "null"],
        "format": "date-time",
        "description": "Last modification time in UTC (ISO 8601)"
      }
    }
  }
}
```

### EventLogs

```json
{
  "EventLogs": {
    "type": "object",
    "required": ["security", "system", "application"],
    "properties": {
      "security": {
        "type": "array",
        "items": { "$ref": "#/definitions/EventLogEntry" },
        "description": "Security event log entries"
      },
      "system": {
        "type": "array",
        "items": { "$ref": "#/definitions/EventLogEntry" },
        "description": "System event log entries"
      },
      "application": {
        "type": "array",
        "items": { "$ref": "#/definitions/EventLogEntry" },
        "description": "Application event log entries"
      }
    }
  }
}
```

### EventLogEntry

```json
{
  "EventLogEntry": {
    "type": "object",
    "required": ["event_id", "level", "timestamp", "source", "message"],
    "properties": {
      "event_id": {
        "type": "integer",
        "minimum": 0,
        "description": "Windows Event ID"
      },
      "level": {
        "type": "string",
        "enum": ["Critical", "Error", "Warning", "Information", "Verbose"],
        "description": "Event severity level"
      },
      "timestamp": {
        "type": "string",
        "format": "date-time",
        "description": "Event timestamp in UTC (ISO 8601)"
      },
      "source": {
        "type": "string",
        "description": "Event source or provider name"
      },
      "message": {
        "type": "string",
        "description": "Event message text"
      },
      "user": {
        "type": ["string", "null"],
        "description": "User associated with the event"
      },
      "computer": {
        "type": "string",
        "description": "Computer name where event occurred"
      }
    }
  }
}
```

### ExecutionEvidence

```json
{
  "ExecutionEvidence": {
    "type": "object",
    "required": ["prefetch_files", "shimcache_entries"],
    "properties": {
      "prefetch_files": {
        "type": "array",
        "items": { "$ref": "#/definitions/PrefetchFile" },
        "description": "Windows Prefetch file entries"
      },
      "shimcache_entries": {
        "type": "array",
        "items": { "$ref": "#/definitions/ShimcacheEntry" },
        "description": "Application Compatibility Cache entries"
      }
    }
  }
}
```

### PrefetchFile

```json
{
  "PrefetchFile": {
    "type": "object",
    "required": ["filename", "executable_name", "run_count", "last_run_time", "file_paths"],
    "properties": {
      "filename": {
        "type": "string",
        "description": "Prefetch file name (.pf file)"
      },
      "executable_name": {
        "type": "string",
        "description": "Name of the executed program"
      },
      "run_count": {
        "type": "integer",
        "minimum": 0,
        "description": "Number of times the program was executed"
      },
      "last_run_time": {
        "type": "string",
        "format": "date-time",
        "description": "Last execution time in UTC (ISO 8601)"
      },
      "file_paths": {
        "type": "array",
        "items": { "type": "string" },
        "description": "File paths accessed during execution"
      },
      "prefetch_hash": {
        "type": "string",
        "pattern": "^[a-fA-F0-9]{8}$",
        "description": "Prefetch hash value"
      }
    }
  }
}
```

### ShimcacheEntry

```json
{
  "ShimcacheEntry": {
    "type": "object",
    "required": ["path", "last_modified", "file_size"],
    "properties": {
      "path": {
        "type": "string",
        "description": "Full path to the executable"
      },
      "last_modified": {
        "type": "string",
        "format": "date-time",
        "description": "File last modification time in UTC (ISO 8601)"
      },
      "file_size": {
        "type": "integer",
        "minimum": 0,
        "description": "File size in bytes"
      },
      "executed": {
        "type": ["boolean", "null"],
        "description": "Whether the file was executed (null if unknown)"
      }
    }
  }
}
```

## GUI IPC API

### Main Process → Renderer Events

#### scan-progress
Emitted during CLI execution to provide progress updates.

```javascript
window.electronAPI.onScanProgress((progressData) => {
    // progressData: string - Progress message from CLI stderr
    console.log('Scan progress:', progressData);
});
```

#### scan-complete
Emitted when scan completes successfully.

```javascript
window.electronAPI.onScanComplete((results) => {
    // results: ScanResults - Complete scan results
    console.log('Scan completed:', results);
});
```

#### scan-error
Emitted when scan fails with an error.

```javascript
window.electronAPI.onScanError((error) => {
    // error: { message: string, code?: number }
    console.error('Scan error:', error);
});
```

### Renderer → Main Process Methods

#### runCliScan(options)
Executes the CLI tool with specified options.

```javascript
const results = await window.electronAPI.runCliScan({
    verbose: boolean,           // Enable verbose logging
    skipHashes: boolean,        // Skip process hash calculation
    skipEvents: boolean,        // Skip event log collection
    maxEvents: number,          // Limit event log entries
    only: string[]             // Artifact types to collect
});
```

**Returns**: `Promise<ScanResults>` - Complete scan results

**Throws**: Error if CLI execution fails

#### saveResults(data, filePath?)
Saves scan results to a file.

```javascript
await window.electronAPI.saveResults(scanResults, '/path/to/output.json');
```

**Parameters**:
- `data: ScanResults` - Scan results to save
- `filePath?: string` - Optional file path (shows dialog if not provided)

**Returns**: `Promise<string>` - Path where file was saved

#### loadResults(filePath?)
Loads scan results from a file.

```javascript
const results = await window.electronAPI.loadResults('/path/to/results.json');
```

**Parameters**:
- `filePath?: string` - Optional file path (shows dialog if not provided)

**Returns**: `Promise<ScanResults>` - Loaded scan results

#### exportReport(data, format, filePath?)
Exports scan results as a formatted report.

```javascript
await window.electronAPI.exportReport(scanResults, 'html', '/path/to/report.html');
```

**Parameters**:
- `data: ScanResults` - Scan results to export
- `format: 'html' | 'pdf' | 'csv'` - Export format
- `filePath?: string` - Optional file path (shows dialog if not provided)

**Returns**: `Promise<string>` - Path where report was saved

#### showOpenDialog(options)
Shows a file open dialog.

```javascript
const filePaths = await window.electronAPI.showOpenDialog({
    title: 'Open Scan Results',
    filters: [
        { name: 'JSON Files', extensions: ['json'] },
        { name: 'All Files', extensions: ['*'] }
    ],
    properties: ['openFile']
});
```

**Returns**: `Promise<string[]>` - Selected file paths

#### showSaveDialog(options)
Shows a file save dialog.

```javascript
const filePath = await window.electronAPI.showSaveDialog({
    title: 'Save Scan Results',
    defaultPath: 'scan_results.json',
    filters: [
        { name: 'JSON Files', extensions: ['json'] },
        { name: 'All Files', extensions: ['*'] }
    ]
});
```

**Returns**: `Promise<string>` - Selected file path

## Error Codes and Messages

### CLI Error Codes

| Code | Category | Description |
|------|----------|-------------|
| 0 | Success | No errors occurred |
| 1 | Warning | Non-fatal errors, collection may be incomplete |
| 2 | Fatal | Fatal errors, collection failed |

### Common Error Messages

#### CLI Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "Access denied" | Insufficient privileges | Run as Administrator |
| "File not found" | CLI executable missing | Verify installation |
| "Invalid JSON output" | CLI output corruption | Check CLI logs, retry |
| "Timeout" | CLI execution timeout | Increase timeout, check system load |

#### GUI Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "CLI not found" | CLI executable not located | Check CLI installation path |
| "Parse error" | Invalid JSON from CLI | Verify CLI output format |
| "Permission denied" | File access restrictions | Check file permissions |
| "Out of memory" | Large dataset processing | Reduce dataset size, increase memory |

### Error Handling Patterns

#### CLI Error Handling
```rust
use std::process;

fn handle_error(error: &dyn std::error::Error) -> ! {
    eprintln!("Error: {}", error);
    
    // Log additional context
    if let Some(source) = error.source() {
        eprintln!("Caused by: {}", source);
    }
    
    // Exit with appropriate code
    process::exit(2);
}
```

#### GUI Error Handling
```javascript
class ErrorHandler {
    static handle(error, context = '') {
        console.error(`Error in ${context}:`, error);
        
        // Show user-friendly message
        const message = this.getUserMessage(error);
        this.showErrorDialog(message);
        
        // Log for debugging
        this.logError(error, context);
    }
    
    static getUserMessage(error) {
        if (error.message.includes('CLI not found')) {
            return 'TriageIR CLI executable not found. Please check installation.';
        }
        
        if (error.message.includes('Access denied')) {
            return 'Access denied. Please run as Administrator for complete data collection.';
        }
        
        return 'An unexpected error occurred. Please check the logs for details.';
    }
}
```

## Configuration Options

### CLI Configuration

#### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `TRIAGEIR_LOG_LEVEL` | Logging level (ERROR, WARN, INFO, DEBUG) | INFO |
| `TRIAGEIR_MAX_MEMORY` | Maximum memory usage in MB | 512 |
| `TRIAGEIR_TIMEOUT` | Collection timeout in seconds | 300 |

#### Configuration File
The CLI can optionally read from `triageir.toml`:

```toml
[collection]
max_events = 1000
skip_hashes = false
skip_events = false
timeout_secs = 300

[output]
format = "json"
pretty_print = false
compress = false

[logging]
level = "info"
file = "triageir.log"
```

### GUI Configuration

#### Settings File
GUI settings are stored in `%APPDATA%/TriageIR/settings.json`:

```json
{
  "cli_path": "C:\\Tools\\TriageIR\\triageir-cli.exe",
  "default_output_dir": "C:\\TriageIR\\Results",
  "auto_save": true,
  "theme": "dark",
  "max_table_rows": 1000,
  "export_format": "html"
}
```

#### Runtime Configuration
```javascript
const config = {
    cliPath: 'path/to/triageir-cli.exe',
    defaultOptions: {
        verbose: true,
        skipHashes: false,
        maxEvents: 1000
    },
    ui: {
        theme: 'dark',
        autoRefresh: false,
        maxTableRows: 1000
    }
};
```

## Extension Points

### CLI Extensions

#### Custom Collection Modules
```rust
pub trait CollectionModule {
    fn name(&self) -> &str;
    fn collect(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    fn dependencies(&self) -> Vec<&str> { vec![] }
}

// Register custom module
pub fn register_module(module: Box<dyn CollectionModule>) {
    // Registration logic
}
```

#### Custom Output Formats
```rust
pub trait OutputFormatter {
    fn format(&self, results: &ScanResults) -> Result<String, Box<dyn std::error::Error>>;
    fn file_extension(&self) -> &str;
    fn mime_type(&self) -> &str;
}

// Register custom formatter
pub fn register_formatter(name: &str, formatter: Box<dyn OutputFormatter>) {
    // Registration logic
}
```

### GUI Extensions

#### Custom Visualizations
```javascript
class CustomVisualization {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
    }
    
    render(data) {
        // Custom rendering logic
    }
    
    update(newData) {
        // Update logic
    }
}

// Register visualization
VisualizationRegistry.register('custom-viz', CustomVisualization);
```

#### Plugin System
```javascript
class PluginManager {
    constructor() {
        this.plugins = new Map();
    }
    
    register(name, plugin) {
        if (this.validatePlugin(plugin)) {
            this.plugins.set(name, plugin);
            plugin.initialize();
        }
    }
    
    validatePlugin(plugin) {
        return typeof plugin.initialize === 'function' &&
               typeof plugin.render === 'function';
    }
}
```

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**Applies to**: TriageIR v1.0.0 and later