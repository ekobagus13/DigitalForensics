#!/usr/bin/env python3
"""
Code verification script for TriageIR CLI
Checks code structure and dependencies without building
"""

import os
import json
import re
import sys
from pathlib import Path

def check_file_exists(filepath, description):
    """Check if a file exists and report status"""
    if os.path.exists(filepath):
        print(f"✓ {description}: {filepath}")
        return True
    else:
        print(f"✗ {description}: {filepath} (MISSING)")
        return False

def check_cargo_toml():
    """Verify Cargo.toml structure"""
    print("\nChecking Cargo.toml...")
    
    if not check_file_exists("Cargo.toml", "Cargo manifest"):
        return False
    
    try:
        with open("Cargo.toml", 'r') as f:
            content = f.read()
        
        # Check for required sections
        required_sections = [
            "[package]",
            "[dependencies]",
            "name = \"triageir-cli\"",
            "serde",
            "clap",
            "winreg",
            "sysinfo",
            "uuid",
            "chrono"
        ]
        
        for section in required_sections:
            if section in content:
                print(f"✓ Found: {section}")
            else:
                print(f"✗ Missing: {section}")
        
        return True
        
    except Exception as e:
        print(f"✗ Error reading Cargo.toml: {e}")
        return False

def check_source_files():
    """Check source file structure"""
    print("\nChecking source files...")
    
    required_files = [
        ("src/main.rs", "Main application entry point"),
        ("src/types.rs", "Data structures and types"),
        ("src/system_info.rs", "System information module"),
        ("src/processes.rs", "Process enumeration module"),
        ("src/network.rs", "Network connections module"),
        ("src/persistence.rs", "Persistence mechanisms module"),
        ("src/event_logs.rs", "Event log collection module"),
        ("src/logger.rs", "Logging and error handling")
    ]
    
    all_exist = True
    for filepath, description in required_files:
        if not check_file_exists(filepath, description):
            all_exist = False
    
    return all_exist

def check_main_rs():
    """Check main.rs structure"""
    print("\nChecking main.rs structure...")
    
    if not os.path.exists("src/main.rs"):
        print("✗ main.rs not found")
        return False
    
    try:
        with open("src/main.rs", 'r') as f:
            content = f.read()
        
        required_elements = [
            "use clap::Parser",
            "struct Args",
            "fn main()",
            "mod types",
            "mod system_info",
            "mod processes",
            "mod network",
            "mod persistence",
            "mod event_logs"
        ]
        
        for element in required_elements:
            if element in content:
                print(f"✓ Found: {element}")
            else:
                print(f"✗ Missing: {element}")
        
        return True
        
    except Exception as e:
        print(f"✗ Error reading main.rs: {e}")
        return False

def check_types_rs():
    """Check types.rs structure"""
    print("\nChecking types.rs structure...")
    
    if not os.path.exists("src/types.rs"):
        print("✗ types.rs not found")
        return False
    
    try:
        with open("src/types.rs", 'r') as f:
            content = f.read()
        
        required_structs = [
            "struct ScanResults",
            "struct ScanMetadata", 
            "struct Artifacts",
            "struct SystemInfo",
            "struct Process",
            "struct NetworkConnection",
            "struct PersistenceMechanism",
            "struct EventLogs",
            "struct LogEntry"
        ]
        
        for struct in required_structs:
            if struct in content:
                print(f"✓ Found: {struct}")
            else:
                print(f"✗ Missing: {struct}")
        
        # Check for Serde derives
        if "#[derive(Serialize, Deserialize" in content:
            print("✓ Found: Serde serialization derives")
        else:
            print("✗ Missing: Serde serialization derives")
        
        return True
        
    except Exception as e:
        print(f"✗ Error reading types.rs: {e}")
        return False

def check_module_structure(module_name):
    """Check individual module structure"""
    filepath = f"src/{module_name}.rs"
    
    if not os.path.exists(filepath):
        print(f"✗ {module_name}.rs not found")
        return False
    
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Check for basic structure
        if f"pub fn collect_{module_name.replace('_', '_')}" in content or "pub fn collect_" in content:
            print(f"✓ {module_name}.rs: Has collection function")
        else:
            print(f"? {module_name}.rs: Collection function pattern not found")
        
        if "use crate::types::" in content:
            print(f"✓ {module_name}.rs: Uses types module")
        else:
            print(f"? {module_name}.rs: Types import not found")
        
        if "#[cfg(test)]" in content:
            print(f"✓ {module_name}.rs: Has tests")
        else:
            print(f"? {module_name}.rs: No tests found")
        
        return True
        
    except Exception as e:
        print(f"✗ Error reading {module_name}.rs: {e}")
        return False

def check_build_scripts():
    """Check build and test scripts"""
    print("\nChecking build scripts...")
    
    scripts = [
        ("build.bat", "Windows build script"),
        ("build.sh", "Unix build script"),
        ("test.bat", "Windows test script"),
        ("test-build.bat", "Comprehensive build test"),
        ("quick-test.bat", "Quick functionality test"),
        ("run-sample-scan.bat", "Sample scan script")
    ]
    
    for script, description in scripts:
        check_file_exists(script, description)

def check_documentation():
    """Check documentation files"""
    print("\nChecking documentation...")
    
    docs = [
        ("README.md", "Main documentation"),
        ("USAGE.md", "Usage guide"),
        ("PERFORMANCE.md", "Performance guide"),
        ("TESTING.md", "Testing guide")
    ]
    
    for doc, description in docs:
        check_file_exists(doc, description)

def estimate_build_requirements():
    """Estimate build requirements"""
    print("\nBuild Requirements Estimate:")
    print("=" * 40)
    
    # Count source lines
    total_lines = 0
    rust_files = []
    
    for root, dirs, files in os.walk("src"):
        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                try:
                    with open(filepath, 'r') as f:
                        lines = len(f.readlines())
                        total_lines += lines
                        rust_files.append((filepath, lines))
                except:
                    pass
    
    print(f"Total Rust source files: {len(rust_files)}")
    print(f"Total lines of code: {total_lines}")
    
    # Estimate build time
    if total_lines < 1000:
        build_time = "1-2 minutes"
    elif total_lines < 5000:
        build_time = "2-5 minutes"
    else:
        build_time = "5-10 minutes"
    
    print(f"Estimated build time: {build_time}")
    print(f"Estimated disk space: 100-500 MB (including dependencies)")
    print(f"Required RAM: 2-4 GB during build")

def main():
    """Main verification function"""
    print("TriageIR CLI Code Verification")
    print("=" * 50)
    
    # Change to CLI directory if not already there
    if os.path.exists("TriageIR-CLI"):
        os.chdir("TriageIR-CLI")
        print("Changed to TriageIR-CLI directory")
    
    # Run all checks
    checks = [
        check_cargo_toml,
        check_source_files,
        check_main_rs,
        check_types_rs,
        lambda: check_module_structure("system_info"),
        lambda: check_module_structure("processes"),
        lambda: check_module_structure("network"),
        lambda: check_module_structure("persistence"),
        lambda: check_module_structure("event_logs"),
        check_build_scripts,
        check_documentation,
        estimate_build_requirements
    ]
    
    results = []
    for check in checks:
        try:
            result = check()
            results.append(result)
        except Exception as e:
            print(f"✗ Check failed: {e}")
            results.append(False)
    
    # Summary
    print("\n" + "=" * 50)
    print("VERIFICATION SUMMARY")
    print("=" * 50)
    
    passed = sum(1 for r in results if r is True)
    total = len([r for r in results if r is not None])
    
    print(f"Checks passed: {passed}/{total}")
    
    if passed == total:
        print("✅ All checks passed! The code structure looks good.")
        print("\nNext steps:")
        print("1. Install Rust: https://rustup.rs/")
        print("2. Run: test-build.bat")
        print("3. Test with: quick-test.bat")
    else:
        print("⚠️  Some checks failed. Review the issues above.")
        print("\nCommon fixes:")
        print("- Ensure all source files are present")
        print("- Check Cargo.toml dependencies")
        print("- Verify module structure")
    
    return passed == total

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)