# TriageIR USB Deployment Guide

## Zero-Installation USB Forensic Toolkit

This guide explains how to create and deploy a zero-installation USB version of TriageIR that runs directly from a USB drive without requiring any installation on the target system.

## 🎯 Key Features

- ✅ **Zero Installation** - No system modifications required
- ✅ **Portable** - Runs from any USB drive or network location
- ✅ **Self-Contained** - All dependencies included
- ✅ **Forensically Sound** - Read-only system access
- ✅ **Professional** - Complete forensic toolkit
- ✅ **Automatic Output Organization** - Results saved to USB drive
- ✅ **Environment Detection** - Automatically detects portable mode

## 📦 Creating USB Portable Package

### Method 1: Automated Script (Recommended)

```cmd
# Run the USB portable package creator
scripts\create-usb-portable.bat

# This creates: build\TriageIR-USB-Portable\
```

### Method 2: Manual Creation

1. **Build Components**:
   ```cmd
   # Build CLI with static linking
   cd TriageIR-CLI
   set RUSTFLAGS=-C target-feature=+crt-static
   cargo build --release --target x86_64-pc-windows-msvc
   
   # Build GUI portable version
   cd ../TriageIR-GUI
   npm run build:portable
   ```

2. **Create Directory Structure**:
   ```
   USB-Root/
   ├── CLI/
   │   └── triageir-cli.exe
   ├── GUI/
   │   └── TriageIR.exe
   ├── Output/          (scan results)
   ├── Logs/            (application logs)
   ├── Tools/           (utilities)
   ├── docs/            (documentation)
   └── examples/        (usage examples)
   ```

3. **Copy Files**:
   ```cmd
   # Copy CLI
   copy TriageIR-CLI\target\x86_64-pc-windows-msvc\release\triageir-cli.exe USB-Root\CLI\
   
   # Copy GUI
   xcopy TriageIR-GUI\dist-portable\* USB-Root\GUI\ /s /e
   
   # Copy documentation
   xcopy docs\* USB-Root\docs\ /s /e
   ```

## 🚀 USB Deployment Process

### Step 1: Prepare USB Drive

```cmd
# Format USB drive (optional but recommended)
# Use FAT32 for maximum compatibility or NTFS for large files
format F: /fs:FAT32 /q /v:TriageIR

# Or use NTFS for better performance
format F: /fs:NTFS /q /v:TriageIR
```

### Step 2: Copy Portable Package

```cmd
# Copy entire portable package to USB root
xcopy build\TriageIR-USB-Portable\* F:\ /s /e /h

# Verify deployment
dir F:\ /s
```

### Step 3: Test USB Package

```cmd
# Test on current system
F:\TriageIR-USB.bat

# Or run quick scan
F:\Quick-Scan.bat
```

## 💻 Using USB TriageIR

### Basic Usage

1. **Insert USB Drive** into target system
2. **Navigate to USB drive** (e.g., F:\)
3. **Run launcher**: Double-click `TriageIR-USB.bat`
4. **Use commands** in the opened command prompt

### Available Launchers

| Launcher | Purpose | Usage |
|----------|---------|-------|
| `TriageIR-USB.bat` | Interactive mode | Opens command prompt with TriageIR ready |
| `TriageIR-CLI.bat` | Direct CLI access | `TriageIR-CLI.bat [options]` |
| `TriageIR-GUI.bat` | Launch GUI | Double-click to start GUI |
| `Quick-Scan.bat` | Immediate scan | Runs scan and saves to Output folder |

### Command Examples

```cmd
# Quick system assessment
triageir-cli.exe --output Output\quick_scan.json

# Incident response scan
triageir-cli.exe --only processes,network,persistence --output Output\incident.json

# Comprehensive forensic scan
triageir-cli.exe --max-events 5000 --verbose --output Output\forensic.json

# Launch GUI
TriageIR-GUI.bat
```

## 🔧 Technical Implementation

### Portable Mode Detection

TriageIR automatically detects when running in portable mode through environment variables:

```cmd
# Environment variables set by USB launchers
TRIAGEIR_PORTABLE=1
TRIAGEIR_USB_DRIVE=F:\
TRIAGEIR_OUTPUT_DIR=F:\Output
TRIAGEIR_LOG_DIR=F:\Logs
TRIAGEIR_CLI_PATH=F:\CLI\triageir-cli.exe
```

### CLI Portable Features

- **Static Linking**: No external DLL dependencies
- **Relative Paths**: Automatically uses USB drive paths
- **Portable Output**: Results saved to USB Output folder
- **Environment Detection**: Logs portable mode status

### GUI Portable Features

- **CLI Auto-Detection**: Finds CLI on USB drive automatically
- **Portable Configuration**: No registry or AppData usage
- **USB-Aware**: Saves settings and results to USB drive
- **Self-Contained**: All dependencies bundled

## 📁 Directory Structure

### USB Root Layout

```
F:\                          (USB Drive Root)
├── TriageIR-USB.bat         (Main launcher)
├── TriageIR-CLI.bat         (CLI launcher)
├── TriageIR-GUI.bat         (GUI launcher)
├── Quick-Scan.bat           (Quick scan launcher)
├── README-USB.md            (USB-specific documentation)
├── DEPLOY-TO-USB.md         (Deployment instructions)
├── autorun.inf              (Optional autorun configuration)
│
├── CLI/                     (Command-line interface)
│   └── triageir-cli.exe     (Static executable)
│
├── GUI/                     (Graphical interface)
│   ├── TriageIR.exe         (Portable GUI executable)
│   └── resources/           (GUI resources)
│
├── Output/                  (Scan results - created automatically)
│   ├── quick_scan_*.json    (Quick scan results)
│   ├── incident_*.json      (Incident response scans)
│   └── forensic_*.json      (Comprehensive scans)
│
├── Logs/                    (Application logs - created automatically)
│   ├── triageir_*.log       (CLI logs)
│   └── gui_*.log            (GUI logs)
│
├── Tools/                   (Additional utilities)
│   ├── detect-usb.bat       (USB drive detection)
│   ├── collect-system-info.bat (System info collector)
│   └── vcredist_x64.exe     (Visual C++ Redistributable)
│
├── docs/                    (Documentation)
│   ├── USER_MANUAL.md       (Complete user manual)
│   ├── QUICK_START_GUIDE.md (Quick start guide)
│   └── API_REFERENCE.md     (API documentation)
│
└── examples/                (Usage examples)
    ├── usage-examples.md    (Usage scenarios)
    ├── analysis-scripts/    (Python analysis scripts)
    └── sample-output.json   (Example output)
```

## 🔒 Security Considerations

### USB Security

- **Read-Only Operation**: TriageIR only reads from target system
- **No System Modifications**: No files written to target system
- **Audit Trail**: Complete logging of all operations
- **Integrity Verification**: SHA-256 checksums for all executables

### Antivirus Considerations

```cmd
# Add USB drive to antivirus exclusions (if needed)
# Windows Defender example:
Add-MpPreference -ExclusionPath "F:\"
Add-MpPreference -ExclusionProcess "F:\CLI\triageir-cli.exe"
Add-MpPreference -ExclusionProcess "F:\GUI\TriageIR.exe"
```

### Corporate Environment

- **Policy Compliance**: Check USB usage policies
- **Approval Process**: Get security team approval
- **Documentation**: Maintain usage logs
- **Chain of Custody**: Follow forensic procedures

## 🚨 Incident Response Usage

### Rapid Deployment Scenario

1. **Prepare USB**: Pre-configured USB drives ready for deployment
2. **On-Site Response**: Insert USB and run Quick-Scan.bat
3. **Data Collection**: Results automatically saved to USB
4. **Secure Transport**: Remove USB with collected evidence
5. **Analysis**: Use collected data for investigation

### Quick Response Commands

```cmd
# Emergency triage (30 seconds)
Quick-Scan.bat

# Malware investigation (2 minutes)
TriageIR-CLI.bat --only processes,network,persistence --skip-hashes --output Output\malware_check.json

# Data exfiltration check (1 minute)
TriageIR-CLI.bat --only processes,network --output Output\data_exfil.json

# Comprehensive evidence (5 minutes)
TriageIR-CLI.bat --max-events 10000 --verbose --output Output\comprehensive.json
```

## 🔧 Troubleshooting

### Common Issues

#### Issue: "Access Denied" errors
**Solution**:
```cmd
# Run as Administrator
runas /user:Administrator "F:\TriageIR-USB.bat"
```

#### Issue: USB drive not detected
**Solution**:
```cmd
# Check USB drive manually
F:\Tools\detect-usb.bat

# Verify environment variables
echo %TRIAGEIR_USB_DRIVE%
```

#### Issue: CLI not found
**Solution**:
```cmd
# Verify CLI exists
dir F:\CLI\triageir-cli.exe

# Check environment
echo %TRIAGEIR_CLI_PATH%
```

#### Issue: Slow performance
**Solution**:
```cmd
# Use faster USB 3.0 drive
# Copy to local temp for better performance
xcopy F:\* C:\Temp\TriageIR\ /s /e /h
C:\Temp\TriageIR\TriageIR-USB.bat
```

### Performance Optimization

#### USB Drive Selection
- **USB 3.0+**: Minimum recommended speed
- **SSD USB Drives**: Best performance
- **High-Quality Drives**: Avoid cheap/slow drives

#### System Optimization
```cmd
# Disable Windows Defender real-time scanning temporarily
Set-MpPreference -DisableRealtimeMonitoring $true

# Re-enable after use
Set-MpPreference -DisableRealtimeMonitoring $false
```

## 📊 Validation and Testing

### USB Package Validation

```cmd
# Run validation script
F:\scripts\validate-deployment.bat F:\

# Check all components
F:\Tools\test-usb-package.bat
```

### Functionality Testing

```cmd
# Test CLI
F:\CLI\triageir-cli.exe --version

# Test GUI
F:\GUI\TriageIR.exe --help

# Test quick scan
F:\Quick-Scan.bat
```

## 🔄 Updates and Maintenance

### Updating USB Package

1. **Download New Version**: Get latest TriageIR release
2. **Backup Current**: Save current USB contents
3. **Replace Files**: Update CLI and GUI executables
4. **Test Package**: Verify functionality
5. **Deploy Updated**: Replace old USB drives

### Version Management

```cmd
# Check current version
F:\CLI\triageir-cli.exe --version

# Update documentation
copy new-docs\* F:\docs\ /y

# Update examples
copy new-examples\* F:\examples\ /y
```

## 📋 Best Practices

### Preparation
- **Multiple USB Drives**: Prepare several identical drives
- **Regular Updates**: Keep USB packages current
- **Documentation**: Include case-specific instructions
- **Testing**: Regularly test USB packages

### Usage
- **Chain of Custody**: Document all USB usage
- **Secure Storage**: Store USB drives securely
- **Access Control**: Limit USB access to authorized personnel
- **Audit Trail**: Log all forensic activities

### Maintenance
- **Regular Validation**: Test USB packages monthly
- **Update Schedule**: Update with new TriageIR releases
- **Backup Strategy**: Maintain backup USB drives
- **Documentation Updates**: Keep documentation current

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**Applies to**: TriageIR v1.0.0 USB Portable Edition