# TIME Protocol Specification

**1 TIME = 1 verified hour of human work**

---

## Core Rules

1. Every verified human receives **1 birthright TIME per day** they have been alive
2. Humans can earn up to **23 additional TIME per day** through verified work
3. Maximum issuance: **24 TIME per day per human**. No exceptions.
4. TIME is **non-speculative by design** — it cannot be minted by machines, algorithms, or capital
5. Work verification produces **soulbound NFTs** — portable, permanent, owned by the worker

---

## Token Economics

### Birthright TIME (UBI)

```
birthright_claim = days_since_birth × 1 TIME
```

A 30-year-old human claiming for the first time receives:
```
30 × 365 = 10,950 TIME
```

This is not charity. This is a protocol-level recognition that every human's time has value from birth.

### Earned TIME

```
earned_time = verified_hours_worked (max 23 per day)
```

Work verification requires:
- **Employer attestation** — the party receiving the work confirms completion
- **Time bounds** — start and end timestamps
- **Soulbound NFT** — non-transferable record of work, owned by worker

### Daily Cap

```
total_daily_time = birthright (1) + earned (≤23) = max 24 TIME
```

Why 24? Because there are 24 hours in a day. No human can produce more than 24 hours of work in a day. The protocol mirrors physical reality.

---

## Token Properties

| Property | Value |
|----------|-------|
| Standard | ERC-20 (Ethereum) + NEP-141 (NEAR) |
| Decimals | 18 |
| Max supply per human per day | 24 |
| Global max supply | Dynamic (population × days) |
| Transferable | Yes |
| Burnable | Yes |
| Mintable by | Protocol only (birthright + verified work) |

---

## Smart Contract Interface

### Ethereum (ERC-20 Extension)

```solidity
interface ITIMEToken {
    // Standard ERC-20
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);

    // TIME-specific
    function claimBirthright(bytes calldata worldIdProof) external;
    function birthrightBalance(address account) external view returns (uint256);
    function earnedToday(address account) external view returns (uint256);
    function mintWork(
        address worker,
        uint256 hours,
        bytes calldata verificationProof
    ) external;

    // Governance
    function stake(uint256 amount) external;
    function unstake(uint256 amount) external;
    function votingPower(address account) external view returns (uint256);
}
```

### NEAR (NEP-141 Extension)

```rust
pub trait TIMEToken {
    // Standard NEP-141
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&self, account_id: AccountId) -> U128;

    // TIME-specific
    fn claim_birthright(&mut self, world_id_proof: Vec<u8>);
    fn birthright_balance(&self, account_id: AccountId) -> U128;
    fn earned_today(&self, account_id: AccountId) -> U128;
    fn mint_work(&mut self, worker: AccountId, hours: u8, proof: Vec<u8>);

    // Governance
    fn stake(&mut self, amount: U128);
    fn unstake(&mut self, amount: U128);
    fn voting_power(&self, account_id: AccountId) -> U128;
}
```

---

## Work Verification Flow

```
1. Worker and employer agree on task
2. Worker begins → start timestamp recorded
3. Worker completes → end timestamp recorded
4. Employer attests completion on-chain
5. Protocol verifies:
   - Worker is verified human (World ID)
   - Hours ≤ 23 for this day
   - Employer attestation is valid
6. TIME minted to worker
7. Soulbound Work NFT minted to worker
```

### Work NFT Metadata

```json
{
  "worker": "0x...",
  "employer": "0x...",
  "description": "Frontend development for community marketplace",
  "hours": 6,
  "date": "2026-03-01",
  "skills": ["typescript", "react", "web3"],
  "time_minted": 6,
  "verification": "employer_attestation",
  "transferable": false
}
```

---

## Exchange Rates

TIME floats freely against other currencies. However, the protocol establishes a **dignity floor**:

```
1 TIME ≥ minimum_hourly_wage of worker's jurisdiction
```

This floor is enforced by the protocol refusing to execute trades below the dignity threshold. The floor varies by jurisdiction and is updated by governance vote.

### Trading

TIME can be traded on:
- **TIMEdao Marketplace** — Native marketplace for TIME ↔ goods/services
- **DEXs** — Uniswap (Ethereum), Ref Finance (NEAR)
- **P2P** — Direct transfers via Waku messaging

---

## Staking & Governance

```
voting_power = staked_TIME × time_staked_duration_multiplier
```

| Staking Duration | Multiplier |
|-----------------|------------|
| 1 month | 1.0× |
| 6 months | 1.5× |
| 1 year | 2.0× |
| 4 years | 4.0× |

Longer commitment = stronger voice. This prevents drive-by governance attacks.

### Governance Scope

TIME stakers can vote on:
- Dignity floor adjustments
- Protocol upgrades
- Treasury allocation
- Work verification standards
- Bridge parameters

---

## Anti-Gaming Measures

| Attack Vector | Mitigation |
|--------------|-----------|
| Fake work verification | Multi-party attestation, reputation scoring |
| Identity farming | World ID biometric uniqueness |
| TIME hoarding | No mining, no staking rewards in TIME |
| Wash trading | On-chain analytics, community flagging |
| Bot workers | World ID required for all TIME minting |
