#!/bin/bash
echo "🔧 Fixing comprehensive_icp_test.rs async/sync mismatch..."

# Replace #[tokio::test] with #[test] for non-async functions
sed -i '' '/#\[tokio::test\]/,/^    fn/ {
    /#\[tokio::test\]/ {
        N
        /\n.*async fn/ !s/#\[tokio::test\]/#[test]/
    }
}' tests/comprehensive_icp_test.rs

# Alternative approach - check each test function
echo "Checking test functions..."

# For functions that should be sync, use #[test]
# For functions that should be async, use #[tokio::test] and async fn

# List of functions that should remain sync (based on error messages)
sync_functions=(
    "test_wallet_creation_from_principal"
    "test_hd_wallet_derivation"
    "test_transaction_creation"
    "test_transaction_validation"
    "test_account_identifier_generation"
    "test_canister_client_creation"
    "test_icrc1_token_operations"
    "test_cross_chain_bridge_initialization"
    "test_bulk_wallet_creation_performance"
    "test_concurrent_transactions"
)

# Replace tokio::test with test for sync functions
for func in "${sync_functions[@]}"; do
    sed -i '' "/$func/,/^[[:space:]]*}/ {
        s/#\[tokio::test\]/#[test]/
    }" tests/comprehensive_icp_test.rs
done

echo "✅ Fixed async/sync attributes"
