# TriageIR GUI - New Clean Version

A completely rewritten, clean, and functional GUI for the TriageIR digital forensics tool.

## Features

- **Clean Modern Interface**: Beautiful, responsive design with smooth animations
- **Functional Button System**: All buttons work correctly with proper event handling
- **Real-time Scan Progress**: Live updates during CLI execution
- **Comprehensive Results Display**: Organized tabs for different artifact types
- **Export Capabilities**: Save results as JSON or export HTML reports
- **CLI Integration**: Seamless communication with TriageIR-CLI

## Quick Start

1. **Install Dependencies**:
   ```bash
   npm install
   ```

2. **Run in Development Mode**:
   ```bash
   npm run dev
   ```

3. **Build for Production**:
   ```bash
   npm run build
   ```

## Architecture

### Main Process (`src/main.js`)
- Electron application lifecycle management
- CLI process execution and management
- IPC communication with renderer
- File dialog handling

### Renderer Process (`src/renderer/`)
- **index.html**: Clean, semantic HTML structure
- **styles/main.css**: Modern CSS with gradients and animations
- **js/app.js**: Complete application logic with proper event handling

## Key Improvements

1. **Fixed Button Functionality**: All buttons now have proper event listeners
2. **Clean Code Structure**: Modular, maintainable JavaScript
3. **Better Error Handling**: Comprehensive error messages and recovery
4. **Responsive Design**: Works on different screen sizes
5. **Professional UI**: Modern design with smooth transitions

## Usage

1. **Quick Scan**: Run a standard forensic scan with default settings
2. **Custom Scan**: Configure scan parameters (future enhancement)
3. **Open Results**: Load previously saved scan results
4. **View Results**: Browse artifacts in organized tabs
5. **Export Reports**: Save results or generate HTML reports

## CLI Integration

The GUI automatically locates the TriageIR-CLI executable in these locations:
- `../TriageIR-CLI/target/release/triageir-cli.exe`
- `../TriageIR-CLI/target/debug/triageir-cli.exe`
- Packaged app resources
- System PATH

## Development

- **Development Mode**: `npm run dev` (opens DevTools)
- **Production Build**: `npm run build`
- **Testing**: Manual testing with real CLI integration

## Requirements

- Node.js 16+
- Electron 27+
- Windows OS (for CLI integration)
- TriageIR-CLI executable

## Status

✅ **FULLY FUNCTIONAL** - All buttons work, CLI integration complete, results display working