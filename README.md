⚠️ This project is under heavy development. Expect bugs & breaking changes prior to v1.0.0's release.
## What is walletd?
walletd is a collection of Rust libraries (published as crates) that make interacting with various different cryptocurrency chains easier than existing Rust-based solutions.

walletd will be compiled into a number of different bindings to allow for non-Rust developers to consume the libraries in their native languages.

This repo contains the core walletd library that leverages all Rust-based functionality the walletd framework will offer.
[![codecov](https://codecov.io/gh/walletd/mnemonic/branch/main/graph/badge.svg?token=BA4YTBMLEP)](https://codecov.io/gh/walletd/mnemonic)
## Child crates

walletd is the core library that encapsulates our child crates. These child crates are as follows:

`mnemonic` is a hierarchical deterministic (HD) key generator. More information about this crate can be found in the [crate documentation][docs].

`wallet-cli` is a command-line tool used for testing and developing new features, when it is more convenient to do so instead of directly in new crates.

## Release cycles
During this stage of development, expect v0.x.x releases to be highly experimental, and likely to undergo massive change.

Once stable, (that is, from v1.x.x) walletd plan to release a new version of [walletd][walletd-github], the core cryptocurrency library that depends on this mnemonic library. 

The walletd team adhere to traditional semver commit messages.

The authors of walletd provide upgrade guides for any new major version releases. (eg, v1.0.0 -> v2.0.0) As part of this upgrade guide, users will be made aware of features that may be deprecated in future releases at least one major version prior, to provide sufficient time for users to upgrade as necessary.

With regards to security patches, the walletd team will endeavour to backport any security fixes to the last x branches released.

- BIP32.
- BIP39.
- BIP44.
- BIP49.
- BIP84.
- Monero seeds 14 word (mymonero style) or 25 words.

## Usage example

```rust
// still to come
```

You can find this [example][readme-example] as well as other example projects in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

mnemonic's MSRV is 1.60.

## Examples

The [examples] folder contains various examples of how to use `mnemonic`. The
[docs] also provide lots of code snippets and examples.

## Getting Help

In the `mnemonic`'s repo we also have a [number of examples][examples] showing how
to put everything together. You're also welcome to open a [discussion] with your question.

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`mnemonic` project.

## License

Licensed under the [Apache license][license-apache], Version 2.0
or the [MIT license][license-mit], at your option. Files in the project may not be copied, modified, or distributed except according to those terms.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mnemonic` by you, shall be licensed as MIT, without any
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