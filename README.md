⚠️ This project is under heavy development. Expect bugs & breaking changes prior to v1.0.0's release.
## What is walletd?
walletd is a collection of Rust libraries (published as crates) that make interacting with various different cryptocurrency chains easier than existing Rust-based solutions.

walletd will be compiled into a number of different bindings to allow for non-Rust developers to consume the libraries in their native languages.

This repo contains the core walletd library that leverages all Rust-based functionality the walletd framework will offer.
## Child crates

walletd is the core library that encapsulates our child crates. These child crates are as follows:

`mnemonic` is a hierarchical deterministic (HD) key generator. More information about this crate can be found in the [crate documentation][docs]. Supports BIP32, BIP39, BIP44, BIP49, BIP84, Monero seeds (25 words (Monero standard) and 14 words (MyMonero legacy seed phrases)).

`wallet-cli` is a command-line tool used for testing and developing new features, when it is more convenient to do so instead of directly in new crates.

## Release cycles
During this stage of development, expect v0.x.x releases to be highly experimental, and likely to undergo massive change.

Once stable, (that is, from v1.x.x) walletd plan to release new versions of [walletd][walletd-github] every TBD weeks.

TBD: Bindings for popular languages will be provided in each tagged release. 

Should any users want to build from scratch, instructions for building the various packages are also provided.

The walletd team adhere to traditional semantic versioning when releasing code. For more information regarding [semantic versioning, please refer to the semver specification][semver].

TBD: The authors of walletd provide upgrade guides for any new major version releases. (eg, v1.0.0 -> v2.0.0) As part of this upgrade guide, users will be made aware of features that may be deprecated in future releases at least one major version prior, to provide sufficient time for users to upgrade as necessary.

TBD: Release cycle. With regards to security patches, the walletd team will endeavour to backport any security fixes to the last x branches released.
## Usage example

```rust
// TO DO
```

You can find this [example][readme-example] as well as other example projects in the [example directory][examples].

See the [crate documentation][docs] for additional examples.

## Building walletd yourself

Clone the repository from GitHub
Run `cargo build`

The built bindings can be found in the TBD folder.

TBD: Building just one specific set of bindings.
## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

** KB Note: Presently, this version is up for discussion

mnemonic's MSRV is 1.60 for all pacakges, unless explicitly specified otherwise on that package's README. 


## Examples

Once walletd is ready for release, we endeavour to include examples of how to use it in each project, both in the README and in the various examples folders.

The [examples] folder contains various examples of how to use this library. The
[docs] also provide lots of code snippets and examples.

## Getting Help

If you are experiencing a bug / issue, or would like to propose a feature, we encourage you to open up an issue request detailing whatever you'd like us to look at.

In each repo we also have a [number of examples][examples] showing how to leverage it. You're also welcome to open a [discussion] with your question.

## Contributing

:balloon: Thanks for your help improving the project! We are happy to have you get involved. We have a [contributing guide][contributing] to help you get started with contributions to walletd projects.

## License

Licensed under the [Apache license][license-apache], Version 2.0
or the [MIT license][license-mit], at your option. Files in the project may not be copied, modified, or distributed except according to those terms.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in walletd (or its child crates) by you, shall be subject to the same license restrictions walletd adheres to, without any
additional terms or conditions.

[readme-example]: https://github.com/walletd/mnemonic/tree/main/examples/readme
[examples]: https://github.com/walletd/mnemonic/tree/main/examples
[docs]: https://docs.rs/walletd_mnemonic
[contributing]: https://github.com/walletd/mnemonic/blob/main/CONTRIBUTING.md
[discussion]: https://github.com/walletd/mnemonic/discussions/new?category=q-a
[ecosystem]: https://github.com/walletd/mnemonic/blob/main/ECOSYSTEM.md
[license-mit]: https://github.com/walletd/mnemonic/blob/main/LICENSE-MIT
[license-apache]: https://github.com/walletd/mnemonic/blob/main/LICENSE-APACHE
[walletd-github]: https://github.com/walletd/walletd
[semver]: https://semver.org