# TriageIR GUI

A modern Electron-based graphical user interface for the TriageIR forensic triage tool.

## Features

- **Modern UI**: Clean, intuitive interface built with Electron
- **Real-time Progress**: Live progress tracking during scans
- **Data Visualization**: Organized tables and summaries for all collected artifacts
- **Export Capabilities**: Generate reports and export data in multiple formats
- **Cross-platform**: Runs on Windows with native look and feel

## Prerequisites

- **Node.js**: Version 16 or later
- **npm**: Comes with Node.js
- **TriageIR CLI**: The CLI executable must be available

## Installation

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd TriageIR-GUI

# Install dependencies
npm install

# Start in development mode
npm run dev
```

### Building for Production

```bash
# Build the application
npm run build

# Create distributable packages
npm run dist
```

## Usage

### Starting the Application

```bash
# Development mode with hot reload
npm start

# Or with development flag
npm run dev
```

### Building Distributables

```bash
# Create installer packages
npm run make

# Create unpacked directory
npm run pack
```

## Project Structure

```
TriageIR-GUI/
├── src/
│   ├── main.js              # Main Electron process
│   ├── renderer/            # Renderer process files
│   │   ├── index.html       # Main HTML file
│   │   ├── styles/          # CSS stylesheets
│   │   │   ├── main.css     # Main application styles
│   │   │   ├── components.css # Component styles
│   │   │   └── tabs.css     # Tab system styles
│   │   └── js/              # JavaScript modules
│   │       ├── utils.js     # Utility functions
│   │       ├── cli-manager.js # CLI execution manager
│   │       ├── data-renderer.js # Data visualization
│   │       ├── export-manager.js # Export functionality
│   │       └── app.js       # Main application logic
│   └── assets/              # Static assets
│       └── icon.png         # Application icon
├── build/                   # Build resources
│   └── icon.ico            # Windows icon
├── package.json            # Project configuration
└── README.md              # This file
```

## Architecture

The TriageIR GUI follows Electron's multi-process architecture:

### Main Process (`main.js`)
- Manages application lifecycle
- Creates and manages browser windows
- Handles system-level operations
- Provides IPC communication with renderer

### Renderer Process (`renderer/`)
- Handles user interface
- Manages CLI execution
- Processes and displays scan results
- Handles user interactions

## Key Features

### Scan Management
- **Full Scan**: Complete forensic collection with all artifacts
- **Quick Scan**: Fast collection without process hashes
- **Custom Scan**: User-configurable artifact selection

### Data Visualization
- **Overview Tab**: Summary statistics and key findings
- **System Info Tab**: System information and logged-on users
- **Processes Tab**: Running processes with filtering and sorting
- **Network Tab**: Network connections with protocol filtering
- **Persistence Tab**: Autostart mechanisms and services
- **Events Tab**: Windows Event Log entries with filtering
- **Logs Tab**: Collection log with severity filtering

### Export Options
- **JSON Export**: Raw scan data in JSON format
- **CSV Export**: Tabular data for spreadsheet analysis
- **HTML Report**: Formatted report for documentation
- **Text Report**: Plain text summary for quick review

## Configuration

### CLI Integration
The GUI automatically looks for the TriageIR CLI executable in:
1. Same directory as the GUI executable
2. `../TriageIR-CLI/target/release/` (development)
3. System PATH

### Scan Options
- **Artifact Selection**: Choose which artifacts to collect
- **Performance Options**: Skip hashes, limit event logs
- **Output Options**: Specify output location and format

## Development

### Code Style
- Use ESLint for JavaScript linting
- Use Prettier for code formatting
- Follow Electron security best practices

### Testing
```bash
# Run tests
npm test

# Run linting
npm run lint

# Format code
npm run format
```

### Debugging
- Use Chrome DevTools for renderer process debugging
- Use VS Code for main process debugging
- Enable verbose logging with `--dev` flag

## Security

The application follows Electron security best practices:
- Context isolation enabled
- Node integration disabled in renderer
- Content Security Policy implemented
- External URL navigation prevented
- Certificate error handling

## Building and Distribution

### Windows
```bash
# Build Windows installer
npm run build

# The installer will be created in dist/
```

### Configuration
Build configuration is in `package.json` under the `build` section:
- **App ID**: `com.triageir.gui`
- **Product Name**: TriageIR GUI
- **Target**: NSIS installer for Windows
- **Architecture**: x64

## Troubleshooting

### Common Issues

**CLI Not Found**
- Ensure TriageIR CLI is built and available
- Check that the executable path is correct
- Verify file permissions

**Permission Errors**
- Run as Administrator for complete data collection
- Check Windows UAC settings
- Verify file system permissions

**Build Errors**
- Clear node_modules and reinstall: `rm -rf node_modules && npm install`
- Update Node.js to latest LTS version
- Check for platform-specific build requirements

### Logs
- Application logs are available in the DevTools console
- Enable verbose mode for detailed logging
- Check the collection log tab for CLI execution details

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.