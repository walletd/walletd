#!/bin/bash

# First, let's see what line number case "7" starts at
LINE=$(grep -n '"7" => {' walletd_icp_cli/src/main.rs | cut -d: -f1)

# Remove the malformed block (usually 3-4 lines)
sed -i '' "${LINE},/^[[:space:]]*}/d" walletd_icp_cli/src/main.rs

# Now add the correct case at the same position
sed -i '' "${LINE}i\\
                \"7\" => erc20_menu::handle_erc20_menu(&mut wallet_api, &eth_address, &eth_balance).await," walletd_icp_cli/src/main.rs

echo "Fixed case 7 block"
