// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title TIMEToken
 * @notice ERC-20 token representing verified human work hours
 * @dev 1 TIME = 1 hour of verified human work
 *
 * Issuance rules:
 *   - Birthright: 1 TIME per day alive (claimed once via World ID)
 *   - Earned: Up to 23 TIME per day via verified work
 *   - Maximum: 24 TIME per day per human
 *
 * TIME is not speculative. It cannot be minted by capital, machines, or algorithms.
 * Only verified humans produce TIME.
 */
contract TIMEToken is ERC20, ERC20Burnable, AccessControl, ReentrancyGuard {

    bytes32 public constant VERIFIER_ROLE = keccak256("VERIFIER_ROLE");
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");

    uint256 public constant BIRTHRIGHT_PER_DAY = 1 ether;    // 1 TIME (18 decimals)
    uint256 public constant MAX_EARNED_PER_DAY = 23 ether;   // 23 TIME
    uint256 public constant MAX_DAILY_TOTAL = 24 ether;      // 24 TIME

    struct Human {
        bool verified;
        uint256 birthTimestamp;       // Unix timestamp of birth
        uint256 birthrightClaimed;    // Total birthright TIME claimed
        uint256 lastEarnedDay;        // Day number of last earned TIME
        uint256 earnedToday;          // TIME earned on lastEarnedDay
        uint256 totalEarned;          // Lifetime earned TIME
        uint256 staked;               // TIME staked for governance
        uint256 stakeTimestamp;       // When staking began
    }

    mapping(address => Human) public humans;
    mapping(bytes32 => bool) public usedWorldIdProofs;

    uint256 public totalHumans;
    uint256 public totalBirthrightClaimed;
    uint256 public totalWorkMinted;

    // Events
    event HumanVerified(address indexed human, uint256 birthTimestamp);
    event BirthrightClaimed(address indexed human, uint256 amount);
    event WorkVerified(address indexed worker, address indexed employer, uint256 hours);
    event Staked(address indexed human, uint256 amount);
    event Unstaked(address indexed human, uint256 amount);

    constructor() ERC20("TIME", "TIME") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(VERIFIER_ROLE, msg.sender);
    }

    /**
     * @notice Register a verified human via World ID proof
     * @param human Address of the verified human
     * @param birthTimestamp Unix timestamp of birth
     * @param worldIdProof World ID zero-knowledge proof (nullifier hash)
     */
    function verifyHuman(
        address human,
        uint256 birthTimestamp,
        bytes32 worldIdProof
    ) external onlyRole(VERIFIER_ROLE) {
        require(!humans[human].verified, "Already verified");
        require(!usedWorldIdProofs[worldIdProof], "Proof already used");
        require(birthTimestamp < block.timestamp, "Birth must be in past");

        usedWorldIdProofs[worldIdProof] = true;
        humans[human] = Human({
            verified: true,
            birthTimestamp: birthTimestamp,
            birthrightClaimed: 0,
            lastEarnedDay: 0,
            earnedToday: 0,
            totalEarned: 0,
            staked: 0,
            stakeTimestamp: 0
        });

        totalHumans++;
        emit HumanVerified(human, birthTimestamp);
    }

    /**
     * @notice Claim birthright TIME (1 per day alive)
     * @dev Can be called once; mints all accumulated birthright
     */
    function claimBirthright() external nonReentrant {
        Human storage h = humans[msg.sender];
        require(h.verified, "Not verified");

        uint256 daysAlive = (block.timestamp - h.birthTimestamp) / 1 days;
        uint256 totalEntitled = daysAlive * BIRTHRIGHT_PER_DAY;
        uint256 toClaim = totalEntitled - h.birthrightClaimed;

        require(toClaim > 0, "Nothing to claim");

        h.birthrightClaimed = totalEntitled;
        totalBirthrightClaimed += toClaim;
        _mint(msg.sender, toClaim);

        emit BirthrightClaimed(msg.sender, toClaim);
    }

    /**
     * @notice Mint TIME for verified work
     * @param worker Address of the worker
     * @param hours Number of hours worked (max 23 per day)
     */
    function mintWork(
        address worker,
        uint256 hours
    ) external onlyRole(VERIFIER_ROLE) nonReentrant {
        require(hours > 0 && hours <= 23, "Hours must be 1-23");
        Human storage h = humans[worker];
        require(h.verified, "Worker not verified");

        uint256 today = block.timestamp / 1 days;
        if (h.lastEarnedDay != today) {
            h.lastEarnedDay = today;
            h.earnedToday = 0;
        }

        uint256 amount = hours * 1 ether;
        require(h.earnedToday + amount <= MAX_EARNED_PER_DAY, "Daily limit exceeded");

        h.earnedToday += amount;
        h.totalEarned += amount;
        totalWorkMinted += amount;
        _mint(worker, amount);

        emit WorkVerified(worker, msg.sender, hours);
    }

    /**
     * @notice Stake TIME for governance voting power
     * @param amount Amount of TIME to stake
     */
    function stake(uint256 amount) external nonReentrant {
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        Human storage h = humans[msg.sender];
        require(h.verified, "Not verified");

        _transfer(msg.sender, address(this), amount);
        if (h.staked == 0) {
            h.stakeTimestamp = block.timestamp;
        }
        h.staked += amount;

        emit Staked(msg.sender, amount);
    }

    /**
     * @notice Unstake TIME
     * @param amount Amount to unstake
     */
    function unstake(uint256 amount) external nonReentrant {
        Human storage h = humans[msg.sender];
        require(h.staked >= amount, "Insufficient stake");

        h.staked -= amount;
        if (h.staked == 0) {
            h.stakeTimestamp = 0;
        }
        _transfer(address(this), msg.sender, amount);

        emit Unstaked(msg.sender, amount);
    }

    /**
     * @notice Calculate voting power (stake × duration multiplier)
     * @param account Address to check
     * @return Voting power (18 decimals)
     */
    function votingPower(address account) external view returns (uint256) {
        Human storage h = humans[account];
        if (h.staked == 0) return 0;

        uint256 stakeDuration = block.timestamp - h.stakeTimestamp;
        uint256 multiplier;

        if (stakeDuration >= 4 * 365 days) {
            multiplier = 400; // 4x
        } else if (stakeDuration >= 365 days) {
            multiplier = 200; // 2x
        } else if (stakeDuration >= 180 days) {
            multiplier = 150; // 1.5x
        } else {
            multiplier = 100; // 1x
        }

        return (h.staked * multiplier) / 100;
    }

    /**
     * @notice Bridge mint (for cross-chain TIME from NEAR)
     * @param to Recipient address
     * @param amount Amount to mint
     */
    function bridgeMint(address to, uint256 amount) external onlyRole(BRIDGE_ROLE) {
        _mint(to, amount);
    }

    /**
     * @notice Bridge burn (for cross-chain TIME to NEAR)
     * @param amount Amount to burn
     */
    function bridgeBurn(uint256 amount) external {
        _burn(msg.sender, amount);
    }
}
