# walletd hd_key

`hd_key` is a Rust implementation of the BIP-32 standard's hierarchical deterministic (HD) key generation. 

More information about this crate can be found in the [crate documentation][docs].

## High level features

This library supports key generation using the following BIPs (Bitcoin Improvement Proposal).

- [BIP32][bip32] - Hierarchical Deterministic Wallets
- [BIP44][bip44] - Multi-Account Hierarchy for Deterministic Wallets
- [BIP49][bip49] - Derivation scheme for P2WPKH-nested-in-P2SH based accounts
- [BIP84][bip84] - Derivation scheme for P2WPKH based accounts

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

hd_key's MSRV is 1.60.

## Examples

The [examples] folder contains various examples of how to use `hd_key`. The
[docs] also provide lots of code snippets and examples.

## Getting Help

In the `hd_key`'s repo we also have a [number of examples][examples] showing how
to put everything together. You're also welcome to open a [discussion] with your question.

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`hd_key` project.

## License

Licensed under the [Apache license][license-apache], Version 2.0
or the [MIT license][license-mit], at your option. Files in the project may not be copied, modified, or distributed except according to those terms.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `hd_key` by you, shall be licensed as MIT, without any
additional terms or conditions.

[readme-example]: https://github.com/walletd/hd_key/tree/main/examples/readme
[examples]: https://github.com/walletd/hd_key/tree/main/examples
[docs]: https://docs.rs/walletd_hd_key
[contributing]: https://github.com/walletd/hd_key/blob/main/CONTRIBUTING.md
[discussion]: https://github.com/walletd/hd_key/discussions/new?category=q-a
[ecosystem]: https://github.com/walletd/hd_key/blob/main/ECOSYSTEM.md
[license-mit]: https://github.com/walletd/hd_key/blob/main/LICENSE-MIT
[license-apache]: https://github.com/walletd/hd_key/blob/main/LICENSE-APACHE
[bip32]: https://en.bitcoin.it/wiki/BIP_0032
[bip44]: https://en.bitcoin.it/wiki/BIP_0044
[bip49]: https://en.bitcoin.it/wiki/BIP_0049
[bip84]: https://en.bitcoin.it/wiki/BIP_0084