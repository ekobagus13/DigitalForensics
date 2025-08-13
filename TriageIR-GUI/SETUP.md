# TriageIR GUI Setup Guide

This guide will help you set up and run the TriageIR GUI application.

## Prerequisites

### Required Software

1. **Node.js** (version 16 or later)
   - Download from: https://nodejs.org/
   - Choose the LTS (Long Term Support) version
   - This includes npm (Node Package Manager)

2. **TriageIR CLI** (the backend executable)
   - Must be built from the TriageIR-CLI project
   - Should be available in one of these locations:
     - Same directory as the GUI
     - `../TriageIR-CLI/target/release/triageir-cli.exe`
     - System PATH

### System Requirements

- **Operating System**: Windows 10 or later
- **RAM**: 4GB minimum, 8GB recommended
- **Disk Space**: 500MB for the application and dependencies
- **Administrator Rights**: Required for complete forensic data collection

## Installation

### Method 1: Quick Start (Recommended)

1. **Open Command Prompt or PowerShell as Administrator**

2. **Navigate to the TriageIR-GUI directory**
   ```cmd
   cd path\to\TriageIR-GUI
   ```

3. **Run the development start script**
   ```cmd
   dev-start.bat
   ```

   This script will:
   - Check for Node.js installation
   - Install dependencies automatically
   - Start the application in development mode

### Method 2: Manual Installation

1. **Install Dependencies**
   ```cmd
   npm install
   ```

2. **Start the Application**
   ```cmd
   npm start
   ```

   Or for development mode with hot reload:
   ```cmd
   npm run dev
   ```

## Verification

### Test the Installation

1. **Run the test script**
   ```cmd
   node test-gui.js
   ```

2. **Check the console output** for:
   - ✓ All modules loaded successfully
   - ✓ Main application initialized
   - ✓ Utility functions working

3. **Verify CLI Integration**
   - The status bar should show "Ready - CLI available" if the CLI is found
   - If it shows "Warning - CLI not found", ensure the CLI executable is built and accessible

### Expected Behavior

When the application starts successfully, you should see:

1. **Main Window**: A modern interface with the TriageIR logo and navigation
2. **Welcome Screen**: Instructions and options to start scanning or open existing results
3. **Status Bar**: Shows "Ready" or "Ready - CLI available"
4. **No Console Errors**: Check the Developer Tools (F12) for any JavaScript errors

## Troubleshooting

### Common Issues

#### "Node.js not found"
- **Solution**: Install Node.js from https://nodejs.org/
- **Verify**: Run `node --version` in command prompt

#### "npm install failed"
- **Solution**: 
  1. Delete `node_modules` folder if it exists
  2. Clear npm cache: `npm cache clean --force`
  3. Run `npm install` again
  4. If still failing, try `npm install --legacy-peer-deps`

#### "CLI not found" warning
- **Solution**: 
  1. Build the TriageIR CLI first: `cd ../TriageIR-CLI && cargo build --release`
  2. Ensure the executable is in the expected location
  3. Or add the CLI to your system PATH

#### Application won't start
- **Check**: 
  1. Node.js version: `node --version` (should be 16+)
  2. npm version: `npm --version`
  3. Dependencies installed: `ls node_modules` or `dir node_modules`
  4. Console errors: Open Developer Tools (F12) and check Console tab

#### Blank white screen
- **Solution**:
  1. Open Developer Tools (F12)
  2. Check Console for JavaScript errors
  3. Verify all script files are loading correctly
  4. Try refreshing with Ctrl+R

#### Permission errors during scan
- **Solution**: Run the application as Administrator
- **Note**: Some forensic data requires elevated privileges

### Development Mode Issues

#### Hot reload not working
- **Solution**: Ensure you're using `npm run dev` instead of `npm start`
- **Alternative**: Manually refresh with Ctrl+R

#### DevTools not opening
- **Solution**: Press F12 or use the View menu → Toggle Developer Tools

## Building for Distribution

### Create Installer

1. **Build the application**
   ```cmd
   npm run build
   ```

2. **Create distributable**
   ```cmd
   npm run dist
   ```

3. **Find the installer** in the `dist` folder

### Build Options

- `npm run pack` - Create unpacked directory
- `npm run make` - Create installer without publishing
- `npm run dist` - Create installer for distribution

## Configuration

### CLI Path Configuration

If the CLI is in a non-standard location, you can modify the search paths in `src/renderer/js/cli-manager.js`:

```javascript
const possiblePaths = [
    // Add your custom path here
    'C:\\path\\to\\your\\triageir-cli.exe',
    // ... existing paths
];
```

### Application Settings

The application uses these default settings:
- **Window Size**: 1400x900 pixels
- **Minimum Size**: 1200x700 pixels
- **Default Scan Options**: Full scan with all artifacts
- **Event Log Limit**: 1000 entries

## Development

### Project Structure

```
TriageIR-GUI/
├── src/
│   ├── main.js              # Electron main process
│   ├── renderer/            # Frontend code
│   │   ├── index.html       # Main UI
│   │   ├── styles/          # CSS files
│   │   └── js/              # JavaScript modules
│   └── assets/              # Images and icons
├── build/                   # Build resources
├── package.json            # Project configuration
└── dev-start.bat          # Development startup script
```

### Making Changes

1. **Edit files** in the `src/` directory
2. **Refresh the application** with Ctrl+R (in dev mode)
3. **Check console** for any errors
4. **Test functionality** thoroughly

### Adding Features

1. **Modify HTML** in `src/renderer/index.html`
2. **Add styles** in `src/renderer/styles/`
3. **Add JavaScript** in `src/renderer/js/`
4. **Update main process** in `src/main.js` if needed

## Support

### Getting Help

1. **Check the console** (F12) for error messages
2. **Review this setup guide** for common solutions
3. **Verify prerequisites** are properly installed
4. **Test with the CLI directly** to isolate issues

### Reporting Issues

When reporting issues, please include:
- Operating system version
- Node.js version (`node --version`)
- npm version (`npm --version`)
- Error messages from console
- Steps to reproduce the issue

### Logs and Debugging

- **Application logs**: Available in Developer Tools Console
- **Electron logs**: Check the terminal where you started the app
- **CLI logs**: Enable verbose mode in scan options
- **Network issues**: Check if CLI executable is accessible

## Next Steps

Once the GUI is running successfully:

1. **Test with a scan**: Try the "Quick Scan" option first
2. **Explore the interface**: Check all tabs and features
3. **Test file operations**: Try opening and saving results
4. **Verify exports**: Test report generation
5. **Check integration**: Ensure CLI and GUI work together properly

For more detailed usage instructions, see the main README.md file.