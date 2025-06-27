#!/bin/bash
echo "🔧 Fixing final two test files..."

# Fix crosschain_e2e_test.rs
echo "Fixing crosschain_e2e_test.rs..."
# Remove extra arguments from transfer calls
sed -i '' '/coordinator\.transfer(/,/)/ {
    # Keep only first 3 arguments
    s/transfer(\([^,]*\), \([^,]*\), \([^,]*\),.*/transfer(\1, \2, \3)/
}' tests/crosschain_e2e_test.rs

# Fix specific problematic lines if any
sed -i '' 's/"rrkah-fqaaa-aaaaa-aaaaq-cai"\.to_string(),//g' tests/crosschain_e2e_test.rs
sed -i '' 's/"0x742d35Cc6634C0532925a3b844Bc9e7595f6d8e3"\.to_string(),//g' tests/crosschain_e2e_test.rs
sed -i '' 's/"ICP"\.to_string()//g' tests/crosschain_e2e_test.rs
sed -i '' 's/"TOKEN"\.to_string()//g' tests/crosschain_e2e_test.rs

# Fix canister_advanced_test.rs
echo "Fixing canister_advanced_test.rs..."
# Add .await to async calls
sed -i '' 's/client\.call(\([^)]*\))$/client.call(\1).await/g' tests/canister_advanced_test.rs
sed -i '' 's/client\.call(\([^)]*\))\.unwrap()/client.call(\1).await.unwrap()/g' tests/canister_advanced_test.rs

# Make test functions async if they use await
awk '
/#\[test\]/ {
    test_attr = $0
    getline
    if ($0 ~ /fn.*\(/) {
        fn_line = $0
        # Look ahead to see if function uses await
        found_await = 0
        brace_count = 0
        start_checking = 0
        saved_lines = ""
        
        while (getline) {
            saved_lines = saved_lines "\n" $0
            if ($0 ~ /{/) {
                brace_count++
                start_checking = 1
            }
            if (start_checking && $0 ~ /\.await/) {
                found_await = 1
            }
            if ($0 ~ /}/) {
                brace_count--
                if (brace_count == 0) break
            }
        }
        
        if (found_await) {
            print "#[tokio::test]"
            gsub(/fn /, "async fn ", fn_line)
        } else {
            print test_attr
        }
        print fn_line
        print saved_lines
        next
    }
}
{print}
' tests/canister_advanced_test.rs > tests/canister_advanced_test.rs.tmp
mv tests/canister_advanced_test.rs.tmp tests/canister_advanced_test.rs

echo "✅ Fixes applied!"
