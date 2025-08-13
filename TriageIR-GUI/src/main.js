const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const path = require('path');
const { spawn } = require('child_process');
const fs = require('fs');

class TriageIRMain {
    constructor() {
        this.mainWindow = null;
        this.cliPath = null;
        this.setupApp();
    }

    setupApp() {
        app.whenReady().then(() => {
            this.createWindow();
            this.findCLIExecutable();
            this.setupIPC();
        });

        app.on('window-all-closed', () => {
            if (process.platform !== 'darwin') {
                app.quit();
            }
        });

        app.on('activate', () => {
            if (BrowserWindow.getAllWindows().length === 0) {
                this.createWindow();
            }
        });
    }

    createWindow() {
        this.mainWindow = new BrowserWindow({
            width: 1200,
            height: 800,
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
                enableRemoteModule: true
            },
            icon: path.join(__dirname, '../assets/icon.png'),
            title: 'TriageIR - Digital Forensics Triage Tool'
        });

        this.mainWindow.loadFile(path.join(__dirname, 'renderer/index.html'));

        // Open DevTools in development
        if (process.argv.includes('--dev')) {
            this.mainWindow.webContents.openDevTools();
        }
    }

    findCLIExecutable() {
        const possiblePaths = [
            // Development paths
            path.join(__dirname, '../../TriageIR-CLI/target/release/triageir-cli.exe'),
            path.join(__dirname, '../../TriageIR-CLI/target/debug/triageir-cli.exe'),
            // Packaged app path
            path.join(process.resourcesPath, 'triageir-cli.exe'),
            // Same directory
            path.join(__dirname, 'triageir-cli.exe'),
            // Current working directory
            path.join(process.cwd(), 'triageir-cli.exe')
        ];

        for (const cliPath of possiblePaths) {
            if (fs.existsSync(cliPath)) {
                this.cliPath = cliPath;
                console.log(`Found CLI at: ${cliPath}`);
                return;
            }
        }

        console.log('CLI not found in expected paths, will try system PATH');
        this.cliPath = 'triageir-cli.exe';
    }

    setupIPC() {
        // Handle scan requests
        ipcMain.handle('start-scan', async (event, options) => {
            return this.executeScan(options);
        });

        // Handle CLI status check
        ipcMain.handle('check-cli-status', async () => {
            return this.checkCLIStatus();
        });

        // Handle file operations
        ipcMain.handle('open-file-dialog', async () => {
            const result = await dialog.showOpenDialog(this.mainWindow, {
                properties: ['openFile'],
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ]
            });
            return result;
        });

        ipcMain.handle('save-file-dialog', async () => {
            const result = await dialog.showSaveDialog(this.mainWindow, {
                filters: [
                    { name: 'JSON Files', extensions: ['json'] },
                    { name: 'All Files', extensions: ['*'] }
                ]
            });
            return result;
        });
    }

    async checkCLIStatus() {
        return new Promise((resolve) => {
            const process = spawn(this.cliPath, ['--version'], { shell: true });
            let output = '';

            process.stdout.on('data', (data) => {
                output += data.toString();
            });

            process.on('close', (code) => {
                resolve({
                    available: code === 0,
                    version: output.trim(),
                    path: this.cliPath
                });
            });

            process.on('error', () => {
                resolve({
                    available: false,
                    version: null,
                    path: this.cliPath
                });
            });
        });
    }

    async executeScan(options = {}) {
        return new Promise((resolve, reject) => {
            // Create a temporary output file for clean JSON
            const tempFile = path.join(__dirname, `temp-scan-${Date.now()}.json`);
            const args = ['--output', tempFile];
            
            if (options.verbose) {
                args.push('--verbose');
            }

            console.log(`Executing: ${this.cliPath} ${args.join(' ')}`);

            const process = spawn(this.cliPath, args, { shell: true });
            let stdout = '';
            let stderr = '';

            process.stdout.on('data', (data) => {
                const output = data.toString();
                stdout += output;
                // Send progress updates to renderer (verbose output)
                this.mainWindow.webContents.send('scan-progress', output.trim());
            });

            process.stderr.on('data', (data) => {
                stderr += data.toString();
            });

            process.on('close', (code) => {
                if (code === 0 || code === 2) { // Accept exit code 2 as success (warnings)
                    try {
                        // Read the clean JSON from output file
                        if (fs.existsSync(tempFile)) {
                            const jsonData = JSON.parse(fs.readFileSync(tempFile, 'utf8'));
                            
                            // Clean up temp file
                            try {
                                fs.unlinkSync(tempFile);
                            } catch (cleanupError) {
                                console.warn('Failed to cleanup temp file:', cleanupError);
                            }
                            
                            resolve({
                                success: true,
                                data: jsonData,
                                stdout: stdout,
                                stderr: stderr
                            });
                        } else {
                            // Fallback: try to extract JSON from stdout
                            const jsonMatch = stdout.match(/\{[\s\S]*\}$/);
                            if (jsonMatch) {
                                const jsonData = JSON.parse(jsonMatch[0]);
                                resolve({
                                    success: true,
                                    data: jsonData,
                                    stdout: stdout,
                                    stderr: stderr
                                });
                            } else {
                                reject({
                                    success: false,
                                    error: 'No JSON output found',
                                    stdout: stdout,
                                    stderr: stderr
                                });
                            }
                        }
                    } catch (error) {
                        reject({
                            success: false,
                            error: `Failed to parse CLI output: ${error.message}`,
                            stdout: stdout,
                            stderr: stderr
                        });
                    }
                } else {
                    reject({
                        success: false,
                        error: `CLI exited with code ${code}`,
                        stdout: stdout,
                        stderr: stderr
                    });
                }
            });

            process.on('error', (error) => {
                reject({
                    success: false,
                    error: `Failed to start CLI: ${error.message}`,
                    stdout: stdout,
                    stderr: stderr
                });
            });
        });
    }
}

// Create the application
new TriageIRMain();