// Advanced Scan Configuration Manager

class ScanConfigManager {
    constructor() {
        this.presets = {
            quick: {
                name: 'Quick Scan',
                description: 'Fast scan for immediate threat assessment',
                options: {
                    collectSystem: true,
                    collectProcesses: true,
                    collectNetwork: true,
                    collectPersistence: false,
                    collectEvents: false,
                    skipHashes: true,
                    maxEvents: 100
                },
                estimatedTime: '30 seconds - 2 minutes',
                icon: '⚡'
            },
            standard: {
                name: 'Standard Scan',
                description: 'Balanced scan with most artifacts',
                options: {
                    collectSystem: true,
                    collectProcesses: true,
                    collectNetwork: true,
                    collectPersistence: true,
                    collectEvents: true,
                    skipHashes: false,
                    maxEvents: 1000
                },
                estimatedTime: '2 - 5 minutes',
                icon: '🔍'
            },
            comprehensive: {
                name: 'Comprehensive Scan',
                description: 'Complete forensic collection',
                options: {
                    collectSystem: true,
                    collectProcesses: true,
                    collectNetwork: true,
                    collectPersistence: true,
                    collectEvents: true,
                    skipHashes: false,
                    maxEvents: 5000
                },
                estimatedTime: '5 - 15 minutes',
                icon: '🔬'
            },
            incident: {
                name: 'Incident Response',
                description: 'Focused on active threats and IOCs',
                options: {
                    collectSystem: true,
                    collectProcesses: true,
                    collectNetwork: true,
                    collectPersistence: true,
                    collectEvents: true,
                    skipHashes: true,
                    maxEvents: 2000
                },
                estimatedTime: '3 - 8 minutes',
                icon: '🚨'
            },
            minimal: {
                name: 'Minimal Scan',
                description: 'System info and basic process list only',
                options: {
                    collectSystem: true,
                    collectProcesses: true,
                    collectNetwork: false,
                    collectPersistence: false,
                    collectEvents: false,
                    skipHashes: true,
                    maxEvents: 0
                },
                estimatedTime: '10 - 30 seconds',
                icon: '📋'
            }
        };
        
        this.currentConfig = this.getDefaultConfig();
        this.setupEventListeners();
    }

    /**
     * Get default configuration
     */
    getDefaultConfig() {
        return {
            preset: 'standard',
            options: { ...this.presets.standard.options },
            outputPath: '',
            autoSave: true,
            notifications: true
        };
    }

    /**
     * Setup event listeners for configuration UI
     */
    setupEventListeners() {
        // Preset selection
        document.addEventListener('change', (e) => {
            if (e.target.name === 'scanPreset') {
                this.applyPreset(e.target.value);
            }
        });

        // Advanced options toggle
        document.addEventListener('click', (e) => {
            if (e.target.id === 'toggleAdvancedOptions') {
                this.toggleAdvancedOptions();
            }
        });

        // Save/Load configuration
        document.addEventListener('click', (e) => {
            if (e.target.id === 'saveConfig') {
                this.saveConfiguration();
            } else if (e.target.id === 'loadConfig') {
                this.loadConfiguration();
            }
        });

        // Real-time validation
        document.addEventListener('input', (e) => {
            if (e.target.type === 'number' && e.target.id === 'maxEvents') {
                this.validateMaxEvents(e.target);
            }
        });
    }

    /**
     * Apply a preset configuration
     */
    applyPreset(presetName) {
        if (!this.presets[presetName]) {
            console.warn(`Unknown preset: ${presetName}`);
            return;
        }

        const preset = this.presets[presetName];
        this.currentConfig.preset = presetName;
        this.currentConfig.options = { ...preset.options };

        // Update UI
        this.updateConfigurationUI();
        this.updateEstimatedTime();
        
        // Show preset description
        this.showPresetInfo(preset);
    }

    /**
     * Update the configuration UI with current settings
     */
    updateConfigurationUI() {
        const options = this.currentConfig.options;

        // Update checkboxes
        document.getElementById('collectSystem').checked = options.collectSystem;
        document.getElementById('collectProcesses').checked = options.collectProcesses;
        document.getElementById('collectNetwork').checked = options.collectNetwork;
        document.getElementById('collectPersistence').checked = options.collectPersistence;
        document.getElementById('collectEvents').checked = options.collectEvents;
        document.getElementById('skipHashes').checked = options.skipHashes;

        // Update numeric inputs
        document.getElementById('maxEvents').value = options.maxEvents;

        // Update preset selection
        const presetRadios = document.querySelectorAll('input[name="scanPreset"]');
        presetRadios.forEach(radio => {
            radio.checked = radio.value === this.currentConfig.preset;
        });
    }

    /**
     * Show preset information
     */
    showPresetInfo(preset) {
        const infoElement = document.getElementById('presetInfo');
        if (infoElement) {
            infoElement.innerHTML = `
                <div class="preset-info">
                    <div class="preset-header">
                        <span class="preset-icon">${preset.icon}</span>
                        <span class="preset-name">${preset.name}</span>
                    </div>
                    <div class="preset-description">${preset.description}</div>
                    <div class="preset-time">Estimated time: ${preset.estimatedTime}</div>
                </div>
            `;
        }
    }

    /**
     * Update estimated scan time
     */
    updateEstimatedTime() {
        const preset = this.presets[this.currentConfig.preset];
        const timeElement = document.getElementById('estimatedTime');
        
        if (timeElement && preset) {
            timeElement.textContent = preset.estimatedTime;
        }
    }

    /**
     * Toggle advanced options visibility
     */
    toggleAdvancedOptions() {
        const advancedPanel = document.getElementById('advancedOptions');
        const toggleButton = document.getElementById('toggleAdvancedOptions');
        
        if (advancedPanel.style.display === 'none' || !advancedPanel.style.display) {
            advancedPanel.style.display = 'block';
            toggleButton.textContent = 'Hide Advanced Options';
        } else {
            advancedPanel.style.display = 'none';
            toggleButton.textContent = 'Show Advanced Options';
        }
    }

    /**
     * Validate max events input
     */
    validateMaxEvents(input) {
        const value = parseInt(input.value);
        const warningElement = document.getElementById('maxEventsWarning');
        
        if (value > 10000) {
            input.classList.add('warning');
            if (warningElement) {
                warningElement.textContent = 'High event count may significantly increase scan time';
                warningElement.style.display = 'block';
            }
        } else {
            input.classList.remove('warning');
            if (warningElement) {
                warningElement.style.display = 'none';
            }
        }
    }

    /**
     * Get current configuration for CLI execution
     */
    getCurrentConfig() {
        // Get current UI state
        const uiConfig = {
            collectSystem: document.getElementById('collectSystem').checked,
            collectProcesses: document.getElementById('collectProcesses').checked,
            collectNetwork: document.getElementById('collectNetwork').checked,
            collectPersistence: document.getElementById('collectPersistence').checked,
            collectEvents: document.getElementById('collectEvents').checked,
            skipHashes: document.getElementById('skipHashes').checked,
            maxEvents: parseInt(document.getElementById('maxEvents').value) || 1000
        };

        return {
            ...this.currentConfig,
            options: uiConfig
        };
    }

    /**
     * Save configuration to local storage
     */
    saveConfiguration() {
        try {
            const config = this.getCurrentConfig();
            localStorage.setItem('triageir-config', JSON.stringify(config));
            TriageUtils.showToast('Configuration saved', 'success');
        } catch (error) {
            console.error('Failed to save configuration:', error);
            TriageUtils.showToast('Failed to save configuration', 'error');
        }
    }

    /**
     * Load configuration from local storage
     */
    loadConfiguration() {
        try {
            const saved = localStorage.getItem('triageir-config');
            if (saved) {
                const config = JSON.parse(saved);
                this.currentConfig = { ...this.getDefaultConfig(), ...config };
                this.updateConfigurationUI();
                this.updateEstimatedTime();
                TriageUtils.showToast('Configuration loaded', 'success');
            } else {
                TriageUtils.showToast('No saved configuration found', 'info');
            }
        } catch (error) {
            console.error('Failed to load configuration:', error);
            TriageUtils.showToast('Failed to load configuration', 'error');
        }
    }

    /**
     * Export configuration to file
     */
    async exportConfiguration() {
        try {
            const config = this.getCurrentConfig();
            const configJson = JSON.stringify(config, null, 2);
            
            const result = await ipcRenderer.invoke('show-save-dialog', {
                title: 'Export Scan Configuration',
                defaultPath: 'triageir-config.json',
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ]
            });

            if (!result.canceled && result.filePath) {
                const writeResult = await ipcRenderer.invoke('write-file', result.filePath, configJson);
                if (writeResult.success) {
                    TriageUtils.showToast(`Configuration exported to ${result.filePath}`, 'success');
                } else {
                    throw new Error(writeResult.error);
                }
            }
        } catch (error) {
            console.error('Failed to export configuration:', error);
            TriageUtils.showToast(`Failed to export configuration: ${error.message}`, 'error');
        }
    }

    /**
     * Import configuration from file
     */
    async importConfiguration() {
        try {
            const result = await ipcRenderer.invoke('show-open-dialog', {
                title: 'Import Scan Configuration',
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ],
                properties: ['openFile']
            });

            if (!result.canceled && result.filePaths.length > 0) {
                const readResult = await ipcRenderer.invoke('read-file', result.filePaths[0]);
                if (readResult.success) {
                    const config = JSON.parse(readResult.data);
                    this.currentConfig = { ...this.getDefaultConfig(), ...config };
                    this.updateConfigurationUI();
                    this.updateEstimatedTime();
                    TriageUtils.showToast('Configuration imported successfully', 'success');
                } else {
                    throw new Error(readResult.error);
                }
            }
        } catch (error) {
            console.error('Failed to import configuration:', error);
            TriageUtils.showToast(`Failed to import configuration: ${error.message}`, 'error');
        }
    }

    /**
     * Reset configuration to defaults
     */
    resetConfiguration() {
        this.currentConfig = this.getDefaultConfig();
        this.updateConfigurationUI();
        this.updateEstimatedTime();
        TriageUtils.showToast('Configuration reset to defaults', 'info');
    }

    /**
     * Get scan summary for confirmation
     */
    getScanSummary() {
        const config = this.getCurrentConfig();
        const preset = this.presets[config.preset];
        
        const artifacts = [];
        if (config.options.collectSystem) artifacts.push('System Information');
        if (config.options.collectProcesses) artifacts.push('Running Processes');
        if (config.options.collectNetwork) artifacts.push('Network Connections');
        if (config.options.collectPersistence) artifacts.push('Persistence Mechanisms');
        if (config.options.collectEvents) artifacts.push('Event Logs');

        return {
            preset: preset.name,
            description: preset.description,
            estimatedTime: preset.estimatedTime,
            artifacts: artifacts,
            options: {
                skipHashes: config.options.skipHashes,
                maxEvents: config.options.maxEvents
            }
        };
    }

    /**
     * Validate configuration before scan
     */
    validateConfiguration() {
        const config = this.getCurrentConfig();
        const issues = [];

        // Check if at least one artifact type is selected
        const hasArtifacts = Object.values(config.options).some(value => 
            typeof value === 'boolean' && value === true
        );

        if (!hasArtifacts) {
            issues.push('At least one artifact type must be selected');
        }

        // Check max events value
        if (config.options.maxEvents < 0) {
            issues.push('Max events cannot be negative');
        }

        if (config.options.maxEvents > 50000) {
            issues.push('Max events is very high and may cause performance issues');
        }

        return {
            valid: issues.length === 0,
            issues: issues
        };
    }

    /**
     * Create CLI options from current configuration
     */
    createCLIOptions() {
        const config = this.getCurrentConfig();
        return CLIManager.createScanOptions({
            verbose: true,
            ...config.options
        });
    }
}

// Export for use in other modules
window.ScanConfigManager = ScanConfigManager;