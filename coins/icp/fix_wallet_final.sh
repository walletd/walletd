#!/bin/bash
# Fix wallet.rs struct initialization
perl -i -pe 's/(network,)$/$1\n            did: None,/ if /network,$/' src/wallet.rs
