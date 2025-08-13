# TriageIR Professional - Live System Forensic Collector

## 🎯 **Overview**

TriageIR Professional is a portable, zero-installation digital forensics tool designed for incident responders and forensic analysts. It rapidly collects volatile data from live Windows systems in a forensically sound manner, creating secure evidence packages with complete chain of custody documentation.

## 🚀 **Key Features**

### **Zero-Installation Deployment**
- ✅ Single portable executable
- ✅ Runs directly from USB drive
- ✅ No registry modifications
- ✅ No temporary files left behind
- ✅ Administrator privilege detection

### **Comprehensive Forensic Collection**
- 🔍 **System Snapshot**: Running processes, loaded modules, memory regions
- 🌐 **Network Artifacts**: Active connections, listening ports, DNS cache, ARP table
- ⚡ **Volatile Data**: Clipboard contents, recent documents, NetBIOS sessions
- 📁 **Execution Artifacts**: Prefetch files, Shimcache entries, AmCache data
- ⏰ **Persistence Mechanisms**: Scheduled tasks, registry run keys, services
- 👤 **User Activity**: Login history, browser artifacts, recent files
- 📝 **Security Events**: Windows event logs with filtering and analysis
- 🔬 **Memory Scanning**: Optional YARA rule-based process memory analysis

### **Forensically Sound Processing**
- 🔐 **SHA-256 Hashing**: All evidence files cryptographically hashed
- 📋 **Chain of Custody**: Complete audit trail with timestamps
- 🔒 **Secure Packaging**: Password-protected ZIP archives
- ✅ **Integrity Verification**: Hash verification for evidence validation
- 📊 **Comprehensive Logging**: Detailed collection audit logs

### **Professional Output**
- 📦 **Evidence Packages**: Timestamped, password-protected archives
- 📄 **Documentation**: Chain of custody, collection reports, README files
- 🔍 **JSON Format**: Structured data for analysis tools
- 📈 **Statistics**: Collection metrics and performance data

## 🛠️ **Installation & Usage**

### **Prerequisites**
- Windows 10 or later
- Administrator privileges (recommended for complete collection)
- Minimum 8GB RAM, 16GB recommended
- 2GB free disk space for evidence storage

### **Quick Start**

1. **Download** the latest release executable
2. **Run** from command line with required parameters:

```cmd
triageir-cli.exe -c CASE001 --collector-name "John Doe" --collector-org "ACME Corp" --collector-contact "john@acme.com"
```

### **Command Line Options**

#### **Required Parameters**
```cmd
-c, --case-id <ID>              Case identifier for evidence tracking
--collector-name <NAME>         Name of the evidence collector
--collector-org <ORGANIZATION>  Collector's organization
--collector-contact <CONTACT>   Collector's contact information
```

#### **Optional Parameters**
```cmd
-o, --output <DIRECTORY>        Output directory for evidence package (default: ./evidence)
-p, --password <PASSWORD>       Password for evidence package (auto-generated if not provided)
--legal-authority <AUTHORITY>   Legal authority for collection (warrant, consent, etc.)
-v, --verbose                   Enable verbose output
-q, --quick                     Quick collection (skip time-intensive artifacts)
```

#### **Artifact Control**
```cmd
--skip-prefetch                 Skip Prefetch file analysis
--skip-shimcache               Skip Shimcache analysis
--skip-tasks                   Skip scheduled tasks analysis
--skip-events                  Skip event log collection
--max-events <COUNT>           Maximum event log entries (default: 10000)
--yara-rules <PATH>            Path to YARA rules for memory scanning
```

### **Usage Examples**

#### **Basic Collection**
```cmd
triageir-cli.exe -c CASE001 --collector-name "Jane Smith" --collector-org "Digital Forensics Unit" --collector-contact "jane.smith@dfu.gov"
```

#### **Verbose Collection with Custom Output**
```cmd
triageir-cli.exe -c INCIDENT-2024-001 --collector-name "John Doe" --collector-org "ACME Security" --collector-contact "john@acme.com" -v -o "D:\Evidence"
```

#### **Quick Collection (Time-Sensitive)**
```cmd
triageir-cli.exe -c URGENT-001 --collector-name "Emergency Response" --collector-org "IR Team" --collector-contact "ir@company.com" -q -v
```

#### **Collection with Legal Authority**
```cmd
triageir-cli.exe -c WARRANT-001 --collector-name "Detective Smith" --collector-org "Metro Police" --collector-contact "smith@metro.gov" --legal-authority "Search Warrant #2024-001"
```

#### **Memory Scanning with YARA**
```cmd
triageir-cli.exe -c MALWARE-001 --collector-name "Malware Analyst" --collector-org "SOC Team" --collector-contact "analyst@soc.com" --yara-rules "rules\malware.yar" -v
```

## 📊 **Collected Artifacts**

### **System Information**
- Hostname, domain, IP addresses, MAC addresses
- OS version, architecture, timezone
- System uptime, last boot time
- Logged-on users and session information

### **Process Analysis**
- Complete process list with PIDs, PPIDs, command lines
- Loaded DLLs and modules for each process
- Memory usage statistics
- Process hashes (MD5, SHA1, SHA256, Import Hash)
- Digital signature verification
- Network connections per process

### **Network Intelligence**
- Active TCP/UDP connections with process mapping
- Listening ports and associated services
- DNS cache entries
- ARP table contents
- Network shares and permissions
- WiFi profiles and stored passwords
- Firewall rules and proxy settings

### **Execution Evidence**
- **Prefetch Files**: Program execution history with run counts and timestamps
- **Shimcache**: Application compatibility cache showing executed programs
- **AmCache**: Application installation and execution tracking
- **UserAssist**: User program execution statistics
- **Jump Lists**: Recent document and application usage
- **LNK Files**: Shortcut file analysis

### **Persistence Analysis**
- Registry run keys (HKLM and HKCU)
- Scheduled tasks with triggers and actions
- Windows services configuration
- Startup folder contents
- WinLogon entries
- Image hijacking detection
- WMI persistence mechanisms

### **User Activity**
- Login/logout events and history
- Browser artifacts (history, downloads, cookies)
- Recent documents and file access
- Email artifacts (if accessible)
- Clipboard contents
- User profile information

### **Security Events**
- Windows Security event log
- System event log
- Application event log
- PowerShell execution logs
- Sysmon events (if installed)
- Windows Defender logs

## 🔐 **Security & Chain of Custody**

### **Evidence Integrity**
- **SHA-256 Hashing**: Every collected file is cryptographically hashed
- **Integrity Verification**: Hash manifests for evidence validation
- **Digital Signatures**: Support for code signing (configurable)
- **Tamper Detection**: Any modification invalidates evidence integrity

### **Chain of Custody**
- **Complete Audit Trail**: Every action logged with timestamps
- **Collector Information**: Full identification and contact details
- **Legal Authority**: Documentation of collection authorization
- **Custody Transfers**: Support for evidence handoff documentation
- **Case Tracking**: Unique case and evidence identifiers

### **Secure Packaging**
- **Password Protection**: AES-256 encrypted evidence archives
- **Timestamped Archives**: Unique filenames with collection timestamps
- **Comprehensive Documentation**: README, chain of custody, audit logs
- **Hash Verification**: External hash files for archive validation

## 📋 **Output Structure**

### **Evidence Package Contents**
```
CASE001_20241213_143022_evidence.zip
├── evidence.json                    # Complete forensic data (JSON format)
├── chain_of_custody.txt            # Chain of custody documentation
├── collection_audit.txt            # Detailed collection audit log
├── integrity_verification.txt      # Hash verification information
├── digital_signature_info.txt      # Digital signature details
└── README.txt                      # Package documentation
```

### **Additional Files**
```
CASE001_20241213_143022_evidence.zip.sha256    # Archive hash verification
collection_summary.txt                         # Human-readable summary
```

### **JSON Data Structure**
The evidence.json file contains structured forensic data:
```json
{
  "case_metadata": { /* Case and custody information */ },
  "system_snapshot": { /* Running processes, modules, handles */ },
  "volatile_artifacts": { /* Network, DNS, ARP, clipboard */ },
  "execution_artifacts": { /* Prefetch, Shimcache, AmCache */ },
  "persistence_artifacts": { /* Tasks, services, registry */ },
  "network_artifacts": { /* Connections, shares, firewall */ },
  "user_activity": { /* Logins, browser, recent files */ },
  "security_events": { /* Event logs, PowerShell, Sysmon */ },
  "integrity_verification": { /* Hashes and signatures */ },
  "collection_audit": { /* Complete audit trail */ }
}
```

## 🎯 **Use Cases**

### **Incident Response**
- **Rapid Triage**: Quick assessment of compromised systems
- **Evidence Preservation**: Secure collection before system shutdown
- **Threat Hunting**: Search for indicators of compromise
- **Timeline Analysis**: Reconstruct attack sequences

### **Digital Forensics**
- **Live System Analysis**: Volatile data collection
- **Malware Investigation**: Process and memory analysis
- **Data Breach Response**: User activity and data access tracking
- **Compliance Auditing**: System configuration and access review

### **Law Enforcement**
- **Criminal Investigations**: Evidence collection with legal documentation
- **Corporate Investigations**: Employee activity monitoring
- **Fraud Detection**: Financial system access tracking
- **Intellectual Property Theft**: File access and transfer analysis

## ⚖️ **Legal Considerations**

### **Authorization Requirements**
- Ensure proper legal authority before collection
- Document authorization in the tool (--legal-authority option)
- Maintain chain of custody documentation
- Follow organizational data handling policies

### **Privacy Protection**
- Collect only necessary data for investigation
- Secure evidence packages with strong passwords
- Limit access to authorized personnel only
- Follow data retention and disposal policies

### **Court Admissibility**
- Complete audit trails for evidence authentication
- Hash verification for integrity proof
- Professional documentation standards
- Tool validation and testing records

## 🔧 **Advanced Features**

### **YARA Memory Scanning**
Integrate YARA rules for advanced threat detection:
```cmd
triageir-cli.exe -c CASE001 --collector-name "Analyst" --collector-org "SOC" --collector-contact "soc@company.com" --yara-rules "malware_rules.yar"
```

### **Custom Collection Profiles**
- **Quick Mode**: Essential artifacts only (--quick)
- **Full Mode**: Complete artifact collection (default)
- **Custom Mode**: Selective artifact collection (--skip-* options)

### **Performance Optimization**
- Multi-threaded collection for speed
- Memory-efficient processing
- Configurable limits (--max-events)
- Progress reporting in verbose mode

### **Integration Support**
- JSON output for SIEM integration
- API-friendly data structures
- Scriptable command-line interface
- Batch processing capabilities

## 🛡️ **Security Best Practices**

### **Deployment Security**
1. **Verify Tool Integrity**: Check digital signatures and hashes
2. **Secure Storage**: Store tool on encrypted, write-protected media
3. **Access Control**: Limit tool access to authorized personnel
4. **Version Control**: Use specific tool versions for consistency

### **Collection Security**
1. **Administrator Privileges**: Run with elevated privileges when possible
2. **Network Isolation**: Consider network isolation during collection
3. **Antivirus Exclusions**: Add tool to AV exclusions if necessary
4. **System Impact**: Monitor system resources during collection

### **Evidence Security**
1. **Strong Passwords**: Use complex passwords for evidence packages
2. **Secure Transport**: Encrypt evidence during transmission
3. **Access Logging**: Log all evidence access and modifications
4. **Backup Strategy**: Maintain secure evidence backups

## 📞 **Support & Documentation**

### **Technical Support**
- **User Manual**: Comprehensive usage documentation
- **Best Practices**: Forensic collection guidelines
- **Troubleshooting**: Common issues and solutions
- **Updates**: Regular tool updates and improvements

### **Training Resources**
- **Quick Start Guide**: Basic usage instructions
- **Advanced Techniques**: Expert-level features
- **Case Studies**: Real-world usage examples
- **Certification**: Professional training programs

### **Community**
- **User Forums**: Community support and discussions
- **Bug Reports**: Issue tracking and resolution
- **Feature Requests**: Enhancement suggestions
- **Contributions**: Open-source development

## 🔄 **Version History**

### **v1.0.0 - Professional Release**
- Complete forensic artifact collection
- Secure evidence packaging
- Chain of custody documentation
- Professional audit logging
- YARA memory scanning support
- Advanced persistence detection
- Comprehensive network analysis

---

**TriageIR Professional** - The definitive tool for live system forensic triage.

*Developed for incident responders, by incident responders.*