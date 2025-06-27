#!/bin/bash
error_count=$(cargo check --tests 2>&1 | grep -c "error:")
if [ $error_count -eq 0 ]; then
    echo "All tests compile successfully!"
else
    echo "Remaining errors:"
    cargo check --tests 2>&1 | grep "error\[" | head -10
fi
