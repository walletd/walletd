⚠️ This project is under heavy development. Expect bugs & breaking changes prior to v1.0.0's release.
## What is WalletD?
WalletD is a collection of Rust libraries (published as crates) that make interacting with various different cryptocurrency chains easier than existing Rust-based solutions.

WalletD will be compiled into a number of different bindings to allow for non-Rust developers to consume the libraries in their native languages.

This repo contains the core WalletD library that leverages all Rust-based functionality the WalletD framework will offer.
## Child crates

WalletD is the core library that encapsulates our child crates. These child crates are as follows:

`mnemonic` is a hierarchical deterministic (HD) key generator. More information about this crate can be found in the [crate documentation][docs]. Supports BIP32, BIP39, BIP44, BIP49, BIP84, Monero seeds (25 words (Monero standard) and 14 words (MyMonero legacy seed phrases)).

`wallet-cli` is a command-line tool used for testing and developing new features, when it is more convenient to do so instead of directly in new crates.

## Release cycles
During this stage of development, expect v0.x.x releases to be highly experimental, and likely to undergo massive change.

Once stable, (that is, from v1.x.x) WalletD plan to release new versions of [WalletD][WalletD-github] every TBD weeks.

TBD: Bindings for popular languages will be provided in each tagged release. 

Should any users want to build from scratch, instructions for building the various packages are also provided.

The WalletD team adhere to traditional semantic versioning when releasing code. For more information regarding [semantic versioning, please refer to the semver specification][semver].

TBD: The authors of WalletD provide upgrade guides for any new major version releases. (eg, v1.0.0 -> v2.0.0) As part of this upgrade guide, users will be made aware of features that may be deprecated in future releases at least one major version prior, to provide sufficient time for users to upgrade as necessary.

TBD: Release cycle. With regards to security patches, the WalletD team will endeavour to backport any security fixes to the last x branches released.
## Usage example

```rust
// TO DO
```

You can find this [example][readme-example] as well as other example projects in the [example directory][examples].

See the [crate documentation][docs] for additional examples.

## Building WalletD yourself

Clone the repository from GitHub
Run `cargo build`

The built bindings can be found in the TBD folder.

TBD: Building just one specific set of bindings.