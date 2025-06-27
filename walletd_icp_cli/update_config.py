import json
import os

config_path = "walletd_config.json"

# Create testnet config
testnet_config = {
    "mode": "testnet",
    "demo_mode": False,
    "bitcoin": {
        "network": "testnet",
        "rpc_url": "https://blockstream.info/testnet/api",
        "electrum_url": "testnet.aranguren.org:51002"
    },
    "ethereum": {
        "chain_id": 11155111,
        "rpc_url": "https://eth-sepolia.g.alchemy.com/v2/demo",
        "etherscan_api_key": None
    },
    "solana": {
        "cluster": "devnet",
        "rpc_url": "https://api.devnet.solana.com"
    },
    "monero": {
        "network": "stagenet",
        "daemon_url": "http://stagenet.xmr-tw.org:38081"
    },
    "hedera": {
        "network": "testnet",
        "operator_id": None,
        "operator_key": None
    },
    "icp": {
        "network": "local",
        "identity_path": None,
        "ic_url": "http://localhost:8000"
    }
}

# Save it
with open(config_path, 'w') as f:
    json.dump(testnet_config, f, indent=2)

print(f"✅ Updated {config_path} to testnet mode!")
print("🔄 Restart the application to connect to testnets")
