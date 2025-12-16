# Video Slide Presentation
Video Slides Presentation Link:

# Video Demo
Video Demo Link: https://youtu.be/49kVeaJu-cE


# Final Report
Team members information:

Chen, Yuanhan |            |

Wu, Chia-Chun | 1012134101 | chiachun910711@gmail.com

## Motivation
Blockchain technology has been popular in recent years, especially within decentralized finance (DeFi). Smart contracts in DeFi enable transparent, autonomous, and trustless financial transactions by executing predetermined rules without intermediaries. However, the complexity and immutability of smart contracts mean that vulnerabilities or inefficiencies can lead to substantial financial losses, as evidenced by several high-profile DeFi exploits in recent years. To address these challenges, developers are increasingly turning to safer and more performant programming languages like Rust to enhance the reliability and robustness of blockchain applications.

Rust is known for its memory safety guarantees, strong support for concurrency, and high-performance execution. These features have made Rust a language of choice for building secure and performant blockchain platforms. Major blockchains like Solana use Rust to implement smart contracts, enhancing code correctness and execution efficiency. Despite Rust's rising prominence in blockchain core development, there remains a noticeable gap in end-to-end projects that integrate both on-chain smart contract logic and off-chain client applications entirely in Rust. We found that established Solana DEXs (e.g., Raydium, Orca, Meteora) typically implement client components in TypeScript or Python. This gap presents an opportunity to explore cohesive Rust-based decentralized application stacks.

Motivated by this gap, our team aims to develop an Automated Market Maker (AMM) decentralized exchange on Solana, complemented by a Rust-based command-line wallet. This project excites us as it challenges us to apply Rust across the entire stack, from smart contract development to client interaction, while deepening our understanding of decentralized trading mechanisms and secure blockchain systems.

## Objectives
We aim to design and implement a small-scale, fully functional AMM decentralized exchange (DEX) on the Solana blockchain using Rust. This will be accompanied by a Rust-based command-line wallet interface. The system will enable users to:

* Initialize new token pools.
* Provide (Deposit) Liquidity to the pools.
* Remove (Withdraw) Liquidity to the pools.
* Swap tokens through pools.
* Inspect the pool status
* Get all pools information on our AMM Dex

The final product will demonstrate a comprehensive decentralized financial application built entirely in Rust, showcasing cohesive on-chain smart contract design and off-chain CLI within one ecosystem.


We proposed an automated trading agent in our project proposal; however, we are not able to complete it as a two member group. Nevertheless, we did some research and have learned two trading strategies that can use in the market. These strategies will be delivered in the **Lessons learned and concluding remarks** section.


## Features
## Automated Market Maker – Decentralized Exchange

This project delivers a fully functional Automated Market Maker (AMM) decentralized exchange implemented as an on-chain Solana program using **Rust** and the **Anchor framework**. The AMM follows a **constant-product liquidity model** and supports core decentralized exchange operations.

### Pool Initialization and PoolCounter

* Supports permissionless creation of new liquidity pools for arbitrary SPL token pairs.
* Each pool is associated with a dedicated on-chain Pool account that stores:
    * Token mint addresses.
    * Vault token accounts.
    * LP token mint.
    * Fee parameters.
    * PDA bump values.
* A global PoolCounter account is maintained to track the total number of pools created by the program.

#### PoolCounter Functionality

PoolCounter is a program-owned account that stores a monotonically increasing counter.

Each time a new pool is initialized:

* The counter is incremented.
* The current counter value can be used as a unique pool identifier or index.

This mechanism enables:
* Deterministic enumeration of pools.
* Lightweight indexing for off-chain clients and trading agents.
* Cleaner integration with analytics, monitoring, and future registry-style features.

By separating pool metadata (`Pool`) from global program state (`PoolCounter`), the design remains modular and extensible while avoiding the need for expensive on-chain scans.

---
### Liquidity Provision and Withdrawal

The AMM supports permissionless liquidity provision and withdrawal through a standardized LP (liquidity provider) token mechanism. Liquidity providers contribute pairs of SPL tokens to a pool and receive LP tokens that represent proportional ownership of the pool’s reserves.

**Adding Liquidity**

Users add liquidity by depositing both assets of the pool (Token A and Token B) into the pool’s vault accounts. Deposits are transferred from the user’s associated token accounts (ATAs) into the pool-controlled vaults via CPI calls to the SPL Token program.

Liquidity provisioning follows two distinct cases:

**Initial Liquidity Provision**

When a pool is first created and has no existing LP supply, the initial liquidity provider:

* Defines the initial reserve ratio between Token A and Token B.

* Receives an initial LP token supply minted by the program.

The LP mint authority is held by a program-derived pool authority (PDA), ensuring that LP issuance is strictly controlled on-chain.

**Subsequent Liquidity Provision**

For an existing pool, the program enforces proportional deposits relative to current reserves.

The amount of LP tokens minted is calculated based on the user’s contribution relative to total pool liquidity, ensuring fairness and preserving ownership proportions. Any deviation from the required ratio is handled deterministically by the program logic, preventing reserve imbalance.

All LP tokens are minted directly to the user’s LP associated token account, and the pool itself never holds LP tokens.

#### Removing Liquidity

Users remove liquidity by burning LP tokens from their own LP token account. The number of LP tokens burned determines the user’s fractional share of the pool.

Based on this share, the program computes the exact amounts of Token A and Token B owed to the user. Assets are transferred from the pool’s vaults back to the user’s ATAs using PDA-authorized CPI transfers.

This mechanism guarantees that:

* Users always receive assets proportional to their ownership.
* Pool reserves and LP supply remain internally consistent after each withdrawal.

#### Invariants and Safety Properties

1. The pool’s reserves and LP supply maintain consistent proportional relationships across all liquidity operations.
2. Vault token accounts are owned by a pool authority PDA, preventing unauthorized withdrawals.
3. LP minting and burning can only be performed by the program via PDA-signed instructions.
4. Arithmetic operations are checked to prevent overflow and invalid state transitions.
5. Liquidity operations cannot be executed with invalid accounts, mismatched mints, or unauthorized signers.

This design ensures that liquidity provision and withdrawal are fair, deterministic, and secure, while remaining fully compatible with standard SPL token tooling and off-chain clients.

---
### Token Swap
The AMM enables permissionless token swaps between the two assets in each liquidity pool using a constant-product pricing mechanism. Swaps are executed entirely on-chain and draw liquidity from the pool’s vaults, ensuring deterministic and trustless execution.
#### Swap Mechanics
Token swaps follow the constant-product invariant:`x * y = k`

where:
* `x` and `y` are the reserves of Token A and Token B stored in the pool’s vaults
* `k` remains constant across swaps (excluding fees).

Users may swap in either direction: Token A → Token B or Token B → Token A.
The output amount is computed based on the current reserve balances and the specified input amount, ensuring that pool liquidity is conserved and the invariant is respected.

#### Fees and Price Impact

Each swap applies a configurable fee, expressed in basis points, which is deducted from the input amount. Swap fees remain in the pool’s reserves, thereby increasing the value of LP shares over time and incentivizing liquidity providers.

The pricing model naturally accounts for price impact: larger trades result in greater reserve imbalance and therefore incur higher slippage. This mechanism discourages excessively large trades relative to the available pool liquidity.

#### Slippage Protection

A slippage protection mechanism is implemented to protect users from adverse price movement. Swap instructions require users to specify a minimum acceptable output amount. If the computed output amount falls below this threshold, the transaction is reverted.

This mechanism provides deterministic slippage bounds and helps protect users from sudden reserve changes as well as front-running and sandwich-style attacks in shared block environments.

#### Execution and Authorization

Swap execution consists of:
* Transferring the input tokens from the user’s associated token account (ATA) to the appropriate pool vault.
* Transferring the output tokens from the opposite vault back to the user.

All vault transfers are authorized using a pool authority PDA, ensuring that only the AMM program can move pool funds and that no external account can drain reserves. All token movements are performed through CPI calls to the canonical SPL Token program.

#### Correctness and Safety Guarantees

Swap instructions validate:

* Correct token mints and vault accounts,
* Pool state consistency, and
* Valid signer authorization.

Arithmetic operations are guarded against overflow and invalid reserve states, and the constant-product invariant is preserved across all successful swap executions.

This swap design provides deterministic pricing, strong safety guarantees, and economic correctness, closely mirroring the behavior of production-grade AMMs while remaining compact and auditable for educational and experimental use.

---

## CLI Client

## User’s (or Developer’s) Guide

## Reproducibility Guide:

## Contrubution

Below is the table showing the contirbution of both teammebers:


|  | Wu, Chia-Chun | Chen, Yuanhan |
| -------- | -------- | -------- |
| AMM Dex     | Implemented Pool Initialization,<br>Added bash scrips for testing and running,<br>Implemented Token Swap,<br> Added pool counter to enable multiple pools creation even the tokens are same    | Text     |
| CLI Client     | Pool Initialization function,<br>Added bash scrips for testing and running,<br>Token Swap function in accordance to the AMM Dex, <br> Pool(s) inspection implementation,<br>Beautify CLI interface and outputs| Text     |
| Trading Agent    | Researching arbitrage tading strategy,<br>Testing fetching pool information on the AMM Dex.|  Text  |
|Documentation<br>and <br>Videos|Final Report<br>Video Demo|Text|

## Lessons learned and concluding remarks:


## References
1. https://github.com/raydium-io/raydium-amm/tree/master
2. https://github.com/alphaengine9/Market-Maker-Framework/tree/main
3. https://www.anchor-lang.com/docs
