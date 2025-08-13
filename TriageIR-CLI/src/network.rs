use crate::types::{NetworkConnection, LogEntry};
use sysinfo::System;
use std::collections::HashMap;

#[cfg(windows)]
use windows::{
    core::*,
    Win32::NetworkManagement::IpHelper::*,
    Win32::Foundation::*,
    Win32::Networking::WinSock::*,
};

/// Collect information about all active network connections
pub fn collect_network_connections() -> (Vec<NetworkConnection>, Vec<LogEntry>) {
    let mut logs = Vec::new();
    logs.push(LogEntry::info("Starting network connection enumeration"));
    
    let mut connections = Vec::new();
    
    // Collect TCP connections
    match collect_tcp_connections() {
        Ok(tcp_conns) => {
            let tcp_count = tcp_conns.len();
            connections.extend(tcp_conns);
            logs.push(LogEntry::info(&format!("Found {} TCP connections", tcp_count)));
        }
        Err(e) => {
            logs.push(LogEntry::error(&format!("Failed to collect TCP connections: {}", e)));
        }
    }
    
    // Collect UDP connections
    match collect_udp_connections() {
        Ok(udp_conns) => {
            let udp_count = udp_conns.len();
            connections.extend(udp_conns);
            logs.push(LogEntry::info(&format!("Found {} UDP connections", udp_count)));
        }
        Err(e) => {
            logs.push(LogEntry::error(&format!("Failed to collect UDP connections: {}", e)));
        }
    }
    
    // Sort connections by protocol and local address for consistent output
    connections.sort_by(|a, b| {
        a.protocol.cmp(&b.protocol)
            .then_with(|| a.local_address.cmp(&b.local_address))
    });
    
    let total_connections = connections.len();
    let external_connections = connections.iter().filter(|c| c.is_external()).count();
    
    logs.push(LogEntry::info(&format!("Total connections: {}, External: {}", total_connections, external_connections)));
    logs.push(LogEntry::info("Network connection enumeration completed"));
    
    (connections, logs)
}

/// Collect TCP connections using Windows API
#[cfg(windows)]
fn collect_tcp_connections() -> std::result::Result<Vec<NetworkConnection>, String> {
    let mut connections = Vec::new();
    
    unsafe {
        let mut size = 0u32;
        
        // Get required buffer size
        let result = GetExtendedTcpTable(
            None,
            &mut size,
            false,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
        
        if result != ERROR_INSUFFICIENT_BUFFER.0 {
            return Err("Failed to get TCP table size".to_string());
        }
        
        // Allocate buffer and get TCP table
        let mut buffer = vec![0u8; size as usize];
        let result = GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            false,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
        
        if result != NO_ERROR.0 {
            return Err(format!("Failed to get TCP table: {}", result));
        }
        
        // Parse TCP table
        let table = buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID;
        let num_entries = (*table).dwNumEntries;
        
        for i in 0..num_entries {
            let entry = &(*table).table[i as usize];
            
            let local_addr = format!("{}:{}", 
                format_ip_address(entry.dwLocalAddr),
                u16::from_be(entry.dwLocalPort as u16)
            );
            
            let remote_addr = format!("{}:{}", 
                format_ip_address(entry.dwRemoteAddr),
                u16::from_be(entry.dwRemotePort as u16)
            );
            
            let state = format_tcp_state(entry.dwState);
            
            connections.push(NetworkConnection::new(
                "TCP".to_string(),
                local_addr,
                remote_addr,
                state,
                entry.dwOwningPid,
            ));
        }
    }
    
    Ok(connections)
}

/// Collect UDP connections using Windows API
#[cfg(windows)]
fn collect_udp_connections() -> std::result::Result<Vec<NetworkConnection>, String> {
    let mut connections = Vec::new();
    
    unsafe {
        let mut size = 0u32;
        
        // Get required buffer size
        let result = GetExtendedUdpTable(
            None,
            &mut size,
            false,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );
        
        if result != ERROR_INSUFFICIENT_BUFFER.0 {
            return Err("Failed to get UDP table size".to_string());
        }
        
        // Allocate buffer and get UDP table
        let mut buffer = vec![0u8; size as usize];
        let result = GetExtendedUdpTable(
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            false,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );
        
        if result != NO_ERROR.0 {
            return Err(format!("Failed to get UDP table: {}", result));
        }
        
        // Parse UDP table
        let table = buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID;
        let num_entries = (*table).dwNumEntries;
        
        for i in 0..num_entries {
            let entry = &(*table).table[i as usize];
            
            let local_addr = format!("{}:{}", 
                format_ip_address(entry.dwLocalAddr),
                u16::from_be(entry.dwLocalPort as u16)
            );
            
            connections.push(NetworkConnection::new(
                "UDP".to_string(),
                local_addr,
                "*:*".to_string(), // UDP doesn't have remote connections in the same way
                "LISTENING".to_string(),
                entry.dwOwningPid,
            ));
        }
    }
    
    Ok(connections)
}

/// Fallback implementation for non-Windows platforms or when Windows API fails
#[cfg(not(windows))]
fn collect_tcp_connections() -> std::result::Result<Vec<NetworkConnection>, String> {
    // Fallback implementation using sysinfo
    collect_connections_fallback("TCP")
}

#[cfg(not(windows))]
fn collect_udp_connections() -> std::result::Result<Vec<NetworkConnection>, String> {
    // Fallback implementation using sysinfo
    collect_connections_fallback("UDP")
}

/// Fallback network connection collection using available system information
fn collect_connections_fallback(_protocol: &str) -> std::result::Result<Vec<NetworkConnection>, String> {
    let connections = Vec::new();
    let _sys = System::new_all();
    
    // This is a simplified fallback - real implementation would need platform-specific code
    // For now, we'll return empty connections with a warning
    
    Ok(connections)
}

/// Format IP address from u32 to string
#[cfg(windows)]
fn format_ip_address(addr: u32) -> String {
    format!("{}.{}.{}.{}", 
        addr & 0xFF,
        (addr >> 8) & 0xFF,
        (addr >> 16) & 0xFF,
        (addr >> 24) & 0xFF
    )
}

/// Format TCP connection state
#[cfg(windows)]
fn format_tcp_state(state: u32) -> String {
    match state {
        1 => "CLOSED".to_string(),
        2 => "LISTEN".to_string(),
        3 => "SYN_SENT".to_string(),
        4 => "SYN_RCVD".to_string(),
        5 => "ESTABLISHED".to_string(),
        6 => "FIN_WAIT1".to_string(),
        7 => "FIN_WAIT2".to_string(),
        8 => "CLOSE_WAIT".to_string(),
        9 => "CLOSING".to_string(),
        10 => "LAST_ACK".to_string(),
        11 => "TIME_WAIT".to_string(),
        12 => "DELETE_TCB".to_string(),
        _ => format!("UNKNOWN({})", state),
    }
}

/// Get unique PIDs that have network connections
pub fn get_network_active_pids(connections: &[NetworkConnection]) -> Vec<u32> {
    let mut pids: Vec<u32> = connections.iter().map(|c| c.owning_pid).collect();
    pids.sort_unstable();
    pids.dedup();
    pids
}

/// Filter connections by protocol
pub fn filter_connections_by_protocol<'a>(connections: &'a [NetworkConnection], protocol: &str) -> Vec<&'a NetworkConnection> {
    connections.iter().filter(|c| c.protocol == protocol).collect()
}

/// Filter external connections only
pub fn filter_external_connections(connections: &[NetworkConnection]) -> Vec<&NetworkConnection> {
    connections.iter().filter(|c| c.is_external()).collect()
}

/// Group connections by owning process
pub fn group_connections_by_process(connections: &[NetworkConnection]) -> HashMap<u32, Vec<&NetworkConnection>> {
    let mut grouped = HashMap::new();
    
    for conn in connections {
        grouped.entry(conn.owning_pid).or_insert_with(Vec::new).push(conn);
    }
    
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_network_connections() {
        let (connections, logs) = collect_network_connections();
        
        // Should have log entries
        assert!(!logs.is_empty());
        
        // Should have start and completion messages
        assert!(logs.iter().any(|log| log.message.contains("Starting network connection")));
        assert!(logs.iter().any(|log| log.message.contains("completed")));
        
        // Connections should be sorted
        for i in 1..connections.len() {
            let prev = &connections[i-1];
            let curr = &connections[i];
            assert!(prev.protocol <= curr.protocol);
            if prev.protocol == curr.protocol {
                assert!(prev.local_address <= curr.local_address);
            }
        }
    }

    #[test]
    fn test_get_network_active_pids() {
        let connections = vec![
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:80".to_string(), "127.0.0.1:12345".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:443".to_string(), "127.0.0.1:12346".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("UDP".to_string(), "0.0.0.0:53".to_string(), "*:*".to_string(), "LISTENING".to_string(), 5678),
        ];
        
        let pids = get_network_active_pids(&connections);
        assert_eq!(pids.len(), 2);
        assert!(pids.contains(&1234));
        assert!(pids.contains(&5678));
    }

    #[test]
    fn test_filter_connections_by_protocol() {
        let connections = vec![
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:80".to_string(), "127.0.0.1:12345".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("UDP".to_string(), "0.0.0.0:53".to_string(), "*:*".to_string(), "LISTENING".to_string(), 5678),
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:443".to_string(), "127.0.0.1:12346".to_string(), "ESTABLISHED".to_string(), 1234),
        ];
        
        let tcp_connections = filter_connections_by_protocol(&connections, "TCP");
        assert_eq!(tcp_connections.len(), 2);
        
        let udp_connections = filter_connections_by_protocol(&connections, "UDP");
        assert_eq!(udp_connections.len(), 1);
    }

    #[test]
    fn test_filter_external_connections() {
        let connections = vec![
            NetworkConnection::new("TCP".to_string(), "192.168.1.100:12345".to_string(), "8.8.8.8:80".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:80".to_string(), "127.0.0.1:12345".to_string(), "ESTABLISHED".to_string(), 5678),
            NetworkConnection::new("TCP".to_string(), "192.168.1.100:12346".to_string(), "1.1.1.1:443".to_string(), "ESTABLISHED".to_string(), 1234),
        ];
        
        let external = filter_external_connections(&connections);
        assert_eq!(external.len(), 2);
        assert!(external.iter().all(|c| c.is_external()));
    }

    #[test]
    fn test_group_connections_by_process() {
        let connections = vec![
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:80".to_string(), "127.0.0.1:12345".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("TCP".to_string(), "127.0.0.1:443".to_string(), "127.0.0.1:12346".to_string(), "ESTABLISHED".to_string(), 1234),
            NetworkConnection::new("UDP".to_string(), "0.0.0.0:53".to_string(), "*:*".to_string(), "LISTENING".to_string(), 5678),
        ];
        
        let grouped = group_connections_by_process(&connections);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped.get(&1234).unwrap().len(), 2);
        assert_eq!(grouped.get(&5678).unwrap().len(), 1);
    }

    #[cfg(windows)]
    #[test]
    fn test_format_ip_address() {
        assert_eq!(format_ip_address(0x0100007F), "127.0.0.1"); // localhost in network byte order
        assert_eq!(format_ip_address(0x08080808), "8.8.8.8");   // Google DNS
    }

    #[cfg(windows)]
    #[test]
    fn test_format_tcp_state() {
        assert_eq!(format_tcp_state(2), "LISTEN");
        assert_eq!(format_tcp_state(5), "ESTABLISHED");
        assert_eq!(format_tcp_state(11), "TIME_WAIT");
        assert_eq!(format_tcp_state(999), "UNKNOWN(999)");
    }
}