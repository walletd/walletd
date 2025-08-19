#!/bin/bash

# Find line with "7" => { and add the missing code
sed -i '' '/"7" => {/,/^[[:space:]]*}/ {
    /"7" => {/a\
                    erc20_menu::handle_erc20_menu(&mut wallet_api, &eth_address, &eth_balance).await
}' walletd_icp_cli/src/main.rs

echo "Fixed case 7"
