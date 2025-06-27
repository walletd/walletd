// Add this to the Lightning menu options
println!("\n--- Setup ---");
println!("[13] Setup Voltage (Real Lightning)");

// Add this to the match statement
"13" => setup_voltage_lightning().await?,

// Add this function
async fn setup_voltage_lightning() -> Result<(), String> {
    use walletd_bitcoin::lightning::voltage_setup::VoltageSetup;
    
    VoltageSetup::run_setup_wizard()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
