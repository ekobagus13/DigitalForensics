// TriageIR GUI - Clean and Functional Application
const { ipcRenderer } = require('electron');
const fs = require('fs');
const path = require('path');

class TriageIRApp {
    constructor() {
        this.currentScanData = null;
        this.isScanning = false;
        this.cliStatus = null;
        
        this.init();
    }

    async init() {
        console.log('Initializing TriageIR GUI...');
        
        // Set up event listeners
        this.setupEventListeners();
        
        // Check CLI status
        await this.checkCLIStatus();
        
        // Set up IPC listeners
        this.setupIPCListeners();
        
        console.log('TriageIR GUI initialized successfully');
    }

    setupEventListeners() {
        // Welcome screen buttons
        document.getElementById('quickScanBtn').addEventListener('click', () => {
            this.startQuickScan();
        });
        
        document.getElementById('customScanBtn').addEventListener('click', () => {
            this.startCustomScan();
        });
        
        document.getElementById('openFileBtn').addEventListener('click', () => {
            this.openResultsFile();
        });

        // Scan screen buttons
        document.getElementById('cancelScanBtn').addEventListener('click', () => {
            this.cancelScan();
        });

        // Results screen buttons
        document.getElementById('newScanBtn').addEventListener('click', () => {
            this.showWelcomeScreen();
        });
        
        document.getElementById('saveResultsBtn').addEventListener('click', () => {
            this.saveResults();
        });
        
        document.getElementById('exportReportBtn').addEventListener('click', () => {
            this.exportReport();
        });

        // Tab buttons
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                this.switchTab(e.target.dataset.tab);
            });
        });
    }

    setupIPCListeners() {
        // Listen for scan progress updates
        ipcRenderer.on('scan-progress', (event, message) => {
            this.updateScanProgress(message);
        });
    }

    async checkCLIStatus() {
        try {
            this.cliStatus = await ipcRenderer.invoke('check-cli-status');
            this.updateCLIStatusDisplay();
        } catch (error) {
            console.error('Failed to check CLI status:', error);
            this.cliStatus = { available: false, version: null, path: null };
            this.updateCLIStatusDisplay();
        }
    }

    updateCLIStatusDisplay() {
        const statusIndicator = document.getElementById('statusIndicator');
        const statusText = document.getElementById('statusText');
        
        if (this.cliStatus.available) {
            statusIndicator.textContent = '🟢';
            statusText.textContent = `CLI Ready (${this.cliStatus.version})`;
            statusText.className = 'status-text text-success';
        } else {
            statusIndicator.textContent = '🔴';
            statusText.textContent = 'CLI Not Available';
            statusText.className = 'status-text text-error';
        }
    }

    async startQuickScan() {
        if (!this.cliStatus.available) {
            alert('CLI is not available. Please ensure triageir-cli.exe is accessible.');
            return;
        }

        this.showScanScreen();
        this.isScanning = true;
        
        try {
            const result = await ipcRenderer.invoke('start-scan', {
                verbose: true
            });
            
            if (result.success) {
                this.currentScanData = result.data;
                this.showResultsScreen();
            } else {
                throw new Error(result.error);
            }
        } catch (error) {
            console.error('Scan failed:', error);
            alert(`Scan failed: ${error.message || error}`);
            this.showWelcomeScreen();
        } finally {
            this.isScanning = false;
        }
    }

    async startCustomScan() {
        if (!this.cliStatus.available) {
            alert('CLI is not available. Please ensure triageir-cli.exe is accessible.');
            return;
        }

        // For now, just do a verbose scan (can be enhanced later)
        this.showScanScreen();
        this.isScanning = true;
        
        try {
            const result = await ipcRenderer.invoke('start-scan', {
                verbose: true
            });
            
            if (result.success) {
                this.currentScanData = result.data;
                this.showResultsScreen();
            } else {
                throw new Error(result.error);
            }
        } catch (error) {
            console.error('Scan failed:', error);
            alert(`Scan failed: ${error.message || error}`);
            this.showWelcomeScreen();
        } finally {
            this.isScanning = false;
        }
    }

    async openResultsFile() {
        try {
            const result = await ipcRenderer.invoke('open-file-dialog');
            
            if (!result.canceled && result.filePaths.length > 0) {
                const filePath = result.filePaths[0];
                const jsonData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
                
                this.currentScanData = jsonData;
                this.showResultsScreen();
            }
        } catch (error) {
            console.error('Failed to open file:', error);
            alert(`Failed to open file: ${error.message}`);
        }
    }

    cancelScan() {
        // For now, just return to welcome screen
        // In a full implementation, this would terminate the CLI process
        this.isScanning = false;
        this.showWelcomeScreen();
    }

    async saveResults() {
        if (!this.currentScanData) {
            alert('No scan data to save');
            return;
        }

        try {
            const result = await ipcRenderer.invoke('save-file-dialog');
            
            if (!result.canceled && result.filePath) {
                fs.writeFileSync(result.filePath, JSON.stringify(this.currentScanData, null, 2));
                alert('Results saved successfully!');
            }
        } catch (error) {
            console.error('Failed to save results:', error);
            alert(`Failed to save results: ${error.message}`);
        }
    }

    exportReport() {
        if (!this.currentScanData) {
            alert('No scan data to export');
            return;
        }

        // Simple HTML report export
        const html = this.generateHTMLReport(this.currentScanData);
        const blob = new Blob([html], { type: 'text/html' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `triageir-report-${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.html`;
        a.click();
        
        URL.revokeObjectURL(url);
    }

    generateHTMLReport(data) {
        return `
<!DOCTYPE html>
<html>
<head>
    <title>TriageIR Forensic Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .section { margin: 20px 0; }
        table { width: 100%; border-collapse: collapse; margin: 10px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .summary { background: #e8f4fd; padding: 15px; border-radius: 5px; }
    </style>
</head>
<body>
    <div class="header">
        <h1>TriageIR Forensic Report</h1>
        <p><strong>Generated:</strong> ${new Date().toISOString()}</p>
        <p><strong>Hostname:</strong> ${data.scan_metadata?.hostname || 'Unknown'}</p>
        <p><strong>Scan Duration:</strong> ${data.scan_metadata?.scan_duration_ms || 0}ms</p>
    </div>
    
    <div class="summary">
        <h2>Summary</h2>
        <p><strong>Processes:</strong> ${data.artifacts?.running_processes?.length || 0}</p>
        <p><strong>Network Connections:</strong> ${data.artifacts?.network_connections?.length || 0}</p>
        <p><strong>Persistence Mechanisms:</strong> ${data.artifacts?.persistence_mechanisms?.length || 0}</p>
    </div>
    
    <div class="section">
        <h2>System Information</h2>
        <p><strong>Uptime:</strong> ${data.artifacts?.system_info?.uptime_secs || 0} seconds</p>
        <p><strong>Logged Users:</strong> ${data.artifacts?.system_info?.logged_on_users?.length || 0}</p>
    </div>
    
    <div class="section">
        <h2>Running Processes</h2>
        <table>
            <tr><th>PID</th><th>Name</th><th>Command Line</th></tr>
            ${(data.artifacts?.running_processes || []).slice(0, 50).map(proc => 
                `<tr><td>${proc.pid}</td><td>${proc.name}</td><td>${proc.command_line || ''}</td></tr>`
            ).join('')}
        </table>
    </div>
    
    <div class="section">
        <h2>Network Connections</h2>
        <table>
            <tr><th>Protocol</th><th>Local</th><th>Remote</th><th>State</th></tr>
            ${(data.artifacts?.network_connections || []).slice(0, 50).map(conn => 
                `<tr><td>${conn.protocol}</td><td>${conn.local_address}</td><td>${conn.remote_address}</td><td>${conn.state}</td></tr>`
            ).join('')}
        </table>
    </div>
</body>
</html>`;
    }

    showWelcomeScreen() {
        document.getElementById('welcomeScreen').classList.remove('hidden');
        document.getElementById('scanScreen').classList.add('hidden');
        document.getElementById('resultsScreen').classList.add('hidden');
        this.updateFooterStatus('Ready');
    }

    showScanScreen() {
        document.getElementById('welcomeScreen').classList.add('hidden');
        document.getElementById('scanScreen').classList.remove('hidden');
        document.getElementById('resultsScreen').classList.add('hidden');
        this.updateFooterStatus('Scanning...');
        
        // Reset progress
        document.getElementById('progressFill').style.width = '0%';
        document.getElementById('progressText').textContent = 'Initializing scan...';
        document.getElementById('logContent').textContent = '';
    }

    showResultsScreen() {
        document.getElementById('welcomeScreen').classList.add('hidden');
        document.getElementById('scanScreen').classList.add('hidden');
        document.getElementById('resultsScreen').classList.remove('hidden');
        this.updateFooterStatus('Results loaded');
        
        // Populate results
        this.populateResults();
    }

    updateScanProgress(message) {
        const logContent = document.getElementById('logContent');
        const progressText = document.getElementById('progressText');
        const progressFill = document.getElementById('progressFill');
        
        // Add message to log
        logContent.textContent += message + '\n';
        logContent.scrollTop = logContent.scrollHeight;
        
        // Update progress text
        progressText.textContent = message;
        
        // Simulate progress (in real implementation, CLI would provide progress info)
        const currentWidth = parseInt(progressFill.style.width) || 0;
        if (currentWidth < 90) {
            progressFill.style.width = (currentWidth + 10) + '%';
        }
    }

    populateResults() {
        if (!this.currentScanData) return;
        
        console.log('Populating results with data:', this.currentScanData);
        
        // Populate summary cards
        this.populateSummaryCards();
        
        // Populate overview
        this.populateOverview();
        
        // Populate system info
        this.populateSystemInfo();
        
        // Populate processes table
        this.populateProcessesTable();
        
        // Populate network table
        this.populateNetworkTable();
        
        // Populate persistence table
        this.populatePersistenceTable();
        
        // Populate events table
        this.populateEventsTable();
    }

    populateSummaryCards() {
        const summaryCards = document.getElementById('summaryCards');
        const data = this.currentScanData;
        
        // Handle both old and new data structures
        const processes = data.artifacts?.running_processes || [];
        const connections = data.artifacts?.network_connections || [];
        const persistence = data.artifacts?.persistence_mechanisms || [];
        const events = data.artifacts?.event_logs || [];
        
        summaryCards.innerHTML = `
            <div class="summary-card">
                <h3>Processes</h3>
                <div class="value">${processes.length}</div>
            </div>
            <div class="summary-card">
                <h3>Network Connections</h3>
                <div class="value">${connections.length}</div>
            </div>
            <div class="summary-card">
                <h3>Persistence Items</h3>
                <div class="value">${persistence.length}</div>
            </div>
            <div class="summary-card">
                <h3>Event Logs</h3>
                <div class="value">${events.length}</div>
            </div>
        `;
    }

    populateOverview() {
        const overviewContent = document.getElementById('overviewContent');
        const data = this.currentScanData;
        
        // Handle the actual CLI data structure
        const metadata = data.scan_metadata || {};
        const systemInfo = data.artifacts?.system_info || {};
        
        overviewContent.innerHTML = `
            <div class="overview-info">
                <h4>Scan Information</h4>
                <p><strong>Hostname:</strong> ${metadata.hostname || systemInfo.hostname || 'Unknown'}</p>
                <p><strong>Current User:</strong> ${systemInfo.current_user || 'Unknown'}</p>
                <p><strong>OS Version:</strong> ${systemInfo.os_version || 'Unknown'}</p>
                <p><strong>Architecture:</strong> ${systemInfo.architecture || 'Unknown'}</p>
                <p><strong>Scan Duration:</strong> ${metadata.scan_duration_seconds ? (metadata.scan_duration_seconds * 1000).toFixed(2) + 'ms' : 'Unknown'}</p>
                <p><strong>CLI Version:</strong> ${metadata.version || 'Unknown'}</p>
                <p><strong>Timestamp:</strong> ${metadata.timestamp || 'Unknown'}</p>
                <p><strong>Total Artifacts:</strong> ${metadata.total_artifacts || 'Unknown'}</p>
            </div>
        `;
    }

    populateSystemInfo() {
        const systemContent = document.getElementById('systemContent');
        const systemInfo = this.currentScanData.artifacts?.system_info;
        
        if (!systemInfo) {
            systemContent.innerHTML = '<p>No system information available</p>';
            return;
        }
        
        systemContent.innerHTML = `
            <div class="system-info">
                <h4>System Details</h4>
                <p><strong>Hostname:</strong> ${systemInfo.hostname || 'Unknown'}</p>
                <p><strong>Current User:</strong> ${systemInfo.current_user || 'Unknown'}</p>
                <p><strong>OS Name:</strong> ${systemInfo.os_name || 'Unknown'}</p>
                <p><strong>OS Version:</strong> ${systemInfo.os_version || 'Unknown'}</p>
                <p><strong>Architecture:</strong> ${systemInfo.architecture || 'Unknown'}</p>
                <p><strong>Uptime:</strong> ${systemInfo.uptime_hours ? systemInfo.uptime_hours.toFixed(1) + ' hours' : 'Unknown'}</p>
                <p><strong>Last Boot:</strong> ${systemInfo.last_boot_time || 'Unknown'}</p>
            </div>
        `;
    }

    populateProcessesTable() {
        const tableBody = document.getElementById('processesTableBody');
        const processes = this.currentScanData.artifacts?.running_processes || [];
        
        tableBody.innerHTML = processes.slice(0, 100).map(proc => `
            <tr>
                <td>${proc.pid || 'N/A'}</td>
                <td>${proc.name || 'N/A'}</td>
                <td>${proc.user || 'N/A'}</td>
                <td>${proc.memory_usage_mb ? proc.memory_usage_mb.toFixed(1) : 'N/A'}</td>
                <td title="${proc.command_line || ''}">${(proc.command_line || '').substring(0, 50)}${(proc.command_line || '').length > 50 ? '...' : ''}</td>
            </tr>
        `).join('');
    }

    populateNetworkTable() {
        const tableBody = document.getElementById('networkTableBody');
        const connections = this.currentScanData.artifacts?.network_connections || [];
        
        tableBody.innerHTML = connections.slice(0, 100).map(conn => `
            <tr>
                <td>${conn.protocol || 'N/A'}</td>
                <td>${conn.local_address || 'N/A'}</td>
                <td>${conn.local_port || 'N/A'}</td>
                <td>${conn.remote_address || 'N/A'}</td>
                <td>${conn.remote_port || 'N/A'}</td>
                <td>${conn.state || 'N/A'}</td>
                <td>${conn.pid || conn.process_name || 'N/A'}</td>
            </tr>
        `).join('');
    }

    populatePersistenceTable() {
        const tableBody = document.getElementById('persistenceTableBody');
        const persistence = this.currentScanData.artifacts?.persistence_mechanisms || [];
        
        tableBody.innerHTML = persistence.slice(0, 100).map(item => `
            <tr>
                <td>${item.type || 'N/A'}</td>
                <td>${item.name || 'N/A'}</td>
                <td>${item.location || 'N/A'}</td>
                <td title="${item.value || ''}">${(item.value || '').substring(0, 50)}${(item.value || '').length > 50 ? '...' : ''}</td>
                <td>${item.is_suspicious ? '⚠️ Yes' : '✅ No'}</td>
            </tr>
        `).join('');
    }

    populateEventsTable() {
        const tableBody = document.getElementById('eventsTableBody');
        const events = this.currentScanData.artifacts?.event_logs || [];
        
        tableBody.innerHTML = events.slice(0, 100).map(event => `
            <tr>
                <td>${event.log_name || 'N/A'}</td>
                <td>${event.event_id || 'N/A'}</td>
                <td>${event.level || 'N/A'}</td>
                <td>${event.source || 'N/A'}</td>
                <td>${event.timestamp || 'N/A'}</td>
                <td title="${event.message || ''}">${(event.message || '').substring(0, 100)}${(event.message || '').length > 100 ? '...' : ''}</td>
            </tr>
        `).join('');
    }

    switchTab(tabName) {
        // Remove active class from all tabs and panels
        document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
        document.querySelectorAll('.tab-panel').forEach(panel => panel.classList.remove('active'));
        
        // Add active class to selected tab and panel
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');
        document.getElementById(`${tabName}Panel`).classList.add('active');
    }

    updateFooterStatus(status) {
        document.getElementById('footerStatus').textContent = status;
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new TriageIRApp();
});