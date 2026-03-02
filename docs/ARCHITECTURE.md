# Architecture

**Multi-chain infrastructure for Parallel Society**

---

## Design Principles

1. **One human, one agent** — World ID ensures no duplicate identities
2. **UBI by protocol, not by policy** — Birthright TIME cannot be cancelled or defunded
3. **Local first** — Allocate to what you know, not what strangers recommend
4. **Privacy by default** — Your governance data is yours alone
5. **Censorship resistant** — No single point of failure in any layer

---

## Chain Responsibilities

### Worldchain — Identity

Worldchain hosts World ID, the zero-knowledge proof of personhood. In Parallel Society:

- Verifies each participant is a unique human
- Issues one Governance Agent per verified human
- Prevents sybil attacks on birthright TIME claims
- Privacy-preserving: proves humanity without revealing identity

### NEAR — Intelligence & Economy

NEAR handles the AI and economic layers:

- **Governance Agent** — AI model execution, on-chain state persistence
- **TIME Token** — NEAR-native TIME for fast, low-cost transactions
- **Work Verification** — Soulbound NFTs recording verified work
- **Staking & Governance** — Stake TIME for voting power
- **Waku Bridge** — Oracle for relaying off-chain messages on-chain

Why NEAR: account model (human-readable names), low gas fees, built-in AI compute infrastructure, sharding for scale.

### Ethereum — Settlement & Liquidity

Ethereum provides the settlement layer:

- **TIME Token (ERC-20)** — For DeFi integration and cross-chain liquidity
- **Cross-chain bridge** — NEAR TIME ↔ Ethereum TIME
- **Land Commons contracts** — Long-term settlement of commons fees
- **Treasury** — Foundation treasury and grants

Why Ethereum: deepest liquidity, strongest DeFi ecosystem, institutional trust.

---

## System Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                         USER LAYER                            │
│                                                                │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐   │
│  │ Mobile App   │  │ Web App      │  │ CLI / Developer    │   │
│  └──────┬───────┘  └──────┬───────┘  └───────┬───────────┘   │
│         └──────────────────┼─────────────────┘               │
│                            │                                  │
├────────────────────────────┼──────────────────────────────────┤
│                     AGENT LAYER                               │
│                            │                                  │
│  ┌─────────────────────────┴─────────────────────────────┐   │
│  │              Governance Agent (per human)               │   │
│  │                                                         │   │
│  │  ┌─────────────┐ ┌──────────────┐ ┌────────────────┐  │   │
│  │  │ Discovery   │ │ Monitoring   │ │ Allocation     │  │   │
│  │  │             │ │              │ │ Advisor        │  │   │
│  │  │ Maps who    │ │ Watches for  │ │ Surfaces local │  │   │
│  │  │ governs you │ │ changes      │ │ opportunities  │  │   │
│  │  └─────────────┘ └──────────────┘ └────────────────┘  │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│                   COMMUNICATION LAYER                          │
│                                                                │
│  ┌────────────────────────────────────────────────────────┐   │
│  │                  Waku (Logos Network)                    │   │
│  │                                                         │   │
│  │  Governance Alerts │ TIME Offers │ Local Coordination   │   │
│  └────────────────────────────────────────────────────────┘   │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│                    BLOCKCHAIN LAYER                             │
│                                                                │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐   │
│  │  Worldchain   │  │    NEAR      │  │    Ethereum       │   │
│  │               │  │              │  │                   │   │
│  │  World ID     │  │  TIME Token  │  │  TIME Token       │   │
│  │  Proof of     │  │  Work NFTs   │  │  (ERC-20)         │   │
│  │  Personhood   │  │  Gov Agent   │  │  Cross-chain      │   │
│  │               │  │  Staking     │  │  bridge            │   │
│  │               │  │  Waku Bridge │  │  Land Commons     │   │
│  └──────────────┘  └──────────────┘  └───────────────────┘   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## Data Flow: New User

```
1. User downloads app
2. Scans World ID (iris/face) → Worldchain verifies uniqueness
3. Governance Agent spawns on NEAR
   - Detects user location
   - Discovers governance structure (officials, laws, regulations)
   - Presents: "33 people govern you. 12,847 laws apply."
4. Birthright TIME claimed
   - User's age × 1 TIME/day minted
   - Deposited to NEAR wallet
5. Agent subscribes to Waku topics
   - Governance alerts for user's jurisdiction
   - Local TIME marketplace
   - Community coordination channels
6. User begins participating
   - Views governance map
   - Offers/accepts TIME work
   - Allocates to local opportunities
   - Stakes TIME for governance voice
```

---

## Security Model

| Threat | Mitigation |
|--------|-----------|
| Sybil attacks (fake identities) | World ID biometric verification |
| Governance data manipulation | Multi-source verification + community attestation |
| TIME inflation | Hard cap: 24 TIME/day/human, no exceptions |
| Communication surveillance | Waku encrypted relay, no central servers |
| Smart contract exploits | Formal verification, timelocked upgrades |
| Oracle manipulation | Multi-oracle consensus for Waku bridge |

---

## Cross-Chain Communication

```
NEAR TIME ←──bridge──→ Ethereum TIME (ERC-20)
    │                        │
    │                        │
    ▼                        ▼
  NEAR DeFi              Uniswap/DEX
  TIME Staking           DeFi Composability
  Work Verification      Institutional Access
  Governance             Settlement
```

Bridge uses a lock-and-mint mechanism:
- Lock TIME on NEAR → Mint wrapped TIME on Ethereum
- Burn wrapped TIME on Ethereum → Unlock TIME on NEAR
- Bridge secured by multi-sig + time-delayed execution

---

## Scalability

| Component | Current | Target (2034) |
|-----------|---------|---------------|
| Verified humans | 0 | 1B+ |
| TIME transactions/day | 0 | 100M+ |
| Governance agents | 0 | 1B+ |
| Waku messages/day | 0 | 10B+ |
| Jurisdictions mapped | 0 | All sovereign nations |

NEAR's sharding handles economic transaction scale. Waku's relay mesh handles communication scale. Worldchain handles identity scale. Each layer scales independently.
