# Stellar Pi Coin SDK API Reference

**Comprehensive API Reference for Soroban Contracts and CLI Tools**

This reference covers all Soroban contracts (Rust) and CLI tools (Python) in the Stellar Pi Coin SDK. Each entry includes parameters, return types, and hyper-tech features. See `/cli/examples_cli.py` for usage demos.

## Table of Contents
- [Soroban Contracts](#soroban-contracts)
- [CLI Tools](#cli-tools)

## Soroban Contracts
Located in `/contracts/`. Deploy via Soroban CLI.

### PiCoin Contract
**Location**: `contracts/pi_coin/src/lib.rs`  
**Description**: Core stablecoin with minting, transfers, supply cap.  
**Hyper-Tech**: Quantum signatures, Pi-math hashing, AI-modulated supply.

- `init(env: Env, admin: Address)`  
  Initializes contract with supply cap, PI value, sources, quantum key.  
  - `admin`: Admin address.

- `mint(env: Env, to: Address, amount: u64, source: Symbol) -> PiCoin`  
  Mints PI with verification.  
  - `to`: Recipient.  
  - `amount`: Amount.  
  - `source`: Origin.  
  - Returns: PiCoin instance.

- `transfer(env: Env, from: Address, to: Address, amount: u64, coin_id: BytesN<32>)`  
  Transfers PI.  
  - `from/to`: Addresses.  
  - `amount`: Amount.  
  - `coin_id`: Coin ID.

- `get_usd_value(env: Env, amount: u64) -> u64`  
  Converts to USD.  
  - `amount`: PI amount.  
  - Returns: USD value.

### Verification Contract
**Location**: `contracts/verification/src/lib.rs`  
**Description**: Origin verification with AI and quantum hashing.  
**Hyper-Tech**: AI pattern recognition, quantum RSA, anomaly detection.

- `verify_origin(env: Env, source: Symbol, coin_id: BytesN<32>, amount: u64, freq: u32) -> VerificationResult`  
  Verifies origin.  
  - `source`: Origin.  
  - `coin_id`: ID.  
  - `amount/freq`: Amount/frequency.  
  - Returns: VerificationResult.

- `batch_verify(env: Env, verifications: Vec<...>) -> Vec<VerificationResult>`  
  Batch verifies.

### Transaction Contract
**Location**: `contracts/transaction/src/lib.rs`  
**Description**: Transaction processing with consensus and routing.  
**Hyper-Tech**: AI routing, simulated consensus, quantum ledgers.

- `process_transaction(env: Env, sender: Address, receiver: Address, amount: u64, source: Symbol) -> Transaction`  
  Processes transaction.  
  - `sender/receiver`: Addresses.  
  - `amount`: Amount.  
  - `source`: Origin.  
  - Returns: Transaction.

- `get_transaction(env: Env, tx_id: BytesN<32>) -> Transaction`  
  Retrieves transaction.

### Ecosystem Contract
**Location**: `contracts/ecosystem/src/lib.rs`  
**Description**: Ecosystem integrations with oracles and analytics.  
**Hyper-Tech**: AI pricing, oracle simulations, real-time analytics.

- `register_merchant(env: Env, name: Symbol, products: Map<Symbol, u64>) -> Merchant`  
  Registers merchant with AI pricing.  
  - `name`: Merchant name.  
  - `products`: Product map.  
  - Returns: Merchant.

- `standardize_value(env: Env, usd_value: u64) -> u64`  
  Standardizes USD to PI.  
  - `usd_value`: USD amount.  
  - Returns: PI amount.

- `get_analytics(env: Env) -> EcosystemAnalytics`  
  Returns analytics.

## CLI Tools
Located in `/cli/`. Run with `python cli/<file>.py <command>`.

### pi_coin_cli.py
**Description**: Main CLI for Pi Coin interactions.

- `mint --amount <int> --source <str>`  
  Mints PI (calls contract).

- `transfer --from <addr> --to <addr> --amount <int>`  
  Transfers PI.

### pi_math_cli.py
**Description**: Pi math utilities.

- `generate-pi --digits <int>`  
  Generates Pi with AI optimization.

- `pi-based-hash --data <str>`  
  Computes Pi-based hash.

### examples_cli.py
**Description**: Simulation examples.

- `merchant-example --product <str> --base-price <float>`  
  Simulates merchant pricing.

- `p2p-example --amount <float> --source <str>`  
  Simulates P2P trade.

### config_cli.py
**Description**: Config management.

- `set-config --key <str> --value <str>`  
  Sets config value.

- `ai-adjust --anomaly-rate <float>`  
  AI adjusts PI value.

For full code, see source files. Report issues in the repo.
