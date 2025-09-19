#!/bin/bash

# WalletD Prasaga Module
# This integrates with the main WalletD CLI

PRASAGA_BIN="$(dirname $0)/walletd-prasaga-avio/target/release/walletd_prasaga"

case "$1" in
    address)
        shift
        $PRASAGA_BIN address "$@"
        ;;
    balance)
        shift
        $PRASAGA_BIN balance "$@"
        ;;
    transfer)
        shift
        $PRASAGA_BIN transfer "$@"
        ;;
    *)
        echo "Usage: walletd prasaga [address|balance|transfer]"
        ;;
esac
