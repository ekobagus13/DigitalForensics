\# TriageIR: A Hybrid Forensic Triage Framework

\*\*Project Status:\*\* In Development (As of August 2025\)  
\*\*Lead Developer:\*\* \[Ankit\]

\---

\#\# 1\. Project Overview

TriageIR is a hybrid digital forensics tool designed for rapid, live-system evidence collection on Windows platforms. It consists of two primary components: a high-performance, command-line forensic collector (\`TriageIR-CLI\`) built in Rust, and a user-friendly graphical interface (\`TriageIR-GUI\`) built with Electron.js. This decoupled architecture ensures forensic soundness at the core while providing modern accessibility through the GUI.

\---

\#\# 2\. Component Motives & Purpose

\#\#\# 🧠 \*\*TriageIR-CLI (The Forensic Engine)\*\*

\*\*Motive:\*\* To create a forensically sound, lightweight, and blazingly fast portable tool that incident responders can trust. The primary goal is \*\*data integrity and speed\*\*, minimizing the observer effect on a potentially compromised system. It must be fully functional as a standalone tool for power users and for integration into automated security scripts.

\*\*Core Principles:\*\*  
\* \*\*Minimal Footprint:\*\* No installation, no external dependencies, and minimal resource usage (CPU/Memory).  
\* \*\*Forensically Sound:\*\* Does not alter system state unnecessarily. All actions are logged, and all collected evidence is cryptographically hashed ($SHA-256$).  
\* \*\*Portability:\*\* A single, static executable (\`TriageIR-CLI.exe\`) that can run from a trusted USB drive.  
\* \*\*Automation-Friendly:\*\* Designed to be scripted and integrated into larger DFIR workflows via its structured JSON output.

\#\#\# 🖥️ \*\*TriageIR-GUI (The Control Cockpit)\*\*

\*\*Motive:\*\* To make the powerful capabilities of the CLI engine accessible to a broader range of users, including junior analysts, IT administrators, and academic users. The primary goal is \*\*usability and clear visualization of complex data\*\*, enabling quicker and more intuitive analysis.

\*\*Core Principles:\*\*  
\* \*\*Intuitive Interface:\*\* A clean, modern UI that simplifies the process of running a scan and reviewing findings.  
\* \*\*Data Visualization:\*\* Presents collected artifacts in organized tables, lists, and summaries, making it easy to spot anomalies.  
\* \*\*Safe Execution:\*\* Acts purely as a front-end launcher and parser. It does not contain any forensic logic itself, ensuring a clean separation of concerns.  
\* \*\*Report Generation:\*\* Allows users to easily export the findings into a human-readable report.

\---

\#\# 3\. System Architecture

The project follows a decoupled, hybrid architecture. The GUI does not perform any forensic collection; it executes the CLI in the background and parses its output.

\*\*Workflow:\*\*  
1\.  The user configures a scan in \`TriageIR-GUI\`.  
2\.  The GUI invokes the \`TriageIR-CLI.exe\` process with the \`--format json\` argument.  
3\.  The CLI engine performs the forensic collection, generates a single JSON object containing all findings, and prints it to \`stdout\`.  
4\.  The GUI captures the \`stdout\` stream, parses the JSON, and dynamically populates the UI with the collected data.

\---

\#\# 4\. Technology Stack

\* \*\*CLI Engine:\*\*  
    \* \*\*Language:\*\* \*\*Rust\*\* (for performance, memory safety, and producing a static binary)  
    \* \*\*Key Crates:\*\* \`serde\` (for JSON serialization), \`clap\` (for command-line parsing), \`winreg\` (for Registry access), \`sysinfo\` (for system/process info).  
\* \*\*GUI Front-End:\*\*  
    \* \*\*Framework:\*\* \*\*Electron.js\*\*  
    \* \*\*Language:\*\* TypeScript, HTML, CSS  
    \* \*\*Key Libraries:\*\* \`node:child\_process\` (to run the CLI), a front-end framework like Vue.js or Svelte (optional, for better UI structure).

\---

\#\# 5\. The JSON Contract (Version 1.0)

This schema defines the structure of the data passed from the CLI to the GUI.

\`\`\`json  
{  
  "scan\_metadata": {  
    "scan\_id": "string (UUID)",  
    "scan\_start\_utc": "string (ISO 8601)",  
    "scan\_duration\_ms": "number",  
    "hostname": "string",  
    "os\_version": "string",  
    "cli\_version": "string"  
  },  
  "artifacts": {  
    "system\_info": {  
      "uptime\_secs": "number",  
      "logged\_on\_users": \[  
        { "username": "string", "domain": "string", "logon\_time": "string" }  
      \]  
    },  
    "running\_processes": \[  
      {  
        "pid": "number",  
        "parent\_pid": "number",  
        "name": "string",  
        "command\_line": "string",  
        "executable\_path": "string",  
        "sha256\_hash": "string"  
      }  
    \],  
    "network\_connections": \[  
      {  
        "protocol": "string (TCP/UDP)",  
        "local\_address": "string",  
        "remote\_address": "string",  
        "state": "string",  
        "owning\_pid": "number"  
      }  
    \],  
    "persistence\_mechanisms": \[  
      {  
        "type": "string (e.g., 'Scheduled Task', 'Registry Run Key')",  
        "name": "string",  
        "command": "string",  
        "source": "string (e.g., path to key/task)"  
      }  
    \],  
    "event\_logs": {  
      "security": \[  
        { "event\_id": "number", "level": "string", "timestamp": "string", "message": "string" }  
      \],  
      "system": \[  
        { "event\_id": "number", "level": "string", "timestamp": "string", "message": "string" }  
      \]  
    }  
  },  
  "collection\_log": \[  
    { "timestamp": "string", "level": "string (INFO/WARN/ERROR)", "message": "string" }  
  \]  
}

## **6\. Development Roadmap**

### **Phase 1: CLI Engine Development (Focus: Rust)**

1. **Setup:** Initialize a new Rust project with cargo new TriageIR-CLI.  
2. **CLI Parser:** Use clap to define the command-line interface (--format, \--output, \--password).  
3. **Logging & Hashing:** Implement the core logging and hashing functionalities.  
4. **Artifact Modules:** Create separate Rust modules (processes.rs, network.rs, etc.) for each artifact category.  
5. **JSON Serialization:** Use serde to serialize the collected data structures into the defined JSON contract.  
6. **Build & Test:** Continuously build and test the CLI as a standalone tool.

### **Phase 2: GUI Front-End Development (Focus: Electron)**

1. **Setup:** Initialize a new Electron project.  
2. **UI Scaffolding:** Design and build the main window, tabs, and tables with static placeholder data.  
3. **Process Execution:** Implement the child\_process logic to call the compiled TriageIR-CLI.exe.  
4. **Data Binding:** Write the code to parse the JSON from the CLI's stdout and render it in the UI.  
5. **Packaging:** Set up the Electron builder to package the app into an installer.

### **Phase 3: Integration & Finalization**

1. **End-to-End Testing:** Test the full workflow from GUI \-\> CLI \-\> GUI.  
2. **Error Handling:** Implement robust error handling (e.g., what happens if the CLI fails to run?).  
3. **Documentation:** Write the final project report and user manuals.

## **7\. Setup & Build Instructions**

### **TriageIR-CLI (Rust)**

\# Navigate to the CLI project directory  
cd TriageIR-CLI

\# Build a debug version  
cargo build

\# Build a release (optimized) version  
cargo build \--release

\# The executable will be in ./target/release/TriageIR-CLI.exe

### **TriageIR-GUI (Electron)**

\# Navigate to the GUI project directory  
cd TriageIR-GUI

\# Install dependencies  
npm install

\# Run in development mode  
npm start

\# Build the distributable installer  
npm run make  
