# Project Proposal

## Motivation

## Objectives and Key Features

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

