# 📜 SOLFUTURENFT Smart Contracts

This repository contains **Anchor-based smart contracts** powering **SOLFUTURENFT** — a decentralized protocol for trading **NFT futures and options** on the Solana blockchain.

---

## ⚙️ Overview

These smart contracts enable:

- Creation of **futures contracts** based on NFT prices
- Creation and exercise of **call/put options** tied to NFT floor prices
- Secure deposit, locking, and release of SOL
- (Upcoming) **Oracle integration** for floor price validation

All interactions are **non-custodial**, **trustless**, and enforced by programmatic rules directly on-chain.

---

## 🔁 Contract Types

### 🔹 NFT Futures Contract

- **Initiator** deposits 1 NFT and defines:
  - Expiry date
  - Target price (in SOL or USDC)
- **Counterparty** fulfills the contract by depositing the agreed amount
- Upon expiry:
  - If the NFT floor price ≥ target price → NFT is transferred to buyer
  - Else → NFT is returned to seller and funds refunded

### 🔹 NFT Options Contract *(WIP)*

- Supports **Call** and **Put** options based on floor price
- **Writer** locks NFT or funds
- **Buyer** pays a premium
- Option can be **exercised before expiry**
- Uses an **on-chain oracle** (e.g., Tensor) to validate price at settlement

---

## 🛠 Development Guide

### 1. Prerequisites

Ensure the following tools are installed:

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs)
- [Rust](https://www.rust-lang.org/tools/install)

### 2. Build and Deploy

```bash
git clone https://github.com/solfuturenft/sfn-fo-exp-date.git
cd sfn-fo-exp-date/programs/solfuturenft
anchor build
anchor deploy
```

> **Devnet Program ID:** `ERXrCZBaHPRgzTwHE71TJUh6zGCqPqQyRHL8xJgK59z4`

---

## 🧪 Testing the Contracts

- Contracts can be tested on **Solana Devnet**
- Use the **Anchor IDL** file: `.nft_futures.json`
- Compatible front-end: [sfn-frontend](https://github.com/solfuturenft/sfn-frontend)
- Smart contracts use SPL Token + Anchor framework
- Price feeds are sourced from **Tensor floor price API** (coming)

---

## 💬 Community & Support

- For support, ideas, or partnerships, join our Discord: [https://discord.gg/vbywpQXQ](https://discord.gg/vbywpQXQ)
- Please open an issue for bugs or feature requests

---

## 📌 Notes

- The protocol allows **two parties** to enter trustless agreements on NFT price speculation.
- Contracts support **mutable terms**, allowing for margin deposits and specific **collection IDs**.
- Oracle-based validation ensures fair and decentralized settlement.
