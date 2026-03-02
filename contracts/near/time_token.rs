use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

/// 1 TIME = 10^24 yoctoTIME (NEAR standard)
const ONE_TIME: Balance = 1_000_000_000_000_000_000_000_000;
const MAX_EARNED_PER_DAY: u8 = 23;
const SECONDS_PER_DAY: u64 = 86400;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Human {
    pub verified: bool,
    pub birth_timestamp: u64,
    pub birthright_claimed: Balance,
    pub last_earned_day: u64,
    pub earned_today: Balance,
    pub total_earned: Balance,
    pub staked: Balance,
    pub stake_timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TIMEToken {
    /// Token balances
    balances: LookupMap<AccountId, Balance>,
    /// Human records (verified via World ID)
    humans: LookupMap<AccountId, Human>,
    /// Total supply
    total_supply: Balance,
    /// Contract owner
    owner: AccountId,
    /// Authorized verifiers
    verifiers: LookupMap<AccountId, bool>,
    /// Stats
    total_humans: u64,
    total_birthright_claimed: Balance,
    total_work_minted: Balance,
}

impl Default for TIMEToken {
    fn default() -> Self {
        Self {
            balances: LookupMap::new(b"b"),
            humans: LookupMap::new(b"h"),
            total_supply: 0,
            owner: env::predecessor_account_id(),
            verifiers: LookupMap::new(b"v"),
            total_humans: 0,
            total_birthright_claimed: 0,
            total_work_minted: 0,
        }
    }
}

#[near_bindgen]
impl TIMEToken {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let mut contract = Self::default();
        contract.owner = owner.clone();
        contract.verifiers.insert(&owner, &true);
        contract
    }

    // ========== Identity ==========

    /// Register a verified human (called by authorized verifier)
    pub fn verify_human(&mut self, account: AccountId, birth_timestamp: u64) {
        self.assert_verifier();
        assert!(
            self.humans.get(&account).is_none(),
            "Already verified"
        );
        assert!(
            birth_timestamp < env::block_timestamp() / 1_000_000_000,
            "Birth must be in past"
        );

        let human = Human {
            verified: true,
            birth_timestamp,
            birthright_claimed: 0,
            last_earned_day: 0,
            earned_today: 0,
            total_earned: 0,
            staked: 0,
            stake_timestamp: 0,
        };

        self.humans.insert(&account, &human);
        self.total_humans += 1;

        env::log_str(&format!(
            "Human verified: {} (born: {})",
            account, birth_timestamp
        ));
    }

    // ========== Birthright TIME (UBI) ==========

    /// Claim birthright TIME: 1 TIME per day alive
    pub fn claim_birthright(&mut self) {
        let account = env::predecessor_account_id();
        let mut human = self.get_human(&account);

        let now_seconds = env::block_timestamp() / 1_000_000_000;
        let days_alive = (now_seconds - human.birth_timestamp) / SECONDS_PER_DAY;
        let total_entitled = days_alive as u128 * ONE_TIME;
        let to_claim = total_entitled - human.birthright_claimed;

        assert!(to_claim > 0, "Nothing to claim");

        human.birthright_claimed = total_entitled;
        self.humans.insert(&account, &human);

        self.mint(&account, to_claim);
        self.total_birthright_claimed += to_claim;

        env::log_str(&format!(
            "Birthright claimed: {} TIME by {}",
            to_claim / ONE_TIME,
            account
        ));
    }

    // ========== Earned TIME ==========

    /// Mint TIME for verified work hours
    pub fn mint_work(&mut self, worker: AccountId, hours: u8) {
        self.assert_verifier();
        assert!(hours > 0 && hours <= MAX_EARNED_PER_DAY, "Hours must be 1-23");

        let mut human = self.get_human(&worker);
        let today = (env::block_timestamp() / 1_000_000_000) / SECONDS_PER_DAY;

        if human.last_earned_day != today {
            human.last_earned_day = today;
            human.earned_today = 0;
        }

        let amount = hours as u128 * ONE_TIME;
        assert!(
            human.earned_today + amount <= MAX_EARNED_PER_DAY as u128 * ONE_TIME,
            "Daily earned limit exceeded"
        );

        human.earned_today += amount;
        human.total_earned += amount;
        self.humans.insert(&worker, &human);

        self.mint(&worker, amount);
        self.total_work_minted += amount;

        env::log_str(&format!(
            "Work minted: {} hours ({} TIME) for {}",
            hours, hours, worker
        ));
    }

    // ========== Transfers ==========

    /// Transfer TIME to another account
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128) {
        let sender = env::predecessor_account_id();
        let amount: Balance = amount.into();

        let sender_balance = self.balances.get(&sender).unwrap_or(0);
        assert!(sender_balance >= amount, "Insufficient balance");

        self.balances.insert(&sender, &(sender_balance - amount));
        let receiver_balance = self.balances.get(&receiver_id).unwrap_or(0);
        self.balances.insert(&receiver_id, &(receiver_balance + amount));
    }

    /// Check balance
    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.balances.get(&account_id).unwrap_or(0))
    }

    /// Total supply
    pub fn ft_total_supply(&self) -> U128 {
        U128(self.total_supply)
    }

    // ========== Staking & Governance ==========

    /// Stake TIME for governance voting power
    pub fn stake(&mut self, amount: U128) {
        let account = env::predecessor_account_id();
        let amount: Balance = amount.into();
        let mut human = self.get_human(&account);

        let balance = self.balances.get(&account).unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance");

        self.balances.insert(&account, &(balance - amount));

        if human.staked == 0 {
            human.stake_timestamp = env::block_timestamp() / 1_000_000_000;
        }
        human.staked += amount;
        self.humans.insert(&account, &human);
    }

    /// Unstake TIME
    pub fn unstake(&mut self, amount: U128) {
        let account = env::predecessor_account_id();
        let amount: Balance = amount.into();
        let mut human = self.get_human(&account);

        assert!(human.staked >= amount, "Insufficient stake");

        human.staked -= amount;
        if human.staked == 0 {
            human.stake_timestamp = 0;
        }
        self.humans.insert(&account, &human);

        let balance = self.balances.get(&account).unwrap_or(0);
        self.balances.insert(&account, &(balance + amount));
    }

    /// Calculate voting power (stake × duration multiplier)
    pub fn voting_power(&self, account_id: AccountId) -> U128 {
        let human = match self.humans.get(&account_id) {
            Some(h) => h,
            None => return U128(0),
        };

        if human.staked == 0 {
            return U128(0);
        }

        let now = env::block_timestamp() / 1_000_000_000;
        let duration = now - human.stake_timestamp;

        let multiplier: u128 = if duration >= 4 * 365 * SECONDS_PER_DAY {
            400 // 4x
        } else if duration >= 365 * SECONDS_PER_DAY {
            200 // 2x
        } else if duration >= 180 * SECONDS_PER_DAY {
            150 // 1.5x
        } else {
            100 // 1x
        };

        U128((human.staked * multiplier) / 100)
    }

    // ========== View Methods ==========

    pub fn get_stats(&self) -> (u64, U128, U128) {
        (
            self.total_humans,
            U128(self.total_birthright_claimed),
            U128(self.total_work_minted),
        )
    }

    pub fn is_verified(&self, account_id: AccountId) -> bool {
        self.humans
            .get(&account_id)
            .map_or(false, |h| h.verified)
    }

    // ========== Internal ==========

    fn mint(&mut self, account: &AccountId, amount: Balance) {
        let balance = self.balances.get(account).unwrap_or(0);
        self.balances.insert(account, &(balance + amount));
        self.total_supply += amount;
    }

    fn get_human(&self, account: &AccountId) -> Human {
        self.humans
            .get(account)
            .expect("Not a verified human")
    }

    fn assert_verifier(&self) {
        let caller = env::predecessor_account_id();
        assert!(
            self.verifiers.get(&caller).unwrap_or(false) || caller == self.owner,
            "Not authorized verifier"
        );
    }

    /// Add a verifier (owner only)
    pub fn add_verifier(&mut self, account: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Owner only"
        );
        self.verifiers.insert(&account, &true);
    }
}
