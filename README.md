# Bitcoin Address Miner

## Overview

The Bitcoin Address Miner is a utility written in `Rust` that searches private keys for specific bitcoin wallet address (*Base58 P2PKH*) [BIP-32](https://en.bitcoin.it/wiki/BIP_0032)  
- *Base58 P2PKH* refers to a specific encoding format used in Bitcoin addresses:  
1. **Base58** : This is a binary-to-text encoding scheme that is similar to Base64 but avoids using  
   easily confused characters  (like 0, O, I, and l) to make strings more legible.  
   Base58 is commonly used in Bitcoin addresses to represent data in a human-readable format.  
2. **P2PKH** : This stands for "Pay to Public Key Hash." It is a standard transaction script in Bitcoin that specifies how funds can be spent.  
   In a P2PKH transaction, the recipient's address is derived from the hash of their public key.  

So, when you combine Base58 encoding with a P2PKH address, you get a format that looks like a string of characters, such as `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa`.
This is the well-known address associated with the first-ever Bitcoin transaction.

## Scope
The Bitcoin Address Miner is designed for individuals interested in exploring Bitcoin address generation and conducting experiments related to address matching. Potential use cases include:

- *Educational Purposes*: Learn about Bitcoin address generation, key pairs, and the structure of Bitcoin addresses.

- *Security Research*: Test the robustness of address generation algorithms and study patterns in address creation.

## Features
- Scans specified range of bitcoin private keys in a search for specific wallet address
- Prints target wallet address private key if it is found
- Calculates and displays the elapsed time for each scan iteration
## Prerequisites 
- [Rust](https://www.rust-lang.org/) 


## How to Use 
1. Clone the repository:

```console
git clone https://github.com/konard/bitcoin-address-miner
``` 
2. Navigate to the project directory:

```console
cd bitcoin-address-miner
``` 
3. Build and run the program:

```console
cargo run --release
```