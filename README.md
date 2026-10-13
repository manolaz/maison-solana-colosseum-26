<div align="center">

<img src="./maison-logo.png" alt="Maison Logo" width="160" />

# Project Maison
### *Next-Generation Global Remittance Routing Engine & Multi-Vault Settlement Protocol on Solana*

[![Solana](https://img.shields.io/badge/Solana-1.18%2B-black?style=for-the-badge&logo=solana&logoColor=%2314F195)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-blue?style=for-the-badge&logo=anchor&logoColor=white)](https://anchor-lang.com)
[![Token-2022](https://img.shields.io/badge/Token--2022-Extensions-9945FF?style=for-the-badge)](https://spl.solana.com/token-2022)
[![Colosseum Hackathon](https://img.shields.io/badge/Colosseum-2026%20Submission-FF6B4A?style=for-the-badge)](https://colosseum.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

**Sub-second, ultra-low-cost cross-border remittances with atomic multi-destination split routing, Token-2022 extension hooks, and 1-click Solana Blinks.**

[Key Innovations](#-key-innovations) • [Architecture](#-protocol-architecture) • [Smart Contracts](#-on-chain-contracts-programsmaison-core) • [Token Extensions](#-token-2022--standards) • [Testing & Verification](#-deterministic-verification) • [Colosseum Scope](#-colosseum-hackathon-scope)

</div>

---

## 📌 Executive Summary & The Problem

Global remittances exceed **$860 Billion annually**, yet remitters and migrant workers are burdened by:
- **Exorbitant Fees:** Traditional operators (Western Union, MoneyGram, legacy wire rails) siphon an average of **6.2%** of remittance volume.
- **T+3 Settlement Latency:** Delays and weekend market closures strand capital when families require immediate relief.
- **Rigid All-or-Nothing Transfers:** Remitters must send a lump sum to a single account, forcing recipient households to manually distribute cash between urgent living costs, savings, healthcare, and education.

### The Maison Solution
**Maison** is an on-chain remittance protocol built natively on Solana. It reimagines remittances as **programmable capital flows**:
1. **Atomic Multi-Destination Split Routing:** 1 single remittance is automatically divided according to customizable basis-point rules into multiple beneficiary sub-vaults (e.g., 70% immediate mobile payout, 20% locked savings escrow, 10% school tuition fund).
2. **Dual-Token & Token-2022 Compatibility:** Full native support for standard SPL tokens and Token Extensions (Transfer Fees, Transfer Hooks, and Metadata).
3. **Flexible Multi-Modal Claim Escrows:** Beneficiaries can claim via direct wallet signatures, secret preimage voucher codes (SMS/WhatsApp OTPs), WebAuthn/Passkey biometric signatures (Tempo TIP-403), or verified relayer attestations.
4. **Cross-Chain Ingress (Base CCTP & Tempo):** Seamless liquidity bridging from EVM ecosystems and Tempo payment rails directly into Solana execution vaults.
5. **Solana Actions & Blinks:** Zero-friction, 1-click claim links shared over chat and social platforms.

---

## 🚀 Key Innovations

| Feature | Legacy Remittance Rails | Standard Crypto Wallets | **Project Maison** |
| :--- | :--- | :--- | :--- |
| **Average Transfer Fee** | 6.2% ($62 on $1,000) | 0.5% – 2.0% + Gas | **< 0.25% + < $0.001 Solana TX fee** |
| **Settlement Speed** | 24 – 72 hours (T+3) | 2 – 15 minutes | **Sub-second finality (~400ms)** |
| **Payout Mechanics** | Single recipient cashout | Manual multi-step sends | **Atomic multi-vault split routing (up to 10 destinations)** |
| **Claim Experience** | Physical counter queue & ID | Wallet connection required | **Solana Blinks, SMS/WhatsApp OTP preimage, or Passkey** |
| **Asset Standard** | Fiat closed-loop | Basic SPL Token | **Token-2022 (Transfer Hooks, Fee Config, Memo)** |
| **Interoperability** | Siloed SWIFT/correspondent | Single-chain only | **Cross-chain CCTP (Base) & Tempo TIP-403 adapter** |

---

## 🏗️ Protocol Architecture

```mermaid
flowchart TD
    subgraph Remitters[" Remittance Originators "]
        A1[Solana Wallet / Blink]
        A2[Base CCTP Bridge]
        A3[Tempo TIP-403 Flow]
    end

    subgraph MaisonCore[" Maison Core Protocol (programs/maison-core) "]
        B[Protocol Config PDA\n(Fee Treasury & 2-Step Governance)]
        C[RouteProfile PDA\n(Split Allocations & Daily Spending Cap)]
        D[RemittanceEscrow PDA\n(Multi-Modal Claim Vault)]
        E[SettlementRecord PDA\n(Cross-Chain Nonce Verification)]
    end

    subgraph Execution[" Atomic Distribution & Vaults "]
        F1["Recipient A: Immediate Living Expenses (70%)"]
        F2["Recipient B: Timelocked Family Savings Vault (20%)"]
        F3["Recipient C: Emergency / Education Escrow (10%)"]
        T["Protocol Treasury (25 bps)"]
    end

    A1 -->|Initiate / Route| C
    A1 -->|Escrow Deposit| D
    A2 -->|Relayed Burn/Mint| E
    A3 -->|Passkey Attestation| E

    C -->|Atomic Split| Execution
    D -->|Claim / Preimage / Passkey| Execution
    E -->|Finalize Settlement| Execution
    C -.->|Deduct Fee| T
    D -.->|Deduct Fee| T
```

---

## 📂 Repository Structure

```
maison-solana-colosseum-26/
├── Anchor.toml                      # Anchor workspace configuration
├── Cargo.toml                       # Monorepo Rust workspace configuration
├── package.json                     # TypeScript dependencies and testing scripts
├── tsconfig.json                    # TypeScript compiler configuration
├── maison-logo.png                  # Project Maison official branding asset
├── programs/
│   └── maison-core/                 # Solana Anchor Smart Contract
│       ├── Cargo.toml               # Anchor & SPL Token-2022 dependencies
│       └── src/
│           ├── lib.rs               # Entrypoint & program instruction handlers
│           ├── constants.rs         # Protocol seeds, bounds, and caps
│           ├── errors.rs            # Descriptive error taxonomy
│           ├── events.rs            # Rich event logs for indexers & Blinks
│           ├── state/
│           │   ├── mod.rs
│           │   ├── protocol_config.rs   # Global governance, treasury & circuit breaker
│           │   ├── route_profile.rs     # Multi-split rules, 24h limits & KYC tiers
│           │   ├── remittance_escrow.rs # Timelocked escrow state & claim methods
│           │   └── settlement_record.rs # Base CCTP & Tempo cross-chain attestations
│           ├── instructions/
│           │   ├── mod.rs
│           │   ├── initialize_protocol.rs    # Protocol bootstrap
│           │   ├── update_protocol_config.rs # 2-step admin transfer & parameter tuning
│           │   ├── create_route_profile.rs   # Profile & split rules definition
│           │   ├── update_route_profile.rs   # Split adjustment & limit reset
│           │   ├── initiate_remittance.rs    # Escrow funding with fee deduction
│           │   ├── execute_split_routing.rs  # Atomic multi-account disbursements
│           │   ├── claim_escrow.rs           # Direct / OTP Hash / WebAuthn claim
│           │   ├── refund_escrow.rs          # Timelock expiry sender clawback
│           │   ├── settle_cross_chain.rs     # CCTP / Tempo inbound settlement
│           │   └── swap_and_route.rs         # DEX CPI swap + split distribution
│           └── utils/
│               ├── mod.rs
│               ├── token.rs         # Token-2022 transfer_checked & fee calculations
│               └── crypto.rs        # SHA-256 / Keccak256 & WebAuthn proof verification
└── tests/
    └── maison-core.ts               # Complete integration test suite
```

---

## ⚡ On-Chain Contracts (`programs/maison-core`)

The core program is authored using **Anchor 0.30.1** with strict adherence to high-assurance security practices:

### State Accounts (PDAs)

| Account | PDA Seeds | Description |
| :--- | :--- | :--- |
| **`ProtocolConfig`** | `["protocol_config"]` | Global protocol parameters, fee rate (`protocol_fee_bps`), treasury vault, and two-step admin governance. |
| **`RouteProfile`** | `["route_profile", owner, profile_id]` | User-configured routing policy containing up to 10 split destinations, basis points (`TOTAL_BPS = 10,000`), rolling 24-hour spending caps, and compliance tiers. |
| **`RemittanceEscrow`** | `["escrow", sender, external_reference]` | Holds locked funds, fee records, claim commitment, expiry timestamp, and authorization mode. |
| **`SettlementRecord`** | `["settlement", source_domain, remote_nonce]` | Cross-chain replay-prevention record tracking Base CCTP / Tempo message attestations. |

### Core Instructions

- **`initialize_protocol`**: Bootstraps the protocol and caps the protocol fee below 250 bps (0.25% default).
- **`create_route_profile`**: Registers split allocations. Validates that $\sum \text{bps} = 10,000$, rejects duplicate recipients, and configures daily limits.
- **`execute_split_routing`**: Atomically disburses funds across all configured recipient accounts with remainder/dust allocation to ensure exact precision.
- **`initiate_remittance`**: Locks tokens in an escrow vault with customizable claim conditions and protocol fee deduction.
- **`claim_escrow`**: Disburses escrowed funds to beneficiaries upon validating:
  - *Direct Signature*: Authorized beneficiary wallet.
  - *Preimage Hash*: Correct SHA-256 / Keccak-256 voucher preimage secret (ideal for SMS/WhatsApp distribution).
  - *WebAuthn Passkey*: Valid biometrically authenticated payload matching Tempo TIP-403 requirements.
- **`refund_escrow`**: Protects remitter capital by enabling complete recovery if the timelock expires without a claim.
- **`settle_cross_chain`**: Ingests cross-chain bridge messages from Base (CCTP Domain 6) and Tempo financial networks.
- **`swap_and_route`**: Interacts with DEX routers via CPI to swap input tokens (e.g., USDC) to target settlement currencies with slippage validation before triggering split execution.

---

## 🪙 Token-2022 & Standards

Project Maison leverages the latest Solana Token Extensions standard:
1. **`TokenInterface` Abstraction:** Fully compatible with both standard SPL Token and SPL Token-2022.
2. **Transfer Fee Compatibility:** Pre-computes and accounts for transfer fee deductions to ensure downstream recipients receive exact allocations.
3. **Transfer Hook Support:** Safe execution across hook-enabled mints with complete account resolution.
4. **Memo & Compliance:** Compatible with transfer memo extensions and Tempo TIP-403 regulatory enforcement checks.

---

## 🧪 Deterministic Verification

All protocol instructions are backed by comprehensive, standalone integration tests that execute against a local validator without needing external secrets or API keys.

### Running the Test Suite:

```bash
# 1. Install dependencies
yarn install

# 2. Build the Anchor program
anchor build

# 3. Run the deterministic integration test suite
anchor test
```

### Verified Test Scenarios (`tests/maison-core.ts`):
- ✅ Protocol initialization with fee treasury and 2-step admin transfer
- ✅ `RouteProfile` creation with multi-destination basis points validation
- ✅ Atomic split routing execution across multiple beneficiary token accounts
- ✅ Remittance escrow initialization with SHA-256 voucher preimage hash locks
- ✅ Beneficiary escrow claim via valid secret voucher proof
- ✅ Cross-chain remittance settlement from Base CCTP message nonces

---

## 🏆 Colosseum Hackathon Scope

### Included in this Repository
- **Core Smart Contract:** Full, audited-ready Anchor implementation in [`programs/maison-core`](./programs/maison-core).
- **Cross-Chain Adapters:** Base CCTP & Tempo TIP-403 attestation schemas.
- **Integration Test Harness:** End-to-end deterministic test coverage in [`tests/maison-core.ts`](./tests/maison-core.ts).

### Intentionally Decoupled (Proprietary / Off-Chain Production Rails)
To safeguard commercial intellectual property, partner compliance agreements, and banking security credentials, the following production systems operate via private staging APIs and are omitted from this repository:
- **Banking & Mobile Money Webhooks:** Direct integrations with fiat payout partners (GCash, Pix, M-Pesa, ACH).
- **JIT Liquidity Engine:** Algorithmic FX hedging and liquidity balancing engine used by private market makers.
- **Production Relayer Infrastructure:** Live relayer keys, HSM signers, and Kubernetes clusters.

---

## 🗺️ Roadmap & Vision

- **Q4 2026:** Colosseum Hackathon Submission & Mainnet Beta Deployment.
- **Q1 2027:** Rollout of SEA & LATAM remittance corridors (PHP, BRL, MXN) in partnership with regional mobile money providers.
- **Q2 2027:** Launch of `@maison/sdk` mobile SDK and zero-install Telegram/WhatsApp Mini-Apps powered by Solana Blinks.

---

<div align="center">

Built with ❤️ for the **Solana Colosseum Hackathon 2026**

</div>