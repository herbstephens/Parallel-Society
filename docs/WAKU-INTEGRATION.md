# Waku Integration Specification

**Privacy-preserving communication for Parallel Society via the Logos Network**

---

## Overview

Parallel Society uses [Waku](https://waku.org) as its decentralized communication layer. Waku — part of the [Logos](https://logos.co) technology stack — provides censorship-resistant, privacy-preserving messaging without central servers.

In the Parallel Society architecture, Waku serves as the connective tissue between Governance Agents, enabling them to coordinate, share governance intelligence, and facilitate local allocation — all without surveillance.

---

## Why Waku

| Requirement | Waku Capability |
|------------|----------------|
| No central server | Decentralized relay network |
| Privacy by default | Noise protocol encryption |
| Censorship resistant | No single point of failure |
| Lightweight | Runs in browsers and mobile |
| Content routing | Topic-based pub/sub |
| Offline resilience | Store protocol for message persistence |

Traditional messaging (Telegram, Signal, WhatsApp) relies on centralized infrastructure that can be censored, surveilled, or shut down. For a parallel society to function, its communication layer must be as sovereign as its economy.

---

## Content Topic Architecture

Waku uses content topics to route messages. Parallel Society organizes topics by governance jurisdiction and function:

```
/parallel-society/1/governance/{country_code}/{region}/proto
/parallel-society/1/governance/{country_code}/{region}/{municipality}/proto
/parallel-society/1/economy/marketplace/{region}/proto
/parallel-society/1/economy/time-offers/{region}/proto
/parallel-society/1/coordination/{community_id}/proto
/parallel-society/1/agent/alerts/{world_id_hash}/proto
```

### Examples

```
/parallel-society/1/governance/PT/lisbon/proto
/parallel-society/1/governance/US/michigan/flint/proto
/parallel-society/1/economy/marketplace/lisbon/proto
/parallel-society/1/agent/alerts/0x8a3f.../proto
```

---

## Message Types

### 1. Governance Alert

Broadcast by Governance Agents when political or regulatory changes are detected.

```typescript
interface GovernanceAlert {
  type: "governance_alert";
  jurisdiction: string;       // e.g., "PT/lisbon"
  category: "election" | "legislation" | "regulation" | "appointment";
  severity: "info" | "action_required" | "urgent";
  title: string;
  summary: string;
  source_url: string;
  detected_at: number;        // Unix timestamp
  affects_estimated: number;  // Estimated number of people affected
  agent_id: string;           // World ID hash of originating agent
  signature: string;          // Signed by agent's key
}
```

### 2. TIME Offer

Published when someone offers TIME for exchange or work.

```typescript
interface TIMEOffer {
  type: "time_offer";
  offer_type: "work" | "exchange" | "allocation";
  amount: number;              // TIME tokens
  description: string;
  location: {
    region: string;
    coordinates?: [number, number];  // Optional lat/lng
  };
  duration_hours?: number;
  exchange_rate?: {
    currency: string;          // "ETH", "NEAR", "USD"
    rate: number;
  };
  offerer_id: string;          // World ID hash
  expires_at: number;
  signature: string;
}
```

### 3. Local Coordination

For community-level coordination — mutual aid, cooperative formation, local governance.

```typescript
interface LocalCoordination {
  type: "coordination";
  community_id: string;
  action: "proposal" | "vote" | "announcement" | "request";
  content: string;
  participants_required?: number;
  time_stake?: number;         // TIME staked on this proposal
  author_id: string;
  signature: string;
}
```

---

## Integration Architecture

```
┌─────────────────────┐     ┌──────────────────────┐
│  Governance Agent    │     │  Governance Agent     │
│  (User A - Lisbon)   │     │  (User B - Lisbon)    │
│                      │     │                       │
│  ┌────────────────┐  │     │  ┌────────────────┐   │
│  │ NEAR (state)   │  │     │  │ NEAR (state)   │   │
│  └───────┬────────┘  │     │  └───────┬────────┘   │
│          │           │     │          │            │
│  ┌───────┴────────┐  │     │  ┌───────┴────────┐   │
│  │ Waku (comms)   │◄─┼─────┼─►│ Waku (comms)   │   │
│  └────────────────┘  │     │  └────────────────┘   │
└─────────────────────┘     └──────────────────────┘
          │                           │
          └─────────┬─────────────────┘
                    │
          ┌─────────┴──────────┐
          │   Waku Relay Mesh   │
          │  (Logos Network)    │
          │                     │
          │  No central server  │
          │  No logs            │
          │  No surveillance    │
          └─────────────────────┘
```

---

## Implementation

### JavaScript/TypeScript (Browser + Node.js)

```typescript
import { createLightNode, waitForRemotePeer } from "@waku/sdk";
import { createEncoder, createDecoder } from "@waku/sdk";
import protobuf from "protobufjs";

// Initialize Waku light node
const node = await createLightNode({ defaultBootstrap: true });
await node.start();
await waitForRemotePeer(node);

// Define content topic for Lisbon governance
const TOPIC = "/parallel-society/1/governance/PT/lisbon/proto";
const encoder = createEncoder({ contentTopic: TOPIC });
const decoder = createDecoder(TOPIC);

// Define protobuf schema
const GovernanceAlert = new protobuf.Type("GovernanceAlert")
  .add(new protobuf.Field("type", 1, "string"))
  .add(new protobuf.Field("jurisdiction", 2, "string"))
  .add(new protobuf.Field("category", 3, "string"))
  .add(new protobuf.Field("severity", 4, "string"))
  .add(new protobuf.Field("title", 5, "string"))
  .add(new protobuf.Field("summary", 6, "string"))
  .add(new protobuf.Field("detected_at", 7, "uint64"))
  .add(new protobuf.Field("agent_id", 8, "string"));

// Publish a governance alert
async function publishAlert(alert: any) {
  const payload = GovernanceAlert.encode(
    GovernanceAlert.create(alert)
  ).finish();
  await node.lightPush.send(encoder, { payload });
}

// Subscribe to governance alerts
const subscription = await node.filter.createSubscription();
await subscription.subscribe([decoder], (message) => {
  if (!message.payload) return;
  const alert = GovernanceAlert.decode(message.payload);
  console.log("Governance alert:", alert);
  // Route to Governance Agent for processing
});
```

### Rust (NEAR Contract Integration)

For NEAR-side integration, Waku messages are relayed through an oracle that bridges off-chain Waku messages to on-chain state updates:

```rust
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WakuBridge {
    oracle: AccountId,
    alerts: Vec<GovernanceAlert>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct GovernanceAlert {
    pub jurisdiction: String,
    pub category: String,
    pub title: String,
    pub detected_at: u64,
    pub agent_id: String,
}

#[near_bindgen]
impl WakuBridge {
    /// Oracle submits verified Waku messages on-chain
    pub fn submit_alert(&mut self, alert: GovernanceAlert) {
        assert_eq!(
            env::predecessor_account_id(),
            self.oracle,
            "Only oracle can submit"
        );
        self.alerts.push(alert);
    }

    /// Query alerts by jurisdiction
    pub fn get_alerts(&self, jurisdiction: String) -> Vec<GovernanceAlert> {
        self.alerts
            .iter()
            .filter(|a| a.jurisdiction == jurisdiction)
            .cloned()
            .collect()
    }
}
```

---

## Privacy Model

| Data | Visibility | Storage |
|------|-----------|---------|
| Governance alerts | Public (broadcast) | Waku relay + NEAR |
| TIME offers | Regional (topic-scoped) | Waku relay |
| Agent-to-agent comms | Private (encrypted) | Waku direct |
| Personal governance map | Private (local only) | User device |
| World ID | Zero-knowledge proof | Worldchain |

No entity — not Democracy Earth, not NEAR, not Logos — can see who is talking to whom or what governance data individuals are consuming.

---

## Roadmap

| Phase | Milestone |
|-------|-----------|
| **Q1 2026** | Waku light node integration in Governance Agent |
| **Q2 2026** | Content topic schema finalized, relay mesh deployed |
| **Q3 2026** | TIME marketplace messages over Waku |
| **Q4 2026** | Encrypted agent-to-agent coordination |
| **2027** | Full Logos stack integration (Waku + Codex + Nomos) |

---

## Resources

- [Waku Documentation](https://docs.waku.org)
- [Logos Collective](https://logos.co)
- [Waku SDK (JavaScript)](https://github.com/waku-org/js-waku)
- [NEAR Protocol](https://near.org)
- [World ID](https://worldcoin.org/world-id)
