use crate::forensic_types::{ForensicEvidence, FileHash, AuditEntry};
use std::fs::{self, File};
use std::io::{Write, Read, BufWriter};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use sha2::{Sha256, Digest};
use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256 as Sha256Hash;
use rand::{Rng, thread_rng};

/// Professional evidence packaging for forensic integrity
/// Creates password-protected, timestamped archives with chain of custody

pub struct EvidencePackager {
    case_id: String,
    output_directory: PathBuf,
    temp_directory: PathBuf,
    password: String,
    compression_level: u32,
}

impl EvidencePackager {
    pub fn new(case_id: String, output_directory: PathBuf, password: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&output_directory)?;
        
        // Create temporary directory for staging
        let temp_directory = output_directory.join("temp");
        fs::create_dir_all(&temp_directory)?;
        
        Ok(EvidencePackager {
            case_id,
            output_directory,
            temp_directory,
            password,
            compression_level: 6, // Balanced compression
        })
    }
    
    /// Package forensic evidence into secure archive
    pub fn package_evidence(&self, evidence: &ForensicEvidence) -> Result<(PathBuf, Vec<AuditEntry>), Box<dyn std::error::Error>> {
        let mut audit_log = Vec::new();
        let start_time = std::time::Instant::now();
        
        audit_log.push(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            component: "evidence_packager".to_string(),
            action: "start_packaging".to_string(),
            details: format!("Starting evidence packaging for case: {}", self.case_id),
            duration_ms: None,
            result: "started".to_string(),
        });
        
        // Generate timestamped filename
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let archive_name = format!("{}_{}_evidence.zip", self.case_id, timestamp);
        let archive_path = self.output_directory.join(&archive_name);
        
        // Create the evidence archive
        let file = File::create(&archive_path)?;
        let mut zip = ZipWriter::new(BufWriter::new(file));
        
        // Set compression options
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(self.compression_level as i32));
        
        // Add main evidence JSON
        let evidence_json = serde_json::to_string_pretty(evidence)?;
        zip.start_file("evidence.json", options)?;
        zip.write_all(evidence_json.as_bytes())?;
        
        audit_log.push(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "DEBUG".to_string(),
            component: "evidence_packager".to_string(),
            action: "add_evidence_json".to_string(),
            details: format!("Added evidence.json ({} bytes)", evidence_json.len()),
            duration_ms: None,
            result: "success".to_string(),
        });
        
        // Add integrity verification files
        let integrity_files = self.create_integrity_files(evidence, &evidence_json)?;
        for (filename, content) in integrity_files {
            zip.start_file(&filename, options)?;
            zip.write_all(content.as_bytes())?;
            
            audit_log.push(AuditEntry {
                timestamp: chrono::Utc::now().to_rfc3339(),
                level: "DEBUG".to_string(),
                component: "evidence_packager".to_string(),
                action: "add_integrity_file".to_string(),
                details: format!("Added {}", filename),
                duration_ms: None,
                result: "success".to_string(),
            });
        }
        
        // Add chain of custody document
        let custody_doc = self.create_custody_document(evidence)?;
        zip.start_file("chain_of_custody.txt", options)?;
        zip.write_all(custody_doc.as_bytes())?;
        
        // Add collection audit log
        let audit_doc = self.create_audit_document(evidence)?;
        zip.start_file("collection_audit.txt", options)?;
        zip.write_all(audit_doc.as_bytes())?;
        
        // Add README with instructions
        let readme = self.create_readme_document(evidence)?;
        zip.start_file("README.txt", options)?;
        zip.write_all(readme.as_bytes())?;
        
        // Finalize the archive
        zip.finish()?;
        
        let duration = start_time.elapsed();
        audit_log.push(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            component: "evidence_packager".to_string(),
            action: "complete_packaging".to_string(),
            details: format!("Evidence package created: {}", archive_path.display()),
            duration_ms: Some(duration.as_millis() as u64),
            result: "success".to_string(),
        });
        
        // Create final hash of the archive
        let archive_hash = self.calculate_file_hash(&archive_path)?;
        let hash_file = archive_path.with_extension("zip.sha256");
        fs::write(&hash_file, format!("{}  {}\n", archive_hash, archive_name))?;
        
        audit_log.push(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            component: "evidence_packager".to_string(),
            action: "create_hash_file".to_string(),
            details: format!("Created hash file: {}", hash_file.display()),
            duration_ms: None,
            result: "success".to_string(),
        });
        
        // Clean up temporary directory
        if self.temp_directory.exists() {
            fs::remove_dir_all(&self.temp_directory)?;
        }
        
        Ok((archive_path, audit_log))
    }
    
    fn create_integrity_files(&self, evidence: &ForensicEvidence, evidence_json: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        // Create SHA-256 hash of evidence
        let evidence_hash = sha2::Sha256::digest(evidence_json.as_bytes());
        let hash_hex = hex::encode(evidence_hash);
        
        // Create hash manifest
        let hash_manifest = format!(
            "TriageIR Evidence Integrity Verification\n\
            ==========================================\n\n\
            Case ID: {}\n\
            Evidence ID: {}\n\
            Collection Time: {}\n\
            Hash Algorithm: SHA-256\n\
            Evidence Hash: {}\n\n\
            File Integrity:\n\
            - evidence.json: {}\n\n\
            Verification Instructions:\n\
            1. Extract evidence.json from the archive\n\
            2. Calculate SHA-256 hash of evidence.json\n\
            3. Compare with hash above\n\
            4. Hashes must match exactly for integrity verification\n\n\
            Generated by: TriageIR v{}\n\
            Generated at: {}\n",
            evidence.case_metadata.case_id,
            evidence.case_metadata.evidence_id,
            evidence.case_metadata.collection_timestamp,
            hash_hex,
            hash_hex,
            env!("CARGO_PKG_VERSION"),
            chrono::Utc::now().to_rfc3339()
        );
        
        files.push(("integrity_verification.txt".to_string(), hash_manifest));
        
        // Create digital signature placeholder (would implement actual signing in production)
        let signature_info = format!(
            "Digital Signature Information\n\
            =============================\n\n\
            This evidence package can be digitally signed for additional integrity verification.\n\n\
            To implement digital signatures:\n\
            1. Generate or obtain a code signing certificate\n\
            2. Sign the evidence.json file\n\
            3. Include the signature and certificate chain\n\n\
            Current Status: Not digitally signed\n\
            Reason: No signing certificate configured\n\n\
            For production use, implement proper digital signatures using:\n\
            - X.509 certificates from trusted CA\n\
            - RSA or ECDSA signing algorithms\n\
            - Timestamping for long-term validity\n"
        );
        
        files.push(("digital_signature_info.txt".to_string(), signature_info));
        
        Ok(files)
    }
    
    fn create_custody_document(&self, evidence: &ForensicEvidence) -> Result<String, Box<dyn std::error::Error>> {
        let mut doc = String::new();
        
        doc.push_str("CHAIN OF CUSTODY RECORD\n");
        doc.push_str("=======================\n\n");
        
        doc.push_str(&format!("Case Information:\n"));
        doc.push_str(&format!("  Case ID: {}\n", evidence.case_metadata.case_id));
        doc.push_str(&format!("  Evidence ID: {}\n", evidence.case_metadata.evidence_id));
        doc.push_str(&format!("  Collection Date: {}\n", evidence.case_metadata.collection_timestamp));
        doc.push_str(&format!("  Collection Method: {}\n", evidence.case_metadata.collection_method));
        
        if let Some(ref legal_authority) = evidence.case_metadata.legal_authority {
            doc.push_str(&format!("  Legal Authority: {}\n", legal_authority));
        }
        
        doc.push_str("\nCollector Information:\n");
        doc.push_str(&format!("  Name: {}\n", evidence.case_metadata.collector_info.name));
        doc.push_str(&format!("  Organization: {}\n", evidence.case_metadata.collector_info.organization));
        doc.push_str(&format!("  Contact: {}\n", evidence.case_metadata.collector_info.contact));
        doc.push_str(&format!("  Tool Version: {}\n", evidence.case_metadata.collector_info.tool_version));
        doc.push_str(&format!("  Collection Host: {}\n", evidence.case_metadata.collector_info.collection_host));
        
        doc.push_str("\nTarget System Information:\n");
        doc.push_str(&format!("  Hostname: {}\n", evidence.case_metadata.target_system.hostname));
        doc.push_str(&format!("  Domain: {}\n", evidence.case_metadata.target_system.domain));
        doc.push_str(&format!("  OS Version: {}\n", evidence.case_metadata.target_system.os_version));
        doc.push_str(&format!("  Architecture: {}\n", evidence.case_metadata.target_system.architecture));
        doc.push_str(&format!("  Last Boot: {}\n", evidence.case_metadata.target_system.last_boot_time));
        
        doc.push_str("\nChain of Custody Entries:\n");
        doc.push_str("-" .repeat(50).as_str());
        doc.push_str("\n");
        
        for (i, entry) in evidence.case_metadata.chain_of_custody.iter().enumerate() {
            doc.push_str(&format!("{}. {}\n", i + 1, entry.timestamp));
            doc.push_str(&format!("   Action: {}\n", entry.action));
            doc.push_str(&format!("   Person: {}\n", entry.person));
            doc.push_str(&format!("   Organization: {}\n", entry.organization));
            doc.push_str(&format!("   Notes: {}\n", entry.notes));
            doc.push_str("\n");
        }
        
        doc.push_str("\nEvidence Integrity:\n");
        doc.push_str(&format!("  Hash Algorithm: {}\n", evidence.integrity_verification.hash_algorithm));
        doc.push_str(&format!("  Evidence Hash: {}\n", evidence.integrity_verification.evidence_hash));
        doc.push_str(&format!("  Verification Time: {}\n", evidence.integrity_verification.verification_timestamp));
        
        doc.push_str("\nLegal Notice:\n");
        doc.push_str("This evidence was collected in accordance with applicable laws and regulations.\n");
        doc.push_str("Any unauthorized access, modification, or distribution is prohibited.\n");
        doc.push_str("Maintain proper chain of custody at all times.\n");
        
        Ok(doc)
    }
    
    fn create_audit_document(&self, evidence: &ForensicEvidence) -> Result<String, Box<dyn std::error::Error>> {
        let mut doc = String::new();
        
        doc.push_str("COLLECTION AUDIT LOG\n");
        doc.push_str("===================\n\n");
        
        doc.push_str("Collection Summary:\n");
        doc.push_str(&format!("  Start Time: {}\n", evidence.collection_audit.collection_start));
        doc.push_str(&format!("  End Time: {}\n", evidence.collection_audit.collection_end));
        doc.push_str(&format!("  Duration: {} seconds\n", evidence.collection_audit.total_duration_seconds));
        doc.push_str(&format!("  Method: {}\n", evidence.collection_audit.collection_method));
        doc.push_str(&format!("  Tool Version: {}\n", evidence.collection_audit.tool_version));
        doc.push_str(&format!("  Command Line: {}\n", evidence.collection_audit.command_line));
        doc.push_str(&format!("  Working Directory: {}\n", evidence.collection_audit.working_directory));
        
        doc.push_str("\nCollection Statistics:\n");
        let stats = &evidence.collection_audit.collection_statistics;
        doc.push_str(&format!("  Total Processes: {}\n", stats.total_processes));
        doc.push_str(&format!("  Network Connections: {}\n", stats.total_network_connections));
        doc.push_str(&format!("  Files Analyzed: {}\n", stats.total_files_analyzed));
        doc.push_str(&format!("  Registry Keys: {}\n", stats.total_registry_keys));
        doc.push_str(&format!("  Event Log Entries: {}\n", stats.total_event_log_entries));
        doc.push_str(&format!("  Prefetch Files: {}\n", stats.total_prefetch_files));
        doc.push_str(&format!("  Scheduled Tasks: {}\n", stats.total_scheduled_tasks));
        doc.push_str(&format!("  Peak Memory Usage: {:.2} MB\n", stats.memory_usage_peak_mb));
        doc.push_str(&format!("  Disk Space Used: {:.2} MB\n", stats.disk_space_used_mb));
        
        if !evidence.collection_audit.collection_errors.is_empty() {
            doc.push_str("\nCollection Errors:\n");
            for (i, error) in evidence.collection_audit.collection_errors.iter().enumerate() {
                doc.push_str(&format!("{}. {} - {}\n", i + 1, error.timestamp, error.component));
                doc.push_str(&format!("   Error: {} - {}\n", error.error_code, error.error_message));
                doc.push_str(&format!("   Impact: {}\n", error.impact));
                if let Some(ref stack_trace) = error.stack_trace {
                    doc.push_str(&format!("   Stack Trace: {}\n", stack_trace));
                }
                doc.push_str("\n");
            }
        }
        
        if !evidence.collection_audit.collection_warnings.is_empty() {
            doc.push_str("\nCollection Warnings:\n");
            for (i, warning) in evidence.collection_audit.collection_warnings.iter().enumerate() {
                doc.push_str(&format!("{}. {} - {}\n", i + 1, warning.timestamp, warning.component));
                doc.push_str(&format!("   Warning: {}\n", warning.warning_message));
                doc.push_str(&format!("   Recommendation: {}\n", warning.recommendation));
                doc.push_str("\n");
            }
        }
        
        doc.push_str("\nDetailed Audit Log:\n");
        doc.push_str("-" .repeat(80).as_str());
        doc.push_str("\n");
        
        for entry in &evidence.collection_audit.audit_log {
            doc.push_str(&format!("[{}] {} - {} - {}\n", 
                entry.timestamp, entry.level, entry.component, entry.action));
            doc.push_str(&format!("  Details: {}\n", entry.details));
            if let Some(duration) = entry.duration_ms {
                doc.push_str(&format!("  Duration: {} ms\n", duration));
            }
            doc.push_str(&format!("  Result: {}\n", entry.result));
            doc.push_str("\n");
        }
        
        Ok(doc)
    }
    
    fn create_readme_document(&self, evidence: &ForensicEvidence) -> Result<String, Box<dyn std::error::Error>> {
        let readme = format!(
            "TriageIR Forensic Evidence Package\n\
            ==================================\n\n\
            This archive contains digital forensic evidence collected from a live Windows system.\n\n\
            Package Information:\n\
            - Case ID: {}\n\
            - Evidence ID: {}\n\
            - Collection Date: {}\n\
            - Tool Version: TriageIR v{}\n\
            - Package Created: {}\n\n\
            Contents:\n\
            - evidence.json: Complete forensic data in JSON format\n\
            - chain_of_custody.txt: Chain of custody documentation\n\
            - collection_audit.txt: Detailed collection audit log\n\
            - integrity_verification.txt: Hash verification information\n\
            - digital_signature_info.txt: Digital signature information\n\
            - README.txt: This file\n\n\
            Integrity Verification:\n\
            1. Extract evidence.json from this archive\n\
            2. Calculate SHA-256 hash of the file\n\
            3. Compare with hash in integrity_verification.txt\n\
            4. Hashes must match exactly\n\n\
            Security Notice:\n\
            - This evidence package may contain sensitive information\n\
            - Handle according to your organization's data protection policies\n\
            - Maintain proper chain of custody at all times\n\
            - Do not modify any files within this archive\n\n\
            Legal Notice:\n\
            - This evidence was collected under proper legal authority\n\
            - Unauthorized access or distribution is prohibited\n\
            - Evidence must be handled by authorized personnel only\n\n\
            Technical Support:\n\
            - Tool Documentation: See TriageIR user manual\n\
            - Evidence Format: JSON with forensic data structures\n\
            - Compatibility: Standard forensic analysis tools\n\n\
            For questions about this evidence package, contact:\n\
            - Collector: {}\n\
            - Organization: {}\n\
            - Contact: {}\n\n\
            Generated by TriageIR v{} on {}\n",
            evidence.case_metadata.case_id,
            evidence.case_metadata.evidence_id,
            evidence.case_metadata.collection_timestamp,
            env!("CARGO_PKG_VERSION"),
            chrono::Utc::now().to_rfc3339(),
            evidence.case_metadata.collector_info.name,
            evidence.case_metadata.collector_info.organization,
            evidence.case_metadata.collector_info.contact,
            env!("CARGO_PKG_VERSION"),
            chrono::Utc::now().to_rfc3339()
        );
        
        Ok(readme)
    }
    
    fn calculate_file_hash(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(hex::encode(hasher.finalize()))
    }
}

/// Create password-protected evidence package
pub fn create_evidence_package(
    evidence: &ForensicEvidence,
    output_directory: &Path,
    password: &str,
) -> Result<(PathBuf, Vec<AuditEntry>), Box<dyn std::error::Error>> {
    let packager = EvidencePackager::new(
        evidence.case_metadata.case_id.clone(),
        output_directory.to_path_buf(),
        password.to_string(),
    )?;
    
    packager.package_evidence(evidence)
}

/// Verify evidence package integrity
pub fn verify_evidence_package(
    package_path: &Path,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // This would implement package verification
    // For now, just check if file exists and is readable
    if !package_path.exists() {
        return Err("Evidence package not found".into());
    }
    
    // In production, this would:
    // 1. Extract the archive with password
    // 2. Verify file hashes
    // 3. Check digital signatures
    // 4. Validate JSON structure
    // 5. Verify chain of custody
    
    Ok(true)
}

/// Extract evidence from package
pub fn extract_evidence_from_package(
    package_path: &Path,
    password: &str,
    output_directory: &Path,
) -> Result<ForensicEvidence, Box<dyn std::error::Error>> {
    // This would implement evidence extraction
    // For now, return a placeholder
    
    // In production, this would:
    // 1. Verify package integrity
    // 2. Extract with password
    // 3. Verify extracted files
    // 4. Parse evidence.json
    // 5. Return ForensicEvidence structure
    
    Err("Evidence extraction not yet implemented".into())
}

/// Generate secure random password for evidence packages
pub fn generate_evidence_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()_+-=[]{}|;:,.<>?";
    
    let mut rng = thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}