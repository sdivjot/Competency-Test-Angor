# Liquid CLI

CLI tool for interacting the Liquid Network, built as part of Angor competency test.

## Demo

![Screenshot 2025-04-17 222706](https://github.com/user-attachments/assets/357d1ba2-c4a2-4873-b1c5-7593ffb22745)


## Features

- Generates Liquid addresses (confidential and non-confidential)
- Displays information about Liquid assets

## Prerequisites

- Rust and Cargo
- Elements Core (for running Liquid Network node)

## Installation

1. Clone the repository
2. Build the project:
   ```
   cargo build --release
   ```
3. The binary will be available at `target/release/liquid-cli`

## Usage

### Generate a Liquid Address

Generate a confidential Liquid address (default):
```
liquid-cli generate-address
```

Generate a specific type of address:
```
liquid-cli generate-address --address-type p2pkh
liquid-cli generate-address --address-type p2sh
liquid-cli generate-address --address-type p2wpkh
liquid-cli generate-address --address-type p2wsh
```

Generate a non-confidential address:
```
liquid-cli generate-address --non-confidential
```

### Get Asset Information

Get information about a Liquid asset:
```
liquid-cli asset-info --asset-id <asset_id>
```

Example 
(for L-BTC) :
```
liquid-cli asset-info --asset-id 6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d
```

## Setting up a Liquid Network Node

1. Download Elements Core from [GitHub](https://github.com/ElementsProject/elements/releases)
2. Extract the archive
3. Create a configuration file `elements.conf`:
   ```
   chain=liquidtestnet
   server=1
   rpcuser=user
   rpcpassword=password
   rpcallowip=127.0.0.1
   txindex=1
   validatepegin=0
   fallbackfee=0.00000100
   daemon=1
   [liquidtestnet]
   port=7042
   rpcport=7041
   ```
4. Create a data directory for the Liquid testnet
5. Start the Elements daemon:
   ```bash
   # Start the Elements daemon
   ./elements-23.2.7/bin/elementsd -conf=elements.conf -datadir=liquidtestnet

   # Check the node status
   ./elements-23.2.7/bin/elements-cli -conf=elements.conf -datadir=liquidtestnet getblockchaininfo

   # Stop the Elements daemon
   ./elements-23.2.7/bin/elements-cli -conf=elements.conf -datadir=liquidtestnet stop
   ```
