# TriageIR Deployment Guide

This guide provides comprehensive instructions for deploying TriageIR in various environments, from single-user installations to enterprise-wide deployments.

## Table of Contents

1. [Deployment Overview](#deployment-overview)
2. [Single-User Deployment](#single-user-deployment)
3. [Enterprise Deployment](#enterprise-deployment)
4. [Network Deployment](#network-deployment)
5. [Automated Deployment](#automated-deployment)
6. [Security Considerations](#security-considerations)
7. [Maintenance and Updates](#maintenance-and-updates)
8. [Troubleshooting](#troubleshooting)

## Deployment Overview

### Deployment Options

| Method | Use Case | Complexity | Management |
|--------|----------|------------|------------|
| **Portable** | Individual analysts, USB deployment | Low | Manual |
| **Windows Installer** | Standard workstation deployment | Medium | Semi-automated |
| **Group Policy** | Enterprise domain deployment | High | Centralized |
| **SCCM** | Large enterprise deployment | High | Automated |
| **PowerShell DSC** | Infrastructure as Code | High | Automated |

### System Requirements

#### Minimum Requirements
- **OS**: Windows 10 (1903) or Windows Server 2016
- **Architecture**: x64 (64-bit)
- **RAM**: 4 GB
- **Disk**: 100 MB free space
- **Network**: None required (offline operation)

#### Recommended Requirements
- **OS**: Windows 11 or Windows Server 2022
- **RAM**: 8 GB or more
- **Disk**: 1 GB free space (for large collections)
- **Permissions**: Local Administrator

## Single-User Deployment

### Method 1: Portable Installation

**Best for**: Individual analysts, incident responders, USB deployment

```cmd
# Download and extract
curl -L -o TriageIR-Portable.zip https://github.com/triageir/releases/latest/TriageIR-Portable.zip
powershell -command "Expand-Archive -Path TriageIR-Portable.zip -DestinationPath C:\Tools\TriageIR"

# Verify installation
cd C:\Tools\TriageIR
TriageIR-CLI.bat --version
```

**Advantages**:
- No installation required
- Can run from USB drive
- No registry modifications
- Easy to remove

**Disadvantages**:
- No Start Menu integration
- Manual PATH configuration
- No automatic updates

### Method 2: Windows Installer

**Best for**: Standard workstation deployment

```cmd
# Download installer
curl -L -o TriageIR-Setup.exe https://github.com/triageir/releases/latest/TriageIR-Setup.exe

# Install (requires Administrator)
TriageIR-Setup.exe

# Or silent install
TriageIR-Setup.exe /S
```

**Advantages**:
- Professional installation experience
- Start Menu integration
- PATH automatically configured
- Uninstaller provided

**Disadvantages**:
- Requires Administrator privileges
- Registry modifications
- More complex removal

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

# Add TriageIR bucket and install
scoop bucket add triageir https://github.com/triageir/scoop-bucket
scoop install triageir
```

## Enterprise Deployment

### Group Policy Deployment

**Best for**: Domain-joined Windows environments

#### Step 1: Prepare MSI Package
```cmd
# Convert installer to MSI (using tools like Advanced Installer)
# Or use the provided MSI package
```

#### Step 2: Create Group Policy Object
1. Open Group Policy Management Console
2. Create new GPO: "TriageIR Deployment"
3. Navigate to: Computer Configuration → Policies → Software Settings → Software Installation
4. Right-click → New → Package
5. Select TriageIR.msi
6. Choose "Assigned" deployment method

#### Step 3: Configure Deployment
```xml
<!-- GPO Settings -->
<GroupPolicyObject>
    <SoftwareInstallation>
        <Package>
            <Name>TriageIR</Name>
            <Path>\\domain.com\SYSVOL\domain.com\Policies\{GUID}\Machine\Applications\TriageIR.msi</Path>
            <DeploymentType>Assigned</DeploymentType>
            <InstallationUI>Basic</InstallationUI>
            <UninstallOnPolicyRemoval>true</UninstallOnPolicyRemoval>
        </Package>
    </SoftwareInstallation>
</GroupPolicyObject>
```

#### Step 4: Link and Deploy
1. Link GPO to target Organizational Unit
2. Run `gpupdate /force` on target machines
3. Reboot machines to trigger installation

### SCCM Deployment

**Best for**: Large enterprise environments with SCCM

#### Step 1: Create Application
```powershell
# SCCM PowerShell commands
Import-Module ConfigurationManager
Set-Location "SITE:"

# Create application
$AppName = "TriageIR"
$AppVersion = "1.0.0"
New-CMApplication -Name $AppName -Description "Digital Forensics Triage Tool" -SoftwareVersion $AppVersion
```

#### Step 2: Create Deployment Type
```powershell
# Add deployment type
Add-CMScriptDeploymentType -ApplicationName $AppName -DeploymentTypeName "TriageIR Installer" `
    -ContentLocation "\\server\share\TriageIR-Setup.exe" `
    -InstallCommand "TriageIR-Setup.exe /S" `
    -UninstallCommand "C:\Program Files\TriageIR\uninstall.exe /S"
```

#### Step 3: Distribute and Deploy
```powershell
# Distribute to distribution points
Start-CMContentDistribution -ApplicationName $AppName -DistributionPointName "DP01.domain.com"

# Create deployment
New-CMApplicationDeployment -ApplicationName $AppName -CollectionName "Forensics Workstations" `
    -DeployAction Install -DeployPurpose Required -UserNotification DisplaySoftwareCenterOnly
```

### PowerShell DSC Deployment

**Best for**: Infrastructure as Code environments

```powershell
Configuration TriageIRDeployment {
    param(
        [string[]]$ComputerName = 'localhost'
    )
    
    Import-DscResource -ModuleName PSDesiredStateConfiguration
    
    Node $ComputerName {
        # Ensure TriageIR directory exists
        File TriageIRDirectory {
            DestinationPath = "C:\Program Files\TriageIR"
            Type = "Directory"
            Ensure = "Present"
        }
        
        # Download and install TriageIR
        Package TriageIR {
            Name = "TriageIR"
            Path = "\\server\share\TriageIR-Setup.exe"
            Arguments = "/S"
            ProductId = "{GUID-HERE}"
            Ensure = "Present"
            DependsOn = "[File]TriageIRDirectory"
        }
        
        # Configure Windows Defender exclusions
        Script DefenderExclusions {
            SetScript = {
                Add-MpPreference -ExclusionPath "C:\Program Files\TriageIR"
                Add-MpPreference -ExclusionProcess "triageir-cli.exe"
                Add-MpPreference -ExclusionProcess "TriageIR.exe"
            }
            TestScript = {
                $exclusions = Get-MpPreference | Select-Object -ExpandProperty ExclusionPath
                return $exclusions -contains "C:\Program Files\TriageIR"
            }
            GetScript = {
                return @{Result = (Get-MpPreference | Select-Object -ExpandProperty ExclusionPath)}
            }
            DependsOn = "[Package]TriageIR"
        }
    }
}

# Compile and apply configuration
TriageIRDeployment -ComputerName "FORENSICS-01", "FORENSICS-02"
Start-DscConfiguration -Path .\TriageIRDeployment -Wait -Verbose
```

## Network Deployment

### Shared Network Installation

**Best for**: Environments where local installation is not permitted

#### Step 1: Prepare Network Share
```cmd
# Create network share
mkdir \\server\forensics\TriageIR
xcopy TriageIR-Portable\* \\server\forensics\TriageIR\ /s /e

# Set permissions
icacls \\server\forensics\TriageIR /grant "Domain Users:(RX)"
icacls \\server\forensics\TriageIR /grant "Forensics Team:(F)"
```

#### Step 2: Create Network Launcher
```cmd
# Create launcher script: TriageIR-Network.bat
@echo off
set NETWORK_PATH=\\server\forensics\TriageIR
set LOCAL_TEMP=%TEMP%\TriageIR

# Copy to local temp for better performance
if not exist "%LOCAL_TEMP%" mkdir "%LOCAL_TEMP%"
xcopy "%NETWORK_PATH%\*" "%LOCAL_TEMP%\" /s /e /y /q

# Run from local temp
cd /d "%LOCAL_TEMP%"
TriageIR-CLI.bat %*

# Cleanup
cd /d %TEMP%
rmdir /s /q "%LOCAL_TEMP%"
```

#### Step 3: Deploy Launcher
```cmd
# Copy launcher to all workstations
for /f %i in (workstations.txt) do (
    copy TriageIR-Network.bat \\%i\c$\Tools\
)
```

### Remote Execution

**Best for**: Incident response scenarios

#### PsExec Deployment
```cmd
# Deploy and execute remotely
psexec \\target-computer -c triageir-cli.exe --output \\evidence-server\collections\target-computer.json
```

#### PowerShell Remoting
```powershell
# Remote execution via PowerShell
$computers = @("PC01", "PC02", "PC03")
$scriptBlock = {
    & "\\server\forensics\TriageIR\CLI\triageir-cli.exe" --output "\\evidence-server\collections\$env:COMPUTERNAME.json" --verbose
}

Invoke-Command -ComputerName $computers -ScriptBlock $scriptBlock
```

#### WinRM Deployment
```powershell
# Configure WinRM and deploy
foreach ($computer in $computers) {
    # Enable WinRM
    Enable-PSRemoting -ComputerName $computer -Force
    
    # Copy files
    Copy-Item -Path "TriageIR-Portable\*" -Destination "\\$computer\c$\Tools\TriageIR\" -Recurse
    
    # Execute
    Invoke-Command -ComputerName $computer -ScriptBlock {
        & "C:\Tools\TriageIR\CLI\triageir-cli.exe" --output "C:\Evidence\$env:COMPUTERNAME.json"
    }
}
```

## Automated Deployment

### Ansible Deployment

```yaml
---
- name: Deploy TriageIR to Windows hosts
  hosts: windows
  tasks:
    - name: Create TriageIR directory
      win_file:
        path: C:\Program Files\TriageIR
        state: directory
    
    - name: Download TriageIR installer
      win_get_url:
        url: https://github.com/triageir/releases/latest/TriageIR-Setup.exe
        dest: C:\Temp\TriageIR-Setup.exe
    
    - name: Install TriageIR
      win_package:
        path: C:\Temp\TriageIR-Setup.exe
        arguments: /S
        state: present
    
    - name: Configure Windows Defender exclusions
      win_shell: |
        Add-MpPreference -ExclusionPath "C:\Program Files\TriageIR"
        Add-MpPreference -ExclusionProcess "triageir-cli.exe"
```

### Terraform Deployment

```hcl
# Deploy TriageIR to Azure VMs
resource "azurerm_virtual_machine_extension" "triageir" {
  count                = length(var.vm_names)
  name                 = "TriageIR-Installation"
  virtual_machine_id   = azurerm_windows_virtual_machine.forensics[count.index].id
  publisher            = "Microsoft.Compute"
  type                 = "CustomScriptExtension"
  type_handler_version = "1.10"

  settings = jsonencode({
    commandToExecute = "powershell -ExecutionPolicy Unrestricted -Command \"Invoke-WebRequest -Uri 'https://github.com/triageir/releases/latest/TriageIR-Setup.exe' -OutFile 'C:\\Temp\\TriageIR-Setup.exe'; Start-Process -FilePath 'C:\\Temp\\TriageIR-Setup.exe' -ArgumentList '/S' -Wait\""
  })
}
```

### Docker Deployment (Windows Containers)

```dockerfile
# Dockerfile for TriageIR Windows container
FROM mcr.microsoft.com/windows/servercore:ltsc2022

# Copy TriageIR files
COPY TriageIR-Portable/ C:/TriageIR/

# Set working directory
WORKDIR C:/TriageIR

# Add to PATH
RUN setx PATH "%PATH%;C:\TriageIR\CLI"

# Default command
CMD ["cmd", "/c", "TriageIR-CLI.bat", "--help"]
```

```yaml
# docker-compose.yml
version: '3.8'
services:
  triageir:
    build: .
    volumes:
      - ./evidence:/evidence
    command: ["TriageIR-CLI.bat", "--output", "/evidence/container-scan.json"]
```

## Security Considerations

### Code Signing Verification

```powershell
# Verify digital signatures
$files = @(
    "C:\Program Files\TriageIR\CLI\triageir-cli.exe",
    "C:\Program Files\TriageIR\GUI\TriageIR.exe"
)

foreach ($file in $files) {
    $signature = Get-AuthenticodeSignature $file
    if ($signature.Status -eq "Valid") {
        Write-Host "✓ $file - Valid signature" -ForegroundColor Green
    } else {
        Write-Host "✗ $file - Invalid signature: $($signature.Status)" -ForegroundColor Red
    }
}
```

### Hash Verification

```cmd
# Verify file integrity
certutil -hashfile "TriageIR-Setup.exe" SHA256
# Compare with published hash: [expected-hash]
```

### Permission Configuration

```powershell
# Set appropriate permissions
$path = "C:\Program Files\TriageIR"

# Remove inherited permissions
$acl = Get-Acl $path
$acl.SetAccessRuleProtection($true, $false)

# Add specific permissions
$acl.SetAccessRule((New-Object System.Security.AccessControl.FileSystemAccessRule("BUILTIN\Administrators", "FullControl", "ContainerInherit,ObjectInherit", "None", "Allow")))
$acl.SetAccessRule((New-Object System.Security.AccessControl.FileSystemAccessRule("BUILTIN\Users", "ReadAndExecute", "ContainerInherit,ObjectInherit", "None", "Allow")))

Set-Acl $path $acl
```

### Network Security

```powershell
# Configure Windows Firewall (if needed)
# TriageIR operates offline, but may need rules for network shares

New-NetFirewallRule -DisplayName "TriageIR Evidence Share" -Direction Outbound -Protocol TCP -RemotePort 445 -Action Allow
```

## Maintenance and Updates

### Update Procedures

#### Manual Updates
```cmd
# Download new version
curl -L -o TriageIR-Setup-New.exe https://github.com/triageir/releases/latest/TriageIR-Setup.exe

# Verify signature and hash
certutil -hashfile TriageIR-Setup-New.exe SHA256

# Install update
TriageIR-Setup-New.exe /S
```

#### Automated Updates (PowerShell)
```powershell
function Update-TriageIR {
    param(
        [string]$DownloadPath = "$env:TEMP\TriageIR-Update.exe",
        [switch]$Force
    )
    
    # Check current version
    $currentVersion = & "triageir-cli.exe" --version 2>$null
    if (-not $currentVersion) {
        Write-Error "TriageIR not found or not in PATH"
        return
    }
    
    # Check for updates (implement version checking logic)
    $latestVersion = Get-LatestTriageIRVersion  # Custom function
    
    if ($currentVersion -eq $latestVersion -and -not $Force) {
        Write-Host "TriageIR is already up to date ($currentVersion)"
        return
    }
    
    # Download and install update
    Write-Host "Updating TriageIR from $currentVersion to $latestVersion"
    Invoke-WebRequest -Uri "https://github.com/triageir/releases/latest/TriageIR-Setup.exe" -OutFile $DownloadPath
    
    # Verify download
    $signature = Get-AuthenticodeSignature $DownloadPath
    if ($signature.Status -ne "Valid") {
        Write-Error "Invalid signature on downloaded file"
        return
    }
    
    # Install
    Start-Process -FilePath $DownloadPath -ArgumentList "/S" -Wait
    
    # Cleanup
    Remove-Item $DownloadPath -Force
    
    Write-Host "Update completed successfully"
}
```

#### SCCM Updates
```powershell
# Update application in SCCM
$AppName = "TriageIR"
$NewVersion = "1.1.0"

# Update application properties
Set-CMApplication -Name $AppName -SoftwareVersion $NewVersion

# Update deployment type
Set-CMScriptDeploymentType -ApplicationName $AppName -DeploymentTypeName "TriageIR Installer" `
    -ContentLocation "\\server\share\TriageIR-Setup-v1.1.0.exe"

# Redistribute content
Start-CMContentDistribution -ApplicationName $AppName -DistributionPointName "DP01.domain.com"
```

### Health Monitoring

```powershell
# TriageIR health check script
function Test-TriageIRHealth {
    $results = @{
        CLIInstalled = $false
        GUIInstalled = $false
        CLIVersion = $null
        PathConfigured = $false
        PermissionsOK = $false
        DefenderExclusions = $false
    }
    
    # Check CLI installation
    try {
        $results.CLIVersion = & "triageir-cli.exe" --version 2>$null
        $results.CLIInstalled = $true
    } catch {
        $results.CLIInstalled = $false
    }
    
    # Check GUI installation
    $results.GUIInstalled = Test-Path "C:\Program Files\TriageIR\GUI\TriageIR.exe"
    
    # Check PATH configuration
    $results.PathConfigured = $env:PATH -like "*TriageIR*"
    
    # Check permissions
    try {
        $testFile = "$env:TEMP\triageir-test.json"
        & "triageir-cli.exe" --only system --output $testFile 2>$null
        $results.PermissionsOK = Test-Path $testFile
        Remove-Item $testFile -Force -ErrorAction SilentlyContinue
    } catch {
        $results.PermissionsOK = $false
    }
    
    # Check Windows Defender exclusions
    $exclusions = Get-MpPreference | Select-Object -ExpandProperty ExclusionPath
    $results.DefenderExclusions = $exclusions -contains "C:\Program Files\TriageIR"
    
    return $results
}

# Run health check
$health = Test-TriageIRHealth
$health | Format-Table -AutoSize
```

## Troubleshooting

### Common Deployment Issues

#### Issue: Installation fails with "Access Denied"
**Solution**:
```cmd
# Run as Administrator
runas /user:Administrator "TriageIR-Setup.exe"

# Or use elevated PowerShell
Start-Process -FilePath "TriageIR-Setup.exe" -Verb RunAs
```

#### Issue: Group Policy deployment fails
**Solution**:
```cmd
# Check Group Policy processing
gpresult /r /scope computer

# Force Group Policy update
gpupdate /force /boot

# Check event logs
eventvwr.msc
# Navigate to: Windows Logs > Application
# Look for MSI installer events
```

#### Issue: SCCM deployment stuck
**Solution**:
```powershell
# Check SCCM client logs
Get-Content "C:\Windows\CCM\Logs\AppEnforce.log" -Tail 50

# Restart SCCM client service
Restart-Service -Name "CcmExec"

# Trigger machine policy refresh
Invoke-WmiMethod -Namespace root\ccm -Class SMS_CLIENT -Name TriggerSchedule "{00000000-0000-0000-0000-000000000021}"
```

#### Issue: Network deployment performance
**Solution**:
```cmd
# Use local caching
robocopy \\server\forensics\TriageIR C:\Temp\TriageIR /MIR /Z /W:1 /R:1

# Or use BranchCache
netsh branchcache set service mode=hostedclient
netsh branchcache set cachesize size=1024
```

### Deployment Validation

```powershell
# Comprehensive deployment validation script
function Test-TriageIRDeployment {
    param(
        [string[]]$ComputerName = @($env:COMPUTERNAME)
    )
    
    $results = @()
    
    foreach ($computer in $ComputerName) {
        $result = [PSCustomObject]@{
            ComputerName = $computer
            Online = $false
            CLIInstalled = $false
            GUIInstalled = $false
            Version = $null
            PathConfigured = $false
            CanExecute = $false
            DefenderExclusions = $false
            LastChecked = Get-Date
        }
        
        # Test connectivity
        if (Test-Connection -ComputerName $computer -Count 1 -Quiet) {
            $result.Online = $true
            
            try {
                $session = New-PSSession -ComputerName $computer -ErrorAction Stop
                
                # Check installation
                $installCheck = Invoke-Command -Session $session -ScriptBlock {
                    $cli = Test-Path "C:\Program Files\TriageIR\CLI\triageir-cli.exe"
                    $gui = Test-Path "C:\Program Files\TriageIR\GUI\TriageIR.exe"
                    $version = try { & "C:\Program Files\TriageIR\CLI\triageir-cli.exe" --version 2>$null } catch { $null }
                    $pathOK = $env:PATH -like "*TriageIR*"
                    
                    # Test execution
                    $canExecute = $false
                    try {
                        $testOutput = & "C:\Program Files\TriageIR\CLI\triageir-cli.exe" --only system --output "$env:TEMP\test.json" 2>$null
                        $canExecute = Test-Path "$env:TEMP\test.json"
                        Remove-Item "$env:TEMP\test.json" -Force -ErrorAction SilentlyContinue
                    } catch {}
                    
                    # Check Defender exclusions
                    $exclusions = try { Get-MpPreference | Select-Object -ExpandProperty ExclusionPath } catch { @() }
                    $defenderOK = $exclusions -contains "C:\Program Files\TriageIR"
                    
                    return @{
                        CLI = $cli
                        GUI = $gui
                        Version = $version
                        Path = $pathOK
                        Execute = $canExecute
                        Defender = $defenderOK
                    }
                }
                
                $result.CLIInstalled = $installCheck.CLI
                $result.GUIInstalled = $installCheck.GUI
                $result.Version = $installCheck.Version
                $result.PathConfigured = $installCheck.Path
                $result.CanExecute = $installCheck.Execute
                $result.DefenderExclusions = $installCheck.Defender
                
                Remove-PSSession $session
                
            } catch {
                Write-Warning "Failed to check $computer : $_"
            }
        }
        
        $results += $result
    }
    
    return $results
}

# Usage
$computers = @("FORENSICS-01", "FORENSICS-02", "INCIDENT-01")
$deploymentStatus = Test-TriageIRDeployment -ComputerName $computers
$deploymentStatus | Format-Table -AutoSize
```

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**Applies to**: TriageIR v1.0.0 and later