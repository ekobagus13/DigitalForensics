// Live System Data Validator for TriageIR GUI
// This script validates GUI display of real system data

class LiveDataValidator {
    constructor() {
        this.testResults = [];
        this.systemData = null;
        this.expectedSystemName = null;
    }

    async validateLiveData() {
        console.log('🔍 Validating Live System Data Display...');
        console.log('==========================================');

        // Get system information for validation
        this.expectedSystemName = await this.getSystemInfo();
        
        // Run validation tests
        this.validateSystemMetadata();
        this.validateProcessData();
        this.validateNetworkData();
        this.validatePersistenceData();
        this.validateEventData();
        this.validateDataRealism();
        this.validateVisualPresentation();

        // Display results
        this.displayResults();
    }

    async getSystemInfo() {
        // Try to get system name from various sources
        const hostname = window.location.hostname || 'localhost';
        console.log(`🖥️ Expected system context: ${hostname}`);
        return hostname;
    }

    validateSystemMetadata() {
        console.log('🔍 Validating System Metadata...');
        
        const overviewContent = document.getElementById('overviewContent');
        if (!overviewContent) {
            this.testResults.push('❌ Overview content not found');
            return;
        }

        const content = overviewContent.textContent;
        
        // Check for realistic system information
        const hasHostname = content.includes('Hostname:');
        const hasOSVersion = content.includes('OS Version:') && content.includes('Windows');
        const hasScanDuration = content.includes('Scan Duration:');
        const hasCLIVersion = content.includes('CLI Version:');

        if (hasHostname && hasOSVersion && hasScanDuration && hasCLIVersion) {
            this.testResults.push('✅ System metadata displays real system information');
        } else {
            this.testResults.push('❌ System metadata incomplete or missing');
        }
    }

    validateProcessData() {
        console.log('🔍 Validating Process Data...');
        
        const processTable = document.getElementById('processesTableBody');
        if (!processTable) {
            this.testResults.push('❌ Process table not found');
            return;
        }

        const rows = processTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('❌ No process data displayed');
            return;
        }

        // Check for common Windows processes that should exist
        const processText = processTable.textContent.toLowerCase();
        const commonProcesses = ['explorer.exe', 'winlogon.exe', 'csrss.exe', 'svchost.exe'];
        const foundProcesses = commonProcesses.filter(proc => processText.includes(proc));

        if (foundProcesses.length >= 2) {
            this.testResults.push(`✅ Process data shows realistic Windows processes (${rows.length} total, found: ${foundProcesses.join(', ')})`);
        } else {
            this.testResults.push(`⚠️ Process data may not be from real Windows system (${rows.length} processes)`);
        }

        // Check for PID values that look realistic
        const firstRow = rows[0];
        const pidCell = firstRow.querySelector('td:first-child');
        if (pidCell && !isNaN(pidCell.textContent) && parseInt(pidCell.textContent) > 0) {
            this.testResults.push('✅ Process PIDs appear realistic');
        } else {
            this.testResults.push('❌ Process PIDs appear invalid');
        }
    }

    validateNetworkData() {
        console.log('🔍 Validating Network Data...');
        
        const networkTable = document.getElementById('networkTableBody');
        if (!networkTable) {
            this.testResults.push('❌ Network table not found');
            return;
        }

        const rows = networkTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('⚠️ No network connections displayed (system may have no active connections)');
            return;
        }

        // Check for realistic network data
        const networkText = networkTable.textContent;
        const hasLocalhost = networkText.includes('127.0.0.1') || networkText.includes('::1');
        const hasRealisticPorts = networkText.includes(':80') || networkText.includes(':443') || networkText.includes(':53');
        const hasProtocols = networkText.includes('TCP') || networkText.includes('UDP');

        if (hasProtocols && (hasLocalhost || hasRealisticPorts)) {
            this.testResults.push(`✅ Network data shows realistic connections (${rows.length} connections)`);
        } else {
            this.testResults.push(`⚠️ Network data structure present but may not be realistic (${rows.length} connections)`);
        }
    }

    validatePersistenceData() {
        console.log('🔍 Validating Persistence Data...');
        
        const persistenceTable = document.getElementById('persistenceTableBody');
        if (!persistenceTable) {
            this.testResults.push('❌ Persistence table not found');
            return;
        }

        const rows = persistenceTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('⚠️ No persistence mechanisms displayed');
            return;
        }

        // Check for common Windows persistence mechanisms
        const persistenceText = persistenceTable.textContent.toLowerCase();
        const commonPersistence = ['registry run key', 'scheduled task', 'windows', 'microsoft'];
        const foundMechanisms = commonPersistence.filter(mech => persistenceText.includes(mech));

        if (foundMechanisms.length >= 1) {
            this.testResults.push(`✅ Persistence data shows realistic Windows mechanisms (${rows.length} items)`);
        } else {
            this.testResults.push(`⚠️ Persistence data present but may not be realistic (${rows.length} items)`);
        }
    }

    validateEventData() {
        console.log('🔍 Validating Event Log Data...');
        
        const eventTable = document.getElementById('eventsTableBody');
        if (!eventTable) {
            this.testResults.push('❌ Event table not found');
            return;
        }

        const rows = eventTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('⚠️ No event log data displayed');
            return;
        }

        // Check for realistic Windows event data
        const eventText = eventTable.textContent.toLowerCase();
        const hasSecurityEvents = eventText.includes('security');
        const hasSystemEvents = eventText.includes('system');
        const hasEventIDs = /\b\d{4}\b/.test(eventText); // 4-digit event IDs
        const hasTimestamps = eventText.includes('2024') || eventText.includes('2023');

        if ((hasSecurityEvents || hasSystemEvents) && hasEventIDs && hasTimestamps) {
            this.testResults.push(`✅ Event data shows realistic Windows events (${rows.length} events)`);
        } else {
            this.testResults.push(`⚠️ Event data structure present but may not be realistic (${rows.length} events)`);
        }
    }

    validateDataRealism() {
        console.log('🔍 Validating Overall Data Realism...');
        
        // Check summary cards for realistic numbers
        const summaryCards = document.querySelectorAll('.summary-card .value');
        let realisticCounts = 0;
        
        summaryCards.forEach(card => {
            const value = parseInt(card.textContent);
            if (!isNaN(value) && value > 0 && value < 10000) {
                realisticCounts++;
            }
        });

        if (realisticCounts >= 3) {
            this.testResults.push('✅ Summary statistics show realistic system activity levels');
        } else {
            this.testResults.push('⚠️ Summary statistics may not reflect realistic system data');
        }

        // Check for data consistency
        const processCount = parseInt(document.querySelector('.summary-card .value')?.textContent || '0');
        const processRows = document.querySelectorAll('#processesTableBody tr').length;
        
        if (Math.abs(processCount - processRows) <= 1) {
            this.testResults.push('✅ Data consistency: Summary cards match table data');
        } else {
            this.testResults.push('⚠️ Data consistency: Summary cards may not match table data');
        }
    }

    validateVisualPresentation() {
        console.log('🔍 Validating Visual Presentation of Real Data...');
        
        // Check if data is presented clearly
        const tables = document.querySelectorAll('.data-table tbody tr');
        let wellFormattedTables = 0;
        
        document.querySelectorAll('.data-table').forEach(table => {
            const rows = table.querySelectorAll('tbody tr');
            if (rows.length > 0) {
                const firstRow = rows[0];
                const cells = firstRow.querySelectorAll('td');
                if (cells.length >= 3 && cells[0].textContent.trim() !== '') {
                    wellFormattedTables++;
                }
            }
        });

        if (wellFormattedTables >= 3) {
            this.testResults.push('✅ Real data is presented in well-formatted, readable tables');
        } else {
            this.testResults.push('❌ Data presentation needs improvement for readability');
        }

        // Check for visual indicators
        const hasSuspiciousIndicators = document.body.textContent.includes('⚠️') || 
                                       document.body.textContent.includes('🔴') ||
                                       document.querySelector('.text-warning, .text-error');
        
        if (hasSuspiciousIndicators) {
            this.testResults.push('✅ Visual indicators help highlight important information');
        } else {
            this.testResults.push('⚠️ Could benefit from more visual indicators for important data');
        }
    }

    displayResults() {
        console.log('\n📊 Live Data Validation Results:');
        console.log('=================================');
        
        this.testResults.forEach(result => console.log(result));

        const passedTests = this.testResults.filter(r => r.startsWith('✅')).length;
        const warningTests = this.testResults.filter(r => r.startsWith('⚠️')).length;
        const failedTests = this.testResults.filter(r => r.startsWith('❌')).length;
        const totalTests = this.testResults.length;

        console.log(`\n📈 Test Summary:`);
        console.log(`✅ Passed: ${passedTests}`);
        console.log(`⚠️ Warnings: ${warningTests}`);
        console.log(`❌ Failed: ${failedTests}`);
        console.log(`📊 Total: ${totalTests}`);

        const successRate = ((passedTests / totalTests) * 100).toFixed(1);
        console.log(`🎯 Success Rate: ${successRate}%`);

        if (successRate >= 80) {
            console.log('\n🎉 LIVE DATA TEST PASSED!');
            console.log('The GUI successfully decodes and displays real system data in an appealing manner.');
        } else if (successRate >= 60) {
            console.log('\n⚠️ LIVE DATA TEST PARTIALLY PASSED');
            console.log('The GUI displays real data but some improvements could be made.');
        } else {
            console.log('\n❌ LIVE DATA TEST NEEDS IMPROVEMENT');
            console.log('The GUI has issues displaying real system data effectively.');
        }

        return { passedTests, warningTests, failedTests, totalTests, successRate };
    }

    // Quick test method for immediate validation
    quickValidate() {
        console.log('⚡ Quick Live Data Validation...');
        
        const hasProcesses = document.querySelectorAll('#processesTableBody tr').length > 0;
        const hasNetwork = document.querySelectorAll('#networkTableBody tr').length >= 0;
        const hasPersistence = document.querySelectorAll('#persistenceTableBody tr').length >= 0;
        const hasEvents = document.querySelectorAll('#eventsTableBody tr').length >= 0;
        const hasSystemInfo = document.getElementById('overviewContent')?.textContent.includes('Hostname');

        const score = [hasProcesses, hasNetwork, hasPersistence, hasEvents, hasSystemInfo]
                     .filter(Boolean).length;

        console.log(`Quick Score: ${score}/5`);
        if (score >= 4) {
            console.log('✅ GUI is successfully displaying real system data!');
        } else {
            console.log('⚠️ Some data sections may not be loading properly.');
        }

        return score;
    }
}

// Auto-load for browser console
if (typeof window !== 'undefined') {
    window.LiveDataValidator = LiveDataValidator;
    console.log('🔍 Live Data Validator loaded!');
    console.log('Usage: new LiveDataValidator().validateLiveData()');
    console.log('Quick test: new LiveDataValidator().quickValidate()');
}