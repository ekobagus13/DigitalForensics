#!/usr/bin/env python3
"""
TriageIR JSON Validator
Validates TriageIR JSON output against schema and performs integrity checks
"""

import json
import sys
import argparse
from datetime import datetime
import re
from pathlib import Path

def validate_json_structure(data):
    """Validate basic JSON structure"""
    errors = []
    warnings = []
    
    # Check required top-level keys
    required_keys = ['scan_metadata', 'artifacts', 'collection_log']
    for key in required_keys:
        if key not in data:
            errors.append(f"Missing required top-level key: {key}")
    
    if 'scan_metadata' in data:
        metadata = data['scan_metadata']
        required_metadata = ['scan_id', 'scan_start_utc', 'scan_duration_ms', 'hostname', 'os_version', 'cli_version']
        for key in required_metadata:
            if key not in metadata:
                errors.append(f"Missing required metadata key: {key}")
        
        # Validate UUID format
        if 'scan_id' in metadata:
            uuid_pattern = r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
            if not re.match(uuid_pattern, metadata['scan_id'], re.IGNORECASE):
                errors.append(f"Invalid UUID format for scan_id: {metadata['scan_id']}")
        
        # Validate timestamp format
        if 'scan_start_utc' in metadata:
            try:
                datetime.fromisoformat(metadata['scan_start_utc'].replace('Z', '+00:00'))
            except ValueError:
                errors.append(f"Invalid timestamp format for scan_start_utc: {metadata['scan_start_utc']}")
        
        # Validate duration
        if 'scan_duration_ms' in metadata:
            if not isinstance(metadata['scan_duration_ms'], int) or metadata['scan_duration_ms'] < 0:
                errors.append(f"Invalid scan_duration_ms: {metadata['scan_duration_ms']}")
    
    if 'artifacts' in data:
        artifacts = data['artifacts']
        required_artifacts = ['system_info', 'running_processes', 'network_connections', 
                            'persistence_mechanisms', 'event_logs', 'execution_evidence']
        for key in required_artifacts:
            if key not in artifacts:
                errors.append(f"Missing required artifacts key: {key}")
    
    return errors, warnings

def validate_processes(processes):
    """Validate process data"""
    errors = []
    warnings = []
    
    if not isinstance(processes, list):
        errors.append("running_processes must be a list")
        return errors, warnings
    
    for i, process in enumerate(processes):
        if not isinstance(process, dict):
            errors.append(f"Process {i} is not a dictionary")
            continue
        
        # Check required fields
        required_fields = ['pid', 'parent_pid', 'name', 'command_line', 'executable_path', 'start_time']
        for field in required_fields:
            if field not in process:
                errors.append(f"Process {i} missing required field: {field}")
        
        # Validate PID
        if 'pid' in process:
            if not isinstance(process['pid'], int) or process['pid'] < 0:
                errors.append(f"Process {i} has invalid PID: {process['pid']}")
        
        # Validate parent PID
        if 'parent_pid' in process:
            if not isinstance(process['parent_pid'], int) or process['parent_pid'] < 0:
                errors.append(f"Process {i} has invalid parent PID: {process['parent_pid']}")
        
        # Validate SHA-256 hash format
        if 'sha256_hash' in process and process['sha256_hash'] is not None:
            if not re.match(r'^[a-fA-F0-9]{64}$', process['sha256_hash']):
                errors.append(f"Process {i} has invalid SHA-256 hash format: {process['sha256_hash']}")
        
        # Validate timestamp
        if 'start_time' in process:
            try:
                datetime.fromisoformat(process['start_time'].replace('Z', '+00:00'))
            except ValueError:
                errors.append(f"Process {i} has invalid start_time format: {process['start_time']}")
        
        # Check for suspicious indicators
        if 'executable_path' in process:
            path = process['executable_path'].lower()
            if any(sus in path for sus in ['temp', 'appdata\\local\\temp', 'downloads']):
                warnings.append(f"Process {i} ({process.get('name', 'unknown')}) in suspicious location: {process['executable_path']}")
        
        if 'sha256_hash' not in process or process['sha256_hash'] is None:
            warnings.append(f"Process {i} ({process.get('name', 'unknown')}) has no SHA-256 hash (unsigned)")
    
    return errors, warnings

def validate_network_connections(connections):
    """Validate network connection data"""
    errors = []
    warnings = []
    
    if not isinstance(connections, list):
        errors.append("network_connections must be a list")
        return errors, warnings
    
    valid_protocols = ['TCP', 'UDP']
    valid_states = ['LISTENING', 'ESTABLISHED', 'TIME_WAIT', 'CLOSE_WAIT', 
                   'FIN_WAIT1', 'FIN_WAIT2', 'SYN_SENT', 'SYN_RECV', 'LAST_ACK', 'CLOSED']
    
    for i, conn in enumerate(connections):
        if not isinstance(conn, dict):
            errors.append(f"Connection {i} is not a dictionary")
            continue
        
        # Check required fields
        required_fields = ['protocol', 'local_address', 'remote_address', 'state', 'owning_pid']
        for field in required_fields:
            if field not in conn:
                errors.append(f"Connection {i} missing required field: {field}")
        
        # Validate protocol
        if 'protocol' in conn and conn['protocol'] not in valid_protocols:
            errors.append(f"Connection {i} has invalid protocol: {conn['protocol']}")
        
        # Validate state
        if 'state' in conn and conn['state'] not in valid_states:
            errors.append(f"Connection {i} has invalid state: {conn['state']}")
        
        # Validate address format
        for addr_field in ['local_address', 'remote_address']:
            if addr_field in conn:
                if ':' not in conn[addr_field]:
                    errors.append(f"Connection {i} has invalid {addr_field} format: {conn[addr_field]}")
        
        # Validate PID
        if 'owning_pid' in conn:
            if not isinstance(conn['owning_pid'], int) or conn['owning_pid'] < 0:
                errors.append(f"Connection {i} has invalid owning_pid: {conn['owning_pid']}")
        
        # Check for external connections
        if 'remote_address' in conn:
            remote_ip = conn['remote_address'].split(':')[0]
            private_ranges = ['127.', '192.168.', '10.', '172.16.', '172.17.', '172.18.', '172.19.',
                            '172.20.', '172.21.', '172.22.', '172.23.', '172.24.', '172.25.',
                            '172.26.', '172.27.', '172.28.', '172.29.', '172.30.', '172.31.']
            
            if not any(remote_ip.startswith(prefix) for prefix in private_ranges):
                warnings.append(f"Connection {i} to external address: {conn['remote_address']}")
    
    return errors, warnings

def validate_persistence_mechanisms(mechanisms):
    """Validate persistence mechanism data"""
    errors = []
    warnings = []
    
    if not isinstance(mechanisms, list):
        errors.append("persistence_mechanisms must be a list")
        return errors, warnings
    
    valid_types = ['Registry Run Key', 'Scheduled Task', 'Service', 'Startup Folder', 'WMI Event', 'DLL Hijacking']
    
    for i, mech in enumerate(mechanisms):
        if not isinstance(mech, dict):
            errors.append(f"Persistence mechanism {i} is not a dictionary")
            continue
        
        # Check required fields
        required_fields = ['type', 'name', 'command', 'source']
        for field in required_fields:
            if field not in mech:
                errors.append(f"Persistence mechanism {i} missing required field: {field}")
        
        # Validate type
        if 'type' in mech and mech['type'] not in valid_types:
            warnings.append(f"Persistence mechanism {i} has unknown type: {mech['type']}")
        
        # Check for suspicious commands
        if 'command' in mech:
            command = mech['command'].lower()
            if any(sus in command for sus in ['powershell', 'cmd', 'temp', 'appdata']):
                warnings.append(f"Persistence mechanism {i} has suspicious command: {mech['command']}")
    
    return errors, warnings

def validate_event_logs(event_logs):
    """Validate event log data"""
    errors = []
    warnings = []
    
    if not isinstance(event_logs, dict):
        errors.append("event_logs must be a dictionary")
        return errors, warnings
    
    required_logs = ['security', 'system', 'application']
    valid_levels = ['Critical', 'Error', 'Warning', 'Information', 'Verbose']
    
    for log_type in required_logs:
        if log_type not in event_logs:
            errors.append(f"Missing event log type: {log_type}")
            continue
        
        if not isinstance(event_logs[log_type], list):
            errors.append(f"Event log {log_type} must be a list")
            continue
        
        for i, event in enumerate(event_logs[log_type]):
            if not isinstance(event, dict):
                errors.append(f"Event {i} in {log_type} log is not a dictionary")
                continue
            
            # Check required fields
            required_fields = ['event_id', 'level', 'timestamp', 'source', 'message']
            for field in required_fields:
                if field not in event:
                    errors.append(f"Event {i} in {log_type} log missing required field: {field}")
            
            # Validate event ID
            if 'event_id' in event:
                if not isinstance(event['event_id'], int) or event['event_id'] < 0:
                    errors.append(f"Event {i} in {log_type} log has invalid event_id: {event['event_id']}")
            
            # Validate level
            if 'level' in event and event['level'] not in valid_levels:
                errors.append(f"Event {i} in {log_type} log has invalid level: {event['level']}")
            
            # Validate timestamp
            if 'timestamp' in event:
                try:
                    datetime.fromisoformat(event['timestamp'].replace('Z', '+00:00'))
                except ValueError:
                    errors.append(f"Event {i} in {log_type} log has invalid timestamp: {event['timestamp']}")
    
    return errors, warnings

def validate_execution_evidence(execution_evidence):
    """Validate execution evidence data"""
    errors = []
    warnings = []
    
    if not isinstance(execution_evidence, dict):
        errors.append("execution_evidence must be a dictionary")
        return errors, warnings
    
    # Validate prefetch files
    if 'prefetch_files' in execution_evidence:
        prefetch_files = execution_evidence['prefetch_files']
        if not isinstance(prefetch_files, list):
            errors.append("prefetch_files must be a list")
        else:
            for i, pf in enumerate(prefetch_files):
                if not isinstance(pf, dict):
                    errors.append(f"Prefetch file {i} is not a dictionary")
                    continue
                
                # Check required fields
                required_fields = ['filename', 'executable_name', 'run_count', 'last_run_time', 'file_paths']
                for field in required_fields:
                    if field not in pf:
                        errors.append(f"Prefetch file {i} missing required field: {field}")
                
                # Validate filename format
                if 'filename' in pf and not pf['filename'].endswith('.pf'):
                    errors.append(f"Prefetch file {i} has invalid filename format: {pf['filename']}")
                
                # Validate run count
                if 'run_count' in pf:
                    if not isinstance(pf['run_count'], int) or pf['run_count'] < 0:
                        errors.append(f"Prefetch file {i} has invalid run_count: {pf['run_count']}")
                
                # Validate timestamp
                if 'last_run_time' in pf:
                    try:
                        datetime.fromisoformat(pf['last_run_time'].replace('Z', '+00:00'))
                    except ValueError:
                        errors.append(f"Prefetch file {i} has invalid last_run_time: {pf['last_run_time']}")
    
    # Validate shimcache entries
    if 'shimcache_entries' in execution_evidence:
        shimcache_entries = execution_evidence['shimcache_entries']
        if not isinstance(shimcache_entries, list):
            errors.append("shimcache_entries must be a list")
        else:
            for i, entry in enumerate(shimcache_entries):
                if not isinstance(entry, dict):
                    errors.append(f"Shimcache entry {i} is not a dictionary")
                    continue
                
                # Check required fields
                required_fields = ['path', 'last_modified', 'file_size']
                for field in required_fields:
                    if field not in entry:
                        errors.append(f"Shimcache entry {i} missing required field: {field}")
                
                # Validate file size
                if 'file_size' in entry:
                    if not isinstance(entry['file_size'], int) or entry['file_size'] < 0:
                        errors.append(f"Shimcache entry {i} has invalid file_size: {entry['file_size']}")
                
                # Validate timestamp
                if 'last_modified' in entry:
                    try:
                        datetime.fromisoformat(entry['last_modified'].replace('Z', '+00:00'))
                    except ValueError:
                        errors.append(f"Shimcache entry {i} has invalid last_modified: {entry['last_modified']}")
    
    return errors, warnings

def validate_collection_log(collection_log):
    """Validate collection log data"""
    errors = []
    warnings = []
    
    if not isinstance(collection_log, list):
        errors.append("collection_log must be a list")
        return errors, warnings
    
    valid_levels = ['ERROR', 'WARN', 'INFO', 'DEBUG']
    
    for i, log_entry in enumerate(collection_log):
        if not isinstance(log_entry, dict):
            errors.append(f"Log entry {i} is not a dictionary")
            continue
        
        # Check required fields
        required_fields = ['timestamp', 'level', 'message']
        for field in required_fields:
            if field not in log_entry:
                errors.append(f"Log entry {i} missing required field: {field}")
        
        # Validate level
        if 'level' in log_entry and log_entry['level'] not in valid_levels:
            errors.append(f"Log entry {i} has invalid level: {log_entry['level']}")
        
        # Validate timestamp
        if 'timestamp' in log_entry:
            try:
                datetime.fromisoformat(log_entry['timestamp'].replace('Z', '+00:00'))
            except ValueError:
                errors.append(f"Log entry {i} has invalid timestamp: {log_entry['timestamp']}")
    
    return errors, warnings

def perform_integrity_checks(data):
    """Perform cross-reference integrity checks"""
    errors = []
    warnings = []
    
    if 'artifacts' not in data:
        return errors, warnings
    
    artifacts = data['artifacts']
    
    # Check if all network connection PIDs exist in process list
    if 'running_processes' in artifacts and 'network_connections' in artifacts:
        process_pids = {p['pid'] for p in artifacts['running_processes'] if 'pid' in p}
        
        for i, conn in enumerate(artifacts['network_connections']):
            if 'owning_pid' in conn and conn['owning_pid'] not in process_pids:
                warnings.append(f"Network connection {i} references non-existent PID: {conn['owning_pid']}")
    
    # Check for reasonable data ranges
    if 'running_processes' in artifacts:
        process_count = len(artifacts['running_processes'])
        if process_count == 0:
            warnings.append("No processes found - this is unusual for a Windows system")
        elif process_count > 1000:
            warnings.append(f"Very high process count ({process_count}) - verify this is expected")
    
    if 'network_connections' in artifacts:
        conn_count = len(artifacts['network_connections'])
        if conn_count > 500:
            warnings.append(f"Very high connection count ({conn_count}) - verify this is expected")
    
    return errors, warnings

def main():
    parser = argparse.ArgumentParser(description='Validate TriageIR JSON output')
    parser.add_argument('input_file', help='TriageIR JSON output file to validate')
    parser.add_argument('-v', '--verbose', action='store_true', help='Show detailed validation results')
    parser.add_argument('--warnings-as-errors', action='store_true', help='Treat warnings as errors')
    
    args = parser.parse_args()
    
    try:
        print(f"Validating TriageIR JSON file: {args.input_file}")
        
        # Load JSON file
        with open(args.input_file, 'r') as f:
            data = json.load(f)
        
        print("✓ JSON file loaded successfully")
        
        all_errors = []
        all_warnings = []
        
        # Validate JSON structure
        print("\n📋 Validating JSON structure...")
        errors, warnings = validate_json_structure(data)
        all_errors.extend(errors)
        all_warnings.extend(warnings)
        
        if 'artifacts' in data:
            artifacts = data['artifacts']
            
            # Validate processes
            if 'running_processes' in artifacts:
                print("🔍 Validating processes...")
                errors, warnings = validate_processes(artifacts['running_processes'])
                all_errors.extend(errors)
                all_warnings.extend(warnings)
            
            # Validate network connections
            if 'network_connections' in artifacts:
                print("🌐 Validating network connections...")
                errors, warnings = validate_network_connections(artifacts['network_connections'])
                all_errors.extend(errors)
                all_warnings.extend(warnings)
            
            # Validate persistence mechanisms
            if 'persistence_mechanisms' in artifacts:
                print("🔄 Validating persistence mechanisms...")
                errors, warnings = validate_persistence_mechanisms(artifacts['persistence_mechanisms'])
                all_errors.extend(errors)
                all_warnings.extend(warnings)
            
            # Validate event logs
            if 'event_logs' in artifacts:
                print("📝 Validating event logs...")
                errors, warnings = validate_event_logs(artifacts['event_logs'])
                all_errors.extend(errors)
                all_warnings.extend(warnings)
            
            # Validate execution evidence
            if 'execution_evidence' in artifacts:
                print("⚡ Validating execution evidence...")
                errors, warnings = validate_execution_evidence(artifacts['execution_evidence'])
                all_errors.extend(errors)
                all_warnings.extend(warnings)
        
        # Validate collection log
        if 'collection_log' in data:
            print("📋 Validating collection log...")
            errors, warnings = validate_collection_log(data['collection_log'])
            all_errors.extend(errors)
            all_warnings.extend(warnings)
        
        # Perform integrity checks
        print("🔍 Performing integrity checks...")
        errors, warnings = perform_integrity_checks(data)
        all_errors.extend(errors)
        all_warnings.extend(warnings)
        
        # Display results
        print("\n" + "="*60)
        print("VALIDATION RESULTS")
        print("="*60)
        
        if args.verbose or all_errors:
            if all_errors:
                print(f"\n❌ ERRORS ({len(all_errors)}):")
                for error in all_errors:
                    print(f"  • {error}")
        
        if args.verbose or all_warnings:
            if all_warnings:
                print(f"\n⚠️  WARNINGS ({len(all_warnings)}):")
                for warning in all_warnings:
                    print(f"  • {warning}")
        
        # Summary
        print(f"\nSUMMARY:")
        print(f"  Errors: {len(all_errors)}")
        print(f"  Warnings: {len(all_warnings)}")
        
        if len(all_errors) == 0 and (not args.warnings_as_errors or len(all_warnings) == 0):
            print("\n✅ VALIDATION PASSED")
            if len(all_warnings) > 0:
                print(f"   (with {len(all_warnings)} warnings)")
            sys.exit(0)
        else:
            print("\n❌ VALIDATION FAILED")
            if args.warnings_as_errors and len(all_warnings) > 0:
                print("   (warnings treated as errors)")
            sys.exit(1)
    
    except FileNotFoundError:
        print(f"❌ Error: File not found: {args.input_file}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"❌ Error: Invalid JSON format: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()