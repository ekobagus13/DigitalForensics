#!/usr/bin/env python3
"""
TriageIR Report Generator
Generates comprehensive HTML reports from TriageIR JSON output
"""

import json
import sys
import argparse
from datetime import datetime
from pathlib import Path
import base64

def load_triageir_results(json_file):
    """Load and validate TriageIR JSON results"""
    try:
        with open(json_file, 'r') as f:
            data = json.load(f)
        
        # Basic validation
        required_keys = ['scan_metadata', 'artifacts', 'collection_log']
        for key in required_keys:
            if key not in data:
                raise ValueError(f"Missing required key: {key}")
        
        return data
    except json.JSONDecodeError as e:
        raise ValueError(f"Invalid JSON format: {e}")
    except FileNotFoundError:
        raise ValueError(f"File not found: {json_file}")

def analyze_processes(processes):
    """Analyze process data for suspicious indicators"""
    analysis = {
        'total_processes': len(processes),
        'unsigned_processes': [],
        'suspicious_locations': [],
        'high_memory_processes': [],
        'recent_processes': []
    }
    
    for process in processes:
        # Check for unsigned processes
        if not process.get('sha256_hash'):
            analysis['unsigned_processes'].append(process)
        
        # Check for suspicious locations
        path = process.get('executable_path', '').lower()
        if any(loc in path for loc in ['temp', 'appdata', 'downloads', 'users']):
            analysis['suspicious_locations'].append(process)
        
        # Check for high memory usage (>100MB)
        if process.get('memory_usage', 0) > 100 * 1024 * 1024:
            analysis['high_memory_processes'].append(process)
        
        # Check for recently started processes (within last hour)
        try:
            start_time = datetime.fromisoformat(process['start_time'].replace('Z', '+00:00'))
            scan_time = datetime.now().replace(tzinfo=start_time.tzinfo)
            if (scan_time - start_time).total_seconds() < 3600:
                analysis['recent_processes'].append(process)
        except (ValueError, KeyError):
            pass
    
    return analysis

def analyze_network_connections(connections):
    """Analyze network connections for anomalies"""
    analysis = {
        'total_connections': len(connections),
        'external_connections': [],
        'listening_ports': [],
        'established_connections': []
    }
    
    private_ranges = [
        '127.', '192.168.', '10.',
        '172.16.', '172.17.', '172.18.', '172.19.',
        '172.20.', '172.21.', '172.22.', '172.23.',
        '172.24.', '172.25.', '172.26.', '172.27.',
        '172.28.', '172.29.', '172.30.', '172.31.'
    ]
    
    for conn in connections:
        remote_ip = conn['remote_address'].split(':')[0]
        
        # Check for external connections
        if not any(remote_ip.startswith(prefix) for prefix in private_ranges):
            analysis['external_connections'].append(conn)
        
        # Check for listening ports
        if conn['state'] == 'LISTENING':
            analysis['listening_ports'].append(conn)
        
        # Check for established connections
        if conn['state'] == 'ESTABLISHED':
            analysis['established_connections'].append(conn)
    
    return analysis

def analyze_persistence_mechanisms(persistence):
    """Analyze persistence mechanisms"""
    analysis = {
        'total_mechanisms': len(persistence),
        'by_type': {},
        'suspicious_mechanisms': []
    }
    
    for mech in persistence:
        mech_type = mech.get('type', 'Unknown')
        if mech_type not in analysis['by_type']:
            analysis['by_type'][mech_type] = []
        analysis['by_type'][mech_type].append(mech)
        
        # Check for suspicious characteristics
        command = mech.get('command', '').lower()
        if any(sus in command for sus in ['temp', 'appdata', 'downloads', 'powershell', 'cmd']):
            analysis['suspicious_mechanisms'].append(mech)
    
    return analysis

def generate_html_report(data, output_file):
    """Generate comprehensive HTML report"""
    
    # Analyze data
    process_analysis = analyze_processes(data['artifacts']['running_processes'])
    network_analysis = analyze_network_connections(data['artifacts']['network_connections'])
    persistence_analysis = analyze_persistence_mechanisms(data['artifacts']['persistence_mechanisms'])
    
    # Count errors and warnings in collection log
    log_stats = {'ERROR': 0, 'WARN': 0, 'INFO': 0, 'DEBUG': 0}
    for log_entry in data['collection_log']:
        level = log_entry.get('level', 'INFO')
        log_stats[level] = log_stats.get(level, 0) + 1
    
    # Generate HTML
    html_content = f"""
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TriageIR Report - {data['scan_metadata']['hostname']}</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        .header {{
            border-bottom: 3px solid #2c3e50;
            padding-bottom: 20px;
            margin-bottom: 30px;
        }}
        .header h1 {{
            color: #2c3e50;
            margin: 0;
            font-size: 2.5em;
        }}
        .header .subtitle {{
            color: #7f8c8d;
            font-size: 1.2em;
            margin-top: 10px;
        }}
        .summary-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .summary-card {{
            background: #ecf0f1;
            padding: 20px;
            border-radius: 6px;
            border-left: 4px solid #3498db;
        }}
        .summary-card.warning {{
            border-left-color: #f39c12;
        }}
        .summary-card.danger {{
            border-left-color: #e74c3c;
        }}
        .summary-card h3 {{
            margin: 0 0 10px 0;
            color: #2c3e50;
        }}
        .summary-card .value {{
            font-size: 2em;
            font-weight: bold;
            color: #2c3e50;
        }}
        .section {{
            margin-bottom: 40px;
        }}
        .section h2 {{
            color: #2c3e50;
            border-bottom: 2px solid #ecf0f1;
            padding-bottom: 10px;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-top: 15px;
        }}
        th, td {{
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background-color: #34495e;
            color: white;
            font-weight: 600;
        }}
        tr:hover {{
            background-color: #f8f9fa;
        }}
        .highlight {{
            background-color: #fff3cd;
            padding: 2px 4px;
            border-radius: 3px;
        }}
        .danger-highlight {{
            background-color: #f8d7da;
            padding: 2px 4px;
            border-radius: 3px;
        }}
        .metadata {{
            background: #f8f9fa;
            padding: 15px;
            border-radius: 6px;
            margin-bottom: 20px;
        }}
        .metadata table {{
            margin: 0;
        }}
        .metadata th, .metadata td {{
            border: none;
            padding: 8px 12px;
        }}
        .log-entry {{
            padding: 8px 12px;
            margin: 5px 0;
            border-radius: 4px;
            font-family: monospace;
            font-size: 0.9em;
        }}
        .log-error {{
            background-color: #f8d7da;
            border-left: 4px solid #dc3545;
        }}
        .log-warn {{
            background-color: #fff3cd;
            border-left: 4px solid #ffc107;
        }}
        .log-info {{
            background-color: #d1ecf1;
            border-left: 4px solid #17a2b8;
        }}
        .expandable {{
            cursor: pointer;
            user-select: none;
        }}
        .expandable:hover {{
            background-color: #e9ecef;
        }}
        .collapsed {{
            display: none;
        }}
    </style>
    <script>
        function toggleSection(id) {{
            const element = document.getElementById(id);
            element.classList.toggle('collapsed');
        }}
    </script>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>TriageIR Forensic Report</h1>
            <div class="subtitle">System: {data['scan_metadata']['hostname']} | Scan ID: {data['scan_metadata']['scan_id']}</div>
        </div>

        <div class="metadata">
            <h3>Scan Information</h3>
            <table>
                <tr><th>Hostname</th><td>{data['scan_metadata']['hostname']}</td></tr>
                <tr><th>OS Version</th><td>{data['scan_metadata']['os_version']}</td></tr>
                <tr><th>Scan Start</th><td>{data['scan_metadata']['scan_start_utc']}</td></tr>
                <tr><th>Duration</th><td>{data['scan_metadata']['scan_duration_ms']} ms</td></tr>
                <tr><th>CLI Version</th><td>{data['scan_metadata']['cli_version']}</td></tr>
                <tr><th>System Uptime</th><td>{data['artifacts']['system_info']['uptime_secs']} seconds</td></tr>
            </table>
        </div>

        <div class="summary-grid">
            <div class="summary-card">
                <h3>Total Processes</h3>
                <div class="value">{process_analysis['total_processes']}</div>
            </div>
            <div class="summary-card {'warning' if len(process_analysis['unsigned_processes']) > 0 else ''}">
                <h3>Unsigned Processes</h3>
                <div class="value">{len(process_analysis['unsigned_processes'])}</div>
            </div>
            <div class="summary-card">
                <h3>Network Connections</h3>
                <div class="value">{network_analysis['total_connections']}</div>
            </div>
            <div class="summary-card {'warning' if len(network_analysis['external_connections']) > 0 else ''}">
                <h3>External Connections</h3>
                <div class="value">{len(network_analysis['external_connections'])}</div>
            </div>
            <div class="summary-card">
                <h3>Persistence Mechanisms</h3>
                <div class="value">{persistence_analysis['total_mechanisms']}</div>
            </div>
            <div class="summary-card {'danger' if log_stats['ERROR'] > 0 else 'warning' if log_stats['WARN'] > 0 else ''}">
                <h3>Collection Issues</h3>
                <div class="value">{log_stats['ERROR'] + log_stats['WARN']}</div>
            </div>
        </div>
"""

    # Suspicious Processes Section
    if process_analysis['unsigned_processes'] or process_analysis['suspicious_locations']:
        html_content += """
        <div class="section">
            <h2>⚠️ Suspicious Processes</h2>
"""
        
        if process_analysis['unsigned_processes']:
            html_content += """
            <h3>Unsigned Processes</h3>
            <table>
                <tr><th>PID</th><th>Name</th><th>Path</th><th>Command Line</th></tr>
"""
            for process in process_analysis['unsigned_processes'][:20]:  # Limit to 20
                html_content += f"""
                <tr>
                    <td>{process['pid']}</td>
                    <td class="danger-highlight">{process['name']}</td>
                    <td>{process.get('executable_path', 'N/A')}</td>
                    <td>{process.get('command_line', 'N/A')[:100]}{'...' if len(process.get('command_line', '')) > 100 else ''}</td>
                </tr>
"""
            html_content += "</table>"
        
        if process_analysis['suspicious_locations']:
            html_content += """
            <h3>Processes in Suspicious Locations</h3>
            <table>
                <tr><th>PID</th><th>Name</th><th>Path</th></tr>
"""
            for process in process_analysis['suspicious_locations'][:20]:
                html_content += f"""
                <tr>
                    <td>{process['pid']}</td>
                    <td>{process['name']}</td>
                    <td class="highlight">{process.get('executable_path', 'N/A')}</td>
                </tr>
"""
            html_content += "</table>"
        
        html_content += "</div>"

    # Network Connections Section
    if network_analysis['external_connections']:
        html_content += """
        <div class="section">
            <h2>🌐 External Network Connections</h2>
            <table>
                <tr><th>Protocol</th><th>Local Address</th><th>Remote Address</th><th>State</th><th>PID</th></tr>
"""
        for conn in network_analysis['external_connections'][:50]:  # Limit to 50
            html_content += f"""
            <tr>
                <td>{conn['protocol']}</td>
                <td>{conn['local_address']}</td>
                <td class="highlight">{conn['remote_address']}</td>
                <td>{conn['state']}</td>
                <td>{conn['owning_pid']}</td>
            </tr>
"""
        html_content += "</table></div>"

    # Persistence Mechanisms Section
    if persistence_analysis['total_mechanisms'] > 0:
        html_content += """
        <div class="section">
            <h2>🔄 Persistence Mechanisms</h2>
"""
        for mech_type, mechanisms in persistence_analysis['by_type'].items():
            html_content += f"""
            <h3 class="expandable" onclick="toggleSection('{mech_type.replace(' ', '_')}_table')">{mech_type} ({len(mechanisms)})</h3>
            <table id="{mech_type.replace(' ', '_')}_table">
                <tr><th>Name</th><th>Command</th><th>Source</th></tr>
"""
            for mech in mechanisms[:20]:  # Limit to 20 per type
                is_suspicious = mech in persistence_analysis['suspicious_mechanisms']
                highlight_class = 'danger-highlight' if is_suspicious else ''
                html_content += f"""
                <tr>
                    <td class="{highlight_class}">{mech.get('name', 'N/A')}</td>
                    <td>{mech.get('command', 'N/A')[:100]}{'...' if len(mech.get('command', '')) > 100 else ''}</td>
                    <td>{mech.get('source', 'N/A')}</td>
                </tr>
"""
            html_content += "</table>"
        html_content += "</div>"

    # System Information Section
    html_content += f"""
        <div class="section">
            <h2>💻 System Information</h2>
            <table>
                <tr><th>Property</th><th>Value</th></tr>
                <tr><td>Architecture</td><td>{data['artifacts']['system_info'].get('architecture', 'N/A')}</td></tr>
                <tr><td>Total Memory</td><td>{data['artifacts']['system_info'].get('total_memory', 0) // (1024**3)} GB</td></tr>
                <tr><td>Available Memory</td><td>{data['artifacts']['system_info'].get('available_memory', 0) // (1024**3)} GB</td></tr>
                <tr><td>Logged-on Users</td><td>{len(data['artifacts']['system_info'].get('logged_on_users', []))}</td></tr>
            </table>
        </div>
"""

    # Collection Log Section (if there are errors or warnings)
    if log_stats['ERROR'] > 0 or log_stats['WARN'] > 0:
        html_content += """
        <div class="section">
            <h2>📋 Collection Issues</h2>
"""
        for log_entry in data['collection_log']:
            level = log_entry.get('level', 'INFO')
            if level in ['ERROR', 'WARN']:
                css_class = f'log-{level.lower()}'
                html_content += f"""
                <div class="log-entry {css_class}">
                    <strong>{level}</strong> [{log_entry.get('timestamp', 'N/A')}]: {log_entry.get('message', 'N/A')}
                </div>
"""
        html_content += "</div>"

    # Footer
    html_content += f"""
        <div class="section">
            <hr>
            <p><em>Report generated on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')} by TriageIR Report Generator</em></p>
        </div>
    </div>
</body>
</html>
"""

    # Write HTML file
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(html_content)

def main():
    parser = argparse.ArgumentParser(description='Generate HTML report from TriageIR JSON output')
    parser.add_argument('input_file', help='TriageIR JSON output file')
    parser.add_argument('-o', '--output', default='triageir_report.html', 
                       help='Output HTML file (default: triageir_report.html)')
    
    args = parser.parse_args()
    
    try:
        print(f"Loading TriageIR results from {args.input_file}...")
        data = load_triageir_results(args.input_file)
        
        print(f"Generating HTML report...")
        generate_html_report(data, args.output)
        
        print(f"Report generated successfully: {args.output}")
        print(f"Open {args.output} in your web browser to view the report.")
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()