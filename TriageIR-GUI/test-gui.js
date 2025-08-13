// Simple test script to verify GUI functionality

const { app, BrowserWindow } = require('electron');
const path = require('path');

function createTestWindow() {
    const testWindow = new BrowserWindow({
        width: 1200,
        height: 800,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false
        },
        show: false
    });

    testWindow.loadFile(path.join(__dirname, 'src', 'renderer', 'index.html'));

    testWindow.once('ready-to-show', () => {
        testWindow.show();
        testWindow.webContents.openDevTools();
    });

    // Test basic functionality
    testWindow.webContents.once('dom-ready', () => {
        console.log('DOM ready, testing basic functionality...');
        
        testWindow.webContents.executeJavaScript(`
            // Test if all required classes are available
            const tests = [
                { name: 'TriageUtils', obj: window.TriageUtils },
                { name: 'CLIManager', obj: window.CLIManager },
                { name: 'DataRenderer', obj: window.DataRenderer },
                { name: 'ExportManager', obj: window.ExportManager }
            ];
            
            tests.forEach(test => {
                if (test.obj) {
                    console.log('✓ ' + test.name + ' loaded successfully');
                } else {
                    console.error('✗ ' + test.name + ' failed to load');
                }
            });
            
            // Test utility functions
            if (window.TriageUtils) {
                console.log('Testing utility functions...');
                console.log('formatDuration(5000):', window.TriageUtils.formatDuration(5000));
                console.log('formatUptime(3661):', window.TriageUtils.formatUptime(3661));
                console.log('formatBytes(1048576):', window.TriageUtils.formatBytes(1048576));
            }
            
            // Test if main app is initialized
            setTimeout(() => {
                if (window.triageApp) {
                    console.log('✓ Main application initialized');
                } else {
                    console.error('✗ Main application failed to initialize');
                }
            }, 1000);
        `).then(() => {
            console.log('Test script executed successfully');
        }).catch(err => {
            console.error('Test script failed:', err);
        });
    });

    return testWindow;
}

app.whenReady().then(() => {
    console.log('Starting GUI test...');
    createTestWindow();
});

app.on('window-all-closed', () => {
    app.quit();
});

app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
        createTestWindow();
    }
});