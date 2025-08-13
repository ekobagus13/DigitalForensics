# TriageIR CLI

A forensically sound digital forensics triage tool for Windows systems, designed for rapid evidence collection during incident response and forensic analysis.

## Features

- **System Information**: Uptime, logged-on users, OS version
- **Process Analysis**: Running processes with SHA-256 hashes and metadata
- **Network Connections**: Active TCP/UDP connections with owning processes
- **Persistence Mechanisms**: Registry Run keys, services, startup folders
- **Event Log Collection**: Security and System event logs with filtering
- **Forensically Sound**: Minimal system impact, comprehensive logging
- **Portable**: Single static executable with no dependencies

## Quick Start

### Download and Run

1. Download the latest release executable
2. Run as Administrator for complete data collection
3. Execute basic scan:

```cmd
triageir-cli.exe --output scan_results.json
```

### Build from Source

#### Prerequisites

- **Rust toolchain**: Install from https://rustup.rs/
- **Windows SDK**: Required for Windows API access
- **Administrative privileges**: Recommended for complete data collection

#### Building

```cmd
# Clone and build
git clone <repository-url>
cd TriageIR-CLI

# Run build script
build.bat

# Or build manually
cargo build --release
```

#### Testing

```cmd
# Run all tests
test.bat

# Or run manually
cargo test
cargo test --test integration_tests
```

## Usage

### Basic Commands

```cmd
# Basic scan to stdout
triageir-cli.exe

# Scan with output to file
triageir-cli.exe --output results.json

# Verbose scan with progress information
triageir-cli.exe --verbose --output verbose_results.json

# Fast scan without process hashes
triageir-cli.exe --skip-hashes --output quick_scan.json

# Targeted collection (processes and network only)
triageir-cli.exe --only processes,network --output targeted.json
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--output`, `-o` | Output file path | stdout |
| `--verbose`, `-v` | Enable verbose logging | false |
| `--skip-hashes` | Skip process hash calculation | false |
| `--skip-events` | Skip event log collection | false |
| `--max-events` | Limit event log entries | 1000 |
| `--only` | Collect specific artifacts only | all |
| `--help`, `-h` | Show help information | - |
| `--version`, `-V` | Show version information | - |

See [USAGE.md](USAGE.md) for detailed usage examples and [PERFORMANCE.md](PERFORMANCE.md) for performance optimization.

## Output Format

The tool outputs structured JSON data:

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

## Project Structure

```
TriageIR-CLI/
├── src/
│   ├── main.rs              # Main application and CLI parsing
│   ├── types.rs             # Data structures and JSON serialization
│   ├── system_info.rs       # System information collection
│   ├── processes.rs         # Process enumeration and analysis
│   ├── network.rs           # Network connection enumeration
│   ├── persistence.rs       # Persistence mechanism detection
│   ├── event_logs.rs        # Windows Event Log collection
│   ├── logger.rs            # Logging and error handling
│   └── integration_tests.rs # Integration tests
├── build.bat                # Windows build script
├── build.sh                 # Unix build script
├── test.bat                 # Windows test script
├── validate_output.py       # JSON output validator
├── USAGE.md                 # Detailed usage guide
├── PERFORMANCE.md           # Performance optimization guide
└── Cargo.toml              # Rust project configuration
```

## Development

### Running Tests

```cmd
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration_tests

# All tests
cargo test

# Test with coverage
cargo test --verbose
```

### Code Quality

```cmd
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for issues
cargo check
```

### Performance Testing

```cmd
# Build optimized release
cargo build --release

# Run performance tests
target\release\triageir-cli.exe --verbose --output perf_test.json

# Validate output
python validate_output.py perf_test.json --verbose
```

## Dependencies

### Core Dependencies

- **clap** - Command line argument parsing
- **serde** - JSON serialization and deserialization
- **serde_json** - JSON format support
- **uuid** - UUID generation for scan metadata
- **chrono** - Date and time handling

### Windows-Specific Dependencies

- **winreg** - Windows Registry access
- **sysinfo** - Cross-platform system information
- **windows** - Windows API bindings
- **sha2** - SHA-256 hash calculation
- **hex** - Hexadecimal encoding

### Development Dependencies

- **tempfile** - Temporary file handling for tests

## Security Considerations

1. **Administrative Privileges**: Required for complete data collection
2. **Data Sensitivity**: Output contains sensitive system information
3. **Network Isolation**: Tool makes no network connections
4. **Audit Trail**: All operations are logged with timestamps
5. **File Permissions**: Secure output files appropriately

## Performance

- **Typical scan time**: 30-120 seconds (depends on system and options)
- **Memory usage**: 10-100 MB (depends on artifact count)
- **Output size**: 1-50 MB (depends on event log settings)
- **CPU impact**: Low-medium (high during hash calculation)

See [PERFORMANCE.md](PERFORMANCE.md) for detailed performance characteristics and optimization strategies.

## Exit Codes

- **0**: Success, collection completed without errors
- **1**: Non-fatal errors occurred, collection may be incomplete
- **2**: Fatal errors occurred, collection failed

## Validation

Use the included Python validator to verify output format:

```cmd
python validate_output.py scan_results.json --verbose
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Support

For issues, questions, or feature requests:
1. Check existing documentation
2. Search existing issues
3. Create a new issue with detailed information
4. Include system information and error messages