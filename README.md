# maison-solana-colosseum-26
Maison : Solana Remittance Routing engine SDK

The public repository represents the core protocol, on-chain execution layer, and developer SDK of Project Maison. It is structured as a clean, standalone monorepo built specifically for this submission and contains no legacy or unrelated code.

1. What is Included in this Public Repository
Solana Core Contracts (programs/maison-core): Full Anchor implementation of the RouteProfile state PDAs, multi-destination split routing, Token Extensions (Token-2022) transfer hooks, and DEX routing CPIs.

Tempo & Cross-Chain Settlement Modules: Smart contract adapters and client scripts for Tempo (TIP-20/TIP-403 compliance and WebAuthn/Passkey flows) and Base CCTP settlement.

Maison TypeScript SDK & Actions: The @maison/sdk client package and Solana Blinks/Actions handlers powering the 1-click claim flow.

Integration Tests: Standalone, deterministic test suites (anchor test) verifying split allocations, token swaps, and multi-vault escrow releases against a local validator without requiring external credentials.

2. What is Intentionally Decoupled (Proprietary / Off-Chain)
To protect commercial IP, security credentials, and partner compliance standards, the following services operate behind our private staging API and are not published in this repository:

Off-Chain Mobile Money & Banking Rails: Direct partner webhooks and API connectors to local fiat payout endpoints (GCash, Pix, M-Pesa, ACH).

Proprietary JIT Liquidity Matching Engine: The off-chain algorithmic balancing and FX hedging engine used by private liquidity providers (LPs).

Production Cloud Infrastructure & Relayers: Production keys, relayer services, and Kubernetes deployment scripts.

The public repository provides a complete, runnable, and verifiable proof of our on-chain protocol architecture. All on-chain logic can be audited and tested locally with a single command (anchor test).