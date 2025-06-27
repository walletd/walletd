async fn list_channels(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Lightning Channels ===");
    
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    match manager.list_channels(user_id.trim()).await {
        Ok(channels) => {
            if channels.is_empty() {
                println!("No channels found for this user.");
            } else {
                println!("┌──────────────┬───────────┬────────────┬──────────┐");
                println!("│ Channel ID   │ Peer      │ Capacity   │ Status   │");
                println!("├──────────────┼───────────┼────────────┼──────────┤");
                for channel in channels {
                    println!("│ {:12} │ {:9} │ {:10} │ {:8} │",
                        &channel.channel_id[..12.min(channel.channel_id.len())],
                        &channel.peer_node_id[..9.min(channel.peer_node_id.len())],
                        channel.capacity_sats,
                        if channel.active { "Active" } else { "Pending" }
                    );
                }
                println!("└──────────────┴───────────┴────────────┴──────────┘");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
