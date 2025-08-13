// JSON Data Parsing and Validation for TriageIR GUI

class DataValidator {
    constructor() {
        this.schema = this.getTriageIRSchema();
        this.validationErrors = [];
    }

    /**
     * Get the expected TriageIR JSON schema
     */
    getTriageIRSchema() {
        return {
            scan_metadata: {
                required: true,
                type: 'object',
                properties: {
                    scan_id: { type: 'string', required: true, pattern: /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i },
                    scan_start_utc: { type: 'string', required: true, format: 'iso8601' },
                    scan_duration_ms: { type: 'number', required: true, min: 0 },
                    hostname: { type: 'string', required: true, minLength: 1 },
                    os_version: { type: 'string', required: true, minLength: 1 },
                    cli_version: { type: 'string', required: true, pattern: /^\d+\.\d+\.\d+/ }
                }
            },
            artifacts: {
                required: true,
                type: 'object',
                properties: {
                    system_info: {
                        type: 'object',
                        required: true,
                        properties: {
                            uptime_secs: { type: 'number', required: true, min: 0 },
                            logged_on_users: {
                                type: 'array',
                                required: true,
                                items: {
                                    type: 'object',
                                    properties: {
                                        username: { type: 'string', required: true },
                                        domain: { type: 'string', required: true },
                                        logon_time: { type: 'string', required: true, format: 'iso8601' }
                                    }
                                }
                            }
                        }
                    },
                    running_processes: {
                        type: 'array',
                        required: true,
                        items: {
                            type: 'object',
                            properties: {
                                pid: { type: 'number', required: true, min: 0 },
                                parent_pid: { type: 'number', required: true, min: 0 },
                                name: { type: 'string', required: true },
                                command_line: { type: 'string', required: true },
                                executable_path: { type: 'string', required: true },
                                sha256_hash: { type: 'string', required: true }
                            }
                        }
                    },
                    network_connections: {
                        type: 'array',
                        required: true,
                        items: {
                            type: 'object',
                            properties: {
                                protocol: { type: 'string', required: true, enum: ['TCP', 'UDP'] },
                                local_address: { type: 'string', required: true },
                                remote_address: { type: 'string', required: true },
                                state: { type: 'string', required: true },
                                owning_pid: { type: 'number', required: true, min: 0 }
                            }
                        }
                    },
                    persistence_mechanisms: {
                        type: 'array',
                        required: true,
                        items: {
                            type: 'object',
                            properties: {
                                type: { type: 'string', required: true },
                                name: { type: 'string', required: true },
                                command: { type: 'string', required: true },
                                source: { type: 'string', required: true }
                            }
                        }
                    },
                    event_logs: {
                        type: 'object',
                        required: true,
                        properties: {
                            security: {
                                type: 'array',
                                required: true,
                                items: {
                                    type: 'object',
                                    properties: {
                                        event_id: { type: 'number', required: true, min: 0 },
                                        level: { type: 'string', required: true },
                                        timestamp: { type: 'string', required: true, format: 'iso8601' },
                                        message: { type: 'string', required: true }
                                    }
                                }
                            },
                            system: {
                                type: 'array',
                                required: true,
                                items: {
                                    type: 'object',
                                    properties: {
                                        event_id: { type: 'number', required: true, min: 0 },
                                        level: { type: 'string', required: true },
                                        timestamp: { type: 'string', required: true, format: 'iso8601' },
                                        message: { type: 'string', required: true }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            collection_log: {
                type: 'array',
                required: true,
                items: {
                    type: 'object',
                    properties: {
                        timestamp: { type: 'string', required: true, format: 'iso8601' },
                        level: { type: 'string', required: true, enum: ['INFO', 'WARN', 'ERROR'] },
                        message: { type: 'string', required: true }
                    }
                }
            }
        };
    }

    /**
     * Validate TriageIR JSON data
     */
    validateTriageIRData(data) {
        this.validationErrors = [];
        
        try {
            // Basic structure validation
            if (!data || typeof data !== 'object') {
                this.addError('root', 'Data must be a valid JSON object');
                return this.getValidationResult();
            }

            // Validate each top-level section
            this.validateObject(data, this.schema, 'root');
            
            // Additional business logic validation
            this.validateBusinessRules(data);
            
            return this.getValidationResult();
            
        } catch (error) {
            this.addError('validation', `Validation error: ${error.message}`);
            return this.getValidationResult();
        }
    }

    /**
     * Validate an object against a schema
     */
    validateObject(obj, schema, path) {
        for (const [key, rules] of Object.entries(schema)) {
            const currentPath = `${path}.${key}`;
            const value = obj[key];

            // Check required fields
            if (rules.required && (value === undefined || value === null)) {
                this.addError(currentPath, `Required field '${key}' is missing`);
                continue;
            }

            // Skip validation if field is not present and not required
            if (value === undefined || value === null) {
                continue;
            }

            // Type validation
            if (rules.type && !this.validateType(value, rules.type)) {
                this.addError(currentPath, `Field '${key}' must be of type ${rules.type}`);
                continue;
            }

            // Additional validations based on type
            if (rules.type === 'string') {
                this.validateString(value, rules, currentPath, key);
            } else if (rules.type === 'number') {
                this.validateNumber(value, rules, currentPath, key);
            } else if (rules.type === 'array') {
                this.validateArray(value, rules, currentPath, key);
            } else if (rules.type === 'object' && rules.properties) {
                this.validateObject(value, rules.properties, currentPath);
            }
        }
    }

    /**
     * Validate data type
     */
    validateType(value, expectedType) {
        switch (expectedType) {
            case 'string':
                return typeof value === 'string';
            case 'number':
                return typeof value === 'number' && !isNaN(value);
            case 'boolean':
                return typeof value === 'boolean';
            case 'array':
                return Array.isArray(value);
            case 'object':
                return typeof value === 'object' && !Array.isArray(value) && value !== null;
            default:
                return true;
        }
    }

    /**
     * Validate string fields
     */
    validateString(value, rules, path, key) {
        if (rules.minLength && value.length < rules.minLength) {
            this.addError(path, `Field '${key}' must be at least ${rules.minLength} characters long`);
        }

        if (rules.maxLength && value.length > rules.maxLength) {
            this.addError(path, `Field '${key}' must be no more than ${rules.maxLength} characters long`);
        }

        if (rules.pattern && !rules.pattern.test(value)) {
            this.addError(path, `Field '${key}' does not match required pattern`);
        }

        if (rules.enum && !rules.enum.includes(value)) {
            this.addError(path, `Field '${key}' must be one of: ${rules.enum.join(', ')}`);
        }

        if (rules.format) {
            this.validateFormat(value, rules.format, path, key);
        }
    }

    /**
     * Validate number fields
     */
    validateNumber(value, rules, path, key) {
        if (rules.min !== undefined && value < rules.min) {
            this.addError(path, `Field '${key}' must be at least ${rules.min}`);
        }

        if (rules.max !== undefined && value > rules.max) {
            this.addError(path, `Field '${key}' must be no more than ${rules.max}`);
        }
    }

    /**
     * Validate array fields
     */
    validateArray(value, rules, path, key) {
        if (rules.minItems && value.length < rules.minItems) {
            this.addError(path, `Array '${key}' must have at least ${rules.minItems} items`);
        }

        if (rules.maxItems && value.length > rules.maxItems) {
            this.addError(path, `Array '${key}' must have no more than ${rules.maxItems} items`);
        }

        // Validate array items
        if (rules.items) {
            value.forEach((item, index) => {
                const itemPath = `${path}[${index}]`;
                if (rules.items.type === 'object' && rules.items.properties) {
                    this.validateObject(item, rules.items.properties, itemPath);
                } else if (!this.validateType(item, rules.items.type)) {
                    this.addError(itemPath, `Array item must be of type ${rules.items.type}`);
                }
            });
        }
    }

    /**
     * Validate format constraints
     */
    validateFormat(value, format, path, key) {
        switch (format) {
            case 'iso8601':
                if (!this.isValidISO8601(value)) {
                    this.addError(path, `Field '${key}' must be a valid ISO 8601 timestamp`);
                }
                break;
            case 'uuid':
                if (!this.isValidUUID(value)) {
                    this.addError(path, `Field '${key}' must be a valid UUID`);
                }
                break;
            case 'sha256':
                if (!this.isValidSHA256(value)) {
                    this.addError(path, `Field '${key}' must be a valid SHA-256 hash`);
                }
                break;
        }
    }

    /**
     * Validate business rules
     */
    validateBusinessRules(data) {
        try {
            // Check scan duration consistency
            if (data.scan_metadata && data.scan_metadata.scan_duration_ms > 3600000) { // 1 hour
                this.addWarning('scan_metadata.scan_duration_ms', 'Scan duration is unusually long (> 1 hour)');
            }

            // Check for reasonable process counts
            if (data.artifacts && data.artifacts.running_processes) {
                const processCount = data.artifacts.running_processes.length;
                if (processCount > 1000) {
                    this.addWarning('artifacts.running_processes', `Very high process count: ${processCount}`);
                } else if (processCount === 0) {
                    this.addError('artifacts.running_processes', 'No processes found - this is unusual');
                }
            }

            // Check for suspicious network connections
            if (data.artifacts && data.artifacts.network_connections) {
                const externalConnections = data.artifacts.network_connections.filter(conn => 
                    TriageUtils.isExternalIP(conn.remote_address.split(':')[0])
                );
                if (externalConnections.length > 100) {
                    this.addWarning('artifacts.network_connections', `High number of external connections: ${externalConnections.length}`);
                }
            }

            // Check collection log for errors
            if (data.collection_log) {
                const errorCount = data.collection_log.filter(log => log.level === 'ERROR').length;
                if (errorCount > 0) {
                    this.addWarning('collection_log', `Collection contained ${errorCount} errors`);
                }
            }

            // Validate timestamp consistency
            this.validateTimestampConsistency(data);

        } catch (error) {
            this.addError('business_rules', `Business rule validation failed: ${error.message}`);
        }
    }

    /**
     * Validate timestamp consistency
     */
    validateTimestampConsistency(data) {
        try {
            const scanStart = new Date(data.scan_metadata.scan_start_utc);
            const scanDuration = data.scan_metadata.scan_duration_ms;
            const scanEnd = new Date(scanStart.getTime() + scanDuration);

            // Check if any log entries are outside the scan timeframe
            if (data.collection_log) {
                data.collection_log.forEach((log, index) => {
                    const logTime = new Date(log.timestamp);
                    if (logTime < scanStart || logTime > scanEnd) {
                        this.addWarning(`collection_log[${index}]`, 'Log timestamp outside scan timeframe');
                    }
                });
            }
        } catch (error) {
            this.addWarning('timestamp_validation', 'Could not validate timestamp consistency');
        }
    }

    /**
     * Check if string is valid ISO 8601 timestamp
     */
    isValidISO8601(dateString) {
        try {
            const date = new Date(dateString);
            return date.toISOString() === dateString || 
                   /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d{3})?Z?$/.test(dateString);
        } catch {
            return false;
        }
    }

    /**
     * Check if string is valid UUID
     */
    isValidUUID(uuid) {
        const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
        return uuidRegex.test(uuid);
    }

    /**
     * Check if string is valid SHA-256 hash
     */
    isValidSHA256(hash) {
        if (hash === 'N/A' || hash === 'ERROR') return true; // Allow these special values
        const sha256Regex = /^[a-f0-9]{64}$/i;
        return sha256Regex.test(hash);
    }

    /**
     * Add validation error
     */
    addError(path, message) {
        this.validationErrors.push({
            type: 'error',
            path: path,
            message: message
        });
    }

    /**
     * Add validation warning
     */
    addWarning(path, message) {
        this.validationErrors.push({
            type: 'warning',
            path: path,
            message: message
        });
    }

    /**
     * Get validation result
     */
    getValidationResult() {
        const errors = this.validationErrors.filter(e => e.type === 'error');
        const warnings = this.validationErrors.filter(e => e.type === 'warning');

        return {
            valid: errors.length === 0,
            errors: errors,
            warnings: warnings,
            summary: {
                totalIssues: this.validationErrors.length,
                errorCount: errors.length,
                warningCount: warnings.length
            }
        };
    }

    /**
     * Parse and validate JSON string
     */
    parseAndValidate(jsonString) {
        try {
            // Parse JSON
            const data = JSON.parse(jsonString);
            
            // Validate structure
            const validation = this.validateTriageIRData(data);
            
            return {
                success: true,
                data: data,
                validation: validation
            };
            
        } catch (parseError) {
            return {
                success: false,
                error: `JSON parsing failed: ${parseError.message}`,
                validation: {
                    valid: false,
                    errors: [{ type: 'error', path: 'root', message: parseError.message }],
                    warnings: [],
                    summary: { totalIssues: 1, errorCount: 1, warningCount: 0 }
                }
            };
        }
    }

    /**
     * Sanitize data for safe display
     */
    sanitizeData(data) {
        try {
            // Deep clone to avoid modifying original
            const sanitized = JSON.parse(JSON.stringify(data));
            
            // Sanitize potentially dangerous content
            this.sanitizeObject(sanitized);
            
            return sanitized;
        } catch (error) {
            console.error('Data sanitization failed:', error);
            return data; // Return original if sanitization fails
        }
    }

    /**
     * Recursively sanitize object properties
     */
    sanitizeObject(obj) {
        if (typeof obj !== 'object' || obj === null) {
            return;
        }

        for (const [key, value] of Object.entries(obj)) {
            if (typeof value === 'string') {
                // Sanitize strings that might contain HTML or scripts
                obj[key] = this.sanitizeString(value);
            } else if (Array.isArray(value)) {
                value.forEach(item => this.sanitizeObject(item));
            } else if (typeof value === 'object') {
                this.sanitizeObject(value);
            }
        }
    }

    /**
     * Sanitize individual string values
     */
    sanitizeString(str) {
        // Remove potentially dangerous characters/patterns
        return str
            .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '[SCRIPT_REMOVED]')
            .replace(/<iframe\b[^<]*(?:(?!<\/iframe>)<[^<]*)*<\/iframe>/gi, '[IFRAME_REMOVED]')
            .replace(/javascript:/gi, 'javascript_removed:')
            .replace(/on\w+\s*=/gi, 'event_removed=');
    }

    /**
     * Generate validation report
     */
    generateValidationReport(validation) {
        let report = `TriageIR Data Validation Report\n`;
        report += `${'='.repeat(40)}\n\n`;
        
        report += `Overall Status: ${validation.valid ? '✅ VALID' : '❌ INVALID'}\n`;
        report += `Total Issues: ${validation.summary.totalIssues}\n`;
        report += `Errors: ${validation.summary.errorCount}\n`;
        report += `Warnings: ${validation.summary.warningCount}\n\n`;

        if (validation.errors.length > 0) {
            report += `ERRORS:\n`;
            report += `${'-'.repeat(20)}\n`;
            validation.errors.forEach((error, index) => {
                report += `${index + 1}. [${error.path}] ${error.message}\n`;
            });
            report += '\n';
        }

        if (validation.warnings.length > 0) {
            report += `WARNINGS:\n`;
            report += `${'-'.repeat(20)}\n`;
            validation.warnings.forEach((warning, index) => {
                report += `${index + 1}. [${warning.path}] ${warning.message}\n`;
            });
            report += '\n';
        }

        if (validation.valid) {
            report += `✅ Data structure is valid and ready for analysis.\n`;
        } else {
            report += `❌ Data structure has issues that should be addressed.\n`;
        }

        return report;
    }
}

// Export for use in other modules
window.DataValidator = DataValidator;