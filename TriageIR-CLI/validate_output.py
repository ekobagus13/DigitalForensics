#!/usr/bin/env python3
"""
JSON Output Validator for TriageIR CLI

This script validates that the JSON output from TriageIR CLI conforms to the expected schema.
"""

import json
import sys
import argparse
from datetime import datetime
import uuid

def validate_scan_metadata(metadata):
    """Validate scan_metadata section"""
    required_fields = ['scan_id', 'scan_start_utc', 'scan_duration_ms', 'hostname', 'os_version', 'cli_version']
    
    for field in required_fields:
        if field not in metadata:
            return False, f"Missing required field: scan_metadata.{field}"
    
    # Validate UUID format
    try:
        uuid.UUID(metadata['scan_id'])
    except ValueError:
        return False, f"Invalid UUID format: {metadata['scan_id']}"
    
    # Validate timestamp format
    try:
        datetime.fromisoformat(metadata['scan_start_utc'].replace('Z', '+00:00'))
    except ValueError:
        return False, f"Invalid timestamp format: {metadata['scan_start_utc']}"
    
    # Validate duration is non-negative
    if not isinstance(metadata['scan_duration_ms'], int) or metadata['scan_duration_ms'] < 0:
        return False, f"Invalid scan duration: {metadata['scan_duration_ms']}"
    
    return True, "scan_metadata validation passed"

def validate_system_info(system_info):
    """Validate system_info section"""
    required_fields = ['uptime_secs', 'logged_on_users']
    
    for field in required_fields:
        if field not in system_info:
            return False, f"Missing required field: system_info.{field}"
    
    if not isinstance(system_info['uptime_secs'], int) or system_info['uptime_secs'] < 0:
        return False, f"Invalid uptime: {system_info['uptime_secs']}"
    
    if not isinstance(system_info['logged_on_users'], list):
        return False, "logged_on_users must be an array"
    
    # Validate user entries
    for i, user in enumerate(system_info['logged_on_users']):
        user_fields = ['username', 'domain', 'logon_time']
        for field in user_fields:
            if field not in user:
                return False, f"Missing field in logged_on_users[{i}]: {field}"
    
    return True, "system_info validation passed"

def validate_processes(processes):
    """Validate running_processes section"""
    if not isinstance(processes, list):
        return False, "running_processes must be an array"
    
    required_fields = ['pid', 'parent_pid', 'name', 'command_line', 'executable_path', 'sha256_hash']
    
    for i, process in enumerate(processes):
        for field in required_fields:
            if field not in process:
                return False, f"Missing field in running_processes[{i}]: {field}"
        
        # Validate PID is positive integer
        if not isinstance(process['pid'], int) or process['pid'] <= 0:
            return False, f"Invalid PID in running_processes[{i}]: {process['pid']}"
        
        # Validate parent PID is non-negative integer
        if not isinstance(process['parent_pid'], int) or process['parent_pid'] < 0:
            return False, f"Invalid parent PID in running_processes[{i}]: {process['parent_pid']}"
    
    return True, f"running_processes validation passed ({len(processes)} processes)"

def validate_network_connections(connections):
    """Validate network_connections section"""
    if not isinstance(connections, list):
        return False, "network_connections must be an array"
    
    required_fields = ['protocol', 'local_address', 'remote_address', 'state', 'owning_pid']
    
    for i, conn in enumerate(connections):
        for field in required_fields:
            if field not in conn:
                return False, f"Missing field in network_connections[{i}]: {field}"
        
        # Validate protocol
        if conn['protocol'] not in ['TCP', 'UDP']:
            return False, f"Invalid protocol in network_connections[{i}]: {conn['protocol']}"
        
        # Validate owning PID
        if not isinstance(conn['owning_pid'], int) or conn['owning_pid'] < 0:
            return False, f"Invalid owning PID in network_connections[{i}]: {conn['owning_pid']}"
    
    return True, f"network_connections validation passed ({len(connections)} connections)"

def validate_persistence_mechanisms(mechanisms):
    """Validate persistence_mechanisms section"""
    if not isinstance(mechanisms, list):
        return False, "persistence_mechanisms must be an array"
    
    required_fields = ['type', 'name', 'command', 'source']
    
    for i, mechanism in enumerate(mechanisms):
        for field in required_fields:
            if field not in mechanism:
                return False, f"Missing field in persistence_mechanisms[{i}]: {field}"
    
    return True, f"persistence_mechanisms validation passed ({len(mechanisms)} mechanisms)"

def validate_event_logs(event_logs):
    """Validate event_logs section"""
    required_fields = ['security', 'system']
    
    for field in required_fields:
        if field not in event_logs:
            return False, f"Missing required field: event_logs.{field}"
        
        if not isinstance(event_logs[field], list):
            return False, f"event_logs.{field} must be an array"
    
    # Validate event entries
    for log_type in ['security', 'system']:
        for i, event in enumerate(event_logs[log_type]):
            event_fields = ['event_id', 'level', 'timestamp', 'message']
            for field in event_fields:
                if field not in event:
                    return False, f"Missing field in event_logs.{log_type}[{i}]: {field}"
            
            # Validate event ID
            if not isinstance(event['event_id'], int) or event['event_id'] <= 0:
                return False, f"Invalid event ID in event_logs.{log_type}[{i}]: {event['event_id']}"
    
    total_events = len(event_logs['security']) + len(event_logs['system'])
    return True, f"event_logs validation passed ({total_events} total events)"

def validate_collection_log(collection_log):
    """Validate collection_log section"""
    if not isinstance(collection_log, list):
        return False, "collection_log must be an array"
    
    required_fields = ['timestamp', 'level', 'message']
    valid_levels = ['INFO', 'WARN', 'ERROR']
    
    for i, log_entry in enumerate(collection_log):
        for field in required_fields:
            if field not in log_entry:
                return False, f"Missing field in collection_log[{i}]: {field}"
        
        # Validate log level
        if log_entry['level'] not in valid_levels:
            return False, f"Invalid log level in collection_log[{i}]: {log_entry['level']}"
        
        # Validate timestamp format
        try:
            datetime.fromisoformat(log_entry['timestamp'].replace('Z', '+00:00'))
        except ValueError:
            return False, f"Invalid timestamp in collection_log[{i}]: {log_entry['timestamp']}"
    
    return True, f"collection_log validation passed ({len(collection_log)} entries)"

def validate_triageir_output(data):
    """Validate complete TriageIR output"""
    results = []
    
    # Check top-level structure
    required_sections = ['scan_metadata', 'artifacts', 'collection_log']
    for section in required_sections:
        if section not in data:
            return False, [f"Missing required top-level section: {section}"]
    
    # Validate scan_metadata
    valid, msg = validate_scan_metadata(data['scan_metadata'])
    results.append(msg)
    if not valid:
        return False, results
    
    # Validate artifacts section
    artifacts = data['artifacts']
    artifact_sections = ['system_info', 'running_processes', 'network_connections', 'persistence_mechanisms', 'event_logs']
    
    for section in artifact_sections:
        if section not in artifacts:
            results.append(f"Missing artifacts section: {section}")
            return False, results
    
    # Validate each artifact section
    validators = [
        (validate_system_info, artifacts['system_info']),
        (validate_processes, artifacts['running_processes']),
        (validate_network_connections, artifacts['network_connections']),
        (validate_persistence_mechanisms, artifacts['persistence_mechanisms']),
        (validate_event_logs, artifacts['event_logs']),
        (validate_collection_log, data['collection_log'])
    ]
    
    for validator_func, section_data in validators:
        valid, msg = validator_func(section_data)
        results.append(msg)
        if not valid:
            return False, results
    
    return True, results

def main():
    parser = argparse.ArgumentParser(description='Validate TriageIR CLI JSON output')
    parser.add_argument('json_file', help='Path to JSON file to validate')
    parser.add_argument('--verbose', '-v', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    try:
        with open(args.json_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except FileNotFoundError:
        print(f"Error: File not found: {args.json_file}")
        return 1
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON format: {e}")
        return 1
    
    valid, messages = validate_triageir_output(data)
    
    if args.verbose or not valid:
        for message in messages:
            print(message)
    
    if valid:
        print(f"✅ Validation passed: {args.json_file}")
        return 0
    else:
        print(f"❌ Validation failed: {args.json_file}")
        return 1

if __name__ == '__main__':
    sys.exit(main())