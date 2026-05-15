# Security Policy

WalletD handles cryptographic keys and signs blockchain transactions. We take
security seriously and welcome responsible disclosure of vulnerabilities.

## Supported Versions

WalletD is in active development and has not yet reached a 1.0 release. Security
fixes are applied to the latest published version on the `main` branch. We
encourage users to track `main` until a stable release is tagged.

| Version | Supported          |
| ------- | ------------------ |
| `main`  | :white_check_mark: |
| < `main` | :x:               |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues,
discussions, or pull requests.**

Instead, email security reports to **security@walletd.dev**.

Please include as much of the following information as you can:

- The type of issue (e.g. memory safety, key handling, signature forgery,
  side-channel, dependency vulnerability)
- The location of the affected source code (path, file, line, commit, branch)
- Steps to reproduce
- Proof-of-concept or exploit code, if available
- Potential impact and any suggested mitigation

## What to Expect

- **Acknowledgement** within 72 hours of receipt
- **Initial assessment** within 7 days
- **Coordinated disclosure** — we will agree on a disclosure timeline with the
  reporter before any public discussion, advisory, or fix is published
- **Credit** in the resulting advisory and release notes, with the reporter's
  permission

## Scope

In scope:

- Cryptographic key generation, storage, and zeroization
- Mnemonic and seed handling (BIP32/BIP39/BIP44 paths and derivations)
- Transaction signing and broadcast logic
- Address generation and verification across supported chains
- Vulnerabilities in any of the workspace crates published from this repository

Out of scope:

- Issues in third-party services or RPC providers WalletD connects to
- Issues that require physical access to a user's machine
- Social engineering of users or maintainers
- Denial-of-service via excessive resource consumption against the user's own
  process

## Good Practices for Users

While the policy above covers our side, users should also:

- Never use test or example mnemonics ("abandon abandon …") for real funds
- Keep dependencies updated and run `cargo audit` periodically
- Use hardware wallets where possible for production workloads
- Treat configuration files containing keys or seeds as sensitive
