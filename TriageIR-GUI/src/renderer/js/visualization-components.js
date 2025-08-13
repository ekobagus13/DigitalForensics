// Advanced Visualization Components for TriageIR GUI

class VisualizationComponents {
    constructor() {
        this.charts = {};
        this.setupEventListeners();
    }

    /**
     * Setup event listeners for visualization interactions
     */
    setupEventListeners() {
        // Table sorting
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('sortable-header')) {
                this.handleTableSort(e.target);
            }
        });

        // Row selection
        document.addEventListener('click', (e) => {
            if (e.target.closest('.data-table tbody tr')) {
                this.handleRowSelection(e.target.closest('tr'));
            }
        });

        // Context menu for table rows
        document.addEventListener('contextmenu', (e) => {
            if (e.target.closest('.data-table tbody tr')) {
                e.preventDefault();
                this.showContextMenu(e, e.target.closest('tr'));
            }
        });
    }

    /**
     * Create an enhanced data table with sorting, filtering, and pagination
     */
    createEnhancedTable(containerId, data, columns, options = {}) {
        const container = document.getElementById(containerId);
        if (!container) return;

        const tableConfig = {
            data: data,
            columns: columns,
            pageSize: options.pageSize || 50,
            sortable: options.sortable !== false,
            filterable: options.filterable !== false,
            selectable: options.selectable !== false,
            exportable: options.exportable !== false
        };

        // Create table structure
        const tableHTML = this.generateTableHTML(tableConfig);
        container.innerHTML = tableHTML;

        // Initialize table functionality
        this.initializeTableFeatures(containerId, tableConfig);

        return tableConfig;
    }

    /**
     * Generate HTML for enhanced table
     */
    generateTableHTML(config) {
        const tableId = `table-${Date.now()}`;
        
        return `
            <div class="enhanced-table-container">
                ${config.filterable ? this.generateTableFilters(config) : ''}
                <div class="table-controls">
                    <div class="table-info">
                        <span class="row-count">Showing ${Math.min(config.pageSize, config.data.length)} of ${config.data.length} rows</span>
                    </div>
                    <div class="table-actions">
                        ${config.exportable ? '<button class="btn btn-outline btn-small export-table">Export</button>' : ''}
                        <button class="btn btn-outline btn-small refresh-table">Refresh</button>
                    </div>
                </div>
                <div class="table-wrapper">
                    <table class="enhanced-data-table" id="${tableId}">
                        <thead>
                            <tr>
                                ${config.selectable ? '<th class="select-column"><input type="checkbox" class="select-all"></th>' : ''}
                                ${config.columns.map(col => `
                                    <th class="${config.sortable ? 'sortable-header' : ''}" data-column="${col.key}">
                                        ${col.title}
                                        ${config.sortable ? '<span class="sort-indicator"></span>' : ''}
                                    </th>
                                `).join('')}
                            </tr>
                        </thead>
                        <tbody class="table-body">
                            ${this.generateTableRows(config)}
                        </tbody>
                    </table>
                </div>
                ${this.generatePagination(config)}
            </div>
        `;
    }

    /**
     * Generate table filter controls
     */
    generateTableFilters(config) {
        return `
            <div class="table-filters">
                <div class="filter-group">
                    <input type="text" class="global-filter" placeholder="Search all columns...">
                    <button class="btn btn-outline btn-small clear-filters">Clear</button>
                </div>
                <div class="column-filters">
                    ${config.columns.filter(col => col.filterable !== false).map(col => `
                        <div class="filter-item">
                            <label>${col.title}:</label>
                            <input type="text" class="column-filter" data-column="${col.key}" placeholder="Filter ${col.title}...">
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
    }

    /**
     * Generate table rows
     */
    generateTableRows(config) {
        return config.data.slice(0, config.pageSize).map((row, index) => `
            <tr data-index="${index}" ${config.selectable ? 'class="selectable-row"' : ''}>
                ${config.selectable ? '<td><input type="checkbox" class="row-select"></td>' : ''}
                ${config.columns.map(col => `
                    <td class="cell-${col.key}" data-column="${col.key}">
                        ${this.formatCellValue(row[col.key], col)}
                    </td>
                `).join('')}
            </tr>
        `).join('');
    }

    /**
     * Format cell value based on column configuration
     */
    formatCellValue(value, column) {
        if (value === null || value === undefined) {
            return '<span class="null-value">-</span>';
        }

        switch (column.type) {
            case 'timestamp':
                return `<span class="timestamp-cell">${TriageUtils.formatTimestamp(value)}</span>`;
            case 'duration':
                return `<span class="duration-cell">${TriageUtils.formatDuration(value)}</span>`;
            case 'bytes':
                return `<span class="bytes-cell">${TriageUtils.formatBytes(value)}</span>`;
            case 'hash':
                return `<span class="hash-cell" title="${value}">${TriageUtils.truncateText(value, 16)}</span>`;
            case 'path':
                return `<span class="path-cell" title="${value}">${TriageUtils.truncateText(value, 50)}</span>`;
            case 'badge':
                return `<span class="cell-badge ${column.badgeClass || 'neutral'}">${value}</span>`;
            case 'boolean':
                return `<span class="boolean-cell ${value ? 'true' : 'false'}">${value ? '✓' : '✗'}</span>`;
            case 'number':
                return `<span class="number-cell">${typeof value === 'number' ? value.toLocaleString() : value}</span>`;
            default:
                return `<span class="text-cell">${TriageUtils.escapeHtml(String(value))}</span>`;
        }
    }

    /**
     * Generate pagination controls
     */
    generatePagination(config) {
        const totalPages = Math.ceil(config.data.length / config.pageSize);
        if (totalPages <= 1) return '';

        return `
            <div class="table-pagination">
                <div class="pagination-info">
                    Page 1 of ${totalPages}
                </div>
                <div class="pagination-controls">
                    <button class="btn btn-outline btn-small" disabled>Previous</button>
                    <button class="btn btn-outline btn-small" ${totalPages <= 1 ? 'disabled' : ''}>Next</button>
                </div>
                <div class="page-size-selector">
                    <label>Rows per page:</label>
                    <select class="page-size-select">
                        <option value="25" ${config.pageSize === 25 ? 'selected' : ''}>25</option>
                        <option value="50" ${config.pageSize === 50 ? 'selected' : ''}>50</option>
                        <option value="100" ${config.pageSize === 100 ? 'selected' : ''}>100</option>
                        <option value="250" ${config.pageSize === 250 ? 'selected' : ''}>250</option>
                    </select>
                </div>
            </div>
        `;
    }

    /**
     * Initialize table features (sorting, filtering, pagination)
     */
    initializeTableFeatures(containerId, config) {
        const container = document.getElementById(containerId);
        
        // Global filter
        const globalFilter = container.querySelector('.global-filter');
        if (globalFilter) {
            globalFilter.addEventListener('input', TriageUtils.debounce((e) => {
                this.applyGlobalFilter(containerId, config, e.target.value);
            }, 300));
        }

        // Column filters
        const columnFilters = container.querySelectorAll('.column-filter');
        columnFilters.forEach(filter => {
            filter.addEventListener('input', TriageUtils.debounce((e) => {
                this.applyColumnFilter(containerId, config, e.target.dataset.column, e.target.value);
            }, 300));
        });

        // Clear filters
        const clearButton = container.querySelector('.clear-filters');
        if (clearButton) {
            clearButton.addEventListener('click', () => {
                this.clearAllFilters(containerId, config);
            });
        }

        // Select all checkbox
        const selectAll = container.querySelector('.select-all');
        if (selectAll) {
            selectAll.addEventListener('change', (e) => {
                this.toggleSelectAll(containerId, e.target.checked);
            });
        }

        // Export button
        const exportButton = container.querySelector('.export-table');
        if (exportButton) {
            exportButton.addEventListener('click', () => {
                this.exportTableData(containerId, config);
            });
        }
    }

    /**
     * Handle table sorting
     */
    handleTableSort(header) {
        const column = header.dataset.column;
        const table = header.closest('table');
        const tbody = table.querySelector('tbody');
        const rows = Array.from(tbody.querySelectorAll('tr'));

        // Determine sort direction
        const currentSort = header.dataset.sort || 'none';
        const newSort = currentSort === 'asc' ? 'desc' : 'asc';

        // Clear other sort indicators
        table.querySelectorAll('.sortable-header').forEach(h => {
            h.dataset.sort = 'none';
            h.querySelector('.sort-indicator').textContent = '';
        });

        // Set new sort
        header.dataset.sort = newSort;
        header.querySelector('.sort-indicator').textContent = newSort === 'asc' ? '↑' : '↓';

        // Sort rows
        rows.sort((a, b) => {
            const aValue = a.querySelector(`[data-column="${column}"]`).textContent.trim();
            const bValue = b.querySelector(`[data-column="${column}"]`).textContent.trim();

            let comparison = 0;
            if (!isNaN(aValue) && !isNaN(bValue)) {
                comparison = parseFloat(aValue) - parseFloat(bValue);
            } else {
                comparison = aValue.localeCompare(bValue);
            }

            return newSort === 'asc' ? comparison : -comparison;
        });

        // Re-append sorted rows
        rows.forEach(row => tbody.appendChild(row));
    }

    /**
     * Handle row selection
     */
    handleRowSelection(row) {
        if (!row.classList.contains('selectable-row')) return;

        const checkbox = row.querySelector('.row-select');
        if (checkbox) {
            checkbox.checked = !checkbox.checked;
            row.classList.toggle('selected', checkbox.checked);
            this.updateSelectAllState(row.closest('.enhanced-table-container'));
        }
    }

    /**
     * Show context menu for table rows
     */
    showContextMenu(event, row) {
        const menu = this.createContextMenu(row);
        document.body.appendChild(menu);

        menu.style.left = `${event.pageX}px`;
        menu.style.top = `${event.pageY}px`;
        menu.style.display = 'block';

        // Remove menu when clicking elsewhere
        const removeMenu = (e) => {
            if (!menu.contains(e.target)) {
                menu.remove();
                document.removeEventListener('click', removeMenu);
            }
        };
        setTimeout(() => document.addEventListener('click', removeMenu), 0);
    }

    /**
     * Create context menu for table row
     */
    createContextMenu(row) {
        const menu = document.createElement('div');
        menu.className = 'context-menu';
        menu.innerHTML = `
            <div class="context-menu-item" data-action="copy-row">Copy Row</div>
            <div class="context-menu-item" data-action="copy-cell">Copy Cell</div>
            <div class="context-menu-separator"></div>
            <div class="context-menu-item" data-action="view-details">View Details</div>
            <div class="context-menu-item" data-action="highlight-related">Highlight Related</div>
        `;

        // Handle menu actions
        menu.addEventListener('click', (e) => {
            const action = e.target.dataset.action;
            if (action) {
                this.handleContextMenuAction(action, row);
                menu.remove();
            }
        });

        return menu;
    }

    /**
     * Handle context menu actions
     */
    handleContextMenuAction(action, row) {
        switch (action) {
            case 'copy-row':
                this.copyRowToClipboard(row);
                break;
            case 'copy-cell':
                // Implementation would depend on which cell was right-clicked
                break;
            case 'view-details':
                this.showRowDetails(row);
                break;
            case 'highlight-related':
                this.highlightRelatedRows(row);
                break;
        }
    }

    /**
     * Copy row data to clipboard
     */
    async copyRowToClipboard(row) {
        const cells = row.querySelectorAll('td:not(.select-column)');
        const rowData = Array.from(cells).map(cell => cell.textContent.trim()).join('\t');
        
        try {
            await TriageUtils.copyToClipboard(rowData);
            TriageUtils.showToast('Row copied to clipboard', 'success');
        } catch (error) {
            TriageUtils.showToast('Failed to copy row', 'error');
        }
    }

    /**
     * Show detailed view of row data
     */
    showRowDetails(row) {
        const cells = row.querySelectorAll('td:not(.select-column)');
        const headers = row.closest('table').querySelectorAll('th:not(.select-column)');
        
        let details = '<div class="row-details">';
        cells.forEach((cell, index) => {
            const header = headers[index]?.textContent.trim() || `Column ${index + 1}`;
            const value = cell.textContent.trim();
            details += `
                <div class="detail-item">
                    <strong>${header}:</strong>
                    <span>${value}</span>
                </div>
            `;
        });
        details += '</div>';

        // Show in a modal or popup
        this.showModal('Row Details', details);
    }

    /**
     * Apply global filter to table
     */
    applyGlobalFilter(containerId, config, filterValue) {
        const container = document.getElementById(containerId);
        const rows = container.querySelectorAll('.table-body tr');
        const searchTerm = filterValue.toLowerCase();

        let visibleCount = 0;
        rows.forEach(row => {
            const text = row.textContent.toLowerCase();
            const visible = text.includes(searchTerm);
            row.style.display = visible ? '' : 'none';
            if (visible) visibleCount++;
        });

        // Update row count
        const rowCount = container.querySelector('.row-count');
        if (rowCount) {
            rowCount.textContent = `Showing ${visibleCount} of ${config.data.length} rows`;
        }
    }

    /**
     * Apply column-specific filter
     */
    applyColumnFilter(containerId, config, column, filterValue) {
        const container = document.getElementById(containerId);
        const rows = container.querySelectorAll('.table-body tr');
        const searchTerm = filterValue.toLowerCase();

        let visibleCount = 0;
        rows.forEach(row => {
            const cell = row.querySelector(`[data-column="${column}"]`);
            if (cell) {
                const text = cell.textContent.toLowerCase();
                const visible = text.includes(searchTerm);
                row.style.display = visible ? '' : 'none';
                if (visible) visibleCount++;
            }
        });

        // Update row count
        const rowCount = container.querySelector('.row-count');
        if (rowCount) {
            rowCount.textContent = `Showing ${visibleCount} of ${config.data.length} rows`;
        }
    }

    /**
     * Clear all filters
     */
    clearAllFilters(containerId, config) {
        const container = document.getElementById(containerId);
        
        // Clear filter inputs
        container.querySelectorAll('.global-filter, .column-filter').forEach(input => {
            input.value = '';
        });

        // Show all rows
        container.querySelectorAll('.table-body tr').forEach(row => {
            row.style.display = '';
        });

        // Update row count
        const rowCount = container.querySelector('.row-count');
        if (rowCount) {
            rowCount.textContent = `Showing ${Math.min(config.pageSize, config.data.length)} of ${config.data.length} rows`;
        }
    }

    /**
     * Toggle select all checkbox state
     */
    toggleSelectAll(containerId, checked) {
        const container = document.getElementById(containerId);
        const checkboxes = container.querySelectorAll('.row-select');
        const rows = container.querySelectorAll('.selectable-row');

        checkboxes.forEach((checkbox, index) => {
            checkbox.checked = checked;
            rows[index].classList.toggle('selected', checked);
        });
    }

    /**
     * Update select all checkbox state based on individual selections
     */
    updateSelectAllState(container) {
        const selectAll = container.querySelector('.select-all');
        const checkboxes = container.querySelectorAll('.row-select');
        
        if (selectAll && checkboxes.length > 0) {
            const checkedCount = Array.from(checkboxes).filter(cb => cb.checked).length;
            selectAll.checked = checkedCount === checkboxes.length;
            selectAll.indeterminate = checkedCount > 0 && checkedCount < checkboxes.length;
        }
    }

    /**
     * Export table data
     */
    async exportTableData(containerId, config) {
        const container = document.getElementById(containerId);
        const visibleRows = Array.from(container.querySelectorAll('.table-body tr')).filter(row => 
            row.style.display !== 'none'
        );

        const headers = config.columns.map(col => col.title);
        const data = visibleRows.map(row => {
            const cells = row.querySelectorAll('td:not(.select-column)');
            return Array.from(cells).map(cell => cell.textContent.trim());
        });

        try {
            const exportManager = new ExportManager();
            await exportManager.exportCSV(data, 'table-export.csv', headers);
            TriageUtils.showToast('Table exported successfully', 'success');
        } catch (error) {
            TriageUtils.showToast('Export failed', 'error');
        }
    }

    /**
     * Show modal dialog
     */
    showModal(title, content) {
        const modal = document.createElement('div');
        modal.className = 'modal';
        modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>${title}</h2>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    ${content}
                </div>
            </div>
        `;

        document.body.appendChild(modal);
        modal.style.display = 'flex';

        // Close modal handlers
        const closeModal = () => {
            modal.remove();
        };

        modal.querySelector('.modal-close').addEventListener('click', closeModal);
        modal.addEventListener('click', (e) => {
            if (e.target === modal) closeModal();
        });

        return modal;
    }

    /**
     * Create a simple chart (using basic HTML/CSS)
     */
    createSimpleChart(containerId, data, type = 'bar') {
        const container = document.getElementById(containerId);
        if (!container) return;

        switch (type) {
            case 'bar':
                this.createBarChart(container, data);
                break;
            case 'pie':
                this.createPieChart(container, data);
                break;
            case 'line':
                this.createLineChart(container, data);
                break;
            default:
                console.warn(`Unsupported chart type: ${type}`);
        }
    }

    /**
     * Create a simple bar chart
     */
    createBarChart(container, data) {
        const maxValue = Math.max(...data.map(d => d.value));
        
        container.innerHTML = `
            <div class="simple-chart bar-chart">
                ${data.map(item => `
                    <div class="chart-bar">
                        <div class="bar-fill" style="height: ${(item.value / maxValue) * 100}%"></div>
                        <div class="bar-label">${item.label}</div>
                        <div class="bar-value">${item.value}</div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * Create a simple pie chart (using CSS)
     */
    createPieChart(container, data) {
        const total = data.reduce((sum, item) => sum + item.value, 0);
        let currentAngle = 0;

        const segments = data.map(item => {
            const percentage = (item.value / total) * 100;
            const angle = (item.value / total) * 360;
            const segment = {
                ...item,
                percentage,
                startAngle: currentAngle,
                endAngle: currentAngle + angle
            };
            currentAngle += angle;
            return segment;
        });

        container.innerHTML = `
            <div class="simple-chart pie-chart">
                <div class="pie-container">
                    <div class="pie-chart-visual">
                        ${segments.map((segment, index) => `
                            <div class="pie-segment" style="
                                --start-angle: ${segment.startAngle}deg;
                                --end-angle: ${segment.endAngle}deg;
                                --color: hsl(${(index * 360) / segments.length}, 70%, 50%);
                            "></div>
                        `).join('')}
                    </div>
                </div>
                <div class="pie-legend">
                    ${segments.map((segment, index) => `
                        <div class="legend-item">
                            <div class="legend-color" style="background: hsl(${(index * 360) / segments.length}, 70%, 50%);"></div>
                            <span class="legend-label">${segment.label}</span>
                            <span class="legend-value">${segment.value} (${segment.percentage.toFixed(1)}%)</span>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
    }
}

// Export for use in other modules
window.VisualizationComponents = VisualizationComponents;