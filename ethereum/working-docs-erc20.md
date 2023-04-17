This doc is intended for clarifying ERC-20 token creation

# Interact with ERC-20 tokens
ERC-20​ is a simple token standard and the most common contract type on Ethereum.

You can:
[Send ERC-20 transactions​ using eth_sendRawTransaction.](https://docs.infura.io/infura/networks/ethereum/how-to/interact-with-erc-20-tokens#sending-transactions)
[Observe event logs of mined ERC-20 transactions​ using eth_getLogs](https://docs.infura.io/infura/networks/ethereum/how-to/interact-with-erc-20-tokens#mined-transactions)
Follow [this tutorial](https://docs.infura.io/infura/tutorials/ethereum/retrieve-the-balance-of-an-erc-20-token) to retrieve the balance of ERC-20 tokens.
Follow [this tutorial](https://docs.infura.io/infura/tutorials/ethereum/track-erc-20-token-transfers) to track ERC-20 token transfers.

## ERC-20 token functions and events

An ERC-20 token must implement the following functions:

totalSupply() - Returns the total token supply.
balanceOf(owner) - Returns the account balance of another account with address owner.
allowance(owner, spender) - Returns the amount which spender is still allowed to withdraw from owner.
transfer(to, value) - Transfers value amount of tokens to address to.
approve(spender, value) - Allows spender to withdraw from your account multiple times, up to the value amount.
transferFrom(from, to, value) - Transfers value amount of tokens from address from to address to.

At certain times, an ERC-20 token also must emit the following events:

Transfer(from, to, value) - Must trigger when tokens are transferred, including zero value transfers.
Approval(owner, spender, value) - Must trigger on any successful call to approve(spender, value).
View  for more details about how these functions work and when to emit these events.
