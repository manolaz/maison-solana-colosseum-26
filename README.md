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

[Key Innovations](#-key-innovations) • [Architecture](#-protocol-architecture) • [Workflows & Diagrams](#-deep-dive-workflow-diagrams) • [Smart Contracts](#-on-chain-contracts-programsmaison-core) • [Token Extensions](#-token-2022--standards) • [Testing & Verification](#-deterministic-verification) • [Colosseum Scope](#-colosseum-hackathon-scope)

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

### End-to-End System Architecture & Data Flow

```mermaid
flowchart TD
    subgraph S1["1. Remittance Ingress Rails"]
        R1["Solana DApp / Wallet"]
        R2["Solana Actions & Blinks"]
        R3["Base CCTP Bridge (Domain 6)"]
        R4["Tempo TIP-403 Network"]
    end

    subgraph S2["2. Maison Core Protocol Layer"]
        PC["ProtocolConfig PDA<br/>(Governance, Treasury, Circuit Breaker)"]
        RP["RouteProfile PDA<br/>(Split Rules, 24h Rolling Limits)"]
        RE["RemittanceEscrow PDA<br/>(Timelocked Vault, Claim Commitments)"]
        SR["SettlementRecord PDA<br/>(Cross-Chain Nonce Verification)"]
        DEX["DEX CPI Router<br/>(Jupiter / Whirlpool / Raydium)"]
    end

    subgraph S3["3. Security & Fee Collection"]
        TF["Protocol Fee Deduction<br/>(25 bps Safety Cap)"]
        TR["Fee Treasury Vault"]
    end

    subgraph S4["4. Settlement & Disbursement Destinations"]
        D1["Beneficiary A: Immediate Living Expenses (70%)"]
        D2["Beneficiary B: Family Savings Vault (20%)"]
        D3["Beneficiary C: Emergency / Education Escrow (10%)"]
        D4["Off-Ramp Relayer<br/>(GCash / Pix / M-Pesa / Bank Rails)"]
    end

    R1 -->|"Direct Route"| RP
    R2 -->|"1-Click Claim"| RE
    R3 -->|"CCTP Attestation"| SR
    R4 -->|"Passkey Signature"| SR

    RP -->|"Check Limit & Deduct"| TF
    RE -->|"Escrow Lock & Deduct"| TF
    TF -->|"Transfer Checked"| TR

    RP -->|"Swap Input Tokens"| DEX
    DEX -->|"Disburse Splits"| D1
    DEX -->|"Disburse Splits"| D2
    DEX -->|"Disburse Splits"| D3

    RE -->|"Direct / Preimage / Passkey Claim"| D1
    SR -->|"Finalize Inbound Settlement"| D4
```

---

## 🔄 Deep-Dive Workflow Diagrams

### 1. Multi-Destination Split Routing Mechanics
When diaspora workers send money home, family capital is needed in distinct buckets. Maison splits transactions atomically in a single Solana transaction with zero precision loss.

```mermaid
flowchart TD
    Start["Sender Initiates Remittance<br/>($1,000 USDC)"] --> Validate["Validate RouteProfile PDA<br/>(Active Status & Daily Limit Check)"]
    Validate --> FeeCalc["Calculate Protocol Fee<br/>(25 bps = $2.50)"]
    FeeCalc --> FeeTransfer["Transfer $2.50 to Treasury<br/>(anchor_spl transfer_checked)"]
    FeeTransfer --> NetAmount["Net Remittance for Distribution<br/>($997.50)"]
    
    NetAmount --> SplitCalc["Compute Basis Points Splits<br/>(Total = 10,000 bps)"]
    
    SplitCalc --> S1["Split 1: Living Expenses (7,000 bps)<br/>$698.25 -> Mom's Wallet"]
    SplitCalc --> S2["Split 2: Family Savings (2,000 bps)<br/>$199.50 -> Locked Savings Vault"]
    SplitCalc --> S3["Split 3: Education Fund (1,000 bps)<br/>$99.75 + Rounding Dust -> Tuition Vault"]
    
    S1 --> Finalize["Atomic Single-Transaction Execution<br/>Zero Dust Loss • Emits SplitRoutingExecuted Event"]
    S2 --> Finalize
    S3 --> Finalize
```

---

### 2. Multi-Modal Escrow Lifecycle & Claim Verification
Escrow deposits are safeguarded under timelocks and can be claimed through multiple authentication paths tailored for unbanked and non-crypto-native beneficiaries.

```mermaid
flowchart TD
    Init["Sender Deposits Tokens<br/>into RemittanceEscrow PDA"] --> Locked["Tokens Held in Escrow Vault<br/>(Token-2022 PDA Authority)"]
    
    Locked --> Branch{"Claim Verification Mode"}
    
    Branch -->|"Direct Signature"| Direct["Beneficiary Signs Transaction<br/>Matches escrow.beneficiary"]
    Branch -->|"Preimage Hash"| HashLock["Claimant Submits Secret Voucher<br/>SHA-256(preimage) == claim_commitment"]
    Branch -->|"WebAuthn / Passkey"| Passkey["Biometric Signature Verification<br/>Tempo TIP-403 clientDataJSON Proof"]
    Branch -->|"Timelock Elapsed"| Expired["Clock >= expiry_timestamp<br/>Unclaimed Status Active"]
    
    Direct --> Release["Escrow Vault Transfers Tokens to Claimant<br/>Status -> Claimed • Emits EscrowClaimed"]
    HashLock --> Release
    Passkey --> Release
    
    Expired --> Refund["refund_escrow Called by Sender<br/>Vault Returns Tokens to Sender • Emits EscrowRefunded"]
```

---

### 3. Cross-Chain Settlement Ingress (Base CCTP & Tempo)
Maison connects liquidity from EVM (Base via Circle CCTP) and Tempo payment networks directly to Solana recipients with on-chain nonce replay protection.

```mermaid
sequenceDiagram
    autonumber
    actor Sender as Remitter (Base / Tempo)
    participant BaseBridge as Base CCTP / Tempo Rail
    participant Relayer as Maison Relayer Engine
    participant SolanaMaison as programs/maison-core
    participant Recipient as Beneficiary / Local Vault

    Sender->>BaseBridge: Burn USDC / Initiate Remittance
    BaseBridge-->>Relayer: Emit MessageTransmitter Nonce & Attestation
    Relayer->>SolanaMaison: settle_cross_chain(source_domain, remote_nonce, amount)
    Note over SolanaMaison: Check Replay Prevention PDA<br/>["settlement", domain, nonce]
    Note over SolanaMaison: Verify TIP-403 Compliance Flag
    SolanaMaison->>Recipient: transfer_checked(amount) to Beneficiary Account
    SolanaMaison-->>Relayer: Emit CrossChainSettled Event
    Relayer->>Recipient: Send WhatsApp / SMS Confirmation Notice
```

---

### 4. 1-Click Solana Blink / Action Claim Flow
Recipients without installed Solana wallets can claim funds directly through chat or social media links via Solana Actions & Blinks.

```mermaid
sequenceDiagram
    autonumber
    actor User as Beneficiary
    participant Client as Twitter / Dialect / Mobile Client
    participant BlinkAPI as Maison Actions API (/api/actions/claim)
    participant CoreProgram as programs/maison-core
    participant Vault as Escrow Vault PDA

    User->>Client: Clicks Maison Remittance Blink URL
    Client->>BlinkAPI: GET /api/actions/claim?ref=REF_ID
    BlinkAPI-->>Client: Returns Action Metadata (Title, Amount, Claim Button)
    User->>Client: Enters Claim OTP Code / Taps Claim
    Client->>BlinkAPI: POST /api/actions/claim with Preimage & User Pubkey
    BlinkAPI-->>Client: Returns Serialized claim_escrow Transaction
    User->>Client: Signs Transaction via Mobile Wallet
    Client->>CoreProgram: Broadcasts claim_escrow(preimage)
    CoreProgram->>Vault: Transfers Net Remittance to User ATA
    CoreProgram-->>User: Instant Confirmation & Sub-Second Payout
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