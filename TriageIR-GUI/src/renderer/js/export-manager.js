// Export Manager for generating reports and exporting data

const { ipcRenderer } = require('electron');

class ExportManager {
    constructor() {
        this.currentData = null;
    }

    /**
     * Set current scan data
     */
    setData(scanData) {
        this.currentData = scanData;
    }

    /**
     * Export results as JSON
     */
    async exportJSON(data = null) {
        const exportData = data || this.currentData;
        if (!exportData) {
            throw new Error('No data to export');
        }

        const result = await ipcRenderer.invoke('show-save-dialog', {
            title: 'Export Scan Results',
            defaultPath: `triageir-scan-${new Date().toISOString().split('T')[0]}.json`,
            filters: [
                { name: 'JSON Files', extensions: ['json'] },
                { name: 'All Files', extensions: ['*'] }
            ]
        });

        if (!result.canceled && result.filePath) {
            const jsonData = JSON.stringify(exportData, null, 2);
            const writeResult = await ipcRenderer.invoke('write-file', result.filePath, jsonData);
            
            if (!writeResult.success) {
                throw new Error(writeResult.error);
            }
            
            return result.filePath;
        }
        
        return null;
    }

    /**
     * Export data as CSV
     */
    async exportCSV(data, filename, headers) {
        if (!data || data.length === 0) {
            throw new Error('No data to export');
        }

        const result = await ipcRenderer.invoke('show-save-dialog', {
            title: 'Export as CSV',
            defaultPath: filename,
            filters: [
                { name: 'CSV Files', extensions: ['csv'] },
                { name: 'All Files', extensions: ['*'] }
            ]
        });

        if (!result.canceled && result.filePath) {
            const csvData = this.convertToCSV(data, headers);
            const writeResult = await ipcRenderer.invoke('write-file', result.filePath, csvData);
            
            if (!writeResult.success) {
                throw new Error(writeResult.error);
            }
            
            return result.filePath;
        }
        
        return null;
    }

    /**
     * Export processes as CSV
     */
    async exportProcessesCSV(processes) {
        const headers = ['PID', 'Parent PID', 'Name', 'Command Line', 'Executable Path', 'SHA-256 Hash'];
        const data = processes.map(process => [
            process.pid,
            process.parent_pid,
            process.name,
            process.command_line,
            process.executable_path,
            process.sha256_hash
        ]);

        return this.exportCSV(data, 'processes.csv', headers);
    }

    /**
     * Export network connections as CSV
     */
    async exportNetworkCSV(connections) {
        const headers = ['Protocol', 'Local Address', 'Remote Address', 'State', 'Process ID'];
        const data = connections.map(conn => [
            conn.protocol,
            conn.local_address,
            conn.remote_address,
            conn.state,
            conn.owning_pid
        ]);

        return this.exportCSV(data, 'network-connections.csv', headers);
    }

    /**
     * Export persistence mechanisms as CSV
     */
    async exportPersistenceCSV(mechanisms) {
        const headers = ['Type', 'Name', 'Command', 'Source'];
        const data = mechanisms.map(mech => [
            mech.type,
            mech.name,
            mech.command,
            mech.source
        ]);

        return this.exportCSV(data, 'persistence-mechanisms.csv', headers);
    }

    /**
     * Export event logs as CSV
     */
    async exportEventsCSV(events) {
        const headers = ['Event ID', 'Level', 'Timestamp', 'Message', 'Source'];
        const data = events.map(event => [
            event.event_id,
            event.level,
            event.timestamp,
            event.message,
            event.source || 'Unknown'
        ]);

        return this.exportCSV(data, 'event-logs.csv', headers);
    }

    /**
     * Export collection log as CSV
     */
    async exportLogsCSV(logs) {
        const headers = ['Timestamp', 'Level', 'Message'];
        const data = logs.map(log => [
            log.timestamp,
            log.level,
            log.message
        ]);

        return this.exportCSV(data, 'collection-log.csv', headers);
    }

    /**
     * Generate HTML report
     */
    async generateHTMLReport() {
        if (!this.currentData) {
            throw new Error('No data to export');
        }

        const result = await ipcRenderer.invoke('show-save-dialog', {
            title: 'Export HTML Report',
            defaultPath: `triageir-report-${new Date().toISOString().split('T')[0]}.html`,
            filters: [
                { name: 'HTML Files', extensions: ['html'] },
                { name: 'All Files', extensions: ['*'] }
            ]
        });

        if (!result.canceled && result.filePath) {
            const htmlContent = this.generateHTMLContent(this.currentData);
            const writeResult = await ipcRenderer.invoke('write-file', result.filePath, htmlContent);
            
            if (!writeResult.success) {
                throw new Error(writeResult.error);
            }
            
            return result.filePath;
        }
        
        return null;
    }

    /**
     * Generate text report
     */
    async generateTextReport() {
        if (!this.currentData) {
            throw new Error('No data to export');
        }

        const result = await ipcRenderer.invoke('show-save-dialog', {
            title: 'Export Text Report',
            defaultPath: `triageir-report-${new Date().toISOString().split('T')[0]}.txt`,
            filters: [
                { name: 'Text Files', extensions: ['txt'] },
                { name: 'All Files', extensions: ['*'] }
            ]
        });

        if (!result.canceled && result.filePath) {
            const textContent = this.generateTextContent(this.currentData);
            const writeResult = await ipcRenderer.invoke('write-file', result.filePath, textContent);
            
            if (!writeResult.success) {
                throw new Error(writeResult.error);
            }
            
            return result.filePath;
        }
        
        return null;
    }

    /**
     * Convert data to CSV format
     */
    convertToCSV(data, headers) {
        const csvRows = [];
        
        // Add headers
        if (headers) {
            csvRows.push(headers.map(header => this.escapeCSVField(header)).join(','));
        }
        
        // Add data rows
        data.forEach(row => {
            const csvRow = row.map(field => this.escapeCSVField(field)).join(',');
            csvRows.push(csvRow);
        });
        
        return csvRows.join('\n');
    }

    /**
     * Escape CSV field
     */
    escapeCSVField(field) {
        if (field === null || field === undefined) {
            return '';
        }
        
        const stringField = String(field);
        
        // If field contains comma, newline, or quote, wrap in quotes and escape quotes
        if (stringField.includes(',') || stringField.includes('\n') || stringField.includes('"')) {
            return '"' + stringField.replace(/"/g, '""') + '"';
        }
        
        return stringField;
    }

    /**
     * Generate HTML report content
     */
    generateHTMLContent(scanData) {
        const metadata = scanData.scan_metadata;
        const artifacts = scanData.artifacts;
        
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TriageIR Forensic Report - ${metadata.hostname}</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        .header { border-bottom: 2px solid #2563eb; padding-bottom: 20px; margin-bottom: 30px; }
        .header h1 { color: #2563eb; margin: 0; }
        .header .subtitle { color: #64748b; margin-top: 5px; }
        .section { margin-bottom: 40px; }
        .section h2 { color: #1e293b; border-bottom: 1px solid #e2e8f0; padding-bottom: 10px; }
        .section h3 { color: #475569; margin-top: 25px; }
        .info-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 20px 0; }
        .info-card { background: #f8fafc; padding: 15px; border-radius: 8px; border-left: 4px solid #2563eb; }
        .info-card .label { font-weight: bold; color: #374151; }
        .info-card .value { color: #1f2937; font-family: monospace; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #e2e8f0; }
        th { background: #f1f5f9; font-weight: 600; color: #374151; }
        tr:hover { background: #f8fafc; }
        .mono { font-family: monospace; font-size: 0.9em; }
        .badge { padding: 4px 8px; border-radius: 12px; font-size: 0.8em; font-weight: 500; }
        .badge.success { background: #dcfce7; color: #166534; }
        .badge.warning { background: #fef3c7; color: #92400e; }
        .badge.error { background: #fee2e2; color: #991b1b; }
        .summary-stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin: 20px 0; }
        .stat-card { background: white; border: 1px solid #e2e8f0; padding: 20px; border-radius: 8px; text-align: center; }
        .stat-number { font-size: 2em; font-weight: bold; color: #2563eb; }
        .stat-label { color: #64748b; margin-top: 5px; }
        .footer { margin-top: 50px; padding-top: 20px; border-top: 1px solid #e2e8f0; color: #64748b; font-size: 0.9em; }
    </style>
</head>
<body>
    <div class="header">
        <h1>TriageIR Forensic Report</h1>
        <div class="subtitle">Digital Forensics Triage Analysis</div>
    </div>

    <div class="section">
        <h2>Scan Summary</h2>
        <div class="info-grid">
            <div class="info-card">
                <div class="label">Hostname</div>
                <div class="value">${this.escapeHtml(metadata.hostname)}</div>
            </div>
            <div class="info-card">
                <div class="label">OS Version</div>
                <div class="value">${this.escapeHtml(metadata.os_version)}</div>
            </div>
            <div class="info-card">
                <div class="label">Scan Time</div>
                <div class="value">${new Date(metadata.scan_start_utc).toLocaleString()}</div>
            </div>
            <div class="info-card">
                <div class="label">Duration</div>
                <div class="value">${TriageUtils.formatDuration(metadata.scan_duration_ms)}</div>
            </div>
            <div class="info-card">
                <div class="label">CLI Version</div>
                <div class="value">${this.escapeHtml(metadata.cli_version)}</div>
            </div>
            <div class="info-card">
                <div class="label">Scan ID</div>
                <div class="value">${metadata.scan_id}</div>
            </div>
        </div>

        <div class="summary-stats">
            <div class="stat-card">
                <div class="stat-number">${artifacts.running_processes.length}</div>
                <div class="stat-label">Running Processes</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">${artifacts.network_connections.length}</div>
                <div class="stat-label">Network Connections</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">${artifacts.persistence_mechanisms.length}</div>
                <div class="stat-label">Persistence Items</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">${artifacts.event_logs.security.length + artifacts.event_logs.system.length}</div>
                <div class="stat-label">Event Log Entries</div>
            </div>
        </div>
    </div>

    <div class="section">
        <h2>System Information</h2>
        <div class="info-grid">
            <div class="info-card">
                <div class="label">System Uptime</div>
                <div class="value">${TriageUtils.formatUptime(artifacts.system_info.uptime_secs)}</div>
            </div>
            <div class="info-card">
                <div class="label">Logged-on Users</div>
                <div class="value">${artifacts.system_info.logged_on_users.length}</div>
            </div>
        </div>
        
        ${artifacts.system_info.logged_on_users.length > 0 ? `
        <h3>Active Users</h3>
        <table>
            <thead>
                <tr><th>Username</th><th>Domain</th><th>Logon Time</th></tr>
            </thead>
            <tbody>
                ${artifacts.system_info.logged_on_users.map(user => `
                    <tr>
                        <td>${this.escapeHtml(user.username)}</td>
                        <td>${this.escapeHtml(user.domain)}</td>
                        <td class="mono">${new Date(user.logon_time).toLocaleString()}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
        ` : ''}
    </div>

    ${artifacts.running_processes.length > 0 ? `
    <div class="section">
        <h2>Running Processes (Top 20)</h2>
        <table>
            <thead>
                <tr><th>PID</th><th>Name</th><th>Parent PID</th><th>Executable Path</th></tr>
            </thead>
            <tbody>
                ${artifacts.running_processes.slice(0, 20).map(process => `
                    <tr>
                        <td class="mono">${process.pid}</td>
                        <td>${this.escapeHtml(process.name)}</td>
                        <td class="mono">${process.parent_pid}</td>
                        <td class="mono">${this.escapeHtml(process.executable_path)}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
        ${artifacts.running_processes.length > 20 ? `<p><em>Showing 20 of ${artifacts.running_processes.length} processes</em></p>` : ''}
    </div>
    ` : ''}

    ${artifacts.network_connections.length > 0 ? `
    <div class="section">
        <h2>Network Connections (Top 20)</h2>
        <table>
            <thead>
                <tr><th>Protocol</th><th>Local Address</th><th>Remote Address</th><th>State</th><th>PID</th></tr>
            </thead>
            <tbody>
                ${artifacts.network_connections.slice(0, 20).map(conn => `
                    <tr>
                        <td><span class="badge ${conn.protocol === 'TCP' ? 'success' : 'warning'}">${conn.protocol}</span></td>
                        <td class="mono">${this.escapeHtml(conn.local_address)}</td>
                        <td class="mono">${this.escapeHtml(conn.remote_address)}</td>
                        <td>${conn.state}</td>
                        <td class="mono">${conn.owning_pid}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
        ${artifacts.network_connections.length > 20 ? `<p><em>Showing 20 of ${artifacts.network_connections.length} connections</em></p>` : ''}
    </div>
    ` : ''}

    ${artifacts.persistence_mechanisms.length > 0 ? `
    <div class="section">
        <h2>Persistence Mechanisms</h2>
        <table>
            <thead>
                <tr><th>Type</th><th>Name</th><th>Command</th><th>Source</th></tr>
            </thead>
            <tbody>
                ${artifacts.persistence_mechanisms.map(mech => `
                    <tr>
                        <td><span class="badge success">${this.escapeHtml(mech.type)}</span></td>
                        <td>${this.escapeHtml(mech.name)}</td>
                        <td class="mono">${this.escapeHtml(mech.command)}</td>
                        <td class="mono">${this.escapeHtml(mech.source)}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    </div>
    ` : ''}

    <div class="footer">
        <p>Report generated by TriageIR GUI on ${new Date().toLocaleString()}</p>
        <p>This report contains forensic data collected from ${metadata.hostname} and should be handled according to your organization's data handling policies.</p>
    </div>
</body>
</html>`;
    }

    /**
     * Generate text report content
     */
    generateTextContent(scanData) {
        const metadata = scanData.scan_metadata;
        const artifacts = scanData.artifacts;
        
        let content = `TRIAGEIR FORENSIC REPORT
${'='.repeat(50)}

SCAN SUMMARY
${'-'.repeat(20)}
Hostname: ${metadata.hostname}
OS Version: ${metadata.os_version}
Scan Time: ${new Date(metadata.scan_start_utc).toLocaleString()}
Duration: ${TriageUtils.formatDuration(metadata.scan_duration_ms)}
CLI Version: ${metadata.cli_version}
Scan ID: ${metadata.scan_id}

ARTIFACT SUMMARY
${'-'.repeat(20)}
Running Processes: ${artifacts.running_processes.length}
Network Connections: ${artifacts.network_connections.length}
Persistence Mechanisms: ${artifacts.persistence_mechanisms.length}
Event Log Entries: ${artifacts.event_logs.security.length + artifacts.event_logs.system.length}

SYSTEM INFORMATION
${'-'.repeat(20)}
System Uptime: ${TriageUtils.formatUptime(artifacts.system_info.uptime_secs)}
Logged-on Users: ${artifacts.system_info.logged_on_users.length}

`;

        if (artifacts.system_info.logged_on_users.length > 0) {
            content += `ACTIVE USERS\n${'-'.repeat(15)}\n`;
            artifacts.system_info.logged_on_users.forEach(user => {
                content += `${user.username}@${user.domain} (${new Date(user.logon_time).toLocaleString()})\n`;
            });
            content += '\n';
        }

        if (artifacts.running_processes.length > 0) {
            content += `RUNNING PROCESSES (Top 10)\n${'-'.repeat(30)}\n`;
            artifacts.running_processes.slice(0, 10).forEach(process => {
                content += `PID ${process.pid}: ${process.name} (${process.executable_path})\n`;
            });
            content += `\n(Showing 10 of ${artifacts.running_processes.length} processes)\n\n`;
        }

        if (artifacts.network_connections.length > 0) {
            content += `NETWORK CONNECTIONS (Top 10)\n${'-'.repeat(35)}\n`;
            artifacts.network_connections.slice(0, 10).forEach(conn => {
                content += `${conn.protocol}: ${conn.local_address} -> ${conn.remote_address} (${conn.state}) [PID ${conn.owning_pid}]\n`;
            });
            content += `\n(Showing 10 of ${artifacts.network_connections.length} connections)\n\n`;
        }

        if (artifacts.persistence_mechanisms.length > 0) {
            content += `PERSISTENCE MECHANISMS\n${'-'.repeat(25)}\n`;
            artifacts.persistence_mechanisms.forEach(mech => {
                content += `${mech.type}: ${mech.name}\n  Command: ${mech.command}\n  Source: ${mech.source}\n\n`;
            });
        }

        content += `\nReport generated by TriageIR GUI on ${new Date().toLocaleString()}\n`;
        content += `This report contains forensic data collected from ${metadata.hostname}.\n`;

        return content;
    }

    /**
     * Escape HTML characters
     */
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Export for use in other modules
window.ExportManager = ExportManager;