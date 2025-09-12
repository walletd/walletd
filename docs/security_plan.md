# Security Plan for WalletD

## Phase 3 Security Measures
- Engage CertiK and Cyvers for audits.
- Implemented input validation in `call_canister`, `approve`, `transfer_from`, `batch_transfer`, and 
`swap_icp_to_btc`.
- Added re-entrancy guards to prevent concurrent modifications.
- Schedule penetration testing post-audit.
