use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Professional forensic data structures for TriageIR
/// Designed to match commercial DFIR tool capabilities

/// Root forensic evidence package
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForensicEvidence {
    pub case_metadata: CaseMetadata,
    pub system_snapshot: SystemSnapshot,
    pub volatile_artifacts: VolatileArtifacts,
    pub execution_artifacts: ExecutionArtifacts,
    pub persistence_artifacts: PersistenceArtifacts,
    pub network_artifacts: NetworkArtifacts,
    pub user_activity: UserActivity,
    pub security_events: SecurityEvents,
    pub integrity_verification: IntegrityVerification,
    pub collection_audit: CollectionAudit,
}

/// Case metadata for chain of custody
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaseMetadata {
    pub case_id: String,
    pub evidence_id: String,
    pub collection_timestamp: String,
    pub collector_info: CollectorInfo,
    pub target_system: TargetSystemInfo,
    pub collection_method: String,
    pub legal_authority: Option<String>,
    pub chain_of_custody: Vec<CustodyEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectorInfo {
    pub name: String,
    pub organization: String,
    pub contact: String,
    pub tool_version: String,
    pub collection_host: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetSystemInfo {
    pub hostname: String,
    pub domain: String,
    pub ip_addresses: Vec<String>,
    pub mac_addresses: Vec<String>,
    pub os_version: String,
    pub architecture: String,
    pub timezone: String,
    pub system_uptime: u64,
    pub last_boot_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustodyEntry {
    pub timestamp: String,
    pub action: String,
    pub person: String,
    pub organization: String,
    pub notes: String,
}

/// System snapshot at time of collection
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemSnapshot {
    pub collection_time: String,
    pub running_processes: Vec<ProcessInfo>,
    pub loaded_modules: Vec<ModuleInfo>,
    pub open_handles: Vec<HandleInfo>,
    pub memory_regions: Vec<MemoryRegion>,
    pub system_services: Vec<ServiceInfo>,
}

/// Enhanced process information with forensic details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub executable_path: String,
    pub command_line: String,
    pub creation_time: String,
    pub user_context: String,
    pub session_id: u32,
    pub memory_usage: MemoryUsage,
    pub loaded_dlls: Vec<DllInfo>,
    pub network_connections: Vec<u32>, // Connection IDs
    pub open_files: Vec<String>,
    pub registry_keys: Vec<String>,
    pub process_hashes: ProcessHashes,
    pub digital_signature: Option<DigitalSignature>,
    pub yara_matches: Vec<YaraMatch>,
    pub suspicious_indicators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryUsage {
    pub working_set_mb: f64,
    pub private_bytes_mb: f64,
    pub virtual_size_mb: f64,
    pub page_faults: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DllInfo {
    pub name: String,
    pub path: String,
    pub base_address: String,
    pub size: u64,
    pub version: String,
    pub file_hash: String,
    pub digital_signature: Option<DigitalSignature>,
    pub load_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessHashes {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub imphash: String,
    pub ssdeep: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DigitalSignature {
    pub is_signed: bool,
    pub is_valid: bool,
    pub signer: String,
    pub issuer: String,
    pub serial_number: String,
    pub timestamp: String,
    pub certificate_chain: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YaraMatch {
    pub rule_name: String,
    pub rule_file: String,
    pub match_offset: u64,
    pub match_length: u32,
    pub match_data: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub path: String,
    pub base_address: String,
    pub size: u64,
    pub process_id: u32,
    pub load_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandleInfo {
    pub process_id: u32,
    pub handle_value: String,
    pub object_type: String,
    pub object_name: String,
    pub access_mask: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryRegion {
    pub process_id: u32,
    pub base_address: String,
    pub size: u64,
    pub protection: String,
    pub state: String,
    pub region_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub service_type: String,
    pub executable_path: String,
    pub account: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

/// Volatile artifacts that disappear on reboot
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolatileArtifacts {
    pub network_connections: Vec<NetworkConnection>,
    pub dns_cache: Vec<DnsCacheEntry>,
    pub arp_table: Vec<ArpEntry>,
    pub routing_table: Vec<RouteEntry>,
    pub netbios_sessions: Vec<NetbiosSession>,
    pub clipboard_contents: Vec<ClipboardEntry>,
    pub recent_documents: Vec<RecentDocument>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_address: String,
    pub local_port: u16,
    pub remote_address: String,
    pub remote_port: u16,
    pub state: String,
    pub process_id: u32,
    pub process_name: String,
    pub creation_time: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_external: bool,
    pub geolocation: Option<GeoLocation>,
    pub threat_intelligence: Option<ThreatIntel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoLocation {
    pub country: String,
    pub region: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub isp: String,
    pub organization: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreatIntel {
    pub is_malicious: bool,
    pub threat_type: String,
    pub confidence: f32,
    pub source: String,
    pub last_seen: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsCacheEntry {
    pub hostname: String,
    pub ip_address: String,
    pub record_type: String,
    pub ttl: u32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArpEntry {
    pub ip_address: String,
    pub mac_address: String,
    pub interface: String,
    pub entry_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteEntry {
    pub destination: String,
    pub netmask: String,
    pub gateway: String,
    pub interface: String,
    pub metric: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetbiosSession {
    pub local_name: String,
    pub remote_name: String,
    pub session_type: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardEntry {
    pub format: String,
    pub content: String,
    pub size: u64,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentDocument {
    pub path: String,
    pub application: String,
    pub last_accessed: String,
    pub access_count: u32,
}

/// Execution artifacts (evidence of program execution)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionArtifacts {
    pub prefetch_files: Vec<PrefetchFile>,
    pub shimcache_entries: Vec<ShimcacheEntry>,
    pub amcache_entries: Vec<AmcacheEntry>,
    pub userassist_entries: Vec<UserAssistEntry>,
    pub jump_lists: Vec<JumpListEntry>,
    pub lnk_files: Vec<LnkFile>,
    pub mru_lists: Vec<MruEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefetchFile {
    pub filename: String,
    pub executable_name: String,
    pub run_count: u32,
    pub last_run_time: String,
    pub creation_time: String,
    pub file_size: u64,
    pub hash: String,
    pub version: u32,
    pub referenced_files: Vec<String>,
    pub volumes: Vec<VolumeInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeInfo {
    pub device_path: String,
    pub volume_name: String,
    pub serial_number: String,
    pub creation_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShimcacheEntry {
    pub path: String,
    pub last_modified: String,
    pub file_size: u64,
    pub last_update: String,
    pub execution_flag: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmcacheEntry {
    pub path: String,
    pub sha1: String,
    pub first_installation: String,
    pub last_modified: String,
    pub publisher: String,
    pub version: String,
    pub language: String,
    pub install_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAssistEntry {
    pub program_name: String,
    pub run_count: u32,
    pub last_execution: String,
    pub focus_count: u32,
    pub focus_time: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JumpListEntry {
    pub application: String,
    pub file_path: String,
    pub arguments: String,
    pub creation_time: String,
    pub last_accessed: String,
    pub access_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LnkFile {
    pub path: String,
    pub target_path: String,
    pub arguments: String,
    pub working_directory: String,
    pub icon_location: String,
    pub creation_time: String,
    pub last_accessed: String,
    pub last_modified: String,
    pub file_size: u64,
    pub file_attributes: String,
    pub drive_type: String,
    pub volume_label: String,
    pub local_base_path: String,
    pub network_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MruEntry {
    pub key_path: String,
    pub value_name: String,
    pub value_data: String,
    pub last_write_time: String,
    pub position: u32,
}

/// Persistence mechanisms
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersistenceArtifacts {
    pub registry_run_keys: Vec<RegistryRunKey>,
    pub scheduled_tasks: Vec<ScheduledTask>,
    pub services: Vec<ServicePersistence>,
    pub startup_folders: Vec<StartupItem>,
    pub winlogon_entries: Vec<WinlogonEntry>,
    pub image_hijacks: Vec<ImageHijack>,
    pub dll_hijacks: Vec<DllHijack>,
    pub wmi_persistence: Vec<WmiPersistence>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryRunKey {
    pub hive: String,
    pub key_path: String,
    pub value_name: String,
    pub value_data: String,
    pub last_write_time: String,
    pub file_exists: bool,
    pub file_hash: Option<String>,
    pub digital_signature: Option<DigitalSignature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduledTask {
    pub name: String,
    pub path: String,
    pub state: String,
    pub last_run_time: String,
    pub next_run_time: String,
    pub run_as_user: String,
    pub command: String,
    pub arguments: String,
    pub working_directory: String,
    pub triggers: Vec<TaskTrigger>,
    pub actions: Vec<TaskAction>,
    pub creation_date: String,
    pub author: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskTrigger {
    pub trigger_type: String,
    pub start_boundary: String,
    pub end_boundary: String,
    pub enabled: bool,
    pub repetition_interval: String,
    pub repetition_duration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskAction {
    pub action_type: String,
    pub path: String,
    pub arguments: String,
    pub working_directory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServicePersistence {
    pub name: String,
    pub display_name: String,
    pub executable_path: String,
    pub start_type: String,
    pub service_type: String,
    pub account: String,
    pub description: String,
    pub creation_time: String,
    pub last_modified: String,
    pub file_hash: Option<String>,
    pub digital_signature: Option<DigitalSignature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartupItem {
    pub name: String,
    pub path: String,
    pub target: String,
    pub arguments: String,
    pub working_directory: String,
    pub creation_time: String,
    pub last_modified: String,
    pub file_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WinlogonEntry {
    pub key_name: String,
    pub value_name: String,
    pub value_data: String,
    pub last_write_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageHijack {
    pub target_executable: String,
    pub hijack_executable: String,
    pub registry_key: String,
    pub last_write_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DllHijack {
    pub target_dll: String,
    pub hijack_dll: String,
    pub search_path: String,
    pub process_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WmiPersistence {
    pub namespace: String,
    pub class_name: String,
    pub filter_name: String,
    pub consumer_name: String,
    pub query: String,
    pub script_text: String,
    pub creation_time: String,
}

/// Network artifacts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkArtifacts {
    pub active_connections: Vec<NetworkConnection>,
    pub listening_ports: Vec<ListeningPort>,
    pub network_shares: Vec<NetworkShare>,
    pub wifi_profiles: Vec<WifiProfile>,
    pub firewall_rules: Vec<FirewallRule>,
    pub proxy_settings: ProxySettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListeningPort {
    pub protocol: String,
    pub local_address: String,
    pub local_port: u16,
    pub process_id: u32,
    pub process_name: String,
    pub service_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkShare {
    pub name: String,
    pub path: String,
    pub description: String,
    pub share_type: String,
    pub permissions: Vec<SharePermission>,
    pub current_connections: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharePermission {
    pub account: String,
    pub access_type: String,
    pub permissions: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WifiProfile {
    pub name: String,
    pub ssid: String,
    pub authentication: String,
    pub encryption: String,
    pub password: Option<String>,
    pub connection_mode: String,
    pub creation_time: String,
    pub last_connected: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub description: String,
    pub direction: String,
    pub action: String,
    pub protocol: String,
    pub local_ports: String,
    pub remote_ports: String,
    pub local_addresses: String,
    pub remote_addresses: String,
    pub enabled: bool,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxySettings {
    pub enabled: bool,
    pub server: String,
    pub port: u16,
    pub bypass_list: Vec<String>,
    pub auto_config_url: String,
}

/// User activity artifacts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserActivity {
    pub logged_on_users: Vec<LoggedOnUser>,
    pub login_history: Vec<LoginEvent>,
    pub user_profiles: Vec<UserProfile>,
    pub recent_activity: Vec<ActivityEntry>,
    pub browser_artifacts: Vec<BrowserArtifact>,
    pub email_artifacts: Vec<EmailArtifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggedOnUser {
    pub username: String,
    pub domain: String,
    pub session_id: u32,
    pub session_type: String,
    pub logon_time: String,
    pub logon_server: String,
    pub client_name: String,
    pub client_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginEvent {
    pub username: String,
    pub domain: String,
    pub logon_type: String,
    pub logon_time: String,
    pub logoff_time: Option<String>,
    pub source_ip: String,
    pub workstation: String,
    pub process_name: String,
    pub authentication_package: String,
    pub success: bool,
    pub failure_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub username: String,
    pub sid: String,
    pub profile_path: String,
    pub creation_time: String,
    pub last_logon: String,
    pub last_logoff: String,
    pub logon_count: u32,
    pub bad_password_count: u32,
    pub account_expires: String,
    pub password_last_set: String,
    pub groups: Vec<String>,
    pub privileges: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityEntry {
    pub timestamp: String,
    pub activity_type: String,
    pub description: String,
    pub user: String,
    pub process: String,
    pub details: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrowserArtifact {
    pub browser: String,
    pub profile: String,
    pub artifact_type: String, // history, downloads, cookies, etc.
    pub url: String,
    pub title: String,
    pub visit_count: u32,
    pub last_visit: String,
    pub typed_count: u32,
    pub download_path: Option<String>,
    pub referrer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailArtifact {
    pub client: String,
    pub account: String,
    pub subject: String,
    pub sender: String,
    pub recipients: Vec<String>,
    pub timestamp: String,
    pub message_id: String,
    pub attachment_count: u32,
    pub attachment_names: Vec<String>,
}

/// Security events and logs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityEvents {
    pub security_log: Vec<SecurityEvent>,
    pub system_log: Vec<SystemEvent>,
    pub application_log: Vec<ApplicationEvent>,
    pub powershell_log: Vec<PowershellEvent>,
    pub sysmon_log: Vec<SysmonEvent>,
    pub defender_log: Vec<DefenderEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub user: String,
    pub computer: String,
    pub process_id: u32,
    pub thread_id: u32,
    pub keywords: Vec<String>,
    pub raw_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub computer: String,
    pub raw_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub computer: String,
    pub application: String,
    pub raw_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PowershellEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub level: String,
    pub script_block: String,
    pub command_line: String,
    pub user: String,
    pub host_application: String,
    pub engine_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysmonEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub process_guid: String,
    pub process_id: u32,
    pub image: String,
    pub command_line: String,
    pub user: String,
    pub parent_process_guid: String,
    pub parent_process_id: u32,
    pub parent_image: String,
    pub parent_command_line: String,
    pub hashes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefenderEvent {
    pub event_id: u32,
    pub timestamp: String,
    pub threat_name: String,
    pub severity: String,
    pub category: String,
    pub path: String,
    pub action_taken: String,
    pub user: String,
    pub detection_source: String,
}

/// Integrity verification for chain of custody
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrityVerification {
    pub evidence_hash: String,
    pub hash_algorithm: String,
    pub file_hashes: HashMap<String, FileHash>,
    pub digital_signature: Option<DigitalSignature>,
    pub verification_timestamp: String,
    pub verification_tool: String,
    pub verification_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileHash {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub file_size: u64,
    pub creation_time: String,
    pub modification_time: String,
}

/// Collection audit trail
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionAudit {
    pub collection_start: String,
    pub collection_end: String,
    pub total_duration_seconds: u64,
    pub collection_method: String,
    pub tool_version: String,
    pub command_line: String,
    pub working_directory: String,
    pub environment_variables: HashMap<String, String>,
    pub collection_errors: Vec<CollectionError>,
    pub collection_warnings: Vec<CollectionWarning>,
    pub collection_statistics: CollectionStatistics,
    pub audit_log: Vec<AuditEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionError {
    pub timestamp: String,
    pub component: String,
    pub error_code: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub impact: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionWarning {
    pub timestamp: String,
    pub component: String,
    pub warning_message: String,
    pub recommendation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionStatistics {
    pub total_processes: u32,
    pub total_network_connections: u32,
    pub total_files_analyzed: u32,
    pub total_registry_keys: u32,
    pub total_event_log_entries: u32,
    pub total_prefetch_files: u32,
    pub total_scheduled_tasks: u32,
    pub memory_usage_peak_mb: f64,
    pub disk_space_used_mb: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: String,
    pub level: String,
    pub component: String,
    pub action: String,
    pub details: String,
    pub duration_ms: Option<u64>,
    pub result: String,
}

impl ForensicEvidence {
    /// Create a new forensic evidence package
    pub fn new(case_id: String, collector_info: CollectorInfo) -> Self {
        let evidence_id = uuid::Uuid::new_v4().to_string();
        let collection_timestamp = chrono::Utc::now().to_rfc3339();
        
        ForensicEvidence {
            case_metadata: CaseMetadata {
                case_id,
                evidence_id,
                collection_timestamp: collection_timestamp.clone(),
                collector_info,
                target_system: TargetSystemInfo::default(),
                collection_method: "Live System Triage".to_string(),
                legal_authority: None,
                chain_of_custody: vec![],
            },
            system_snapshot: SystemSnapshot::default(),
            volatile_artifacts: VolatileArtifacts::default(),
            execution_artifacts: ExecutionArtifacts::default(),
            persistence_artifacts: PersistenceArtifacts::default(),
            network_artifacts: NetworkArtifacts::default(),
            user_activity: UserActivity::default(),
            security_events: SecurityEvents::default(),
            integrity_verification: IntegrityVerification::default(),
            collection_audit: CollectionAudit::new(collection_timestamp),
        }
    }
    
    /// Add a custody entry to the chain of custody
    pub fn add_custody_entry(&mut self, action: String, person: String, organization: String, notes: String) {
        self.case_metadata.chain_of_custody.push(CustodyEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            action,
            person,
            organization,
            notes,
        });
    }
    
    /// Finalize the evidence package with integrity verification
    pub fn finalize(&mut self) {
        self.collection_audit.collection_end = chrono::Utc::now().to_rfc3339();
        
        // Calculate collection duration
        if let (Ok(start), Ok(end)) = (
            chrono::DateTime::parse_from_rfc3339(&self.collection_audit.collection_start),
            chrono::DateTime::parse_from_rfc3339(&self.collection_audit.collection_end)
        ) {
            let duration = end.signed_duration_since(start);
            self.collection_audit.total_duration_seconds = duration.num_seconds().max(0) as u64;
        }
        
        // Generate evidence hash
        let evidence_json = serde_json::to_string(self).unwrap_or_default();
        let evidence_hash = sha2::Sha256::digest(evidence_json.as_bytes());
        self.integrity_verification.evidence_hash = hex::encode(evidence_hash);
        self.integrity_verification.hash_algorithm = "SHA-256".to_string();
        self.integrity_verification.verification_timestamp = chrono::Utc::now().to_rfc3339();
        self.integrity_verification.verification_tool = "TriageIR".to_string();
        self.integrity_verification.verification_version = env!("CARGO_PKG_VERSION").to_string();
    }
}

// Default implementations for complex structures
impl Default for TargetSystemInfo {
    fn default() -> Self {
        TargetSystemInfo {
            hostname: "Unknown".to_string(),
            domain: "Unknown".to_string(),
            ip_addresses: vec![],
            mac_addresses: vec![],
            os_version: "Unknown".to_string(),
            architecture: "Unknown".to_string(),
            timezone: "Unknown".to_string(),
            system_uptime: 0,
            last_boot_time: "Unknown".to_string(),
        }
    }
}

impl Default for SystemSnapshot {
    fn default() -> Self {
        SystemSnapshot {
            collection_time: chrono::Utc::now().to_rfc3339(),
            running_processes: vec![],
            loaded_modules: vec![],
            open_handles: vec![],
            memory_regions: vec![],
            system_services: vec![],
        }
    }
}

impl Default for VolatileArtifacts {
    fn default() -> Self {
        VolatileArtifacts {
            network_connections: vec![],
            dns_cache: vec![],
            arp_table: vec![],
            routing_table: vec![],
            netbios_sessions: vec![],
            clipboard_contents: vec![],
            recent_documents: vec![],
        }
    }
}

impl Default for ExecutionArtifacts {
    fn default() -> Self {
        ExecutionArtifacts {
            prefetch_files: vec![],
            shimcache_entries: vec![],
            amcache_entries: vec![],
            userassist_entries: vec![],
            jump_lists: vec![],
            lnk_files: vec![],
            mru_lists: vec![],
        }
    }
}

impl Default for PersistenceArtifacts {
    fn default() -> Self {
        PersistenceArtifacts {
            registry_run_keys: vec![],
            scheduled_tasks: vec![],
            services: vec![],
            startup_folders: vec![],
            winlogon_entries: vec![],
            image_hijacks: vec![],
            dll_hijacks: vec![],
            wmi_persistence: vec![],
        }
    }
}

impl Default for NetworkArtifacts {
    fn default() -> Self {
        NetworkArtifacts {
            active_connections: vec![],
            listening_ports: vec![],
            network_shares: vec![],
            wifi_profiles: vec![],
            firewall_rules: vec![],
            proxy_settings: ProxySettings {
                enabled: false,
                server: String::new(),
                port: 0,
                bypass_list: vec![],
                auto_config_url: String::new(),
            },
        }
    }
}

impl Default for UserActivity {
    fn default() -> Self {
        UserActivity {
            logged_on_users: vec![],
            login_history: vec![],
            user_profiles: vec![],
            recent_activity: vec![],
            browser_artifacts: vec![],
            email_artifacts: vec![],
        }
    }
}

impl Default for SecurityEvents {
    fn default() -> Self {
        SecurityEvents {
            security_log: vec![],
            system_log: vec![],
            application_log: vec![],
            powershell_log: vec![],
            sysmon_log: vec![],
            defender_log: vec![],
        }
    }
}

impl Default for IntegrityVerification {
    fn default() -> Self {
        IntegrityVerification {
            evidence_hash: String::new(),
            hash_algorithm: String::new(),
            file_hashes: HashMap::new(),
            digital_signature: None,
            verification_timestamp: String::new(),
            verification_tool: String::new(),
            verification_version: String::new(),
        }
    }
}

impl CollectionAudit {
    fn new(start_time: String) -> Self {
        CollectionAudit {
            collection_start: start_time,
            collection_end: String::new(),
            total_duration_seconds: 0,
            collection_method: "Automated Live Triage".to_string(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            command_line: std::env::args().collect::<Vec<_>>().join(" "),
            working_directory: std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            environment_variables: std::env::vars().collect(),
            collection_errors: vec![],
            collection_warnings: vec![],
            collection_statistics: CollectionStatistics::default(),
            audit_log: vec![],
        }
    }
}

impl Default for CollectionStatistics {
    fn default() -> Self {
        CollectionStatistics {
            total_processes: 0,
            total_network_connections: 0,
            total_files_analyzed: 0,
            total_registry_keys: 0,
            total_event_log_entries: 0,
            total_prefetch_files: 0,
            total_scheduled_tasks: 0,
            memory_usage_peak_mb: 0.0,
            disk_space_used_mb: 0.0,
        }
    }
}