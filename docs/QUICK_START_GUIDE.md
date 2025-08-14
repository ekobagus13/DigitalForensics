# TriageIR Quick Start Guide

Get up and running with TriageIR in under 5 minutes.

## 🚀 Installation

### Option 1: Download and Run (Fastest)
1. Download `TriageIR-Portable.zip`
2. Extract to any folder
3. Run `Quick-Start.bat`

### Option 2: Windows Installer
1. Download `TriageIR-Setup.exe`
2. Run as Administrator
3. Follow installation wizard

## 🔍 First Scan

### Using the GUI (Recommended for Beginners)

1. **Launch TriageIR GUI**
   - Double-click `TriageIR-GUI.bat`
   - Or use Start Menu shortcut

2. **Run Quick Scan**
   - Click the "Quick Scan" button
   - Wait for scan to complete (usually 30-60 seconds)

3. **View Results**
   - Browse results in organized tabs
   - Export reports as needed

### Using the CLI (For Advanced Users)

1. **Open Command Prompt as Administrator**
   ```cmd
   # Navigate to TriageIR directory
   cd C:\Program Files\TriageIR
   ```

2. **Run Basic Scan**
   ```cmd
   # Quick system assessment
   TriageIR-CLI.bat --output my_first_scan.json --verbose
   ```

3. **View Results**
   ```cmd
   # Open JSON file in text editor or import into analysis tools
   notepad my_first_scan.json
   ```

## 📊 Understanding Results

### Key Data Points to Review

1. **System Information**
   - Uptime and logged-on users
   - OS version and architecture

2. **Running Processes**
   - Look for unusual process names
   - Check processes without digital signatures
   - Review command-line arguments

3. **Network Connections**
   - External connections to unknown IPs
   - Unusual listening ports
   - Processes with unexpected network activity

4. **Persistence Mechanisms**
   - Unknown startup programs
   - Suspicious scheduled tasks
   - Registry run keys

## 🎯 Common Use Cases

### Incident Response
```cmd
# Fast threat assessment
TriageIR-CLI.bat --skip-hashes --only processes,network,persistence --output incident_response.json
```

### Forensic Analysis
```cmd
# Comprehensive evidence collection
TriageIR-CLI.bat --max-events 5000 --verbose --output forensic_analysis.json
```

### System Monitoring
```cmd
# Daily system check
TriageIR-CLI.bat --skip-events --output daily_check.json
```

## 🔧 Essential Commands

### CLI Quick Reference

| Command | Purpose |
|---------|---------|
| `--help` | Show all available options |
| `--version` | Display version information |
| `--output file.json` | Save results to file |
| `--verbose` | Show detailed progress |
| `--skip-hashes` | Faster scan (skip file hashing) |
| `--only processes,network` | Collect specific artifacts only |

### GUI Quick Actions

| Action | Location |
|--------|----------|
| Quick Scan | Main window, large blue button |
| Save Results | File menu → Save Results |
| Export Report | File menu → Export Report |
| View Logs | Help menu → View Logs |

## ⚠️ Important Notes

### Permissions
- **Standard User**: Basic functionality available
- **Administrator**: Full system access recommended
- **Domain Admin**: Required for some enterprise features

### Performance Tips
- Close unnecessary applications before scanning
- Use SSD storage for better performance
- Run during off-peak hours for large systems

### Security Considerations
- TriageIR operates offline (no network transmission)
- All data remains on local system
- Secure output files with appropriate permissions

## 🆘 Need Help?

### Quick Troubleshooting

**Problem**: CLI not found
**Solution**: Check installation path and run as Administrator

**Problem**: GUI won't start
**Solution**: Verify Windows 10+ and try running from command line

**Problem**: Scan takes too long
**Solution**: Use `--skip-hashes` and `--max-events 1000`

### Documentation
- **Complete Manual**: `docs/USER_MANUAL.md`
- **Developer Guide**: `docs/DEVELOPER_GUIDE.md`
- **API Reference**: `docs/API_REFERENCE.md`

### Support Resources
- Check `collection_log` in JSON output for errors
- Run with `--verbose` for detailed information
- Review Windows Event Logs for system issues

## 📈 Next Steps

### Learn More
1. Read the complete User Manual
2. Try different scan options
3. Explore the GUI features
4. Set up automated scanning

### Advanced Usage
1. Create custom analysis scripts
2. Integrate with SIEM systems
3. Develop custom reporting templates
4. Automate with PowerShell or Python

### Best Practices
1. Document all scans with case numbers
2. Maintain chain of custody for evidence
3. Regular system baseline scans
4. Keep TriageIR updated to latest version

---

**Ready to start? Run your first scan now!**

```cmd
# Windows Command Prompt
cd "C:\Program Files\TriageIR"
TriageIR-CLI.bat --output first_scan.json --verbose
```

---

**Document Version**: 1.0  
**Last Updated**: December 2024  
**For**: TriageIR v1.0.0