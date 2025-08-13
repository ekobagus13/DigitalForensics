// Example program to test the event_logs module
use triageir_cli::event_logs;

fn main() {
    println!("Testing Event Logs Collection Module");
    println!("====================================");
    
    // Collect event logs
    let (event_logs, collection_logs) = event_logs::collect_event_logs();
    
    // Display collection logs
    println!("\nCollection Logs:");
    for log in &collection_logs {
        println!("[{}] {}: {}", log.timestamp, log.level, log.message);
    }
    
    // Display summary
    println!("\nEvent Log Summary:");
    println!("Security events: {}", event_logs.security.len());
    println!("System events: {}", event_logs.system.len());
    println!("Application events: {}", event_logs.application.len());
    println!("Total events: {}", event_logs.total_entries());
    
    // Show some sample events if available
    if !event_logs.security.is_empty() {
        println!("\nSample Security Events:");
        for (i, event) in event_logs.security.iter().take(3).enumerate() {
            println!("  {}. Event ID: {}, Level: {}, Time: {}", 
                i + 1, event.event_id, event.level, event.timestamp);
            println!("     Message: {}", event.message);
        }
    }
    
    if !event_logs.system.is_empty() {
        println!("\nSample System Events:");
        for (i, event) in event_logs.system.iter().take(3).enumerate() {
            println!("  {}. Event ID: {}, Level: {}, Time: {}", 
                i + 1, event.event_id, event.level, event.timestamp);
            println!("     Message: {}", event.message);
        }
    }
    
    if !event_logs.application.is_empty() {
        println!("\nSample Application Events:");
        for (i, event) in event_logs.application.iter().take(3).enumerate() {
            println!("  {}. Event ID: {}, Level: {}, Time: {}", 
                i + 1, event.event_id, event.level, event.timestamp);
            println!("     Message: {}", event.message);
        }
    }
    
    // Test filtering functions
    let mut all_events: Vec<_> = event_logs.security.clone();
    all_events.extend(event_logs.system.clone());
    all_events.extend(event_logs.application.clone());
    
    if !all_events.is_empty() {
        println!("\nFiltering Examples:");
        
        // Find logon events
        let logon_events = event_logs::find_logon_events(&event_logs.security);
        println!("Logon-related events found: {}", logon_events.len());
        
        // Find process events
        let process_events = event_logs::find_process_events(&event_logs.security);
        println!("Process-related events found: {}", process_events.len());
        
        // Get recent events
        let recent_events = event_logs::get_recent_events(&all_events, 5);
        println!("Most recent 5 events: {}", recent_events.len());
    }
    
    println!("\nEvent logs collection test completed successfully!");
}