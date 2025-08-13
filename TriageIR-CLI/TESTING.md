# TriageIR CLI Testing Guide

This guide will help you build, test, and validate the TriageIR CLI application.

## Prerequisites

### Required Software

1. **Rust Toolchain**
   - Install from: https://rustup.rs/
   - This includes `cargo` (Rust package manager) and `rustc` (Rust compiler)
   - Minimum version: Rust 1.70+

2. **Windows SDK** (for Windows API access)
   - Usually installed with Visual Studio or Visual Studio Build Tools
   - Required for Windows-specific forensic functions

3. **Python** (optional, for output validation)
   - Version 3.7 or later
   - Used for the `validate_output.py` script

### System Requirements

- **Windows 10 or later** (required for full functionality)
- **Administrator privileges** (recommended for complete data collection)
- **4GB RAM minimum** (8GB recommended for large scans)
- **1GB free disk space** (for build artifacts and output files)

## Building the CLI

### Method 1: Automated Build (Recommended)

1. **Open Command Prompt as Administrator**

2. **Navigate to the CLI directory**
   ```cmd
   cd path\to\TriageIR-CLI
   ```

3. **Run the test build script**
   ```cmd
   test-build.bat
   ```

   This script will:
   - Check for Rust installation
   - Verify dependencies
   - Run unit tests
   - Build debug and release versions
   - Test basic functionality

### Method 2: Manual Build

1. **Check Rust installation**
   ```cmd
   cargo --version
   rustc --version
   ```

2. **Clean previous builds**
   ```cmd
   cargo clean
   ```

3. **Run tests**
   ```cmd
   cargo test
   ```

4. **Build debug version**
   ```cmd
   cargo build
   ```

5. **Build release version**
   ```cmd
   cargo build --release
   ```

## Testing the CLI

### Quick Functionality Test

Run the quick test script:
```cmd
quick-test.bat
```

This will test:
- Help and version commands
- Basic scan functionality
- JSON output generation
- Output validation (if Python available)

### Manual Testing

1. **Test help output**
   ```cmd
   target\release\triageir-cli.exe --help
   ```

2. **Test version**
   ```cmd
   target\release\triageir-cli.exe --version
   ```

3. **Test basic scan**
   ```cmd
   target\release\triageir-cli.exe --skip-events --skip-hashes --output test.json
   ```

4. **Test verbose scan**
   ```cmd
   target\release\triageir-cli.exe --verbose --output verbose-test.json
   ```

5. **Test custom options**
   ```cmd
   target\release\triageir-cli.exe --only processes,network --output custom-test.json
   ```

### Comprehensive Scan Test

Run a full system scan:
```cmd
run-sample-scan.bat
```

This will:
- Perform a complete forensic scan
- Generate timestamped output file
- Show scan statistics
- Offer validation option

## Validating Output

### Using the Python Validator

1. **Install Python** (if not already installed)

2. **Run the validator**
   ```cmd
   python validate_output.py output-file.json
   ```

3. **Check validation results**
   - ✅ = Validation passed
   - ❌ = Validation failed with error details

### Manual Validation

1. **Check file size** - Should be > 1KB for basic scans
2. **Verify JSON format** - Open in text editor, should be valid JSON
3. **Check required sections**:
   - `scan_metadata`
   - `artifacts`
   - `collection_log`

### Expected Output Structure

```json
{
  "scan_metadata": {
    "scan_id": "uuid-here",
    "scan_start_utc": "2023-01-01T00:00:00Z",
    "scan_duration_ms": 1234,
    "hostname": "COMPUTER-NAME",
    "os_version": "Windows 10",
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

## Performance Testing

### Timing Tests

1. **Quick scan timing**
   ```cmd
   powershell "Measure-Command { .\target\release\triageir-cli.exe --skip-events --skip-hashes --output perf-quick.json }"
   ```

2. **Full scan timing**
   ```cmd
   powershell "Measure-Command { .\target\release\triageir-cli.exe --output perf-full.json }"
   ```

### Resource Usage

Monitor during scans:
- **CPU usage** - Should be moderate (< 50% on modern systems)
- **Memory usage** - Should be < 500MB for typical scans
- **Disk I/O** - High during hash calculation, moderate otherwise

### Expected Performance

| Scan Type | Typical Duration | Memory Usage | Output Size |
|-----------|------------------|--------------|-------------|
| Quick (no hashes/events) | 10-30 seconds | 50-100 MB | 100KB-1MB |
| Standard | 1-3 minutes | 100-300 MB | 1-10 MB |
| Full (with events) | 2-10 minutes | 200-500 MB | 10-100 MB |

## Troubleshooting

### Build Issues

#### "cargo: command not found"
- **Solution**: Install Rust from https://rustup.rs/
- **Verify**: Restart command prompt after installation

#### "linker 'link.exe' not found"
- **Solution**: Install Visual Studio Build Tools
- **Alternative**: Install Visual Studio Community

#### "failed to compile winreg"
- **Solution**: Ensure Windows SDK is installed
- **Check**: Windows version compatibility

#### Tests failing
- **Check**: Run `cargo test --verbose` for detailed output
- **Common**: Some tests may fail in virtualized environments
- **Solution**: Run on physical Windows machine if possible

### Runtime Issues

#### "Access denied" errors
- **Solution**: Run as Administrator
- **Note**: Some forensic data requires elevated privileges

#### "CLI not found" in GUI
- **Check**: Executable exists in `target\release\triageir-cli.exe`
- **Solution**: Build the CLI first, then test GUI

#### Large output files
- **Cause**: Many event log entries
- **Solution**: Use `--max-events 100` to limit entries

#### Slow performance
- **Cause**: Hash calculation on many processes
- **Solution**: Use `--skip-hashes` for faster scans

### Output Issues

#### Empty or invalid JSON
- **Check**: Error messages in console output
- **Common**: Serialization errors with special characters
- **Solution**: Run with `--verbose` to see detailed logs

#### Missing data sections
- **Cause**: Collection errors for specific modules
- **Check**: Collection log for error messages
- **Note**: Some data may be unavailable on certain systems

## Integration Testing

### CLI-GUI Integration

1. **Build CLI first**
   ```cmd
   cargo build --release
   ```

2. **Test CLI standalone**
   ```cmd
   target\release\triageir-cli.exe --output test-for-gui.json
   ```

3. **Test GUI can find CLI**
   - Start GUI
   - Check status bar for "CLI available"
   - Try running a scan from GUI

### Automated Testing

Run the full test suite:
```cmd
cargo test --release --verbose
```

This includes:
- Unit tests for all modules
- Integration tests for complete workflows
- Data structure validation tests
- JSON serialization tests

## Continuous Testing

### Regular Test Commands

```cmd
REM Quick smoke test
cargo test --lib

REM Full test suite
cargo test --release

REM Build and basic functionality
cargo build --release && target\release\triageir-cli.exe --version

REM Performance regression test
powershell "Measure-Command { .\target\release\triageir-cli.exe --skip-events --output perf-test.json }"
```

### Automated Test Script

Create a batch file for regular testing:
```cmd
@echo off
echo Running TriageIR CLI test suite...
cargo test --lib || exit /b 1
cargo build --release || exit /b 1
target\release\triageir-cli.exe --version || exit /b 1
target\release\triageir-cli.exe --skip-events --output auto-test.json || exit /b 1
echo All tests passed!
```

## Next Steps

After successful CLI testing:

1. **Test with GUI**: Ensure CLI-GUI integration works
2. **Performance tuning**: Optimize for your specific environment
3. **Custom configurations**: Adjust scan options for your needs
4. **Deployment**: Package for distribution if needed

For GUI testing, see the `TriageIR-GUI/SETUP.md` file.