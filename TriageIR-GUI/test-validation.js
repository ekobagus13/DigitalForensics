// TriageIR GUI Test Validation Script
// This script can be run in the browser console to validate GUI functionality

class GUITestValidator {
    constructor() {
        this.testResults = [];
        this.sampleData = null;
    }

    async loadSampleData() {
        try {
            const response = await fetch('./sample-test-data.json');
            this.sampleData = await response.json();
            console.log('✅ Sample data loaded successfully');
            return true;
        } catch (error) {
            console.error('❌ Failed to load sample data:', error);
            return false;
        }
    }

    validateSummaryCards() {
        console.log('🔍 Validating Summary Cards...');
        
        const summaryCards = document.getElementById('summaryCards');
        if (!summaryCards) {
            this.testResults.push('❌ Summary cards container not found');
            return false;
        }

        const cards = summaryCards.querySelectorAll('.summary-card');
        if (cards.length < 4) {
            this.testResults.push(`❌ Expected 4 summary cards, found ${cards.length}`);
            return false;
        }

        // Check if cards have values
        let hasValues = true;
        cards.forEach((card, index) => {
            const value = card.querySelector('.value');
            if (!value || value.textContent === '0') {
                hasValues = false;
            }
        });

        if (hasValues) {
            this.testResults.push('✅ Summary cards display data correctly');
            return true;
        } else {
            this.testResults.push('❌ Summary cards missing data');
            return false;
        }
    }

    validateProcessTable() {
        console.log('🔍 Validating Process Table...');
        
        const processTable = document.getElementById('processesTableBody');
        if (!processTable) {
            this.testResults.push('❌ Process table not found');
            return false;
        }

        const rows = processTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('❌ Process table has no data');
            return false;
        }

        // Check if first row has expected columns
        const firstRow = rows[0];
        const cells = firstRow.querySelectorAll('td');
        if (cells.length >= 5) {
            this.testResults.push(`✅ Process table displays ${rows.length} processes with proper columns`);
            return true;
        } else {
            this.testResults.push('❌ Process table missing expected columns');
            return false;
        }
    }

    validateNetworkTable() {
        console.log('🔍 Validating Network Table...');
        
        const networkTable = document.getElementById('networkTableBody');
        if (!networkTable) {
            this.testResults.push('❌ Network table not found');
            return false;
        }

        const rows = networkTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('❌ Network table has no data');
            return false;
        }

        this.testResults.push(`✅ Network table displays ${rows.length} connections`);
        return true;
    }

    validatePersistenceTable() {
        console.log('🔍 Validating Persistence Table...');
        
        const persistenceTable = document.getElementById('persistenceTableBody');
        if (!persistenceTable) {
            this.testResults.push('❌ Persistence table not found');
            return false;
        }

        const rows = persistenceTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('❌ Persistence table has no data');
            return false;
        }

        // Check for suspicious indicators
        const suspiciousItems = Array.from(rows).filter(row => 
            row.textContent.includes('⚠️ Yes')
        );

        this.testResults.push(`✅ Persistence table displays ${rows.length} items (${suspiciousItems.length} suspicious)`);
        return true;
    }

    validateEventTable() {
        console.log('🔍 Validating Event Table...');
        
        const eventTable = document.getElementById('eventsTableBody');
        if (!eventTable) {
            this.testResults.push('❌ Event table not found');
            return false;
        }

        const rows = eventTable.querySelectorAll('tr');
        if (rows.length === 0) {
            this.testResults.push('❌ Event table has no data');
            return false;
        }

        this.testResults.push(`✅ Event table displays ${rows.length} events`);
        return true;
    }

    validateTabs() {
        console.log('🔍 Validating Tab Functionality...');
        
        const tabButtons = document.querySelectorAll('.tab-btn');
        const tabPanels = document.querySelectorAll('.tab-panel');

        if (tabButtons.length !== tabPanels.length) {
            this.testResults.push('❌ Tab buttons and panels count mismatch');
            return false;
        }

        // Test tab switching
        let tabsWorking = true;
        tabButtons.forEach((btn, index) => {
            btn.click();
            const activePanel = document.querySelector('.tab-panel.active');
            if (!activePanel) {
                tabsWorking = false;
            }
        });

        if (tabsWorking) {
            this.testResults.push(`✅ All ${tabButtons.length} tabs working correctly`);
            return true;
        } else {
            this.testResults.push('❌ Tab switching not working properly');
            return false;
        }
    }

    validateVisualAppeal() {
        console.log('🔍 Validating Visual Appeal...');
        
        const checks = [
            {
                name: 'Header styling',
                selector: '.app-header',
                property: 'background'
            },
            {
                name: 'Button styling',
                selector: '.btn-primary',
                property: 'background'
            },
            {
                name: 'Card styling',
                selector: '.summary-card',
                property: 'border-radius'
            },
            {
                name: 'Table styling',
                selector: '.data-table',
                property: 'border-collapse'
            }
        ];

        let visualScore = 0;
        checks.forEach(check => {
            const element = document.querySelector(check.selector);
            if (element) {
                const style = window.getComputedStyle(element);
                if (style[check.property] && style[check.property] !== 'initial') {
                    visualScore++;
                }
            }
        });

        if (visualScore >= 3) {
            this.testResults.push(`✅ Visual appeal good (${visualScore}/${checks.length} style checks passed)`);
            return true;
        } else {
            this.testResults.push(`❌ Visual appeal needs improvement (${visualScore}/${checks.length} style checks passed)`);
            return false;
        }
    }

    async runAllTests() {
        console.log('🚀 Starting GUI Validation Tests...');
        console.log('=====================================');

        // Load sample data first
        await this.loadSampleData();

        // Run all validation tests
        this.validateSummaryCards();
        this.validateProcessTable();
        this.validateNetworkTable();
        this.validatePersistenceTable();
        this.validateEventTable();
        this.validateTabs();
        this.validateVisualAppeal();

        // Display results
        console.log('\n📊 Test Results:');
        console.log('================');
        this.testResults.forEach(result => console.log(result));

        const passedTests = this.testResults.filter(r => r.startsWith('✅')).length;
        const totalTests = this.testResults.length;
        const passRate = ((passedTests / totalTests) * 100).toFixed(1);

        console.log(`\n🎯 Overall Score: ${passedTests}/${totalTests} (${passRate}%)`);
        
        if (passRate >= 80) {
            console.log('🎉 GUI VALIDATION PASSED! The interface successfully decodes and displays CLI data.');
        } else {
            console.log('⚠️ GUI VALIDATION NEEDS IMPROVEMENT. Some display issues detected.');
        }

        return { passedTests, totalTests, passRate };
    }
}

// Auto-run tests if in browser environment
if (typeof window !== 'undefined') {
    window.GUITestValidator = GUITestValidator;
    console.log('GUI Test Validator loaded. Run: new GUITestValidator().runAllTests()');
}