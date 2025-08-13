const { spawn } = require('child_process');
const { ipcRenderer } = require('electron');
const path = require('path');
const fs = require('fs');

class TriageIRGUI {
    constructor() {
        this.currentData = null;
        this.isScanning = false;
        this.cliPath = null;
        
        this.init();
    }

    async init() {
        console.log('Initializing TriageIR GUI...');
        
        // Find CLI executable
        await this.findCLI();
        
        // Set up event listeners
        this.setupEventListeners();
        
        // Set up tabs
        this.setupTabs();
        
        console.log('TriageIR GUI initialized successfully');
        this.log('TriageIR GUI initialized', 'success');
    }

    async findCLI() {
        const possiblePaths = [
            path.join(process.cwd(), '..', 'TriageIR-CLI', 'target', 'release', 'triageir-cli.exe'),
            path.join(process.cwd(), '..', 'TriageIR-CLI', 'target', 'debug', 'triageir-cli.exe'),
            path.join(process.cwd(), 'triageir-cli.exe'),
            'triageir-cli.exe'
        ];

        for (const cliPath of possiblePaths) {
            try {
                if (fs.existsSync(cliPath)) {
                    this.cliPath = cliPath;
                    console.log(`Found CLI at: ${cliPath}`);
                    this.log(`CLI found: ${path.basename(cliPath)}`, 'success');
                    return;
                }
            } catch (error) {
                console.log(`Error checking ${cliPath}:`, error);
            }
        }

        this.log('CLI not found - some features may not work', 'warning');
        this.cliPath = 'triageir-cli.exe'; // Fallback
    }

    setupEventListeners() {
        // Scan buttons
        document.getElementById('startScan').addEventListener('click', () => {
            this.startScan(false);
        });

        document.getElementById('quickScan').addEventListener('click', () => {
            this.startScan(true);
        });

        // File operations
        document.getElementById('openFile').addEventListener('click', () => {
            this.openFile();
        });

        document.getElementById('exportResults').addEventListener('click', () => {
            this.exportResults();
        });

        document.getElementById('clearResults').addEventListener('click', () => {
            this.clearResults();
        });
    }

    setupTabs() {
        const tabs = document.querySelectorAll('.tab');
        const tabContents = document.querySelectorAll('.tab-content');

        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const targetTab = tab.getAttribute('data-tab');
                
                // Remove active class from all tabs and contents
                tabs.forEach(t => t.classList.remove('active'));
                tabContents.forEach(content => content.classList.remove('active'));
                
                // Add active class to clicked tab and corresponding content
                tab.classList.add('active');
                document.getElementById(targetTab).classList.add('active');
            });
        });
    }

    async startScan(isQuick = false) {
        if (this.isScanning) {
            this.log('Scan already in progress', 'warning');
            return;
        }

        this.isScanning = true;
        this.updateStatus('running', isQuick ? 'Running Quick Scan...' : 'Running Full Scan...');
        this.showProgress();
        
        const outputFile = `triageir-scan-${Date.now()}.json`;
        const args = ['-v', '-o', outputFile];

        this.log(`Starting ${isQuick ? 'quick' : 'full'} scan...`, 'info');
        this.log(`Command: ${this.cliPath} ${args.join(' ')}`, 'info');

        try {
            const process = spawn(this.cliPath, args, { shell: true });
            let stdout = '';
            let stderr = '';

            process.stdout.on('data', (data) => {
                stdout += data.toString();
            });

            process.stderr.on('data', (data) => {
                stderr += data.toString();
                // Log CLI progress
                const lines = data.toString().split('\n');
                lines.forEach(line => {
                    if (line.trim()) {
                        this.log(line.trim(), 'info');
                    }
                });
            });

            process.on('close', (code) => {
                this.isScanning = false;
                this.hideProgress();

                if (code === 0) {
                    this.log('Scan completed successfully', 'success');
                    this.updateStatus('complete', 'Scan Complete');
                    this.loadResults(outputFile);
                } else {
                    this.log(`Scan failed with exit code: ${code}`, 'error');
                    this.updateStatus('error', 'Scan Failed');
                    if (stderr) {
                        this.log(`Error: ${stderr}`, 'error');
                    }
                }
            });

            process.on('error', (error) => {
                this.isScanning = false;
                this.hideProgress();
                this.log(`Scan error: ${error.message}`, 'error');
                this.updateStatus('error', 'Scan Error');
            });

        } catch (error) {
            this.isScanning = false;
            this.hideProgress();
            this.log(`Failed to start scan: ${error.message}`, 'error');
            this.updateStatus('error', 'Failed to Start');
        }
    }

    async loadResults(filePath) {
        try {
            if (!fs.existsSync(filePath)) {
                throw new Error(`Results file not found: ${filePath}`);
            }

            const jsonData = fs.readFileSync(filePath, 'utf8');
            const results = JSON.parse(jsonData);
            
            this.currentData = results;
            this.displayResults(results);
            this.enableExportButtons();
            
            this.log(`Results loaded from ${filePath}`, 'success');
            
        } catch (error) {
            this.log(`Failed to load results: ${error.message}`, 'error');
        }
    }

    async openFile() {
        try {
            const result = await ipcRenderer.invoke('show-open-dialog', {
                properties: ['openFile'],
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ]
            });

            if (!result.canceled && result.filePaths.length > 0) {
                await this.loadResults(result.filePaths[0]);
            }
        } catch (error) {
            this.log(`Failed to open file: ${error.message}`, 'error');
        }
    }

    displayResults(results) {
        // Hide welcome screen, show results
        document.getElementById('welcomeScreen').classList.add('hidden');
        document.getElementById('resultsScreen').classList.remove('hidden');

        // Display overview
        this.displayOverview(results);
        
        // Display system info
        this.displaySystemInfo(results.artifacts.system_info);
        
        // Display processes
        this.displayProcesses(results.artifacts.running_processes);
        
        // Display network connections
        this.displayNetwork(results.artifacts.network_connections);
        
        // Display persistence mechanisms
        this.displayPersistence(results.artifacts.persistence_mechanisms);
        
        // Display event logs
        this.displayEvents(results.artifacts.event_logs);
    }

    displayOverview(results) {
        const content = document.getElementById('overviewContent');
        const artifacts = results.artifacts;
        
        content.innerHTML = `
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin: 20px 0;">
                <div style="background: #e8f5e8; padding: 15px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 24px; font-weight: bold; color: #28a745;">${artifacts.running_processes.length}</div>
                    <div style="color: #666;">Running Processes</div>
                </div>
                <div style="background: #e3f2fd; padding: 15px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 24px; font-weight: bold; color: #2196f3;">${artifacts.network_connections.length}</div>
                    <div style="color: #666;">Network Connections</div>
                </div>
                <div style="background: #fff3e0; padding: 15px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 24px; font-weight: bold; color: #ff9800;">${artifacts.persistence_mechanisms.length}</div>
                    <div style="color: #666;">Persistence Items</div>
                </div>
                <div style="background: #f3e5f5; padding: 15px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 24px; font-weight: bold; color: #9c27b0;">${artifacts.event_logs.length}</div>
                    <div style="color: #666;">Event Log Entries</div>
                </div>
            </div>
            
            <h4>System Summary</h4>
            <table class="data-table">
                <tr><td><strong>Hostname:</strong></td><td>${artifacts.system_info.hostname}</td></tr>
                <tr><td><strong>OS:</strong></td><td>${artifacts.system_info.os_name} ${artifacts.system_info.os_version}</td></tr>
                <tr><td><strong>Architecture:</strong></td><td>${artifacts.system_info.architecture}</td></tr>
                <tr><td><strong>Current User:</strong></td><td>${artifacts.system_info.current_user}</td></tr>
                <tr><td><strong>Scan Duration:</strong></td><td>${results.scan_metadata.scan_duration_seconds} seconds</td></tr>
                <tr><td><strong>Total Artifacts:</strong></td><td>${results.scan_metadata.total_artifacts}</td></tr>
            </table>
        `;
    }

    displaySystemInfo(systemInfo) {
        const content = document.getElementById('systemContent');
        content.innerHTML = `
            <table class="data-table">
                <tr><td><strong>Hostname:</strong></td><td>${systemInfo.hostname}</td></tr>
                <tr><td><strong>Operating System:</strong></td><td>${systemInfo.os_name}</td></tr>
                <tr><td><strong>OS Version:</strong></td><td>${systemInfo.os_version}</td></tr>
                <tr><td><strong>Architecture:</strong></td><td>${systemInfo.architecture}</td></tr>
                <tr><td><strong>Current User:</strong></td><td>${systemInfo.current_user}</td></tr>
                <tr><td><strong>Uptime:</strong></td><td>${systemInfo.uptime_hours} hours</td></tr>
                <tr><td><strong>Last Boot:</strong></td><td>${new Date(systemInfo.last_boot_time).toLocaleString()}</td></tr>
            </table>
        `;
    }

    displayProcesses(processes) {
        const content = document.getElementById('processesContent');
        let html = `
            <table class="data-table">
                <thead>
                    <tr>
                        <th>PID</th>
                        <th>Name</th>
                        <th>User</th>
                        <th>Memory (MB)</th>
                        <th>Command Line</th>
                    </tr>
                </thead>
                <tbody>
        `;

        processes.forEach(process => {
            html += `
                <tr>
                    <td>${process.pid}</td>
                    <td>${process.name}</td>
                    <td>${process.user}</td>
                    <td>${process.memory_usage_mb}</td>
                    <td title="${process.command_line}">${process.command_line.substring(0, 50)}${process.command_line.length > 50 ? '...' : ''}</td>
                </tr>
            `;
        });

        html += '</tbody></table>';
        content.innerHTML = html;
    }

    displayNetwork(connections) {
        const content = document.getElementById('networkContent');
        let html = `
            <table class="data-table">
                <thead>
                    <tr>
                        <th>Protocol</th>
                        <th>Local Address</th>
                        <th>Remote Address</th>
                        <th>State</th>
                        <th>Process</th>
                    </tr>
                </thead>
                <tbody>
        `;

        connections.forEach(conn => {
            html += `
                <tr>
                    <td>${conn.protocol}</td>
                    <td>${conn.local_address}:${conn.local_port}</td>
                    <td>${conn.remote_address}:${conn.remote_port}</td>
                    <td>${conn.state}</td>
                    <td>${conn.process_name} (${conn.pid})</td>
                </tr>
            `;
        });

        html += '</tbody></table>';
        content.innerHTML = html;
    }

    displayPersistence(mechanisms) {
        const content = document.getElementById('persistenceContent');
        let html = `
            <table class="data-table">
                <thead>
                    <tr>
                        <th>Type</th>
                        <th>Name</th>
                        <th>Location</th>
                        <th>Value</th>
                        <th>Suspicious</th>
                    </tr>
                </thead>
                <tbody>
        `;

        mechanisms.forEach(mech => {
            html += `
                <tr>
                    <td>${mech.type}</td>
                    <td>${mech.name}</td>
                    <td title="${mech.location}">${mech.location.substring(0, 40)}${mech.location.length > 40 ? '...' : ''}</td>
                    <td title="${mech.value}">${mech.value.substring(0, 30)}${mech.value.length > 30 ? '...' : ''}</td>
                    <td>${mech.is_suspicious ? '⚠️ Yes' : '✅ No'}</td>
                </tr>
            `;
        });

        html += '</tbody></table>';
        content.innerHTML = html;
    }

    displayEvents(events) {
        const content = document.getElementById('eventsContent');
        let html = `
            <table class="data-table">
                <thead>
                    <tr>
                        <th>Time</th>
                        <th>Log</th>
                        <th>Event ID</th>
                        <th>Level</th>
                        <th>Source</th>
                        <th>Message</th>
                    </tr>
                </thead>
                <tbody>
        `;

        events.forEach(event => {
            html += `
                <tr>
                    <td>${new Date(event.timestamp).toLocaleString()}</td>
                    <td>${event.log_name}</td>
                    <td>${event.event_id}</td>
                    <td>${event.level}</td>
                    <td>${event.source}</td>
                    <td title="${event.message}">${event.message.substring(0, 60)}${event.message.length > 60 ? '...' : ''}</td>
                </tr>
            `;
        });

        html += '</tbody></table>';
        content.innerHTML = html;
    }

    async exportResults() {
        if (!this.currentData) {
            this.log('No data to export', 'warning');
            return;
        }

        try {
            const result = await ipcRenderer.invoke('show-save-dialog', {
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ],
                defaultPath: `triageir-export-${Date.now()}.json`
            });

            if (!result.canceled) {
                const jsonData = JSON.stringify(this.currentData, null, 2);
                const saveResult = await ipcRenderer.invoke('save-file', result.filePath, jsonData);
                
                if (saveResult.success) {
                    this.log(`Results exported to: ${result.filePath}`, 'success');
                } else {
                    this.log(`Export failed: ${saveResult.error}`, 'error');
                }
            }
        } catch (error) {
            this.log(`Export error: ${error.message}`, 'error');
        }
    }

    clearResults() {
        this.currentData = null;
        document.getElementById('welcomeScreen').classList.remove('hidden');
        document.getElementById('resultsScreen').classList.add('hidden');
        this.updateStatus('ready', 'Ready');
        this.disableExportButtons();
        this.log('Results cleared', 'info');
    }

    updateStatus(type, message) {
        const statusElement = document.getElementById('status');
        const statusText = document.getElementById('statusText');
        
        statusElement.className = `status status-${type}`;
        statusText.textContent = message;
    }

    showProgress() {
        document.getElementById('progress').classList.remove('hidden');
        // Animate progress bar
        let progress = 0;
        const interval = setInterval(() => {
            progress += Math.random() * 10;
            if (progress >= 90) {
                progress = 90;
                clearInterval(interval);
            }
            document.getElementById('progressBar').style.width = `${progress}%`;
        }, 200);
    }

    hideProgress() {
        document.getElementById('progress').classList.add('hidden');
        document.getElementById('progressBar').style.width = '0%';
    }

    enableExportButtons() {
        document.getElementById('exportResults').disabled = false;
        document.getElementById('clearResults').disabled = false;
    }

    disableExportButtons() {
        document.getElementById('exportResults').disabled = true;
        document.getElementById('clearResults').disabled = true;
    }

    log(message, level = 'info') {
        const logContainer = document.getElementById('logContainer');
        const timestamp = new Date().toLocaleTimeString();
        const logEntry = document.createElement('div');
        logEntry.className = `log-entry log-${level}`;
        logEntry.textContent = `[${timestamp}] ${message}`;
        
        logContainer.appendChild(logEntry);
        logContainer.scrollTop = logContainer.scrollHeight;
        
        // Keep only last 100 log entries
        while (logContainer.children.length > 100) {
            logContainer.removeChild(logContainer.firstChild);
        }
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, starting TriageIR GUI...');
    window.triageApp = new TriageIRGUI();
});

// Handle errors
window.addEventListener('error', (event) => {
    console.error('JavaScript error:', event.error);
});

window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
});