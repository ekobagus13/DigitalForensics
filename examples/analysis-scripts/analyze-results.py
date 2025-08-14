#!/usr/bin/env python3
"""
TriageIR Results Analysis Script

This script provides comprehensive analysis of TriageIR JSON output,
identifying potential security indicators and generating summary reports.

Usage:
    python analyze-results.py scan_results.json [--output report.txt] [--format text|json|html]
"""

import json
import sys
import argparse
import datetime
from collections import Counter, defaultdict
from pathlib import Path
import ipaddress

class TriageIRAnalyzer:
    def __init__(self, results_file):
        """Initialize analyzer with TriageIR results."""
        with open(results_file, 'r', encoding='utf-8') as f:
            self.results = json.load(f)
        
        self.indicators = {
            'suspicious_processes': [],
            'external_connections': [],
            'unusual_persistence': [],
            'execution_anomalies': [],
            'security_events': [],
            'errors': []
        }
        
        self.stats = {
            'total_processes': 0,
            'total_connections': 0,
            'total_persistence': 0,
            'total_events': 0,
            'scan_duration': 0
        }
    
    def analyze(self):
        """Perform comprehensive analysis of the results."""
        print("Analyzing TriageIR results...")
        
        self._analyze_metadata()
        self._analyze_processes()
        self._analyze_network()
        self._analyze_persistence()
        self._analyze_events()
        self._analyze_execution_evidence()
        self._analyze_collection_log()
        
        print("Analysis complete.")
    
    def _analyze_metadata(self):
        """Analyze scan metadata."""
        metadata = self.results['scan_metadata']
        self.stats['scan_duration'] = metadata['scan_duration_ms']
        
        # Check for unusually long scan times
        if metadata['scan_duration_ms'] > 300000:  # 5 minutes
            self.indicators['errors'].append({
                'type': 'performance',
                'message': f"Unusually long scan duration: {metadata['scan_duration_ms']}ms"
            })
    
    def _analyze_processes(self):
        """Analyze running processes for suspicious indicators."""
        processes = self.results['artifacts']['running_processes']
        self.stats['total_processes'] = len(processes)
        
        # Common system directories
        system_dirs = [
            'C:\\Windows\\System32\\',
            'C:\\Windows\\SysWOW64\\',
            'C:\\Program Files\\',
            'C:\\Program Files (x86)\\'
        ]
        
        for process in processes:
            # Check for processes outside system directories
            exe_path = process.get('executable_path', '').upper()
            if exe_path and not any(exe_path.startswith(d.upper()) for d in system_dirs):
                if not exe_path.startswith('C:\\USERS\\') or '\\APPDATA\\' not in exe_path:
                    self.indicators['suspicious_processes'].append({
                        'pid': process['pid'],
                        'name': process['name'],
                        'path': process['executable_path'],
                        'reason': 'Unusual location'
                    })
            
            # Check for processes without executable paths
            if not exe_path and process['name'] not in ['System', '[System Process]']:
                self.indicators['suspicious_processes'].append({
                    'pid': process['pid'],
                    'name': process['name'],
                    'path': process['executable_path'],
                    'reason': 'No executable path'
                })
            
            # Check for processes with unusual names
            name = process['name'].lower()
            if any(char in name for char in ['@', '#', '$', '%', '^', '&', '*']):
                self.indicators['suspicious_processes'].append({
                    'pid': process['pid'],
                    'name': process['name'],
                    'path': process['executable_path'],
                    'reason': 'Unusual characters in name'
                })
            
            # Check for high memory usage
            memory_mb = process.get('memory_usage', 0) / (1024 * 1024)
            if memory_mb > 1000:  # > 1GB
                self.indicators['suspicious_processes'].append({
                    'pid': process['pid'],
                    'name': process['name'],
                    'path': process['executable_path'],
                    'reason': f'High memory usage: {memory_mb:.1f}MB'
                })
    
    def _analyze_network(self):
        """Analyze network connections for suspicious activity."""
        connections = self.results['artifacts']['network_connections']
        self.stats['total_connections'] = len(connections)
        
        for conn in connections:
            remote_addr = conn['remote_address']
            if ':' in remote_addr:
                ip_part = remote_addr.split(':')[0]
                
                # Skip local addresses
                try:
                    ip = ipaddress.ip_address(ip_part)
                    if not ip.is_private and not ip.is_loopback:
                        self.indicators['external_connections'].append({
                            'local': conn['local_address'],
                            'remote': conn['remote_address'],
                            'protocol': conn['protocol'],
                            'state': conn['state'],
                            'pid': conn['owning_pid']
                        })
                except ValueError:
                    # Invalid IP address
                    pass
    
    def _analyze_persistence(self):
        """Analyze persistence mechanisms for unusual entries."""
        mechanisms = self.results['artifacts']['persistence_mechanisms']
        self.stats['total_persistence'] = len(mechanisms)
        
        # Known good persistence sources
        known_good = [
            'Microsoft',
            'Windows',
            'Intel',
            'NVIDIA',
            'AMD',
            'Realtek'
        ]
        
        for mech in mechanisms:
            source = mech.get('source', '')
            command = mech.get('command', '')
            
            # Check if persistence mechanism is from unknown vendor
            if not any(vendor.lower() in source.lower() or vendor.lower() in command.lower() 
                      for vendor in known_good):
                self.indicators['unusual_persistence'].append({
                    'type': mech['type'],
                    'name': mech['name'],
                    'command': command,
                    'source': source
                })
    
    def _analyze_events(self):
        """Analyze event logs for security indicators."""
        event_logs = self.results['artifacts']['event_logs']
        
        # Count total events
        total_events = 0
        for log_type in event_logs:
            total_events += len(event_logs[log_type])
        self.stats['total_events'] = total_events
        
        # Analyze security events
        security_events = event_logs.get('security', [])
        
        # Look for failed logon attempts
        failed_logons = [e for e in security_events if e['event_id'] == 4625]
        if len(failed_logons) > 10:
            self.indicators['security_events'].append({
                'type': 'Multiple failed logons',
                'count': len(failed_logons),
                'details': f"{len(failed_logons)} failed logon attempts detected"
            })
        
        # Look for privilege escalation
        privilege_events = [e for e in security_events if e['event_id'] in [4672, 4673, 4674]]
        if privilege_events:
            self.indicators['security_events'].append({
                'type': 'Privilege usage',
                'count': len(privilege_events),
                'details': f"{len(privilege_events)} privilege usage events"
            })
    
    def _analyze_execution_evidence(self):
        """Analyze execution evidence for anomalies."""
        execution = self.results['artifacts'].get('execution_evidence', {})
        
        # Analyze prefetch files
        prefetch_files = execution.get('prefetch_files', [])
        for pf in prefetch_files:
            # Check for executables with very high run counts
            if pf['run_count'] > 1000:
                self.indicators['execution_anomalies'].append({
                    'type': 'High execution count',
                    'executable': pf['executable_name'],
                    'count': pf['run_count'],
                    'last_run': pf['last_run_time']
                })
        
        # Analyze shimcache entries
        shimcache = execution.get('shimcache_entries', [])
        for entry in shimcache:
            path = entry['path'].lower()
            # Check for executables in unusual locations
            if ('temp' in path or 'downloads' in path) and entry.get('executed'):
                self.indicators['execution_anomalies'].append({
                    'type': 'Execution from temp/downloads',
                    'path': entry['path'],
                    'last_modified': entry['last_modified']
                })
    
    def _analyze_collection_log(self):
        """Analyze collection log for errors and warnings."""
        log_entries = self.results['collection_log']
        
        errors = [e for e in log_entries if e['level'] == 'ERROR']
        warnings = [e for e in log_entries if e['level'] == 'WARN']
        
        for error in errors:
            self.indicators['errors'].append({
                'type': 'collection_error',
                'message': error['message'],
                'module': error.get('module', 'unknown'),
                'timestamp': error['timestamp']
            })
        
        # Report if there were many warnings
        if len(warnings) > 5:
            self.indicators['errors'].append({
                'type': 'collection_warnings',
                'message': f"{len(warnings)} warnings during collection",
                'details': [w['message'] for w in warnings[:5]]  # First 5 warnings
            })
    
    def generate_report(self, format_type='text'):
        """Generate analysis report in specified format."""
        if format_type == 'text':
            return self._generate_text_report()
        elif format_type == 'json':
            return self._generate_json_report()
        elif format_type == 'html':
            return self._generate_html_report()
        else:
            raise ValueError(f"Unsupported format: {format_type}")
    
    def _generate_text_report(self):
        """Generate text format report."""
        report = []
        report.append("=" * 60)
        report.append("TriageIR Analysis Report")
        report.append("=" * 60)
        report.append("")
        
        # Metadata
        metadata = self.results['scan_metadata']
        report.append(f"Scan ID: {metadata['scan_id']}")
        report.append(f"Hostname: {metadata['hostname']}")
        report.append(f"OS Version: {metadata['os_version']}")
        report.append(f"Scan Time: {metadata['scan_start_utc']}")
        report.append(f"Duration: {metadata['scan_duration_ms']}ms")
        report.append("")
        
        # Statistics
        report.append("Collection Statistics:")
        report.append(f"  Processes: {self.stats['total_processes']}")
        report.append(f"  Network Connections: {self.stats['total_connections']}")
        report.append(f"  Persistence Mechanisms: {self.stats['total_persistence']}")
        report.append(f"  Event Log Entries: {self.stats['total_events']}")
        report.append("")
        
        # Indicators
        total_indicators = sum(len(indicators) for indicators in self.indicators.values())
        report.append(f"Security Indicators Found: {total_indicators}")
        report.append("")
        
        # Suspicious processes
        if self.indicators['suspicious_processes']:
            report.append("Suspicious Processes:")
            for proc in self.indicators['suspicious_processes'][:10]:  # Top 10
                report.append(f"  PID {proc['pid']}: {proc['name']} - {proc['reason']}")
                if proc['path']:
                    report.append(f"    Path: {proc['path']}")
            if len(self.indicators['suspicious_processes']) > 10:
                report.append(f"  ... and {len(self.indicators['suspicious_processes']) - 10} more")
            report.append("")
        
        # External connections
        if self.indicators['external_connections']:
            report.append("External Network Connections:")
            for conn in self.indicators['external_connections'][:10]:  # Top 10
                report.append(f"  {conn['protocol']} {conn['local']} -> {conn['remote']} (PID {conn['pid']})")
            if len(self.indicators['external_connections']) > 10:
                report.append(f"  ... and {len(self.indicators['external_connections']) - 10} more")
            report.append("")
        
        # Unusual persistence
        if self.indicators['unusual_persistence']:
            report.append("Unusual Persistence Mechanisms:")
            for pers in self.indicators['unusual_persistence'][:10]:  # Top 10
                report.append(f"  {pers['type']}: {pers['name']}")
                report.append(f"    Command: {pers['command']}")
            if len(self.indicators['unusual_persistence']) > 10:
                report.append(f"  ... and {len(self.indicators['unusual_persistence']) - 10} more")
            report.append("")
        
        # Security events
        if self.indicators['security_events']:
            report.append("Security Events:")
            for event in self.indicators['security_events']:
                report.append(f"  {event['type']}: {event['details']}")
            report.append("")
        
        # Execution anomalies
        if self.indicators['execution_anomalies']:
            report.append("Execution Anomalies:")
            for anom in self.indicators['execution_anomalies']:
                report.append(f"  {anom['type']}: {anom.get('executable', anom.get('path', 'Unknown'))}")
            report.append("")
        
        # Errors
        if self.indicators['errors']:
            report.append("Collection Issues:")
            for error in self.indicators['errors']:
                report.append(f"  {error['type']}: {error['message']}")
            report.append("")
        
        # Summary
        report.append("Analysis Summary:")
        if total_indicators == 0:
            report.append("  No significant security indicators detected.")
        elif total_indicators < 5:
            report.append("  Low risk: Few security indicators detected.")
        elif total_indicators < 15:
            report.append("  Medium risk: Several security indicators detected.")
        else:
            report.append("  High risk: Many security indicators detected.")
        
        report.append("")
        report.append("Recommendations:")
        if self.indicators['suspicious_processes']:
            report.append("  - Investigate suspicious processes and their origins")
        if self.indicators['external_connections']:
            report.append("  - Review external network connections for legitimacy")
        if self.indicators['unusual_persistence']:
            report.append("  - Examine unusual persistence mechanisms")
        if self.indicators['execution_anomalies']:
            report.append("  - Analyze execution evidence for malicious activity")
        if not total_indicators:
            report.append("  - System appears clean, continue regular monitoring")
        
        return "\n".join(report)
    
    def _generate_json_report(self):
        """Generate JSON format report."""
        report_data = {
            'metadata': self.results['scan_metadata'],
            'statistics': self.stats,
            'indicators': self.indicators,
            'summary': {
                'total_indicators': sum(len(indicators) for indicators in self.indicators.values()),
                'risk_level': self._calculate_risk_level(),
                'generated_at': datetime.datetime.utcnow().isoformat() + 'Z'
            }
        }
        return json.dumps(report_data, indent=2)
    
    def _generate_html_report(self):
        """Generate HTML format report."""
        total_indicators = sum(len(indicators) for indicators in self.indicators.values())
        risk_level = self._calculate_risk_level()
        
        html = f"""
<!DOCTYPE html>
<html>
<head>
    <title>TriageIR Analysis Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background-color: #f0f0f0; padding: 20px; border-radius: 5px; }}
        .section {{ margin: 20px 0; }}
        .indicator {{ background-color: #fff3cd; padding: 10px; margin: 5px 0; border-radius: 3px; }}
        .risk-low {{ color: green; }}
        .risk-medium {{ color: orange; }}
        .risk-high {{ color: red; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>TriageIR Analysis Report</h1>
        <p><strong>Hostname:</strong> {self.results['scan_metadata']['hostname']}</p>
        <p><strong>Scan Time:</strong> {self.results['scan_metadata']['scan_start_utc']}</p>
        <p><strong>Risk Level:</strong> <span class="risk-{risk_level.lower()}">{risk_level}</span></p>
    </div>
    
    <div class="section">
        <h2>Statistics</h2>
        <table>
            <tr><th>Metric</th><th>Count</th></tr>
            <tr><td>Processes</td><td>{self.stats['total_processes']}</td></tr>
            <tr><td>Network Connections</td><td>{self.stats['total_connections']}</td></tr>
            <tr><td>Persistence Mechanisms</td><td>{self.stats['total_persistence']}</td></tr>
            <tr><td>Event Log Entries</td><td>{self.stats['total_events']}</td></tr>
            <tr><td>Security Indicators</td><td>{total_indicators}</td></tr>
        </table>
    </div>
"""
        
        # Add indicators sections
        for indicator_type, indicators in self.indicators.items():
            if indicators:
                html += f"""
    <div class="section">
        <h2>{indicator_type.replace('_', ' ').title()}</h2>
"""
                for indicator in indicators[:10]:  # Limit to 10 per section
                    html += f'        <div class="indicator">{self._format_indicator_html(indicator)}</div>\n'
                
                if len(indicators) > 10:
                    html += f'        <p><em>... and {len(indicators) - 10} more</em></p>\n'
                
                html += "    </div>\n"
        
        html += """
</body>
</html>
"""
        return html
    
    def _format_indicator_html(self, indicator):
        """Format an indicator for HTML display."""
        if 'pid' in indicator:
            return f"PID {indicator['pid']}: {indicator['name']} - {indicator['reason']}"
        elif 'type' in indicator and 'details' in indicator:
            return f"{indicator['type']}: {indicator['details']}"
        elif 'message' in indicator:
            return indicator['message']
        else:
            return str(indicator)
    
    def _calculate_risk_level(self):
        """Calculate overall risk level based on indicators."""
        total_indicators = sum(len(indicators) for indicators in self.indicators.values())
        
        if total_indicators == 0:
            return "LOW"
        elif total_indicators < 5:
            return "LOW"
        elif total_indicators < 15:
            return "MEDIUM"
        else:
            return "HIGH"

def main():
    parser = argparse.ArgumentParser(description='Analyze TriageIR results for security indicators')
    parser.add_argument('results_file', help='Path to TriageIR JSON results file')
    parser.add_argument('--output', '-o', help='Output file path (default: stdout)')
    parser.add_argument('--format', '-f', choices=['text', 'json', 'html'], 
                       default='text', help='Output format (default: text)')
    
    args = parser.parse_args()
    
    if not Path(args.results_file).exists():
        print(f"Error: Results file '{args.results_file}' not found", file=sys.stderr)
        sys.exit(1)
    
    try:
        analyzer = TriageIRAnalyzer(args.results_file)
        analyzer.analyze()
        report = analyzer.generate_report(args.format)
        
        if args.output:
            with open(args.output, 'w', encoding='utf-8') as f:
                f.write(report)
            print(f"Report saved to {args.output}")
        else:
            print(report)
    
    except Exception as e:
        print(f"Error analyzing results: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()