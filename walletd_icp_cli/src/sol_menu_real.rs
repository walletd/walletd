// In sol_menu.rs, update case "3" to:
            "3" => {
                if let Err(e) = crate::sol_send_real::handle_solana_airdrop().await {
                    println!("Error: {}", e);
                }
            }
            
// Update case "4" to:
            "4" => {
                if let Err(e) = crate::sol_send_real::handle_send_solana_real().await {
                    println!("Error: {}", e);
                }
            }
