# RustyPallets

A minimal, modular blockchain runtime written in Rust.  
This project demonstrates core blockchain concepts such as accounts, balances, extrinsics, blocks, and proof-of-existence, using a simple and educational architecture.

## Features

- **Balances Pallet:**  
  - Set and transfer balances between accounts.
- **Proof of Existence Pallet:**  
  - Submit and store claims for proof-of-existence.
- **System Pallet:**  
  - Handles block numbers and nonces.
- **Block Execution:**  
  - Simulate block production and extrinsic execution.
- **Modular Runtime:**  
  - Easily extendable with new pallets.

## Structure

- `src/main.rs` — Entry point, runtime definition, block execution logic, and demo.
- `src/balances.rs` — Balances pallet.
- `src/proof_of_existence.rs` — Proof of existence pallet.
- `src/system.rs` — System pallet (block number, nonce).
- `src/support.rs` — Shared types and traits.

## Usage

1. **Build the project:**
    ```sh
    cargo build
    ```

2. **Run the demo:**
    ```sh
    cargo run
    ```

3. **Format the code:**
    ```sh
    cargo fmt
    ```

4. **Run tests:**
    ```sh
    cargo test
    ```

## Example Output

The main function simulates two blocks:
- Block 1: Alice transfers balances to Bob and Charlie.
- Block 2: Alice and Bob submit proof-of-existence claims.

The final state of the runtime is printed.

## Requirements

- Rust (edition 2021)
- [num-traits](https://crates.io/crates/num-traits) crate

## License

MIT

---

**Educational project. Not for production use.**
