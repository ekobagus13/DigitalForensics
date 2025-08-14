# TriageIR Installation Guide

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installation Methods](#installation-methods)
3. [Post-Installation Setup](#post-installation-setup)
4. [Verification](#verification)
5. [Troubleshooting](#troubleshooting)
6. [Uninstallation](#uninstallation)

## System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| **Operating System** | Windows 10 (1903) or Windows Server 2016 |
| **Architecture** | x64 (64-bit) |
| **RAM** | 4 GB |
| **Disk Space** | 100 MB for installation |
| **Permissions** | Standard user (Administrator recommended) |

### Recommended Requirements

| Component | Requirement |
|-----------|-------------|
| **Operating System** | Windows 11 or Windows Server 2022 |
| **RAM** | 8 GB or more |
| **Disk Space** | 1 GB free space |
| **Permissions** | Administrator privileges |
| **Additional** | SSD storage for better performance |

### Software Dependencies

- **CLI Component**: No external dependencies (static executable)
- **GUI Component**: Bundled with all required components
- **Optional**: Python 3.7+ for analysis scripts

## Installation Methods

### Method 1: Windows Installer (Recommended)

1. **Download the Installer**
   ```
   Download: TriageIR-Setup.exe
   Size: ~50 MB
   ```

2. **Run the Installer**
   - Right-click `TriageIR-Setup.exe`
   - Select "Run as administrator"
   - Follow the installation wizard

3. **Installation Options**
   - **Complete**: Installs all components (recommended)
   - **CLI Only**: Command-line interface only
   - **Custom**: Choose specific components

4. **Installation Locations**
   - Default: `C:\Program Files\TriageIR\`
   - Portable: Any directory of your choice

### Method 2: Portable Installation

1. **Download Portable Package**
   ```
   Download: TriageIR-Portable.zip
   Size: ~45 MB
   ```

2. **Extract Package**
   - Extract to desired location (e.g., `C:\Tools\TriageIR\`)
   - No installation required
   - Can be run from USB drive

3. **Verify Extraction**
   ```
   TriageIR/
   ├── CLI/
   │   └── triageir-cli.exe
   ├── GUI/
   │   └── TriageIR.exe
   ├── docs/
   └── README.md
   ```

### Method 3: Package Managers

#### Chocolatey
```powershell
# Install Chocolatey (if not already installed)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install TriageIR
choco install triageir
```

#### Scoop
```powershell
# Install Scoop (if not already installed)
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# Add TriageIR bucket
scoop bucket add triageir https://github.com/triageir/scoop-bucket

# Install TriageIR
scoop install triageir
```

### Method 4: Build from Source

#### Prerequisites
- Rust 1.70+ (for CLI)
- Node.js 16+ (for GUI)
- Git

#### Build Steps
```bash
# Clone repository
git clone https://github.com/triageir/triageir.git
cd triageir

# Build CLI
cd TriageIR-CLI
cargo build --release

# Build GUI
cd ../TriageIR-GUI
npm install
npm run build

# Create deployment package
cd ..
scripts\create-deployment-package.bat
```

## Post-Installation Setup

### Environment Variables

The installer automatically adds TriageIR to your PATH. To verify:

```cmd
echo %PATH%
# Should include C:\Program Files\TriageIR
```

To add manually:
```cmd
setx PATH "%PATH%;C:\Program Files\TriageIR"
```

### File Associations

Associate `.triageir` files with the GUI application:

1. Right-click a `.triageir` file
2. Select "Open with" → "Choose another app"
3. Browse to `TriageIR.exe`
4. Check "Always use this app"

### Desktop Shortcuts

Create desktop shortcuts for quick access:

```cmd
# CLI Shortcut
echo @echo off > "%USERPROFILE%\Desktop\TriageIR CLI.bat"
echo cd /d "C:\Program Files\TriageIR\CLI" >> "%USERPROFILE%\Desktop\TriageIR CLI.bat"
echo cmd /k triageir-cli.exe --help >> "%USERPROFILE%\Desktop\TriageIR CLI.bat"

# GUI Shortcut (created automatically by installer)
```

### Windows Defender Exclusions

Add TriageIR to Windows Defender exclusions to prevent interference:

```powershell
# Run as Administrator
Add-MpPreference -ExclusionPath "C:\Program Files\TriageIR"
Add-MpPreference -ExclusionProcess "triageir-cli.exe"
Add-MpPreference -ExclusionProcess "TriageIR.exe"
```

## Verification

### CLI Verification

```cmd
# Test CLI installation
triageir-cli.exe --version
# Expected output: TriageIR CLI v1.0.0

# Test basic functionality
triageir-cli.exe --help
# Should display help information

# Test quick scan
triageir-cli.exe --only system --output test.json
# Should create test.json with system information
```

### GUI Verification

1. **Launch GUI**
   - Start Menu → TriageIR → TriageIR GUI
   - Or run `TriageIR.exe` directly

2. **Test Basic Functionality**
   - Click "Quick Scan"
   - Verify scan completes successfully
   - Check that results are displayed

3. **Test CLI Integration**
   - GUI should automatically detect CLI executable
   - Check status bar for CLI path

### Integration Tests

Run the included test suite:

```cmd
cd "C:\Program Files\TriageIR\test-scripts"
run-validation-tests.bat
```

Expected output:
```
✓ CLI executable found
✓ CLI version check passed
✓ Basic scan test passed
✓ JSON output validation passed
✓ GUI launch test passed
✓ All tests completed successfully
```

## Troubleshooting

### Common Installation Issues

#### Issue: "Windows protected your PC" SmartScreen warning

**Solution:**
1. Click "More info"
2. Click "Run anyway"
3. Or disable SmartScreen temporarily

#### Issue: Installation fails with "Access denied"

**Solution:**
1. Run installer as Administrator
2. Check user account permissions
3. Temporarily disable antivirus

#### Issue: CLI not found in PATH

**Solution:**
```cmd
# Check current PATH
echo %PATH%

# Add manually if missing
setx PATH "%PATH%;C:\Program Files\TriageIR"

# Restart command prompt
```

#### Issue: GUI fails to start

**Solution:**
1. Check Windows version (Windows 10+ required)
2. Install Visual C++ Redistributable
3. Check Event Viewer for detailed errors

### Performance Issues

#### Issue: Slow scan performance

**Solutions:**
- Run as Administrator for full access
- Close unnecessary applications
- Use SSD storage for output files
- Use `--skip-hashes` for faster scans

#### Issue: High memory usage

**Solutions:**
- Limit event log collection: `--max-events 1000`
- Use targeted collection: `--only processes,network`
- Increase virtual memory

### Compatibility Issues

#### Issue: Antivirus interference

**Solutions:**
1. Add TriageIR to antivirus exclusions
2. Temporarily disable real-time protection
3. Use application whitelisting

#### Issue: Corporate firewall blocking

**Solutions:**
- TriageIR operates offline (no network required)
- Contact IT administrator for approval
- Use portable installation if policy allows

## Uninstallation

### Method 1: Windows Installer

1. **Control Panel Method**
   - Control Panel → Programs → Programs and Features
   - Find "TriageIR" → Uninstall

2. **Settings Method**
   - Settings → Apps → Apps & features
   - Search "TriageIR" → Uninstall

3. **Start Menu Method**
   - Start Menu → TriageIR → Uninstall TriageIR

### Method 2: Portable Installation

1. **Delete Directory**
   ```cmd
   rmdir /s /q "C:\Tools\TriageIR"
   ```

2. **Remove from PATH** (if added manually)
   ```cmd
   # Edit PATH environment variable to remove TriageIR directory
   ```

3. **Delete Shortcuts**
   - Remove desktop shortcuts
   - Remove Start Menu entries (if created manually)

### Method 3: Package Managers

#### Chocolatey
```powershell
choco uninstall triageir
```

#### Scoop
```powershell
scoop uninstall triageir
```

### Complete Cleanup

After uninstallation, remove remaining files:

```cmd
# Remove user data (optional)
rmdir /s /q "%APPDATA%\TriageIR"

# Remove temporary files
del /q "%TEMP%\triageir*"

# Remove Windows Defender exclusions
Remove-MpPreference -ExclusionPath "C:\Program Files\TriageIR"
```

## Advanced Installation Options

### Silent Installation

For automated deployment:

```cmd
# Silent install with all components
TriageIR-Setup.exe /S /D=C:\Program Files\TriageIR

# Silent install CLI only
TriageIR-Setup.exe /S /COMPONENTS=cli /D=C:\Tools\TriageIR-CLI
```

### Network Deployment

For enterprise environments:

1. **Group Policy Deployment**
   - Create MSI package using installer
   - Deploy via Group Policy Software Installation

2. **SCCM Deployment**
   - Import installer into SCCM
   - Create deployment package
   - Deploy to target collections

3. **PowerShell DSC**
   ```powershell
   Configuration TriageIRInstall {
       Package TriageIR {
           Name = "TriageIR"
           Path = "\\server\share\TriageIR-Setup.exe"
           Arguments = "/S"
           Ensure = "Present"
       }
   }
   ```

### Custom Installation Paths

For non-standard installations:

```cmd
# Install to custom directory
TriageIR-Setup.exe /D=D:\Forensics\TriageIR

# Portable installation with custom structure
mkdir D:\Forensics\Tools\TriageIR
xcopy TriageIR-Portable\* D:\Forensics\Tools\TriageIR\ /s /e
```

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**Applies to**: TriageIR v1.0.0 and later