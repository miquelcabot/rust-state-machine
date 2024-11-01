# rust-state-machine
Polkadot SDK-like state machine written in Rust

## Overview

This project is a simplified implementation of a blockchain state machine inspired by the Polkadot SDK. It includes basic modules for handling balances, proof of existence, and system-level operations. The state machine is designed to be extensible and modular, allowing for easy addition of new functionality.

## Modules

### `src/main.rs`

The entry point of the application. It defines the main runtime and integrates various modules.

### `src/support.rs`

Contains fundamental data structures and traits used across the project, such as `Block`, `Header`, `Extrinsic`, and the `Dispatch` trait.

### `src/balances.rs`

Implements the balances module, which handles account balances and related operations.

### `src/proof_of_existence.rs`

Implements the proof of existence module, which allows users to register and verify the existence of digital content.

### `src/system.rs`

Implements the system module, which handles system-level operations and configurations.

## Usage

To build and run the project, use the following commands:

```sh
cargo build
cargo run
```
