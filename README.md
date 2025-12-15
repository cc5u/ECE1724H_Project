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
### Automated Market Maker – Decentralized Exchange

This project delivers a fully functional Automated Market Maker (AMM) decentralized exchange implemented as an on-chain Solana program using **Rust** and the **Anchor framework**. The AMM follows a **constant-product liquidity model** and supports core decentralized exchange operations.

#### Pool Initialization and PoolCounter

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



## User’s (or Developer’s) Guide

## Reproducibility Guide:

## Contrubution

## Lessons learned and concluding remarks:


## References
1. https://github.com/raydium-io/raydium-amm/tree/master
2. https://github.com/alphaengine9/Market-Maker-Framework/tree/main
3. https://www.anchor-lang.com/docs
