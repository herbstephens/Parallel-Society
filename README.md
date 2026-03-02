# Parallel Society

**An operating system for human coordination — built on TIME Protocol.**

---

> *"33 people govern you. 12,847 laws apply to you. You don't know who they are."*

Parallel Society is infrastructure for a world where every human has:

- **A Governance Agent** — AI that shows you who governs your life and helps you make informed decisions
- **TIME tokens** — a parallel economy where 1 TIME = 1 verified hour of human work, with birthright UBI built into the protocol
- **Local allocation** — support what you know, not what Wall Street tells you to invest in
- **Collective governance** — stake TIME, gain voice, shape your community

This is not a protest. It's not a manifesto. It's working infrastructure.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  PARALLEL SOCIETY                     │
│                                                       │
│   ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│   │ IDENTITY │  │ AWARENESS│  │     ECONOMY       │  │
│   │          │  │          │  │                    │  │
│   │ World ID │  │Governance│  │  TIME Protocol     │  │
│   │(Worldchain)│ │  Agent   │  │  (NEAR+Ethereum)  │  │
│   │          │  │  (NEAR)  │  │                    │  │
│   └────┬─────┘  └────┬─────┘  └────────┬─────────┘  │
│        │              │                  │            │
│        └──────────────┼──────────────────┘            │
│                       │                               │
│              ┌────────┴────────┐                      │
│              │   COORDINATION  │                      │
│              │                 │                      │
│              │  Waku P2P Mesh  │                      │
│              │  (Logos Network) │                      │
│              └─────────────────┘                      │
└─────────────────────────────────────────────────────┘
```

### Three Chains. Five Layers. One Operating System.

| Layer | Component | Chain | Purpose |
|-------|-----------|-------|---------|
| **Identity** | World ID | Worldchain | One human = one agent. Sybil-resistant. |
| **Awareness** | Governance Agent | NEAR | AI that maps who governs you |
| **Economy** | TIME Token | NEAR + Ethereum | 1 birthright/day + up to 23 earned = 24 TIME max/day |
| **Governance** | Stake & Vote | NEAR | Stake TIME → proportional voice |
| **Coordination** | Waku Messaging | Logos | Privacy-preserving P2P communication |

---

## TIME Protocol

TIME is not a speculative token. It is a unit of account anchored to human labor.

**Birthright TIME**: Every verified human receives 1 TIME per day they have been alive. This is UBI — not as a government program, but as a protocol-level right.

**Earned TIME**: Up to 23 additional TIME per day, minted upon verified completion of work. Work is recorded as soulbound NFTs — portable, permanent, yours.

**Maximum**: 24 TIME per day per human. No exceptions. No whales. No printing.

```
WORLD ID (verify humanity)
        ↓
BIRTHRIGHT TIME (1 × days lived)
        ↓
    ┌───┴───┬───────┐
    ↓       ↓       ↓
  WORK    TRADE   STAKE
    ↓       ↓       ↓
+TIME    ↔ BTC   → VOTING
         ↔ ETH     POWER
         ↔ USD
    └───────┴───────┘
            ↓
    PARALLEL ECONOMY
```

---

## Governance Agent

Your personal AI political and economic advisor.

The Governance Agent discovers:
- **Who governs you** — elected officials, appointed regulators, institutional decision-makers
- **What laws apply to you** — local, national, international
- **What's changing** — upcoming elections, new legislation, policy shifts
- **What opportunities exist locally** — businesses, cooperatives, initiatives near you

It runs continuously. It monitors changes. It alerts you before decisions affect your life — not after.

Built on NEAR for on-chain data persistence and AI compute. Integrated with World ID so one human gets one agent.

---

## Waku Integration (Logos Network)

Parallel Society uses [Waku](https://waku.org) from the [Logos](https://logos.co) collective as its communication layer:

- **Privacy-preserving messaging** between governance agents
- **Censorship-resistant coordination** for local communities
- **Decentralized relay** — no central servers, no surveillance
- **Content topics** organized by governance jurisdiction

```
governance/{country}/{region}/{municipality}
economy/time/marketplace/{region}
coordination/local/{community-id}
```

See [docs/WAKU-INTEGRATION.md](docs/WAKU-INTEGRATION.md) for technical details.

---

## Repository Structure

```
parallel-society/
├── README.md                    # You are here
├── contracts/
│   ├── ethereum/
│   │   └── TIMEToken.sol        # ERC-20 TIME token
│   └── near/
│       └── time_token/          # NEAR TIME token (Rust)
├── agent/
│   └── governance-agent.ts      # Governance discovery agent
├── frontend/
│   └── index.html               # Demo interface
├── docs/
│   ├── ARCHITECTURE.md          # Technical architecture
│   ├── WAKU-INTEGRATION.md      # Logos/Waku integration spec
│   ├── TIME-SPECIFICATION.md    # TIME Protocol specification
│   ├── LOCAL-ALLOCATION.md      # "Investment is a dirty word"
│   └── PARALLEL-SOCIETY.md      # The vision document
├── scripts/
│   └── deploy.sh                # Deployment scripts
└── .github/
    └── workflows/
        └── ci.yml               # CI pipeline
```

---

## The Great Reset: January 1, 2034

This is not a prediction. It's a Schelling point — a date the world can coordinate around.

| Phase | Years | Goal | Target |
|-------|-------|------|--------|
| **Foundation** | 2025–2026 | Ship core infrastructure | 100K verified humans |
| **Growth** | 2027–2029 | Scale the parallel economy | 10M humans |
| **Integration** | 2030–2032 | Become undeniable | 500M humans |
| **The Great Reset** | 2033–2034 | Global activation | 1B+ humans |

By January 1, 2034, any human on Earth can:
1. Get an agent — *"33 people govern you, 12,847 laws apply"*
2. Claim birthright — 1 TIME × days lived
3. See local opportunities — businesses, cooperatives, initiatives
4. Allocate locally — support what you know
5. Trade freely — TIME ↔ any digital asset
6. Govern collectively — stake TIME, vote on decisions

No permission needed. No government required. No bank involved.

---

## Related Projects

| Repository | Description |
|-----------|-------------|
| [DemocracyEarth/proof](https://github.com/DemocracyEarth/proof) | Proof of Humanity protocol |
| [DemocracyEarth/ubi](https://github.com/DemocracyEarth/ubi) | Universal Basic Income token |
| [DemocracyEarth/paper](https://github.com/DemocracyEarth/paper) | The Social Smart Contract |

---

## Books

This infrastructure is grounded in three books by Herb Stephens:

1. **The Lie Factory** — How institutions manufacture consent ([Amazon](https://www.amazon.com/dp/B0DRHZW4VZ))
2. **The Lie of Investing** — How "investment" became extraction, and why local allocation is the answer
3. **The Great Reset** — The complete blueprint for January 1, 2034

---

## Contributing

This is open infrastructure for humanity. Fork it. Build on it. Translate it.

```bash
git clone https://github.com/herbstephens/parallel-society.git
cd parallel-society
# See docs/ for architecture and integration guides
```

---

## License

MIT — because infrastructure for human coordination should be free.

---

**Democracy Earth Foundation**
**The Great Reset: January 1, 2034**
**[democracy.earth](https://democracy.earth)**

*"You can't build a parallel society by declaring one. You build it by shipping infrastructure."*
