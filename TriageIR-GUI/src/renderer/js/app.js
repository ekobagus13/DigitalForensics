// Main application logic for TriageIR GUI

const { ipcRenderer } = require('electron');

class TriageApp {
    constructor() {
        this.cliManager = new CLIManager();
        this.dataRenderer = new DataRenderer();
        this.exportManager = new ExportManager();
        this.scanConfigManager = new ScanConfigManager();
        this.currentScanData = null;
        this.isScanning = false;
        
        this.initializeApp();
    }

    /**
     * Initialize the application
     */
    async initializeApp() {
        console.log('Initializing TriageIR GUI...');
        
        // Set up event listeners
        this.setupEventListeners();
        
        // Set up CLI manager callbacks
        this.setupCLICallbacks();
        
        // Check CLI availability
        await this.checkCLIStatus();
        
        // Set up IPC listeners
        this.setupIPCListeners();
        
        // Initialize UI state
        this.initializeUI();
        
        console.log('TriageIR GUI initialized successfully');
    }

    /**
     * Set up event listeners
     */
    setupEventListeners() {
        // Scan control buttons
        document.getElementById('startScanBtn').addEventListener('click', () => this.startFullScan());
        document.getElementById('quickScanBtn').addEventListener('click', () => this.startQuickScan());
        document.getElementById('customScanBtn').addEventListener('click', () => this.showCustomScanOptions());
        document.getElementById('startCustomScanBtn').addEventListener('click', () => this.startCustomScan());
        document.getElementById('cancelCustomScanBtn').addEventListener('click', () => this.hideCustomScanOptions());
        document.getElementById('stopScanBtn').addEventListener('click', () => this.stopScan());

        // Welcome screen buttons
        document.getElementById('welcomeStartScan').addEventListener('click', () => this.startFullScan());
        document.getElementById('welcomeOpenFile').addEventListener('click', () => this.openFile());

        // File operations
        document.getElementById('openFileBtn').addEventListener('click', () => this.openFile());
        document.getElementById('saveResultsBtn').addEventListener('click', () => this.saveResults());
        document.getElementById('exportReportBtn').addEventListener('click', () => this.showExportOptions());

        // Tab navigation
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', (e) => this.switchTab(e.target.dataset.tab));
        });

        // Filter inputs
        document.getElementById('processFilter').addEventListener('input', 
            TriageUtils.debounce(() => this.dataRenderer.updateProcessesTable(), 300));
        document.getElementById('networkFilter').addEventListener('change', () => this.dataRenderer.updateNetworkTable());
        document.getElementById('persistenceFilter').addEventListener('change', () => this.dataRenderer.updatePersistenceTable());
        document.getElementById('eventLogFilter').addEventListener('change', () => this.dataRenderer.updateEventsTable());
        document.getElementById('logLevelFilter').addEventListener('change', () => this.dataRenderer.updateLogsDisplay());

        // Export buttons
        document.getElementById('exportSystemBtn').addEventListener('click', () => this.exportSystemInfo());
        document.getElementById('exportProcessesBtn').addEventListener('click', () => this.exportProcesses());
        document.getElementById('exportNetworkBtn').addEventListener('click', () => this.exportNetwork());
        document.getElementById('exportPersistenceBtn').addEventListener('click', () => this.exportPersistence());
        document.getElementById('exportEventsBtn').addEventListener('click', () => this.exportEvents());
        document.getElementById('exportLogsBtn').addEventListener('click', () => this.exportLogs());

        // Configuration management
        document.getElementById('resetConfig').addEventListener('click', () => this.resetScanConfiguration());
        document.getElementById('browseOutput').addEventListener('click', () => this.browseOutputDirectory());

        // Modal controls
        document.getElementById('aboutModalClose').addEventListener('click', () => this.hideAboutModal());
        document.getElementById('aboutModal').addEventListener('click', (e) => {
            if (e.target.id === 'aboutModal') this.hideAboutModal();
        });

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => this.handleKeyboardShortcuts(e));
    }

    /**
     * Set up CLI manager callbacks
     */
    setupCLICallbacks() {
        this.cliManager.setProgressCallback((message, progress) => {
            this.updateScanProgress(message, progress);
        });

        this.cliManager.setCompleteCallback((results) => {
            this.handleScanComplete(results);
        });

        this.cliManager.setErrorCallback((error) => {
            this.handleScanError(error);
        });
    }

    /**
     * Set up IPC listeners for menu events
     */
    setupIPCListeners() {
        ipcRenderer.on('menu-new-scan', () => this.startFullScan());
        ipcRenderer.on('menu-open-file', (event, filePath) => this.openFileFromPath(filePath));
        ipcRenderer.on('menu-export-report', () => this.showExportOptions());
        ipcRenderer.on('menu-start-scan', (event, type) => {
            if (type === 'full') this.startFullScan();
            else if (type === 'quick') this.startQuickScan();
        });
        ipcRenderer.on('menu-custom-scan', () => this.showCustomScanOptions());
        ipcRenderer.on('menu-stop-scan', () => this.stopScan());
        ipcRenderer.on('menu-view-tab', (event, tab) => this.switchTab(tab));
        ipcRenderer.on('menu-show-shortcuts', () => this.showKeyboardShortcuts());
        ipcRenderer.on('menu-show-about', () => this.showAboutModal());
    }

    /**
     * Initialize UI state
     */
    async initializeUI() {
        // Set app version
        try {
            const version = await ipcRenderer.invoke('get-app-version');
            document.getElementById('appVersion').textContent = `v${version}`;
            document.getElementById('aboutVersion').textContent = version;
        } catch (error) {
            console.warn('Could not get app version:', error);
        }

        // Set initial status
        this.updateStatus('Ready');
        
        // Hide progress container
        document.getElementById('progressContainer').style.display = 'none';
        
        // Show welcome screen
        document.getElementById('welcomeScreen').style.display = 'flex';
        document.getElementById('resultsContainer').style.display = 'none';
    }

    /**
     * Check CLI status
     */
    async checkCLIStatus() {
        try {
            const isAvailable = await this.cliManager.checkCLIAvailable();
            if (isAvailable) {
                const version = await this.cliManager.getCLIVersion();
                console.log(`CLI available, version: ${version || 'unknown'}`);
                this.updateStatus('Ready - CLI available');
            } else {
                console.warn('CLI not available');
                this.updateStatus('Warning - CLI not found');
                TriageUtils.showToast('TriageIR CLI not found. Some features may not work.', 'warning', 5000);
            }
        } catch (error) {
            console.error('Error checking CLI status:', error);
            this.updateStatus('Error - CLI check failed');
        }
    }

    /**
     * Start full scan
     */
    async startFullScan() {
        const options = {
            verbose: true,
            skipHashes: false,
            skipEvents: false,
            maxEvents: 1000,
            only: []
        };

        await this.executeScan(options, 'Full Scan');
    }

    /**
     * Start quick scan
     */
    async startQuickScan() {
        const options = {
            verbose: true,
            skipHashes: true,
            skipEvents: true,
            maxEvents: 100,
            only: ['system', 'processes', 'network']
        };

        await this.executeScan(options, 'Quick Scan');
    }

    /**
     * Show custom scan options
     */
    showCustomScanOptions() {
        document.getElementById('scanOptions').style.display = 'block';
    }

    /**
     * Hide custom scan options
     */
    hideCustomScanOptions() {
        document.getElementById('scanOptions').style.display = 'none';
    }

    /**
     * Start custom scan
     */
    async startCustomScan() {
        // Validate configuration
        const validation = this.scanConfigManager.validateConfiguration();
        if (!validation.valid) {
            const issues = validation.issues.join('\n');
            TriageUtils.showToast(`Configuration issues:\n${issues}`, 'error', 5000);
            return;
        }

        // Get scan summary for confirmation
        const summary = this.scanConfigManager.getScanSummary();
        const confirmMessage = `
Start ${summary.preset}?

${summary.description}

Artifacts to collect:
${summary.artifacts.map(a => `• ${a}`).join('\n')}

Estimated time: ${summary.estimatedTime}

${summary.options.skipHashes ? '⚡ Fast mode (no hashes)' : '🔍 Full analysis (with hashes)'}
${summary.options.maxEvents > 0 ? `📊 Max ${summary.options.maxEvents} event log entries` : '🚫 No event logs'}
        `.trim();

        const confirmed = await ipcRenderer.invoke('show-message-box', {
            type: 'question',
            title: 'Confirm Scan Configuration',
            message: confirmMessage,
            buttons: ['Start Scan', 'Cancel'],
            defaultId: 0,
            cancelId: 1
        });

        if (confirmed.response === 0) {
            const options = this.scanConfigManager.createCLIOptions();
            this.hideCustomScanOptions();
            await this.executeScan(options, summary.preset);
        }
    }

    /**
     * Execute scan with options
     */
    async executeScan(options, scanType) {
        if (this.isScanning) {
            TriageUtils.showToast('A scan is already running', 'warning');
            return;
        }

        try {
            this.isScanning = true;
            this.showScanProgress();
            this.updateStatus(`Running ${scanType}...`);
            this.updateScanProgress(`Starting ${scanType}...`, 0);

            console.log(`Starting ${scanType} with options:`, options);
            
            const results = await this.cliManager.executeScan(options);
            console.log('Scan completed successfully');
            
        } catch (error) {
            console.error('Scan failed:', error);
            this.handleScanError(error.message);
        }
    }

    /**
     * Stop current scan
     */
    stopScan() {
        if (this.isScanning) {
            const stopped = this.cliManager.stopScan();
            if (stopped) {
                this.isScanning = false;
                this.hideScanProgress();
                this.updateStatus('Scan stopped by user');
                TriageUtils.showToast('Scan stopped', 'info');
            }
        }
    }

    /**
     * Handle scan completion
     */
    async handleScanComplete(results) {
        this.isScanning = false;
        this.hideScanProgress();
        
        try {
            let scanData = results;
            let validation = null;
            
            // If results contain outputFile, load from file
            if (results.outputFile) {
                const loadResult = await this.cliManager.loadResultsFromFile(results.outputFile);
                scanData = loadResult.data;
                validation = loadResult.validation;
            } else if (results.data) {
                // Direct data from CLI with validation
                scanData = results.data;
                validation = results.validation;
            }
            
            this.currentScanData = scanData;
            this.exportManager.setData(scanData);
            this.dataRenderer.renderResults(scanData);
            
            // Show validation warnings if any
            if (validation && validation.warnings.length > 0) {
                const warningCount = validation.warnings.length;
                TriageUtils.showToast(`Scan completed with ${warningCount} validation warnings`, 'warning', 5000);
                console.warn('Validation warnings:', validation.warnings);
            } else {
                TriageUtils.showToast('Scan completed successfully', 'success');
            }
            
            this.updateStatus('Scan completed successfully');
            
            // Switch to overview tab
            this.switchTab('overview');
            
        } catch (error) {
            console.error('Error processing scan results:', error);
            this.handleScanError(`Failed to process results: ${error.message}`);
        }
    }

    /**
     * Handle scan error
     */
    handleScanError(error) {
        this.isScanning = false;
        this.hideScanProgress();
        this.updateStatus('Scan failed');
        
        console.error('Scan error:', error);
        TriageUtils.showToast(`Scan failed: ${error}`, 'error', 10000);
    }

    /**
     * Show scan progress
     */
    showScanProgress() {
        document.getElementById('progressContainer').style.display = 'block';
        document.getElementById('welcomeScreen').style.display = 'none';
        document.getElementById('resultsContainer').style.display = 'none';
    }

    /**
     * Hide scan progress
     */
    hideScanProgress() {
        document.getElementById('progressContainer').style.display = 'none';
        
        if (this.currentScanData) {
            document.getElementById('resultsContainer').style.display = 'flex';
        } else {
            document.getElementById('welcomeScreen').style.display = 'flex';
        }
    }

    /**
     * Update scan progress
     */
    updateScanProgress(message, progress) {
        document.getElementById('progressText').textContent = message;
        
        if (progress !== null) {
            document.getElementById('progressFill').style.width = `${progress}%`;
        } else {
            // Indeterminate progress
            const fill = document.getElementById('progressFill');
            if (!fill.style.animation) {
                fill.style.animation = 'pulse 2s ease-in-out infinite';
            }
        }
    }

    /**
     * Open file dialog
     */
    async openFile() {
        try {
            const result = await ipcRenderer.invoke('show-open-dialog', {
                title: 'Open TriageIR Results',
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ],
                properties: ['openFile']
            });

            if (!result.canceled && result.filePaths.length > 0) {
                await this.openFileFromPath(result.filePaths[0]);
            }
        } catch (error) {
            console.error('Error opening file:', error);
            TriageUtils.showToast(`Failed to open file: ${error.message}`, 'error');
        }
    }

    /**
     * Open file from path
     */
    async openFileFromPath(filePath) {
        try {
            this.updateStatus('Loading file...');
            
            const loadResult = await this.cliManager.loadResultsFromFile(filePath);
            const scanData = loadResult.data;
            const validation = loadResult.validation;
            
            this.currentScanData = scanData;
            this.exportManager.setData(scanData);
            this.dataRenderer.renderResults(scanData);
            
            // Show validation results
            if (validation) {
                if (validation.warnings.length > 0) {
                    const warningCount = validation.warnings.length;
                    TriageUtils.showToast(`File loaded with ${warningCount} validation warnings`, 'warning', 5000);
                    console.warn('File validation warnings:', validation.warnings);
                } else {
                    TriageUtils.showToast('File loaded successfully', 'success');
                }
                
                // Show validation report if there are issues
                if (validation.summary.totalIssues > 0) {
                    console.log('Validation Report:', new DataValidator().generateValidationReport(validation));
                }
            } else {
                TriageUtils.showToast('File loaded successfully', 'success');
            }
            
            this.updateStatus('File loaded successfully');
            
            // Switch to overview tab
            this.switchTab('overview');
            
        } catch (error) {
            console.error('Error loading file:', error);
            this.updateStatus('Failed to load file');
            TriageUtils.showToast(`Failed to load file: ${error.message}`, 'error');
        }
    }

    /**
     * Save current results
     */
    async saveResults() {
        if (!this.currentScanData) {
            TriageUtils.showToast('No data to save', 'warning');
            return;
        }

        try {
            const filePath = await this.exportManager.exportJSON(this.currentScanData);
            if (filePath) {
                TriageUtils.showToast(`Results saved to ${filePath}`, 'success');
            }
        } catch (error) {
            console.error('Error saving results:', error);
            TriageUtils.showToast(`Failed to save results: ${error.message}`, 'error');
        }
    }

    /**
     * Show export options
     */
    async showExportOptions() {
        if (!this.currentScanData) {
            TriageUtils.showToast('No data to export', 'warning');
            return;
        }

        const result = await ipcRenderer.invoke('show-message-box', {
            type: 'question',
            title: 'Export Report',
            message: 'Choose export format:',
            buttons: ['HTML Report', 'Text Report', 'JSON Data', 'Cancel'],
            defaultId: 0,
            cancelId: 3
        });

        try {
            let filePath = null;
            
            switch (result.response) {
                case 0: // HTML Report
                    filePath = await this.exportManager.generateHTMLReport();
                    break;
                case 1: // Text Report
                    filePath = await this.exportManager.generateTextReport();
                    break;
                case 2: // JSON Data
                    filePath = await this.exportManager.exportJSON();
                    break;
                case 3: // Cancel
                    return;
            }

            if (filePath) {
                TriageUtils.showToast(`Report exported to ${filePath}`, 'success');
            }
        } catch (error) {
            console.error('Error exporting report:', error);
            TriageUtils.showToast(`Failed to export report: ${error.message}`, 'error');
        }
    }

    /**
     * Export system information
     */
    async exportSystemInfo() {
        if (!this.currentScanData) return;
        
        try {
            const data = [
                ['Hostname', this.currentScanData.scan_metadata.hostname],
                ['OS Version', this.currentScanData.scan_metadata.os_version],
                ['Uptime', TriageUtils.formatUptime(this.currentScanData.artifacts.system_info.uptime_secs)],
                ['Scan Time', this.currentScanData.scan_metadata.scan_start_utc],
                ['Duration', TriageUtils.formatDuration(this.currentScanData.scan_metadata.scan_duration_ms)]
            ];
            
            const filePath = await this.exportManager.exportCSV(data, 'system-info.csv', ['Property', 'Value']);
            if (filePath) {
                TriageUtils.showToast(`System info exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Export processes
     */
    async exportProcesses() {
        if (!this.currentScanData) return;
        
        try {
            const filePath = await this.exportManager.exportProcessesCSV(
                this.dataRenderer.getFilteredData('processes') || this.currentScanData.artifacts.running_processes
            );
            if (filePath) {
                TriageUtils.showToast(`Processes exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Export network connections
     */
    async exportNetwork() {
        if (!this.currentScanData) return;
        
        try {
            const filePath = await this.exportManager.exportNetworkCSV(
                this.dataRenderer.getFilteredData('connections') || this.currentScanData.artifacts.network_connections
            );
            if (filePath) {
                TriageUtils.showToast(`Network connections exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Export persistence mechanisms
     */
    async exportPersistence() {
        if (!this.currentScanData) return;
        
        try {
            const filePath = await this.exportManager.exportPersistenceCSV(
                this.dataRenderer.getFilteredData('persistence') || this.currentScanData.artifacts.persistence_mechanisms
            );
            if (filePath) {
                TriageUtils.showToast(`Persistence mechanisms exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Export event logs
     */
    async exportEvents() {
        if (!this.currentScanData) return;
        
        try {
            const filePath = await this.exportManager.exportEventsCSV(
                this.dataRenderer.getFilteredData('events') || []
            );
            if (filePath) {
                TriageUtils.showToast(`Event logs exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Export collection logs
     */
    async exportLogs() {
        if (!this.currentScanData) return;
        
        try {
            const filePath = await this.exportManager.exportLogsCSV(
                this.dataRenderer.getFilteredData('logs') || this.currentScanData.collection_log
            );
            if (filePath) {
                TriageUtils.showToast(`Collection log exported to ${filePath}`, 'success');
            }
        } catch (error) {
            TriageUtils.showToast(`Export failed: ${error.message}`, 'error');
        }
    }

    /**
     * Switch to a different tab
     */
    switchTab(tabName) {
        // Update tab buttons
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.classList.remove('active');
        });
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

        // Update tab content
        document.querySelectorAll('.tab-pane').forEach(pane => {
            pane.classList.remove('active');
        });
        document.getElementById(`${tabName}Tab`).classList.add('active');
    }

    /**
     * Update status message
     */
    updateStatus(message) {
        document.getElementById('statusMessage').textContent = message;
        document.getElementById('statusText').textContent = message;
        
        // Update status indicator
        const indicator = document.getElementById('statusIndicator');
        indicator.className = 'status-indicator';
        
        if (message.includes('Error') || message.includes('failed')) {
            indicator.classList.add('error');
        } else if (message.includes('Warning') || message.includes('stopped')) {
            indicator.classList.add('warning');
        } else if (message.includes('Running') || message.includes('Starting')) {
            indicator.classList.add('scanning');
        }
    }

    /**
     * Show about modal
     */
    showAboutModal() {
        document.getElementById('aboutModal').style.display = 'flex';
    }

    /**
     * Hide about modal
     */
    hideAboutModal() {
        document.getElementById('aboutModal').style.display = 'none';
    }

    /**
     * Show keyboard shortcuts
     */
    showKeyboardShortcuts() {
        const shortcuts = `
Keyboard Shortcuts:

Ctrl+N - New Scan
Ctrl+O - Open Results
Ctrl+S - Save Results
Ctrl+E - Export Report
F5 - Start Full Scan
Shift+F5 - Start Quick Scan
Escape - Stop Scan
Ctrl+1-5 - Switch Tabs
Ctrl+L - Collection Log
F12 - Developer Tools
        `;
        
        ipcRenderer.invoke('show-message-box', {
            type: 'info',
            title: 'Keyboard Shortcuts',
            message: shortcuts.trim(),
            buttons: ['OK']
        });
    }

    /**
     * Reset scan configuration to defaults
     */
    resetScanConfiguration() {
        this.scanConfigManager.resetConfiguration();
    }

    /**
     * Browse for output directory
     */
    async browseOutputDirectory() {
        try {
            const result = await ipcRenderer.invoke('show-open-dialog', {
                title: 'Select Output Directory',
                properties: ['openDirectory', 'createDirectory']
            });

            if (!result.canceled && result.filePaths.length > 0) {
                document.getElementById('outputPath').value = result.filePaths[0];
            }
        } catch (error) {
            console.error('Error browsing for directory:', error);
            TriageUtils.showToast('Failed to browse for directory', 'error');
        }
    }

    /**
     * Handle keyboard shortcuts
     */
    handleKeyboardShortcuts(event) {
        if (event.ctrlKey || event.metaKey) {
            switch (event.key) {
                case 'n':
                    event.preventDefault();
                    this.startFullScan();
                    break;
                case 'o':
                    event.preventDefault();
                    this.openFile();
                    break;
                case 's':
                    event.preventDefault();
                    this.saveResults();
                    break;
                case 'e':
                    event.preventDefault();
                    this.showExportOptions();
                    break;
                case '1':
                    event.preventDefault();
                    this.switchTab('overview');
                    break;
                case '2':
                    event.preventDefault();
                    this.switchTab('system');
                    break;
                case '3':
                    event.preventDefault();
                    this.switchTab('processes');
                    break;
                case '4':
                    event.preventDefault();
                    this.switchTab('network');
                    break;
                case '5':
                    event.preventDefault();
                    this.switchTab('persistence');
                    break;
                case 'l':
                    event.preventDefault();
                    this.switchTab('logs');
                    break;
            }
        } else {
            switch (event.key) {
                case 'F5':
                    event.preventDefault();
                    if (event.shiftKey) {
                        this.startQuickScan();
                    } else {
                        this.startFullScan();
                    }
                    break;
                case 'Escape':
                    event.preventDefault();
                    this.stopScan();
                    break;
            }
        }
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.triageApp = new TriageApp();
});