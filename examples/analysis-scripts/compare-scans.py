#!/usr/bin/env python3
"""
TriageIR Scan Comparison Script

This script compares two TriageIR scan results to identify changes
between scans, useful for monitoring system changes over time.

Usage:
    python compare-scans.py baseline.json current.json [--output changes.json] [--format text|json|html]
"""

import json
import sys
import argparse
import datetime
from pathlib import Path
from collections import defaultdict

class ScanComparator:
    def __init__(self, baseline_file, current_file):
        """Initialize comparator with baseline and current scan results."""
        with open(baseline_file, 'r', encoding='utf-8') as f:
            self.baseline = json.load(f)
        
        with open(current_file, 'r', encoding='utf-8') as f:
            self.current = json.load(f)
        
        self.changes = {
            'new_processes': [],
            'removed_processes': [],
            'new_connections': [],
            'removed_connections': [],
            'new_persistence': [],
            'removed_persistence': [],
            'new_execution': [],
            'system_changes': {},
            'summary': {}
        }
    
    def compare(self):
        """Perform comprehensive comparison between scans."""
        print("Comparing TriageIR scans...")
        
        self._compare_system_info()
        self._compare_processes()
        self._compare_network()
        self._compare_persistence()
        self._compare_execution_evidence()
        self._generate_summary()
        
        print("Comparison complete.")
    
    def _compare_system_info(self):
        """Compare system information between scans."""
        baseline_sys = self.baseline['artifacts']['system_info']
        current_sys = self.current['artifacts']['system_info']
        
        changes = {}
        
        # Compare uptime
        baseline_uptime = baseline_sys.get('uptime_secs', 0)
        current_uptime = current_sys.get('uptime_secs', 0)
        
        if abs(current_uptime - baseline_uptime) > 86400:  # More than 1 day difference
            changes['uptime_change'] = {
                'baseline': baseline_uptime,
                'current': current_uptime,
           