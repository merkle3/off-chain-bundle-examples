# Off-Chain Bundle Examples

This repo contains examples for using off-chain bundles, supported by the Merkle block builder. Off-chain bundles consists of both regular transactions and off-chain transactions. Off-chain transactions are objects that allow the block builder to initially construct and simulate a transaction off-chain. The return calldata of this simulation is used to construct and settle an on-chain transaction. To learn more about off-chain bundles refer to the official [docs](https://docs.merkle.io/block-builder/off-chain-bundles).

## Examples:  

### Basic

This example includes simple skeleton **solidity** contracts for using off-chain bundles and a demo application that creates and submits an off-chain bundle. **Basic.sol** demonstrates the use of a **search()** function that is simulated off-chain and a **settle()** function that is executed on-chain. **BasicOnlySearch.sol** and **BasicOnlySettle.sol** separate **Basic.sol** logic into a deployed contract and a contract passed in the off-chain transaction as bytecode. In this example **BasicOnlySettle.sol** is deployed and **BasicOnlySearch.sol** can be passed as bytecode. This allows for shielding strategy logic in the search contract and smaller deployed code size.

The app is a rust script that constructs off-chain bundles with and without the bytecode argument for the off-chain transaction.

### Blind Backrun

Blind Backrun is an adaptation of the [simple-blind-arbitrage](https://github.com/flashbots/simple-blind-arbitrage) example for blind backruns written by Flashbots. In this example the contracts have been updated and modified to separate the logic into an off-chain search component and an on-chain settle component similar to the Basic example. It demonstrates the massively reduced gas usage of the settle transaction that actually gets settled on-chain.

## Usage

### Prerequisites

-   Rust ([rustup](https://www.rust-lang.org/tools/install))
-   Foundry ([book](https://book.getfoundry.sh))

### Install

Install contract library dependencies.
```bash
$ forge install
```

### Basic Example Commands

```bash
# build
$ FOUNDRY_PROFILE=basic forge build

# test
$ FOUNDRY_PROFILE=basic forge test

# run app
$ cd basic
$ cargo run
```

### Blind Backrun Example Commands
```bash
# build
$ FOUNDRY_PROFILE=blind-backrun forge build —via-ir

# test
$ FOUNDRY_PROFILE=blind-backrun forge test —via-ir -f https://eth.merkle.io
```
