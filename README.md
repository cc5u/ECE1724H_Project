# Project Proposal
## Motivation

Blockchain technology has been popular in recent years, especially within decentralized finance (DeFi). Smart contracts in DeFi enable transparent, autonomous, and trustless financial transactions by executing predetermined rules without intermediaries. However, the complexity and immutability of smart contracts mean that vulnerabilities or inefficiencies can lead to substantial financial losses, as evidenced by several high-profile DeFi exploits in recent years. To address these challenges, developers are increasingly turning to safer and more performant programming languages like Rust to enhance the reliability and robustness of blockchain applications.

Rust is known for its memory safety guarantees, strong support for concurrency, and high-performance execution. These features have made Rust a language of choice for building secure and performant blockchain platforms. Major blockchains like Solana use Rust to implement smart contracts, enhancing code correctness and execution efficiency. Despite Rust's rising prominence in blockchain core development, there remains a noticeable gap in end-to-end projects that integrate both on-chain smart contract logic and off-chain client applications entirely in Rust. We found that established Solana DEXs (e.g., Raydium, Orca, Meteora) typically implement client components in TypeScript or Python. This gap presents an opportunity to explore cohesive Rust-based decentralized application stacks.

Motivated by this gap, our team aims to develop an Automated Market Maker (AMM) decentralized exchange on Solana, complemented by a Rust-based command-line wallet and autonomous trading agent. This project excites us as it challenges us to apply Rust across the entire stack, from smart contract development to client interaction, while deepening our understanding of decentralized trading mechanisms and secure blockchain systems.

---
## Objectives and Key Features
**Objective**

We aim to design and implement a small-scale, fully functional AMM decentralized exchange (DEX) on the Solana blockchain using Rust. This will be accompanied by a Rust-based autonomous trading agent and a command-line wallet interface. The system will enable users to:
* Initialize new token pools.
* Provide or remove liquidity.
* Swap tokens through pools.
* Authorize an automated agent to perform trades or rebalance liquidity based on predefined strategies.

The final product will demonstrate a comprehensive decentralized financial application built entirely in Rust, showcasing cohesive on-chain smart contract design and off-chain automation within one ecosystem.

**Key Features**

1. AMM Smart Contract (On-Chain Program)

* Implemented in Rust using the Anchor framework.
* Employs the constant-product pricing formula ($\text {x×y=k}$) for token swaps.
* Core functions:
    * `initialize_pool()` — create token pair pools.
    * `add_liquidity()` / `remove_liquidity()` — deposit or withdraw assets in exchange for LP tokens.
    * `swap()` — execute token swaps with slippage protection and transaction fees.
* Emits structured events for activity tracking.
* Includes security measures such as overflow checks, signer verification, and account ownership validation

 
2. Command-Line Interface (CLI) Wallet
- Developed in Rust using `clap` and `solana-client` libraries.
- Enables users to:
    * Create and manage pools.
    * Add or remove liquidity.
    * Execute token swaps.
    * Inspect on-chain pool states and transaction history.
    * Supports local Solana test validator and Solana devnet operation.


3. Rust Agent (Autonomous Off-Chain Executor)
- A continuously running daemon implemented with Rust’s async ecosystem (tokio and axum).
- Connects to Solana RPC to fetch pool states and market data.
- Executes predefined trading strategies, such as:
    - Band Market-Making — automatic buy/sell within target price ranges.
    - Arbitrage Simulation — detect price deviations and rebalance liquidity.
- Integrates risk controls, including per-trade/daily limits, slippage thresholds, and whitelist filters.
- Supports HTTP-based pause/resume controls.
- Interfaces with the CLI wallet for authorization and real-time monitoring.

**Architecture and Code Organization**

Our current idea of the entire project architecture is shown below:

```
project-root/
├─ amm_dex/        # Anchor smart contract
├─ cli/            # Rust CLI wallet
├─ agent/          # Rust daemon for strategy automation
├─ tests/          # Integration and stress tests
└─ README.md       # Documentation and project proposal
```

---
## Tentative Plan

Below is our 10-week plan (Oct 6 – Dec 14) for designing, implementing, and testing the Solana AMM DEX, organized into four development phases.

### Phase 1, Environment Setup and Smart Contract Core (Weeks 1–3)

- Set up Solana toolchain and Anchor framework.  
- Define pool data structures and PDA accounts.  
- Implement core AMM logic (pool initialization, liquidity management, swap formula).  
- Write comprehensive unit and integration tests using `solana-program-test`.  
- Deploy and verify program on Solana devnet.

**Deliverables:**

- `amm_dex` program compiled and deployed on devnet.  
-  Local swap and liquidity transactions verified by test suite.

---
### Phase 2 – CLI Wallet Interface (Weeks 4–5)

- Implement Rust CLI using `clap` and `solana-client`.  
- Add command set: `create-pool`, `add-liq`, `remove-liq`, `swap`, `show-pool`, `approve-agent`, `revoke-agent`.  
- Connect CLI to devnet program ID for live interaction.  
- Provide usage documentation and demo scripts.

**Deliverables:**

- Fully functional CLI supporting all major program instructions.  
- Example demo: initialize pool → add liquidity → perform swap → display results.

---
### Phase 3 – Autonomous Rust Agent (Weeks 6–8)

- Build background service (`agent/`) using `tokio` async runtime.  
- Implement market snapshot polling and feature extraction from pool accounts.  
- Integrate simple decision logic (“band market-making” strategy).  
- Execute on-chain transactions through the CLI wallet’s authorized keypair.  
- Add control endpoints (`/pause`, `/resume`, `/status`).

**Deliverables:**

- Rust daemon continuously trading on the deployed pool.  
- Configurable parameters (`agent.toml`): thresholds, limits, polling interval.  
- Logged trade summaries with transaction hashes on devnet.

---
### Phase 4 – Testing, Safety, and Documentation (Weeks 9–10)

- Validate numerical correctness (swap price, liquidity accounting, fee distribution).  
- Run stability tests with simulated RPC disconnections and transaction retries.  
- Finalize README and architecture diagrams.  
- Record short demo video showing:
  1. Pool creation via CLI  
  2. Manual trade execution  
  3. Agent automatic trading behavior  
  4. Pool state evolution over time  

**Deliverables:**

- Repository with reproducible setup instructions.  
- Comprehensive documentation, including design rationale and limitations.  
- Polished demo ready for presentation and submission.

---
### **Division of Responsibilities**

|**Component**|**Developer A**|**Developer B**|
|---|---|---|
|**Solana Program (Anchor)**|Implement pool initialization and liquidity management logic; write unit tests for program modules.|Implement swap and fee logic; handle PDA account structure and integration testing.|
|**CLI Wallet (Rust + clap)**|Build command-line parsing and transaction submission flow.|Implement response handling, result formatting, and CLI documentation.|
|**Autonomous Trading Agent**|Develop async data polling, configuration system, and status control endpoints.|Implement trading decision logic, transaction execution, and performance logging.|
|**Testing & Integration**|Design automated test scripts and continuous integration workflow.|Conduct functional and stress testing with simulated RPC failures; analyze results.|
|**Documentation & Presentation**|Create architecture diagrams, system overview, and setup guide.|Write README usage examples, prepare demo scripts, and edit final video.|

**Collaboration Approach**

- Both developers contribute equally across all subsystems, with distinct but complementary tasks.
- Tasks are coordinated through weekly syncs and code reviews to ensure full knowledge sharing.
- Each developer will rotate to review the other’s code to maintain consistency and shared understanding.

---

