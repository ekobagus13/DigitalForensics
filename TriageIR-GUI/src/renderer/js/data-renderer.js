// Data Renderer for displaying scan results in the UI

class DataRenderer {
    constructor() {
        this.currentData = null;
        this.filteredData = {};
        this.sortState = {};
    }

    /**
     * Render all scan results
     */
    renderResults(scanResults) {
        this.currentData = scanResults;

        // Render each section
        this.renderOverview(scanResults);
        this.renderSystemInfo(scanResults.artifacts.system_info);
        this.renderProcesses(scanResults.artifacts.running_processes);
        this.renderNetworkConnections(scanResults.artifacts.network_connections);
        this.renderPersistenceMechanisms(scanResults.artifacts.persistence_mechanisms);
        this.renderEventLogs(scanResults.artifacts.event_logs);
        this.renderCollectionLog(scanResults.collection_log);

        // Update status bar
        this.updateStatusBar(scanResults);

        // Show results container
        document.getElementById('welcomeScreen').style.display = 'none';
        document.getElementById('resultsContainer').style.display = 'flex';

        // Enable export buttons
        this.enableExportButtons();
    }

    /**
     * Render overview tab
     */
    renderOverview(scanResults) {
        const summaryStats = document.getElementById('summaryStats');
        const systemSummary = document.getElementById('systemSummary');
        const findingsSummary = document.getElementById('findingsSummary');
        const collectionSummary = document.getElementById('collectionSummary');

        // Summary statistics
        const artifacts = scanResults.artifacts;
        const totalArtifacts =
            artifacts.running_processes.length +
            artifacts.network_connections.length +
            artifacts.persistence_mechanisms.length +
            artifacts.event_logs.security.length +
            artifacts.event_logs.system.length;

        summaryStats.innerHTML = `
            <div class="stat-item">
                <span class="stat-label">Scan Duration</span>
                <span class="stat-value">${TriageUtils.formatDuration(scanResults.scan_metadata.scan_duration_ms)}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Total Artifacts</span>
                <span class="stat-value">${totalArtifacts.toLocaleString()}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Running Processes</span>
                <span class="stat-value">${artifacts.running_processes.length.toLocaleString()}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Network Connections</span>
                <span class="stat-value">${artifacts.network_connections.length.toLocaleString()}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Persistence Items</span>
                <span class="stat-value">${artifacts.persistence_mechanisms.length.toLocaleString()}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Event Log Entries</span>
                <span class="stat-value">${(artifacts.event_logs.security.length + artifacts.event_logs.system.length).toLocaleString()}</span>
            </div>
        `;

        // System summary
        systemSummary.innerHTML = `
            <div class="stat-item">
                <span class="stat-label">Hostname</span>
                <span class="stat-value">${TriageUtils.escapeHtml(scanResults.scan_metadata.hostname)}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">OS Version</span>
                <span class="stat-value">${TriageUtils.escapeHtml(scanResults.scan_metadata.os_version)}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Uptime</span>
                <span class="stat-value">${TriageUtils.formatUptime(artifacts.system_info.uptime_secs)}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Logged Users</span>
                <span class="stat-value">${artifacts.system_info.logged_on_users.length}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Scan Time</span>
                <span class="stat-value">${TriageUtils.formatTimestamp(scanResults.scan_metadata.scan_start_utc)}</span>
            </div>
        `;

        // Key findings
        const externalConnections = artifacts.network_connections.filter(conn =>
            TriageUtils.isExternalIP(conn.remote_address.split(':')[0])
        ).length;

        const suspiciousPersistence = artifacts.persistence_mechanisms.filter(mech =>
            mech.command.toLowerCase().includes('temp') ||
            mech.command.toLowerCase().includes('powershell') ||
            mech.command.toLowerCase().includes('cmd.exe')
        ).length;

        const errorEvents = artifacts.event_logs.security.concat(artifacts.event_logs.system)
            .filter(event => event.level.toLowerCase().includes('error')).length;

        findingsSummary.innerHTML = `
            <div class="stat-item">
                <span class="stat-label">External Connections</span>
                <span class="stat-value ${externalConnections > 0 ? 'warning' : 'success'}">${externalConnections}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Suspicious Persistence</span>
                <span class="stat-value ${suspiciousPersistence > 0 ? 'warning' : 'success'}">${suspiciousPersistence}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Error Events</span>
                <span class="stat-value ${errorEvents > 0 ? 'warning' : 'success'}">${errorEvents}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Collection Errors</span>
                <span class="stat-value ${scanResults.collection_log.filter(log => log.level === 'ERROR').length > 0 ? 'danger' : 'success'}">
                    ${scanResults.collection_log.filter(log => log.level === 'ERROR').length}
                </span>
            </div>
        `;

        // Collection summary
        const logCounts = {
            INFO: scanResults.collection_log.filter(log => log.level === 'INFO').length,
            WARN: scanResults.collection_log.filter(log => log.level === 'WARN').length,
            ERROR: scanResults.collection_log.filter(log => log.level === 'ERROR').length
        };

        collectionSummary.innerHTML = `
            <div class="stat-item">
                <span class="stat-label">CLI Version</span>
                <span class="stat-value">${TriageUtils.escapeHtml(scanResults.scan_metadata.cli_version)}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Info Messages</span>
                <span class="stat-value success">${logCounts.INFO}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Warnings</span>
                <span class="stat-value ${logCounts.WARN > 0 ? 'warning' : 'success'}">${logCounts.WARN}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Errors</span>
                <span class="stat-value ${logCounts.ERROR > 0 ? 'danger' : 'success'}">${logCounts.ERROR}</span>
            </div>
            <div class="stat-item">
                <span class="stat-label">Success Rate</span>
                <span class="stat-value ${logCounts.ERROR === 0 ? 'success' : 'warning'}">
                    ${logCounts.ERROR === 0 ? '100%' : Math.round((1 - logCounts.ERROR / scanResults.collection_log.length) * 100) + '%'}
                </span>
            </div>
        `;
    }

    /**
     * Render system information
     */
    renderSystemInfo(systemInfo) {
        const container = document.getElementById('systemInfoContent');

        container.innerHTML = `
            <div class="system-info-grid">
                <div class="system-info-card">
                    <h3>System Status</h3>
                    <div class="system-detail">
                        <span class="system-detail-label">Uptime</span>
                        <span class="system-detail-value">${TriageUtils.formatUptime(systemInfo.uptime_secs)}</span>
                    </div>
                    <div class="system-detail">
                        <span class="system-detail-label">Active Users</span>
                        <span class="system-detail-value">${systemInfo.logged_on_users.length}</span>
                    </div>
                </div>
                
                <div class="system-info-card">
                    <h3>Logged-on Users</h3>
                    ${systemInfo.logged_on_users.length > 0 ? `
                        <ul class="user-list">
                            ${systemInfo.logged_on_users.map(user => `
                                <li class="user-item">
                                    <div class="user-avatar">${user.username.charAt(0).toUpperCase()}</div>
                                    <div class="user-info">
                                        <div class="user-name">${TriageUtils.escapeHtml(user.username)}</div>
                                        <div class="user-domain">${TriageUtils.escapeHtml(user.domain)}</div>
                                    </div>
                                    <div class="user-logon-time">${TriageUtils.formatTimestamp(user.logon_time)}</div>
                                </li>
                            `).join('')}
                        </ul>
                    ` : '<p class="text-muted">No logged-on users found</p>'}
                </div>
            </div>
        `;
    }

    /**
     * Render processes table
     */
    renderProcesses(processes) {
        this.filteredData.processes = processes;
        this.updateProcessesTable();
    }

    /**
     * Update processes table with current filter
     */
    updateProcessesTable() {
        const tbody = document.getElementById('processesTableBody');
        const filter = document.getElementById('processFilter').value.toLowerCase();

        let filteredProcesses = this.filteredData.processes;

        if (filter) {
            filteredProcesses = filteredProcesses.filter(process =>
                process.name.toLowerCase().includes(filter) ||
                process.command_line.toLowerCase().includes(filter) ||
                process.executable_path.toLowerCase().includes(filter) ||
                process.pid.toString().includes(filter)
            );
        }

        tbody.innerHTML = filteredProcesses.map(process => `
            <tr>
                <td><span class="process-pid">${process.pid}</span></td>
                <td><span class="process-name">${TriageUtils.escapeHtml(process.name)}</span></td>
                <td><span class="process-pid">${process.parent_pid}</span></td>
                <td><span class="process-path" title="${TriageUtils.escapeHtml(process.command_line)}">
                    ${TriageUtils.truncateText(TriageUtils.escapeHtml(process.command_line), 100)}
                </span></td>
                <td><span class="process-path" title="${TriageUtils.escapeHtml(process.executable_path)}">
                    ${TriageUtils.truncateText(TriageUtils.escapeHtml(process.executable_path), 80)}
                </span></td>
                <td><span class="process-hash" title="${process.sha256_hash}">
                    ${process.sha256_hash === 'N/A' || process.sha256_hash === 'ERROR' ?
                `<span class="cell-badge neutral">${process.sha256_hash}</span>` :
                TriageUtils.truncateText(process.sha256_hash, 16)}
                </span></td>
            </tr>
        `).join('');
    }

    /**
     * Render network connections
     */
    renderNetworkConnections(connections) {
        this.filteredData.connections = connections;
        this.updateNetworkTable();
    }

    /**
     * Update network table with current filter
     */
    updateNetworkTable() {
        const tbody = document.getElementById('networkTableBody');
        const filter = document.getElementById('networkFilter').value;

        let filteredConnections = this.filteredData.connections;

        if (filter !== 'all') {
            switch (filter) {
                case 'tcp':
                    filteredConnections = filteredConnections.filter(conn => conn.protocol === 'TCP');
                    break;
                case 'udp':
                    filteredConnections = filteredConnections.filter(conn => conn.protocol === 'UDP');
                    break;
                case 'external':
                    filteredConnections = filteredConnections.filter(conn =>
                        TriageUtils.isExternalIP(conn.remote_address.split(':')[0])
                    );
                    break;
            }
        }

        tbody.innerHTML = filteredConnections.map(conn => `
            <tr>
                <td><span class="network-protocol ${conn.protocol}">${conn.protocol}</span></td>
                <td><span class="network-address">${TriageUtils.escapeHtml(conn.local_address)}</span></td>
                <td><span class="network-address">${TriageUtils.escapeHtml(conn.remote_address)}</span></td>
                <td><span class="network-state ${conn.state.replace(/[^A-Z]/g, '')}">${conn.state}</span></td>
                <td><span class="process-pid">${conn.owning_pid}</span></td>
            </tr>
        `).join('');
    }

    /**
     * Render persistence mechanisms
     */
    renderPersistenceMechanisms(mechanisms) {
        this.filteredData.persistence = mechanisms;
        this.updatePersistenceTable();
    }

    /**
     * Update persistence table with current filter
     */
    updatePersistenceTable() {
        const tbody = document.getElementById('persistenceTableBody');
        const filter = document.getElementById('persistenceFilter').value;

        let filteredMechanisms = this.filteredData.persistence;

        if (filter !== 'all') {
            switch (filter) {
                case 'registry':
                    filteredMechanisms = filteredMechanisms.filter(mech =>
                        mech.type.toLowerCase().includes('registry')
                    );
                    break;
                case 'service':
                    filteredMechanisms = filteredMechanisms.filter(mech =>
                        mech.type.toLowerCase().includes('service')
                    );
                    break;
                case 'startup':
                    filteredMechanisms = filteredMechanisms.filter(mech =>
                        mech.type.toLowerCase().includes('startup')
                    );
                    break;
            }
        }

        tbody.innerHTML = filteredMechanisms.map(mech => `
            <tr>
                <td><span class="persistence-type ${mech.type.toLowerCase().replace(/\s+/g, '')}">${TriageUtils.escapeHtml(mech.type)}</span></td>
                <td>${TriageUtils.escapeHtml(mech.name)}</td>
                <td><span class="persistence-command" title="${TriageUtils.escapeHtml(mech.command)}">
                    ${TriageUtils.truncateText(TriageUtils.escapeHtml(mech.command), 100)}
                </span></td>
                <td><span class="persistence-source" title="${TriageUtils.escapeHtml(mech.source)}">
                    ${TriageUtils.truncateText(TriageUtils.escapeHtml(mech.source), 60)}
                </span></td>
            </tr>
        `).join('');
    }

    /**
     * Render event logs
     */
    renderEventLogs(eventLogs) {
        const allEvents = [
            ...eventLogs.security.map(event => ({ ...event, source: 'Security' })),
            ...eventLogs.system.map(event => ({ ...event, source: 'System' }))
        ];

        // Sort by timestamp (newest first)
        allEvents.sort((a, b) => new Date(b.timestamp) - new Date(a.timestamp));

        this.filteredData.events = allEvents;
        this.updateEventsTable();
    }

    /**
     * Update events table with current filter
     */
    updateEventsTable() {
        const tbody = document.getElementById('eventsTableBody');
        const filter = document.getElementById('eventLogFilter').value;

        let filteredEvents = this.filteredData.events;

        if (filter !== 'all') {
            switch (filter) {
                case 'security':
                    filteredEvents = filteredEvents.filter(event => event.source === 'Security');
                    break;
                case 'system':
                    filteredEvents = filteredEvents.filter(event => event.source === 'System');
                    break;
                case 'error':
                    filteredEvents = filteredEvents.filter(event =>
                        event.level.toLowerCase().includes('error')
                    );
                    break;
                case 'warning':
                    filteredEvents = filteredEvents.filter(event =>
                        event.level.toLowerCase().includes('warning')
                    );
                    break;
            }
        }

        tbody.innerHTML = filteredEvents.map(event => `
            <tr>
                <td><span class="event-id">${event.event_id}</span></td>
                <td><span class="event-level ${event.level.replace(/\s+/g, '')}">${event.level}</span></td>
                <td><span class="event-timestamp">${TriageUtils.formatTimestamp(event.timestamp)}</span></td>
                <td><span class="event-message" title="${TriageUtils.escapeHtml(event.message)}">
                    ${TriageUtils.truncateText(TriageUtils.escapeHtml(event.message), 150)}
                </span></td>
                <td>${event.source}</td>
            </tr>
        `).join('');
    }

    /**
     * Render collection log
     */
    renderCollectionLog(collectionLog) {
        this.filteredData.logs = collectionLog;
        this.updateLogsDisplay();
    }

    /**
     * Update logs display with current filter
     */
    updateLogsDisplay() {
        const container = document.getElementById('logContainer');
        const filter = document.getElementById('logLevelFilter').value;

        let filteredLogs = this.filteredData.logs;

        if (filter !== 'all') {
            filteredLogs = filteredLogs.filter(log => log.level === filter);
        }

        container.innerHTML = filteredLogs.map(log => `
            <div class="log-entry">
                <span class="log-timestamp">${TriageUtils.formatTimestamp(log.timestamp)}</span>
                <span class="log-level ${log.level}">${log.level}</span>
                <span class="log-message">${TriageUtils.escapeHtml(log.message)}</span>
            </div>
        `).join('');

        // Scroll to bottom
        container.scrollTop = container.scrollHeight;
    }

    /**
     * Update status bar
     */
    updateStatusBar(scanResults) {
        const statusMessage = document.getElementById('statusMessage');
        const artifactCount = document.getElementById('artifactCount');

        const totalArtifacts =
            scanResults.artifacts.running_processes.length +
            scanResults.artifacts.network_connections.length +
            scanResults.artifacts.persistence_mechanisms.length +
            scanResults.artifacts.event_logs.security.length +
            scanResults.artifacts.event_logs.system.length;

        statusMessage.textContent = 'Scan results loaded';
        artifactCount.textContent = `${totalArtifacts.toLocaleString()} artifacts`;
    }

    /**
     * Enable export buttons
     */
    enableExportButtons() {
        const exportButtons = [
            'saveResultsBtn',
            'exportReportBtn',
            'exportSystemBtn',
            'exportProcessesBtn',
            'exportNetworkBtn',
            'exportPersistenceBtn',
            'exportEventsBtn',
            'exportLogsBtn'
        ];

        exportButtons.forEach(buttonId => {
            const button = document.getElementById(buttonId);
            if (button) {
                button.disabled = false;
            }
        });
    }

    /**
     * Clear all data
     */
    clearResults() {
        this.currentData = null;
        this.filteredData = {};

        // Hide results and show welcome screen
        document.getElementById('resultsContainer').style.display = 'none';
        document.getElementById('welcomeScreen').style.display = 'flex';

        // Disable export buttons
        const exportButtons = document.querySelectorAll('[id$="Btn"]');
        exportButtons.forEach(button => {
            if (button.id.includes('export') || button.id.includes('save')) {
                button.disabled = true;
            }
        });

        // Reset status bar
        document.getElementById('statusMessage').textContent = 'Ready';
        document.getElementById('artifactCount').textContent = 'No data loaded';
    }

    /**
     * Get current data
     */
    getCurrentData() {
        return this.currentData;
    }

    /**
     * Get filtered data for export
     */
    getFilteredData(type) {
        return this.filteredData[type] || [];
    }
}

// Export for use in other modules
window.DataRenderer = DataRenderer;