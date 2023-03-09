README for walletd_ethereum

For "local" examples, the examples require an instance of Ganache-CLI to be running. For "remote" examples, the examples expect a .env file with "INFURA_KEY" to be set. (this is still todo. See [issue #74](https://github.com/walletd/wallet-cli/issues/74))

To get all examples to line up with ganache-cli, instantiate ganache-cli with the following command:

```bash
ganache-cli -b 3 -m "mandate rude write gather vivid inform leg swift usual early bamboo element"
```

To run examples, you can run all of them 

```bash
cargo test --examples
```

You can also run specific examples by name

```bash
cargo run --example get_accounts_and_balances
```