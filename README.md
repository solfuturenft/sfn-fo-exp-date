# 📜 SOLFUTURENFT Smart Contracts

This directory contains the **Anchor-based smart contracts** powering **SOLFUTURENFT** — a decentralized protocol enabling **NFT futures and options** trading on the Solana blockchain.

---

## ⚙️ Overview

These smart contracts allow two parties to:

- Create **futures contracts** tied to NFT prices.
- Create and exercise **call/put options** for NFTs.
- Deposit, lock, and release **NFTs, SOL, or USDC** securely.
- Use **floor price oracles** for fair settlement (coming soon).

All interactions are **non-custodial**, trustless, and settled by programmatic rules on-chain.

---

## 🔁 Contract Types

### 🔹 NFT Futures Contract
- **Initiator** deposits 1 NFT and defines:
  - Expiry date
  - Target price (in SOL/USDC)
- **Counterparty** fulfills the contract by depositing the agreed amount.
- At expiry:
  - NFT goes to buyer if price condition is met
  - Otherwise, NFT is returned and funds refunded

### 🔹 NFT Options Contract (WIP)
- Create **call** or **put** options on NFT floor price.
- Option writer locks NFT or funds
- Option buyer pays premium
- Option can be exercised before expiry
- Oracle-based price feed used for validation

---

## 🛠 Getting Started (Development)

### 1. Prerequisites

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)
- [Rust](https://www.rust-lang.org/tools/install)

### 2. Clone & Build

```bash
git clone https://github.com/solfuturenft/sfn-fo-exp-date.git
cd sfn-fo-exp-date/programs/solfuturenft
anchor build
