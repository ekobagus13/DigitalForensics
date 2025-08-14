/**
 * Comprehensive GUI testing suite for TriageIR-GUI
 * Tests the Electron application functionality, CLI integration, and user interface
 */

const { Application } = require('spectron');
const { expect } = require('chai');
const path = require('path');
const fs = require('fs');
const { spawn } = require('child_process');

describe('TriageIR GUI Application', function() {
    this.timeout(30000); // 30 second timeout for GUI tests

    let app;
    const testDataDir = path.join(__dirname, 'test-data');
    const mockCliPath = path.join(__dirname, 'mock-cli.js');

    before(async function() {
        // Create test data directory
        if (!fs.existsSync(testDataDir)) {
            fs.mkdirSync(testDataDir, { recursive: true });
        }

        // Create mock CLI for testing
        createMockCli();
    });

    beforeEach(async function() {
        app = new Application({
            path: require('electron'),
            args: [path.join(__dirname, '..', 'src', 'main.js')],
            env: {
                NODE_ENV: 'test',
                TRIAGEIR_CLI_PATH: mockCliPath
            }
        });

        await app.start();
        await app.client.waitUntilWindowLoaded();
    });

    afterEach(async function() {
        if (app && app.isRunning()) {
            await app.stop();
        }
    });

    after(function() {
        // Cleanup test files
        if (fs.existsSync(testDataDir)) {
            fs.rmSync(testDataDir, { recursive: true, force: true });
        }
        if (fs.existsSync(mockCliPath)) {
            fs.unlinkSync(mockCliPath);
        }
    });

    describe('Application Initialization', function() {
        it('should start the application successfully', async function() {
            expect(await app.client.getWindowCount()).to.equal(1);
            expect(await app.client.getTitle()).to.equal('TriageIR - Digital Forensics Triage Tool');
        });

        it('should display the main interface elements', async function() {
            // Check for main UI components
            expect(await app.client.$('#scan-config-panel').isExisting()).to.be.true;
            expect(await app.client.$('#results-panel').isExisting()).to.be.true;
            expect(await app.client.$('#start-scan-btn').isExisting()).to.be.true;
            expect(await app.client.$('#progress-indicator').isExisting()).to.be.true;
        });

        it('should have proper window properties', async function() {
            const bounds = await app.browserWindow.getBounds();
            expect(bounds.width).to.be.at.least(800);
            expect(bounds.height).to.be.at.least(600);
            
            expect(await app.browserWindow.isMinimized()).to.be.false;
            expect(await app.browserWindow.isVisible()).to.be.true;
        });
    });

    describe('Scan Configuration', function() {
        it('should display scan configuration options', async function() {
            const configPanel = await app.client.$('#scan-config-panel');
            expect(await configPanel.isDisplayed()).to.be.true;

            // Check for configuration options
            expect(await app.client.$('#verbose-checkbox').isExisting()).to.be.true;
            expect(await app.client.$('#output-path-input').isExisting()).to.be.true;
            expect(await app.client.$('#format-select').isExisting()).to.be.true;
        });

        it('should validate configuration inputs', async function() {
            // Test invalid output path
            await app.client.$('#output-path-input').setValue('/invalid/path/that/does/not/exist');
            await app.client.$('#start-scan-btn').click();
            
            // Should show validation error
            const errorMessage = await app.client.$('.error-message');
            expect(await errorMessage.isDisplayed()).to.be.true;
        });

        it('should save and restore configuration settings', async function() {
            // Set configuration
            await app.client.$('#verbose-checkbox').click();
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'test-output.json'));
            
            // Restart application
            await app.stop();
            await app.start();
            await app.client.waitUntilWindowLoaded();
            
            // Check if settings are restored
            expect(await app.client.$('#verbose-checkbox').isSelected()).to.be.true;
            expect(await app.client.$('#output-path-input').getValue()).to.include('test-output.json');
        });
    });

    describe('CLI Integration', function() {
        it('should execute CLI and capture output', async function() {
            // Configure scan
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'cli-test.json'));
            
            // Start scan
            await app.client.$('#start-scan-btn').click();
            
            // Wait for scan to complete
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 15000 });
            
            // Verify results are displayed
            const resultsPanel = await app.client.$('#results-panel');
            expect(await resultsPanel.isDisplayed()).to.be.true;
        });

        it('should handle CLI execution errors gracefully', async function() {
            // Set mock CLI to fail
            process.env.MOCK_CLI_SHOULD_FAIL = 'true';
            
            await app.client.$('#start-scan-btn').click();
            
            // Wait for error handling
            await app.client.waitUntil(async () => {
                const errorPanel = await app.client.$('#error-panel');
                return await errorPanel.isDisplayed();
            }, { timeout: 10000 });
            
            // Check error message
            const errorMessage = await app.client.$('#error-message').getText();
            expect(errorMessage).to.include('CLI execution failed');
            
            // Cleanup
            delete process.env.MOCK_CLI_SHOULD_FAIL;
        });

        it('should validate JSON output from CLI', async function() {
            // Configure scan with valid settings
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'validation-test.json'));
            await app.client.$('#start-scan-btn').click();
            
            // Wait for completion
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 15000 });
            
            // Check that JSON was parsed successfully
            const artifactCount = await app.client.$('#artifact-count').getText();
            expect(parseInt(artifactCount)).to.be.greaterThan(0);
        });
    });

    describe('Results Visualization', function() {
        beforeEach(async function() {
            // Run a scan to get results
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'results-test.json'));
            await app.client.$('#start-scan-btn').click();
            
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 15000 });
        });

        it('should display scan metadata', async function() {
            const metadataPanel = await app.client.$('#metadata-panel');
            expect(await metadataPanel.isDisplayed()).to.be.true;
            
            // Check metadata fields
            expect(await app.client.$('#scan-id').getText()).to.not.be.empty;
            expect(await app.client.$('#hostname').getText()).to.not.be.empty;
            expect(await app.client.$('#scan-duration').getText()).to.not.be.empty;
        });

        it('should display tabbed artifact views', async function() {
            // Check for tab navigation
            expect(await app.client.$('#processes-tab').isExisting()).to.be.true;
            expect(await app.client.$('#network-tab').isExisting()).to.be.true;
            expect(await app.client.$('#persistence-tab').isExisting()).to.be.true;
            expect(await app.client.$('#events-tab').isExisting()).to.be.true;
            expect(await app.client.$('#execution-tab').isExisting()).to.be.true;
            
            // Test tab switching
            await app.client.$('#network-tab').click();
            expect(await app.client.$('#network-table').isDisplayed()).to.be.true;
            
            await app.client.$('#processes-tab').click();
            expect(await app.client.$('#processes-table').isDisplayed()).to.be.true;
        });

        it('should provide sortable and filterable tables', async function() {
            // Test process table sorting
            await app.client.$('#processes-tab').click();
            await app.client.$('#process-name-header').click();
            
            // Verify sorting (check if first row changed)
            const firstRowBefore = await app.client.$('#processes-table tbody tr:first-child td:first-child').getText();
            await app.client.$('#process-name-header').click();
            const firstRowAfter = await app.client.$('#processes-table tbody tr:first-child td:first-child').getText();
            
            // Should be different after sorting
            expect(firstRowBefore).to.not.equal(firstRowAfter);
            
            // Test filtering
            await app.client.$('#process-filter').setValue('system');
            await app.client.pause(500); // Wait for filter to apply
            
            const visibleRows = await app.client.$$('#processes-table tbody tr:not([style*="display: none"])');
            expect(visibleRows.length).to.be.greaterThan(0);
        });

        it('should highlight suspicious artifacts', async function() {
            // Check for suspicious indicators
            const suspiciousElements = await app.client.$$('.suspicious-indicator');
            
            // Should have some suspicious indicators (from mock data)
            expect(suspiciousElements.length).to.be.greaterThan(0);
            
            // Test suspicious filter
            await app.client.$('#show-suspicious-only').click();
            const visibleSuspicious = await app.client.$$('.artifact-row:not([style*="display: none"]) .suspicious-indicator');
            expect(visibleSuspicious.length).to.be.greaterThan(0);
        });
    });

    describe('Report Generation', function() {
        beforeEach(async function() {
            // Run a scan to get results
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'report-test.json'));
            await app.client.$('#start-scan-btn').click();
            
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 15000 });
        });

        it('should generate comprehensive reports', async function() {
            // Open report generation dialog
            await app.client.$('#generate-report-btn').click();
            
            const reportDialog = await app.client.$('#report-dialog');
            expect(await reportDialog.isDisplayed()).to.be.true;
            
            // Configure report
            await app.client.$('#report-format-select').selectByValue('html');
            await app.client.$('#report-path-input').setValue(path.join(testDataDir, 'test-report.html'));
            
            // Generate report
            await app.client.$('#confirm-generate-btn').click();
            
            // Wait for report generation
            await app.client.waitUntil(async () => {
                const successMessage = await app.client.$('#report-success-message');
                return await successMessage.isDisplayed();
            }, { timeout: 10000 });
            
            // Verify report file was created
            expect(fs.existsSync(path.join(testDataDir, 'test-report.html'))).to.be.true;
        });

        it('should support multiple report formats', async function() {
            const formats = ['html', 'pdf', 'json'];
            
            for (const format of formats) {
                await app.client.$('#generate-report-btn').click();
                await app.client.$('#report-format-select').selectByValue(format);
                await app.client.$('#report-path-input').setValue(path.join(testDataDir, `test-report.${format}`));
                await app.client.$('#confirm-generate-btn').click();
                
                await app.client.waitUntil(async () => {
                    const successMessage = await app.client.$('#report-success-message');
                    return await successMessage.isDisplayed();
                }, { timeout: 10000 });
                
                // Close dialog for next iteration
                await app.client.$('#close-report-dialog').click();
            }
        });

        it('should include customizable report sections', async function() {
            await app.client.$('#generate-report-btn').click();
            
            // Test section toggles
            await app.client.$('#include-processes-checkbox').click();
            await app.client.$('#include-network-checkbox').click();
            
            expect(await app.client.$('#include-processes-checkbox').isSelected()).to.be.false;
            expect(await app.client.$('#include-network-checkbox').isSelected()).to.be.false;
            expect(await app.client.$('#include-events-checkbox').isSelected()).to.be.true; // Should remain selected
        });
    });

    describe('Error Handling and Recovery', function() {
        it('should handle application crashes gracefully', async function() {
            // Simulate error condition
            await app.client.execute(() => {
                window.simulateError = true;
                throw new Error('Simulated application error');
            });
            
            // Application should still be responsive
            expect(await app.client.getTitle()).to.equal('TriageIR - Digital Forensics Triage Tool');
        });

        it('should provide user-friendly error messages', async function() {
            // Trigger various error conditions
            await app.client.$('#output-path-input').setValue('');
            await app.client.$('#start-scan-btn').click();
            
            const errorMessage = await app.client.$('#validation-error').getText();
            expect(errorMessage).to.include('Please specify an output path');
        });

        it('should allow retry after failures', async function() {
            // Set mock CLI to fail
            process.env.MOCK_CLI_SHOULD_FAIL = 'true';
            
            await app.client.$('#start-scan-btn').click();
            
            // Wait for error
            await app.client.waitUntil(async () => {
                const errorPanel = await app.client.$('#error-panel');
                return await errorPanel.isDisplayed();
            }, { timeout: 10000 });
            
            // Click retry button
            await app.client.$('#retry-scan-btn').click();
            
            // Remove failure condition
            delete process.env.MOCK_CLI_SHOULD_FAIL;
            
            // Should be able to retry successfully
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 15000 });
        });
    });

    describe('Performance and Responsiveness', function() {
        it('should remain responsive during long operations', async function() {
            // Start a scan
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'performance-test.json'));
            await app.client.$('#start-scan-btn').click();
            
            // UI should remain responsive
            expect(await app.client.$('#cancel-scan-btn').isClickable()).to.be.true;
            expect(await app.client.$('#minimize-btn').isClickable()).to.be.true;
            
            // Progress should update
            await app.client.waitUntil(async () => {
                const progressBar = await app.client.$('#progress-bar');
                const value = await progressBar.getAttribute('value');
                return parseInt(value) > 0;
            }, { timeout: 5000 });
        });

        it('should handle large datasets efficiently', async function() {
            // Set mock CLI to return large dataset
            process.env.MOCK_CLI_LARGE_DATASET = 'true';
            
            await app.client.$('#output-path-input').setValue(path.join(testDataDir, 'large-dataset-test.json'));
            await app.client.$('#start-scan-btn').click();
            
            await app.client.waitUntil(async () => {
                const progressText = await app.client.$('#progress-text').getText();
                return progressText.includes('Scan completed');
            }, { timeout: 20000 });
            
            // UI should still be responsive with large dataset
            await app.client.$('#processes-tab').click();
            expect(await app.client.$('#processes-table').isDisplayed()).to.be.true;
            
            // Cleanup
            delete process.env.MOCK_CLI_LARGE_DATASET;
        });

        it('should optimize memory usage', async function() {
            const initialMemory = await app.mainProcess.evaluate(() => {
                return process.memoryUsage().heapUsed;
            });
            
            // Perform memory-intensive operations
            for (let i = 0; i < 5; i++) {
                await app.client.$('#output-path-input').setValue(path.join(testDataDir, `memory-test-${i}.json`));
                await app.client.$('#start-scan-btn').click();
                
                await app.client.waitUntil(async () => {
                    const progressText = await app.client.$('#progress-text').getText();
                    return progressText.includes('Scan completed');
                }, { timeout: 15000 });
                
                // Clear results to test memory cleanup
                await app.client.$('#clear-results-btn').click();
            }
            
            const finalMemory = await app.mainProcess.evaluate(() => {
                return process.memoryUsage().heapUsed;
            });
            
            // Memory growth should be reasonable
            const memoryGrowth = finalMemory - initialMemory;
            expect(memoryGrowth).to.be.lessThan(100 * 1024 * 1024); // Less than 100MB growth
        });
    });

    // Helper function to create mock CLI for testing
    function createMockCli() {
        const mockCliContent = `#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
const outputIndex = args.indexOf('--output');
const formatIndex = args.indexOf('--format');
const verboseIndex = args.indexOf('--verbose');

const outputFile = outputIndex !== -1 ? args[outputIndex + 1] : null;
const format = formatIndex !== -1 ? args[formatIndex + 1] : 'json';
const verbose = verboseIndex !== -1;

// Check for failure simulation
if (process.env.MOCK_CLI_SHOULD_FAIL === 'true') {
    console.error('Mock CLI failure simulation');
    process.exit(1);
}

// Generate mock data
const mockData = {
    scan_metadata: {
        scan_id: 'test-scan-' + Date.now(),
        scan_start_utc: new Date().toISOString(),
        scan_duration_ms: 1500,
        hostname: 'TEST-COMPUTER',
        os_version: 'Windows 10 Pro',
        cli_version: '1.0.0-test',
        total_artifacts: process.env.MOCK_CLI_LARGE_DATASET === 'true' ? 10000 : 150,
        collection_summary: {
            total_logs: 25,
            error_count: 1,
            warning_count: 3,
            success_rate: 92.0
        }
    },
    artifacts: {
        system_info: {
            hostname: 'TEST-COMPUTER',
            os_name: 'Windows_NT',
            os_version: 'Windows 10 Pro',
            architecture: 'x86_64',
            current_user: 'TestUser',
            uptime_hours: 24.5,
            last_boot_time: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),
            total_memory: 16777216000,
            used_memory: 8388608000,
            cpu_count: 8
        },
        running_processes: generateMockProcesses(),
        network_connections: generateMockNetworkConnections(),
        persistence_mechanisms: generateMockPersistence(),
        event_logs: {
            security: generateMockEvents('Security'),
            system: generateMockEvents('System'),
            application: generateMockEvents('Application')
        },
        execution_evidence: {
            prefetch_files: generateMockPrefetch(),
            shimcache_entries: generateMockShimcache()
        }
    },
    collection_log: generateMockCollectionLog()
};

function generateMockProcesses() {
    const processes = [];
    const processNames = ['explorer.exe', 'chrome.exe', 'notepad.exe', 'svchost.exe', 'winlogon.exe'];
    const count = process.env.MOCK_CLI_LARGE_DATASET === 'true' ? 5000 : 50;
    
    for (let i = 0; i < count; i++) {
        processes.push({
            pid: 1000 + i,
            parent_pid: i === 0 ? 0 : Math.floor(Math.random() * i) + 1000,
            name: processNames[i % processNames.length],
            command_line: \`C:\\\\Windows\\\\System32\\\\\${processNames[i % processNames.length]}\`,
            executable_path: \`C:\\\\Windows\\\\System32\\\\\${processNames[i % processNames.length]}\`,
            sha256_hash: 'a'.repeat(64),
            user: 'TestUser',
            memory_usage_mb: Math.floor(Math.random() * 100) + 10,
            loaded_modules: []
        });
    }
    return processes;
}

function generateMockNetworkConnections() {
    const connections = [];
    const count = process.env.MOCK_CLI_LARGE_DATASET === 'true' ? 2000 : 20;
    
    for (let i = 0; i < count; i++) {
        connections.push({
            protocol: i % 2 === 0 ? 'TCP' : 'UDP',
            local_address: '127.0.0.1',
            local_port: 8000 + i,
            remote_address: \`192.168.1.\${(i % 254) + 1}\`,
            remote_port: 80,
            state: 'ESTABLISHED',
            owning_pid: 1000 + (i % 50),
            process_name: 'chrome.exe',
            is_external: true
        });
    }
    return connections;
}

function generateMockPersistence() {
    return [
        {
            type: 'Registry Run Key',
            name: 'TestApp',
            command: 'C:\\\\Program Files\\\\TestApp\\\\app.exe',
            source: 'HKLM\\\\SOFTWARE\\\\Microsoft\\\\Windows\\\\CurrentVersion\\\\Run',
            location: 'Registry',
            value: 'TestApp',
            is_suspicious: false
        },
        {
            type: 'Scheduled Task',
            name: 'SuspiciousTask',
            command: 'C:\\\\Temp\\\\malware.exe',
            source: 'Task Scheduler',
            location: 'Tasks',
            value: 'SuspiciousTask',
            is_suspicious: true
        }
    ];
}

function generateMockEvents(logType) {
    const events = [];
    const count = process.env.MOCK_CLI_LARGE_DATASET === 'true' ? 1000 : 10;
    
    for (let i = 0; i < count; i++) {
        events.push({
            event_id: 4624 + (i % 10),
            level: i % 10 === 0 ? 'Error' : (i % 5 === 0 ? 'Warning' : 'Information'),
            timestamp: new Date(Date.now() - i * 60000).toISOString(),
            message: \`\${logType} event message \${i}\`,
            source: logType
        });
    }
    return events;
}

function generateMockPrefetch() {
    return [
        {
            filename: 'CHROME.EXE-12345678.pf',
            executable_name: 'chrome.exe',
            run_count: 25,
            last_run_time: new Date().toISOString(),
            creation_time: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
            file_size: 12345,
            hash: 'abc123',
            version: 8,
            referenced_files: ['C:\\\\Program Files\\\\Google\\\\Chrome\\\\chrome.exe'],
            volumes: []
        }
    ];
}

function generateMockShimcache() {
    return [
        {
            path: 'C:\\\\Windows\\\\System32\\\\notepad.exe',
            last_modified: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),
            file_size: 98765,
            last_update: new Date().toISOString(),
            execution_flag: true
        }
    ];
}

function generateMockCollectionLog() {
    return [
        {
            timestamp: new Date().toISOString(),
            level: 'INFO',
            message: 'Starting forensic data collection'
        },
        {
            timestamp: new Date().toISOString(),
            level: 'WARN',
            message: 'Some processes were inaccessible'
        },
        {
            timestamp: new Date().toISOString(),
            level: 'ERROR',
            message: 'Failed to access registry key'
        }
    ];
}

// Output results
const jsonOutput = JSON.stringify(mockData, null, 2);

if (outputFile) {
    // Ensure directory exists
    const dir = path.dirname(outputFile);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(outputFile, jsonOutput);
    if (verbose) {
        console.error('Mock scan completed successfully');
        console.error(\`Results written to: \${outputFile}\`);
    }
} else {
    console.log(jsonOutput);
}

process.exit(0);
`;

        fs.writeFileSync(mockCliPath, mockCliContent);
        fs.chmodSync(mockCliPath, '755');
    }
});