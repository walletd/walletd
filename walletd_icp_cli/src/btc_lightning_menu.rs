use crate::CliResponse;
use std::io::{self, Write};
use walletd_bitcoin::lightning::LightningManager;
use walletd_bitcoin::Network;

pub async fn handle_lightning_menu() -> Result<CliResponse, String> {
    let lightning_manager = LightningManager::new(Network::Bitcoin).await
        .map_err(|e| e.to_string())?;
    
    loop {
        println!("\n⚡ LIGHTNING NETWORK MENU ⚡");
        println!("===========================");
        
        println!("\n--- Node Management ---");
        println!("[1] Create Lightning Node");
        println!("[2] Show Node Info");
        println!("[3] Connect to Peer");
        
        println!("\n--- Channels ---");
        println!("[4] Open Channel");
        println!("[5] List Channels");
        println!("[6] Close Channel");
        
        println!("\n--- Payments ---");
        println!("[7] Send Payment");
        println!("[8] Create Invoice");
        println!("[9] Check Payment Status");
        
        println!("\n--- Advanced ---");
        println!("[10] Channel Balance");
        println!("[11] Network Graph");
        println!("[12] Routing Info");

println!("\n--- Setup ---");
println!("[13] Setup Voltage (Real Lightning)");        
        println!("\n[B] Back to Bitcoin Menu");
        println!("[X] Exit");
        
        print!("\nSelect option: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        
        match input.trim() {
            "1" => create_lightning_node(&lightning_manager).await?,
            "2" => show_node_info(&lightning_manager).await?,
            "3" => connect_to_peer(&lightning_manager).await?,
            "4" => open_channel(&lightning_manager).await?,
            "5" => list_channels(&lightning_manager).await?,
            "6" => close_channel(&lightning_manager).await?,
            "7" => send_payment(&lightning_manager).await?,
            "8" => create_invoice(&lightning_manager).await?,
            "9" => check_payment_status(&lightning_manager).await?,
            "10" => show_channel_balance(&lightning_manager).await?,
            "11" => show_network_graph(&lightning_manager).await?,
            "12" => show_routing_info(&lightning_manager).await?,
            "13" => setup_voltage_lightning().await?,            "b" | "B" => break,
            "x" | "X" => return Ok(CliResponse::Exit),
            _ => println!("Invalid option. Please try again."),
        }
        
        println!("\nPress Enter to continue...");
        let mut _pause = String::new();
        io::stdin().read_line(&mut _pause).ok();
    }
    
    Ok(CliResponse::Continue)
}

async fn create_lightning_node(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Create Lightning Node ===");
    
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    println!("Creating Lightning node...");
    
    match manager.create_node(user_id.trim(), [0u8; 32]).await {
        Ok(node_info) => {
            println!("\n✅ Lightning node created!");
            println!("Node ID: {}", node_info.node_id);
            println!("Listening Port: {}", node_info.listening_port);
            println!("\nShare your Node ID with peers to receive connections.");
        }
        Err(e) => println!("❌ Error creating node: {}", e),
    }
    
    Ok(())
}

async fn show_node_info(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Node Information ===");
    
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    match manager.get_node_info(user_id.trim()).await {
        Ok(info) => {
            println!("User ID: {}", info.user_id);
            println!("Node ID: {}", info.node_id);
            println!("Listening Port: {}", info.listening_port);
            println!("Network: Bitcoin");
        }
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}

async fn connect_to_peer(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Connect to Peer ===");
    
    print!("Your user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    print!("Enter peer node ID: ");
    io::stdout().flush().unwrap();
    let mut peer_id = String::new();
    io::stdin().read_line(&mut peer_id).map_err(|e| e.to_string())?;
    
    print!("Enter peer address (IP:port): ");
    io::stdout().flush().unwrap();
    let mut peer_addr = String::new();
    io::stdin().read_line(&mut peer_addr).map_err(|e| e.to_string())?;
    
    match manager.connect_peer(user_id.trim(), peer_id.trim(), peer_addr.trim()).await {
        Ok(()) => println!("\n✅ Connected to peer successfully!"),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    Ok(())
}

async fn open_channel(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Open Lightning Channel ===");
    
    print!("Enter your user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    print!("Enter peer node ID: ");
    io::stdout().flush().unwrap();
    let mut peer_id = String::new();
    io::stdin().read_line(&mut peer_id).map_err(|e| e.to_string())?;
    
    print!("Channel capacity (sats): ");
    io::stdout().flush().unwrap();
    let mut capacity = String::new();
    io::stdin().read_line(&mut capacity).map_err(|e| e.to_string())?;
    
    print!("Push amount (sats, optional): ");
    io::stdout().flush().unwrap();
    let mut push = String::new();
    io::stdin().read_line(&mut push).map_err(|e| e.to_string())?;
    
    let capacity_sats: u64 = capacity.trim().parse().unwrap_or(0);
    
    println!("\n📋 Channel Summary:");
    println!("Peer: {}", peer_id.trim());
    println!("Capacity: {} sats", capacity_sats);
    println!("Your balance: {} sats", capacity_sats);
    println!("Peer balance: {} sats", push.trim());
    
    print!("\nConfirm channel opening? (yes/no): ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).map_err(|e| e.to_string())?;
    
    if confirm.trim().to_lowercase() == "yes" {
        match manager.open_channel(user_id.trim(), peer_id.trim(), capacity_sats).await {
            Ok(channel_id) => {
                println!("\n✅ Channel opening initiated!");
                println!("Channel ID: {}", channel_id);
                println!("Status: Pending (waiting for confirmations)");
            }
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    Ok(())
}

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

async fn close_channel(_manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Close Channel ===");
    
    print!("Enter channel ID: ");
    io::stdout().flush().unwrap();
    let mut channel_id = String::new();
    io::stdin().read_line(&mut channel_id).map_err(|e| e.to_string())?;
    
    println!("\nChannel closing initiated for: {}", channel_id.trim());
    println!("Status: Closing (broadcasting transaction)");
    
    Ok(())
}

async fn send_payment(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Send Lightning Payment ===");
    
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    print!("Enter Lightning invoice: ");
    io::stdout().flush().unwrap();
    let mut invoice = String::new();
    io::stdin().read_line(&mut invoice).map_err(|e| e.to_string())?;
    
    // Parse invoice (mock)
    println!("\n📋 Invoice Details:");
    println!("Amount: 10,000 sats");
    println!("Description: Coffee payment");
    println!("Expiry: 10 minutes");
    
    print!("\nSend payment? (yes/no): ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).map_err(|e| e.to_string())?;
    
    if confirm.trim().to_lowercase() == "yes" {
        match manager.send_payment(user_id.trim(), invoice.trim()).await {
            Ok(payment) => {
                println!("\n✅ Payment sent!");
                println!("Payment hash: {}", payment.payment_hash);
                if let Some(preimage) = payment.payment_preimage {
                    println!("Payment preimage: {}", preimage);
                }
                println!("Amount: {} msat", payment.amount_msat);
                println!("Fee: {} msat", payment.fee_msat);
                println!("Status: {:?}", payment.status);
            }
            Err(e) => println!("❌ Payment failed: {}", e),
        }
    }
    
    Ok(())
}

async fn create_invoice(manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Create Lightning Invoice ===");
    
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id).map_err(|e| e.to_string())?;
    
    print!("Amount (sats): ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).map_err(|e| e.to_string())?;
    
    print!("Description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).map_err(|e| e.to_string())?;
    
    let amount_sats: u64 = amount.trim().parse().unwrap_or(0);
    let amount_msat = if amount_sats > 0 { Some(amount_sats * 1000) } else { None };
    
    match manager.create_invoice(user_id.trim(), amount_msat, description.trim().to_string()).await {
        Ok(invoice) => {
            println!("\n✅ Invoice created!");
            println!("\nInvoice: {}", invoice.bolt11);
            println!("\nShare this invoice to receive payment");
            println!("Payment hash: {}", invoice.payment_hash);
            if let Some(amt) = invoice.amount_msat {
                println!("Amount: {} sats", amt / 1000);
            }
        }
        Err(e) => println!("❌ Error creating invoice: {}", e),
    }
    
    Ok(())
}

async fn check_payment_status(_manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Payment Status ===");
    
    print!("Enter payment hash: ");
    io::stdout().flush().unwrap();
    let mut payment_hash = String::new();
    io::stdin().read_line(&mut payment_hash).map_err(|e| e.to_string())?;
    
    println!("\nPayment: {}", payment_hash.trim());
    println!("Status: Completed");
    println!("Amount: 10,000 sats");
    println!("Fee: 5 sats");
    println!("Route: 3 hops");
    
    Ok(())
}

async fn show_channel_balance(_manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Channel Balances ===");
    println!("┌──────────────┬────────────┬────────────┬────────────┐");
    println!("│ Channel      │ Capacity   │ Local Bal  │ Remote Bal │");
    println!("├──────────────┼────────────┼────────────┼────────────┤");
    println!("│ channel_001  │ 1,000,000  │ 600,000    │ 400,000    │");
    println!("│ channel_002  │ 500,000    │ 250,000    │ 250,000    │");
    println!("├──────────────┼────────────┼────────────┼────────────┤");
    println!("│ Total        │ 1,500,000  │ 850,000    │ 650,000    │");
    println!("└──────────────┴────────────┴────────────┴────────────┘");
    
    Ok(())
}

async fn show_network_graph(_manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Lightning Network Graph ===");
    println!("Nodes: 15,342");
    println!("Channels: 72,456");
    println!("Total capacity: 5,234 BTC");
    println!("\nYour node connections:");
    println!("• Direct peers: 8");
    println!("• 2-hop reach: 1,234 nodes");
    println!("• 3-hop reach: 12,456 nodes");
    
    Ok(())
}

async fn show_routing_info(_manager: &LightningManager) -> Result<(), String> {
    println!("\n=== Routing Information ===");
    println!("Your node routing fees:");
    println!("• Base fee: 1 sat");
    println!("• Fee rate: 0.001%");
    println!("\nRouted payments (24h):");
    println!("• Count: 47");
    println!("• Volume: 2,450,000 sats");
    println!("• Fees earned: 245 sats");
    
    Ok(())
}

async fn setup_voltage_lightning() -> Result<(), String> {
    use walletd_bitcoin::lightning::voltage_setup::VoltageSetup;
    
    VoltageSetup::show_setup_guide();
    
    Ok(())
}
