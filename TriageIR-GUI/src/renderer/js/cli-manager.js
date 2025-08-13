// CLI Manager for executing TriageIR CLI and handling results

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const { ipcRenderer } = require('electron');

class CLIManager {
    constructor() {
        this.currentProcess = null;
        this.isRunning = false;
        this.cliPath = null;
        this.onProgress = null;
        this.onComplete = null;
        this.onError = null;
        
        this.findCLIExecutable();
    }

    /**
     * Find the TriageIR CLI executable
     */
    async findCLIExecutable() {
        const possiblePaths = [
            // Same directory as GUI
            path.join(process.cwd(), 'triageir-cli.exe'),
            // Development path
            path.join(process.cwd(), '..', 'TriageIR-CLI', 'target', 'release', 'triageir-cli.exe'),
            // Resource path (packaged app)
            path.join(process.resourcesPath, 'triageir-cli.exe'),
            // Current directory
            './triageir-cli.exe',
            // Just the executable name (in PATH)
            'triageir-cli.exe'
        ];

        for (const cliPath of possiblePaths) {
            try {
                if (await this.checkFileExists(cliPath)) {
                    this.cliPath = cliPath;
                    console.log(`Found CLI at: ${cliPath}`);
                    return;
                }
            } catch (error) {
                // Continue checking other paths
            }
        }

        // If not found in specific paths, try just the executable name
        this.cliPath = 'triageir-cli.exe';
        console.warn('CLI executable not found in expected locations, will try system PATH');
    }

    /**
     * Check if file exists
     */
    async checkFileExists(filePath) {
        try {
            const exists = await ipcRenderer.invoke('check-file-exists', filePath);
            return exists;
        } catch (error) {
            return false;
        }
    }

    /**
     * Execute a scan with the specified options
     */
    async executeScan(options = {}) {
        if (this.isRunning) {
            throw new Error('A scan is already running');
        }

        if (!this.cliPath) {
            throw new Error('CLI executable not found');
        }

        return new Promise((resolve, reject) => {
            this.isRunning = true;
            
            // Build command arguments
            const args = ['--format', 'json'];
            
            if (options.verbose) {
                args.push('--verbose');
            }
            
            if (options.skipHashes) {
                args.push('--skip-hashes');
            }
            
            if (options.skipEvents) {
                args.push('--skip-events');
            }
            
            if (options.maxEvents && options.maxEvents !== 1000) {
                args.push('--max-events', options.maxEvents.toString());
            }
            
            if (options.only && options.only.length > 0) {
                args.push('--only', options.only.join(','));
            }
            
            if (options.outputFile) {
                args.push('--output', options.outputFile);
            }

            console.log(`Executing: ${this.cliPath} ${args.join(' ')}`);

            // Spawn the CLI process
            this.currentProcess = spawn(this.cliPath, args, {
                stdio: ['pipe', 'pipe', 'pipe'],
                shell: true
            });

            let stdout = '';
            let stderr = '';
            let lastProgressUpdate = Date.now();

            // Handle stdout data
            this.currentProcess.stdout.on('data', (data) => {
                stdout += data.toString();
                
                // Throttle progress updates
                const now = Date.now();
                if (now - lastProgressUpdate > 500) {
                    this.emitProgress('Collecting data...', null);
                    lastProgressUpdate = now;
                }
            });

            // Handle stderr data (verbose output)
            this.currentProcess.stderr.on('data', (data) => {
                stderr += data.toString();
                
                // Parse progress from stderr if verbose
                if (options.verbose) {
                    const lines = data.toString().split('\n');
                    for (const line of lines) {
                        if (line.trim()) {
                            this.emitProgress(line.trim(), null);
                        }
                    }
                }
            });

            // Handle process completion
            this.currentProcess.on('close', (code) => {
                this.isRunning = false;
                this.currentProcess = null;

                if (code === 0) {
                    try {
                        let results;
                        
                        if (options.outputFile) {
                            // Read from file
                            results = { outputFile: options.outputFile };
                        } else {
                            // Parse and validate JSON from stdout
                            if (!stdout.trim()) {
                                throw new Error('No output received from CLI');
                            }
                            
                            const validator = new DataValidator();
                            const parseResult = validator.parseAndValidate(stdout);
                            
                            if (!parseResult.success) {
                                throw new Error(parseResult.error);
                            }
                            
                            // Check for validation errors
                            if (!parseResult.validation.valid) {
                                const errorMessages = parseResult.validation.errors.map(e => e.message).join('; ');
                                console.warn('CLI output validation issues:', parseResult.validation);
                                
                                if (parseResult.validation.summary.errorCount > 0) {
                                    throw new Error(`CLI output validation failed: ${errorMessages}`);
                                }
                            }
                            
                            // Sanitize and return data
                            results = {
                                data: validator.sanitizeData(parseResult.data),
                                validation: parseResult.validation
                            };
                        }
                        
                        this.emitComplete(results);
                        resolve(results);
                    } catch (error) {
                        const errorMsg = `Failed to parse CLI output: ${error.message}`;
                        this.emitError(errorMsg);
                        reject(new Error(errorMsg));
                    }
                } else {
                    const errorMsg = `CLI process exited with code ${code}${stderr ? ': ' + stderr : ''}`;
                    this.emitError(errorMsg);
                    reject(new Error(errorMsg));
                }
            });

            // Handle process errors
            this.currentProcess.on('error', (error) => {
                this.isRunning = false;
                this.currentProcess = null;
                
                let errorMsg = `Failed to start CLI process: ${error.message}`;
                
                if (error.code === 'ENOENT') {
                    errorMsg = 'TriageIR CLI executable not found. Please ensure it is installed and accessible.';
                }
                
                this.emitError(errorMsg);
                reject(new Error(errorMsg));
            });

            // Initial progress
            this.emitProgress('Starting scan...', 0);
        });
    }

    /**
     * Stop the current scan
     */
    stopScan() {
        if (this.currentProcess && this.isRunning) {
            this.currentProcess.kill('SIGTERM');
            this.isRunning = false;
            this.currentProcess = null;
            this.emitProgress('Scan stopped by user', null);
            return true;
        }
        return false;
    }

    /**
     * Check if CLI is available
     */
    async checkCLIAvailable() {
        if (!this.cliPath) {
            return false;
        }

        return new Promise((resolve) => {
            const testProcess = spawn(this.cliPath, ['--version'], {
                stdio: ['pipe', 'pipe', 'pipe'],
                shell: true
            });

            let hasOutput = false;

            testProcess.stdout.on('data', () => {
                hasOutput = true;
            });

            testProcess.on('close', (code) => {
                resolve(code === 0 && hasOutput);
            });

            testProcess.on('error', () => {
                resolve(false);
            });

            // Timeout after 5 seconds
            setTimeout(() => {
                testProcess.kill();
                resolve(false);
            }, 5000);
        });
    }

    /**
     * Get CLI version
     */
    async getCLIVersion() {
        if (!this.cliPath) {
            return null;
        }

        return new Promise((resolve) => {
            const versionProcess = spawn(this.cliPath, ['--version'], {
                stdio: ['pipe', 'pipe', 'pipe'],
                shell: true
            });

            let output = '';

            versionProcess.stdout.on('data', (data) => {
                output += data.toString();
            });

            versionProcess.on('close', (code) => {
                if (code === 0 && output.trim()) {
                    // Extract version from output
                    const match = output.match(/(\d+\.\d+\.\d+)/);
                    resolve(match ? match[1] : output.trim());
                } else {
                    resolve(null);
                }
            });

            versionProcess.on('error', () => {
                resolve(null);
            });

            // Timeout after 3 seconds
            setTimeout(() => {
                versionProcess.kill();
                resolve(null);
            }, 3000);
        });
    }

    /**
     * Load results from a file
     */
    async loadResultsFromFile(filePath) {
        try {
            const result = await ipcRenderer.invoke('read-file', filePath);
            if (!result.success) {
                throw new Error(result.error);
            }

            // Use DataValidator to parse and validate
            const validator = new DataValidator();
            const parseResult = validator.parseAndValidate(result.data);
            
            if (!parseResult.success) {
                throw new Error(parseResult.error);
            }

            // Check validation results
            const validation = parseResult.validation;
            if (!validation.valid) {
                const errorMessages = validation.errors.map(e => e.message).join('; ');
                console.warn('Data validation issues:', validation);
                
                // Still allow loading with warnings, but show them to user
                if (validation.summary.errorCount > 0) {
                    throw new Error(`Data validation failed: ${errorMessages}`);
                }
            }

            // Log warnings if any
            if (validation.warnings.length > 0) {
                console.warn('Data validation warnings:', validation.warnings);
            }

            // Sanitize data for safe display
            const sanitizedData = validator.sanitizeData(parseResult.data);
            
            return {
                data: sanitizedData,
                validation: validation
            };
            
        } catch (error) {
            throw new Error(`Failed to load results: ${error.message}`);
        }
    }

    /**
     * Save results to a file
     */
    async saveResultsToFile(results, filePath) {
        try {
            const jsonData = JSON.stringify(results, null, 2);
            const result = await ipcRenderer.invoke('write-file', filePath, jsonData);
            
            if (!result.success) {
                throw new Error(result.error);
            }

            return true;
        } catch (error) {
            throw new Error(`Failed to save results: ${error.message}`);
        }
    }

    /**
     * Set progress callback
     */
    setProgressCallback(callback) {
        this.onProgress = callback;
    }

    /**
     * Set completion callback
     */
    setCompleteCallback(callback) {
        this.onComplete = callback;
    }

    /**
     * Set error callback
     */
    setErrorCallback(callback) {
        this.onError = callback;
    }

    /**
     * Emit progress event
     */
    emitProgress(message, progress) {
        if (this.onProgress) {
            this.onProgress(message, progress);
        }
    }

    /**
     * Emit completion event
     */
    emitComplete(results) {
        if (this.onComplete) {
            this.onComplete(results);
        }
    }

    /**
     * Emit error event
     */
    emitError(error) {
        if (this.onError) {
            this.onError(error);
        }
    }

    /**
     * Get scan status
     */
    getStatus() {
        return {
            isRunning: this.isRunning,
            cliPath: this.cliPath,
            hasProcess: !!this.currentProcess
        };
    }

    /**
     * Create scan options from UI form
     */
    static createScanOptions(formData) {
        const options = {
            verbose: formData.verbose || false,
            skipHashes: formData.skipHashes || false,
            skipEvents: formData.skipEvents || false,
            maxEvents: parseInt(formData.maxEvents) || 1000,
            only: [],
            outputFile: formData.outputFile || null
        };

        // Build artifact filter
        if (formData.collectSystem) options.only.push('system');
        if (formData.collectProcesses) options.only.push('processes');
        if (formData.collectNetwork) options.only.push('network');
        if (formData.collectPersistence) options.only.push('persistence');
        if (formData.collectEvents) options.only.push('events');

        // If all are selected, don't use the filter
        if (options.only.length === 5) {
            options.only = [];
        }

        return options;
    }
}

// Export for use in other modules
window.CLIManager = CLIManager;