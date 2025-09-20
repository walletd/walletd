use std::env;
use walletd_prasaga_avio::types::PrasagaAvioAddress;
use walletd_prasaga_avio::PrasagaAvioKeypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "keygen" => handle_keygen(),
        "address" => {
            if args.len() < 3 {
                println!("Usage: simple_cli address <public_key_hex>");
                return Ok(());
            }
            handle_address(&args[2])
        }
        "sign" => {
            if args.len() < 4 {
                println!("Usage: simple_cli sign <message> <private_key_hex>");
                return Ok(());
            }
            handle_sign(&args[2], &args[3])
        }
        "help" | "--help" => {
            print_help();
            Ok(())
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("Prasaga Avio Simple CLI");
    println!("=======================");
    println!();
    println!("Commands:");
    println!("  keygen              Generate a new keypair");
    println!("  address <pubkey>    Get address from public key");
    println!("  sign <msg> <key>    Sign a message");
    println!("  help                Show this help");
}

fn handle_keygen() -> Result<(), Box<dyn std::error::Error>> {
    let random_seed = rand::random::<[u8; 32]>();
    let keypair = PrasagaAvioKeypair::from_seed(&random_seed, "m/44'/9000'/0'/0/0")?;
    let address = PrasagaAvioAddress::from_public_key(&keypair.public_key_bytes())?;

    println!("Keypair Generated:");
    println!("==================");
    println!("Public Key:  0x{}", hex::encode(keypair.public_key_bytes()));
    println!(
        "Private Key: 0x{}",
        hex::encode(keypair.private_key_bytes())
    );
    println!("Address:     {address}");

    Ok(())
}

fn handle_address(pubkey_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubkey_hex = pubkey_hex.trim_start_matches("0x");
    let pubkey = hex::decode(pubkey_hex)?;
    let address = PrasagaAvioAddress::from_public_key(&pubkey)?;
    println!("Address: {address}");
    Ok(())
}

fn handle_sign(message: &str, key_hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_hex = key_hex.trim_start_matches("0x");
    let key_bytes = hex::decode(key_hex)?;

    if key_bytes.len() != 32 {
        return Err("Private key must be 32 bytes".into());
    }

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&key_bytes);

    let keypair = PrasagaAvioKeypair::from_seed(&seed, "m/44'/9000'/0'/0/0")?;
    let signature = keypair.sign(message.as_bytes());

    println!("Message:   {message}");
    println!("Signature: 0x{}", hex::encode(signature));

    Ok(())
}
