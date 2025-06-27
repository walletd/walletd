#!/bin/bash
echo "🔧 Removing duplicate verify_secret methods..."

# Create a temporary file with deduplicated content
awk '
/pub fn verify_secret\(/ {
    if (seen_verify_secret) {
        # Skip this function definition
        in_verify_secret = 1
        brace_count = 0
        next
    }
    seen_verify_secret = 1
}
in_verify_secret {
    if ($0 ~ /{/) brace_count++
    if ($0 ~ /}/) {
        brace_count--
        if (brace_count == 0) {
            in_verify_secret = 0
            next
        }
    }
    next
}
{ print }
' src/crosschain/mod.rs > src/crosschain/mod.rs.tmp

# Replace the original file
mv src/crosschain/mod.rs.tmp src/crosschain/mod.rs

echo "✅ Duplicates removed!"
