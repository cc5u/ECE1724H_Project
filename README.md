# Video Slide Presentation
Video Presentation Link: https://youtu.be/i1awa09sefw
Video Slides Link: https://docs.google.com/presentation/d/1SD3embVnKnCANyhMzNi9Zb8PkbF5Krzncy6MY1zlXBY/edit?usp=sharing

# Video Demo
Video Demo Link: https://youtu.be/49kVeaJu-cE


# Final Report
Team members information:

Chen, Yuanhan | 1006741705 | yuanhan.chen@mail.utoronto.ca

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

To interact with our on-chain AMM program, we implemented a Rust-based CLI client (`cli_client`). Instead of relying on TypeScript/Python SDKs, the client uses `anchor-client` to construct and send transactions to our Anchor program, making the entire stack (on-chain + off-chain) Rust-native.

The CLI supports two usage styles:

* **Command mode (recommended for scripting):** run a single subcommand (e.g., initialize pool, add liquidity, swap).
* **Interactive mode:** run the binary without a subcommand to open a simple menu-driven interface.

### Common flags

* `--cluster`: `localnet` / `devnet` / `mainnet` (default: `localnet`)
* `--keypair`: path to the wallet keypair used as the transaction signer (default: `~/.config/solana/id.json`)

These options make it easy to switch between different clusters and to simulate multiple users by providing different keypairs.

### Supported commands (core features)

* **Initialize pool (`init-pool`)**  
  Create a new liquidity pool by selecting Token A mint, Token B mint, and a swap fee (`fee_bps`) in basis points.  
  The CLI derives required PDAs (PoolCounter / Pool / PoolAuthority), sends the initialize transaction, and prints the resulting addresses (pool PDA, vaults, LP mint).

* **Add liquidity (`add-liquidity`)**  
  Deposit Token A and Token B into the pool vaults, and receive LP tokens that represent ownership of the pool.  
  The CLI checks that the user has the required LP token ATA, and it will create the LP ATA automatically if missing.

* **Remove liquidity (`remove-liquidity`)**  
  Burn LP tokens and redeem the proportional underlying Token A and Token B back to the user’s ATAs.

* **Swap (`swap`)**  
  Swap between Token A and Token B using the constant-product pricing rule implemented on-chain.  
  Users provide `minimum_out` for slippage protection. Direction is controlled by `--is-a-to-b` (A→B) or omitting it (B→A).

### User-friendly commands (inspection and wallet)

* **Inspect one pool (`inspect-pool`)**  
  Prints a table of pool metadata and on-chain balances (vault reserves, LP supply, fee).

* **Show all pools (`showing-dex`)**  
  Scans all `Pool` accounts owned by the program and prints a summary table for each pool.

* **Wallet view (`wallet`)**  
  Displays current SOL balance and all SPL token account balances for the selected keypair.

### Example core functions usage

Initialize a pool:
```
cargo run -p cli_client -- \
  --cluster localnet \
  --keypair ~/.config/solana/id.json \
  init-pool \
  --token-a-mint <TOKEN_A_MINT> \
  --token-b-mint <TOKEN_B_MINT> \
  --fee-bps 30
```

Add liquidity:
```
cargo run -p cli_client -- \
  --cluster localnet \
  --keypair ~/.config/solana/id.json \
  add-liquidity \
  --pool <POOL_PDA> \
  --amount-a 1000000000 \
  --amount-b 1000000000
```

Remove liquidity:
```
cargo run -- \
  --cluster localnet \
  --keypair ~/.config/solana/id.json \
  remove-liquidity \
  --pool <POOL_PUBKEY> \
  --lp-amount 5000
```

Swap A→B with slippage protection:
```
cargo run -p cli_client -- \
  --cluster localnet \
  --keypair ~/.config/solana/id.json \
  swap \
  --pool <POOL_PDA> \
  --amount-in 1000000000 \
  --minimum-out 9000000 \
  --is-a-to-b
```

### CLI interactive interface

```
=================================================================
-------------------- Interactive AMM DEX CLI --------------------
=================================================================
Wallet: /Users/oscar178/.config/solana/id.json
Balance: 499999997.360804 SOL

- choose an option:
1) InitPool
2) AddLiquidity
3) RemoveLiquidity
4) Swap
5) InspectPool
6) ShowingDex
7) Wallet (SOL + tokens)
q) Quit
```

The interactive interface allow users to interact with our AMM DEX without memorizing all subcommands. After running the CLI without specifying a command (optionally use `--keypair <KEYPAIR_ADDR>` to switch to another wallet), it will show the current wallet path, SOL balance, and a menu of actions:

* `1) InitPool`: input Token A mint, Token B mint, and swap fee (`fee_bps`, default 30).
* `2) AddLiquidity`: input pool address, `amount_a`, `amount_b` and deposit both tokens into the pool to receive LP tokens.
* `3) RemoveLiquidity`: input pool address and `lp_amount` to burn LP tokens and redeem Token A/B.
* `4) Swap`: input pool address, `amount_in`, `minimum_out`, and the direction (A→B or B→A).
* `5) InspectPool`: input pool address to print pool status (vault balances, LP supply, fee).
* `6) ShowingDex`: list all pools created by our program.
* `7) Wallet`: print SOL + all token balances for the current keypair.
* `q) Quit`: exit the menu.

For `amount_a`, `amount_b`, `amount_in`, `lp_amount`, the interactive mode uses the same raw `u64` units as the subcommands (see the **Token unit consistency** section below for how raw amounts map to UI token amounts).

## Reproducibility Guide:

### Environment setup on Mac
Need to install Solana toolchain + Anchor (we use Anchor framework to build/deploy the program, and `spl-token` to create test tokens).
Following install guide are from Solana guide (https://solana.com/docs/intro/installation)

1) Install dependencies for Solana + Anchor
```
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```
After installation, restart the terminal so `solana` / `anchor` are in PATH.

2) Verify installation by checking versions
```
rustc --version && solana --version && anchor --version && spl-token --version
```

3) Generate the first Solana wallet (payer)
If you do not have a default keypair yet:
```
solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
solana address
```

Optional: create a second wallet (useful for testing / simulating another user):
```
solana-keygen new --no-bip39-passphrase -o ~/id2.json
solana-keygen pubkey ~/id2.json
```
You can switch users in our CLI by passing `--keypair ~/id2.json`.

4) Point Solana CLI to localnet (for testing)
```
solana config set --url localhost
solana config get
```

Before running `spl-token` commands on localnet, start `solana-test-validator --reset` (see next section) and fund the wallet:
```
solana airdrop 5
solana balance
```

Continue with next section (User’s Guide) Guto reproduce results.

### Token unit consistency

Token amount units (why numbers look different)
* `spl-token mint <MINT> <TOKEN_AMOUNT>` uses **token amount** (human-readable). Example: `spl-token mint <MINT> 10` means “mint 10 tokens”.
* Our Rust CLI uses **raw amount** (`u64`, smallest unit) in arguments like `--amount-a`, `--amount-b`, `--amount-in`.
  * If a token has **9 decimals** (default), then `1 token = 1_000_000_000` raw units.
  * If a token has **0 decimals**, then `1 token = 1` raw unit.
* CLI outputs like `wallet` / `inspect-pool` show **UI amounts** for readability, while the on-chain program always uses raw units.
  * Example (9 decimals): `spl-token mint <MINT> 100` mints 100 tokens = `100_000_000_000` raw units, and `--amount-a 30000000000` in our CLI means 30 tokens.

## User’s Guide

1. Clone the github repository: 
    `git clone https://github.com/cc5u/ECE1724H_Project.git`
2. Go to the project directory: 
    ```
    cd ECE1724H_Project
    ```
4. Start the `solana-test-validator`:
    `solana-test-validator --reset`
5. Open another terminal and go to the `amm_dex` folder:
    ```
    cd amm_dex/
    anchor keys sync
    anchor build
    anchor deploy
    ```
    This will sync program ids for the configured cluster (localnet) and build the AMM Dex, then deploy it to your localnet.
6. Create SPL Tokens
   
    Use this command to create SPL token (for this project, you need to crete at least two tokens): `spl-token create-token`
    The output after running the command should look like this:
    ```
    Creating token 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
    Address:  2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv
    Decimals:  9
    Signature: UkASmn1MvEAsicixCx1RpyxS8jsj4BPZj33LpncBHn3K8yccuBEmsfBmXurusyrhDEJzzVBpg76aGiB77ngGsGj
    ```
7. Create User ATA accounts and Mint Tokens

   After creating the token, you need to create the ATA account for each token and mint the token for yourself.
    
    Replace the token address (e.g. 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv) to the TOKEN_A and specifying how many tokens you want to mint:
    ```
    spl-token create-account TOKEN_A
    spl-token mint TOKEN_A AMOUNT
    ```
    
    To check the accounts: 
    `spl-token accounts`
    You will see something like:
    ```
    Token                                         Balance
    -----------------------------------------------------
    2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv  100
    AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4  100
    ```

9. Open another terminal and go to the `cli_client` folder:

    ```
      cd cli_client/
    ```
10. Build and run the project: 
    ```
    cargo build
    cargo run
    ```
    or
    ```
    cargo build --release
    ./target/release/cli_client
    ```
11. Then you will see the interactive interface:
    ```
    =================================================================
    -------------------- Interactive AMM DEX CLI --------------------
    =================================================================
    Wallet: Your_Key_Path
    Balance: Your_SOL_Balance SOL

    - choose an option:
    1) InitPool
    2) AddLiquidity
    3) RemoveLiquidity
    4) Swap
    5) InspectPool
    6) ShowingDex
    7) Wallet (SOL + tokens)
    q) Quit
    ```
    To add the SOL balance: `solana airdrop 5`
    1) InitPool

        Enter `1` to initilaize a new liquidity pool, you need to provide both token addresses:
        ```
        > 1
        token_a_mint: 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv
        token_b_mint: AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4
        fee_bps (number, default 30): 30
        Initialized pool 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex 
        tx: 49nkgvpeAHDWNZZeHzGjUUT9fCB2srWAyGjJ6BC9vC4YfRtmjChRdPHEgzdDZXHDUrrpWCndnSLUxGs8giZZXZA7
        Pool ID           : 0
        Pool counter PDA  : 3UvRQwbo9J8o9iZ9o4SPdfgRL17fNotqc6yH2PhqUNe1
        Pool PDA          : 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        Pool authority PDA: 9Y58a1oBrg1Mighn875isxU8nPLoyLXvC2ZMbPJ8n1oZ
        Token A mint      : 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv
        Token B mint      : AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4
        Token A vault     : 9ncATe7EbiCuV6fM9Jgnf8fASeRQW5Kq5kiuopztTh7E
        Token B vault     : Bk3dWB2LomoEaZry4X99YKAdjUPFKQehDQQaxf1EfQxm
        LP mint           : Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro
        ```
    3) AddLiquidity

        Enter `2` to add liquidity to a pool, you need to input the pool address (Pool PDA) and the amounts you want to deposit:
        ```
        > 2
        pool: 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        amount_a (u64): 20000000000
        amount_b (u64): 40000000000
        Added liquidity to pool 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        tx: 4WszhmX5XWnhiudTngxFqgtTE14oNKGJtgJDnxSFiQ3gCiCaFDXC1pJu3Ay1MayhEkjiS41CaUvkZEei7a2HC96h
        Created user LP ATA: G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M
        User token A ATA: 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy
        User token B ATA: 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd
        User LP ATA: G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M
        ```
        After deposit some liquidity, your account balance should change:
        ```
        +----------------------------------------------------------------------------------------------------------------------------------+
        | Token                                          Mint                                           Amount                             |
        +==================================================================================================================================+
        | 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy   2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv   80 (raw: 80000000000, decimals: 9) |
        |                                                                                                                                  |
        | 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd   AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4   60 (raw: 60000000000, decimals: 9) |
        |                                                                                                                                  |
        | G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M   Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro   60 (raw: 60000000000, decimals: 9) |
        +----------------------------------------------------------------------------------------------------------------------------------+
        ```
    5) RemoveLiquidity

        Enter `3` to remove liquidity to a pool, you need to input the pool address (Pool PDA) and the amount of LP token you want to burn:
        ```
        > 3
        pool: 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        lp_amount (u64): 30000000000
        Removed 30000000000 LP from pool
        7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex tx: icgHRX6S5SmJENvTnSUxUjevzYjv7UuSjuNqLgzQiLi4z1qtJ6PhGUKAWM4RzNmNPSsyP8oemgJicDZhQX1FDCc
        User token A ATA: 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy
        User token B ATA: 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd
        User LP ATA: G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M
        ```
        After withdraw some liquidity, your account balance should change:
        ```
        +----------------------------------------------------------------------------------------------------------------------------------+
        | Token                                          Mint                                           Amount                             |
        +==================================================================================================================================+
        | 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy   2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv   90 (raw: 90000000000, decimals: 9) |
        |                                                                                                                                  |
        | 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd   AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4   80 (raw: 80000000000, decimals: 9) |
        |                                                                                                                                  |
        | G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M   Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro   30 (raw: 30000000000, decimals: 9) |
        +----------------------------------------------------------------------------------------------------------------------------------+
        ```
    7) Swap

         Enter `4` to perform swap token, you need to input the pool address (Pool PDA), the amounts you want to trade in, the minimum outputm, and specifying the direction (`y` or `n`):
         ```
         > 4
         pool: 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
         amount_in (u64): 5000000000
         minimum_out (u64): 100
         direction (A->B type 'y' else 'n'): y
         Swap A -> B complete.
         Pool: 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        tx: 5joMQPAUjHMFmUMGivD27TY3tnJyX2bR8CxPeDNFqGqdopwBxTcTQisbAEunageN1wo1fMzYf5Gd8rAeXN47sPW8
        Source ATA: 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy
        Destination ATA: 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd

         ```
         After swapping your token, your account balance should change:
         ```
         > 7
        Wallet: 6jE69LoQ7pgMu4DAXrFxuvY8NZkKMGnR6A7FuXfwVP4n | SOL: 499999998.363112
        Token accounts (ATAs):
        +--------------------------------------------------------------------------------------------------------------------------------------------+
        | Token                                          Mint                                           Amount                                       |
        +============================================================================================================================================+
        | 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy   2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv   85 (raw: 85000000000, decimals: 9)           |
        |                                                                                                                                            |
        | 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd   AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4   86.653319986 (raw: 86653319986, decimals: 9) |
        |                                                                                                                                            |
        | G7XBQQsvBtcfZwEpG3Y3WEWAH7zqNCGP4oh84n47Z84M   Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro   30 (raw: 30000000000, decimals: 9)           |
        +--------------------------------------------------------------------------------------------------------------------------------------------+
         ```
    9) InspectPool

        Enter `5` to inspec the pool information. You need to input the pool address (Pool PDA):
        ```
        > 5
        pool: 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex
        +---------------+----------------------------------------------------------------------+
        | Field         | Value                                                                |
        +======================================================================================+
        | Pool account  | 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex                         |
        |---------------+----------------------------------------------------------------------|
        | Pool ID       | 0                                                                    |
        |---------------+----------------------------------------------------------------------|
        | Token A mint  | 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv                         |
        |---------------+----------------------------------------------------------------------|
        | Token A vault | 9ncATe7EbiCuV6fM9Jgnf8fASeRQW5Kq5kiuopztTh7E (balance: 15)           |
        |---------------+----------------------------------------------------------------------|
        | Token B mint  | AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4                         |
        |---------------+----------------------------------------------------------------------|
        | Token B vault | Bk3dWB2LomoEaZry4X99YKAdjUPFKQehDQQaxf1EfQxm (balance: 13.346680014) |
        |---------------+----------------------------------------------------------------------|
        | LP mint       | Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro                         |
        |---------------+----------------------------------------------------------------------|
        | LP supply     | 30                                                                   |
        |---------------+----------------------------------------------------------------------|
        | Fee (bps)     | 30                                                                   |
        |---------------+----------------------------------------------------------------------|
        | Price A/B     | 1.123875                                                             |
        +---------------+----------------------------------------------------------------------+
        ```
    11) ShowingDex

        Enter `6` to inspec all pools information on the AMM Dex:
        ```
        > 6
        Found 1 pool(s) for program 6uumabkAhB7jd7BftfQgS78hbaXd3AHDDrMtyAnABdor
        +---------------+----------------------------------------------------------------------+
        | Field         | Value                                                                |
        +======================================================================================+
        | Pool #        | 1                                                                    |
        |---------------+----------------------------------------------------------------------|
        | Pool account  | 7bUZe22pRWsP7Kbx1UAz5Gvnx9pApPjFfXrvGYdcNVex                         |
        |---------------+----------------------------------------------------------------------|
        | Pool ID       | 0                                                                    |
        |---------------+----------------------------------------------------------------------|
        | Token A mint  | 2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv                         |
        |---------------+----------------------------------------------------------------------|
        | Token A vault | 9ncATe7EbiCuV6fM9Jgnf8fASeRQW5Kq5kiuopztTh7E (balance: 15)           |
        |---------------+----------------------------------------------------------------------|
        | Token B mint  | AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4                         |
        |---------------+----------------------------------------------------------------------|
        | Token B vault | Bk3dWB2LomoEaZry4X99YKAdjUPFKQehDQQaxf1EfQxm (balance: 13.346680014) |
        |---------------+----------------------------------------------------------------------|
        | LP mint       | Dx5Gwd6AyCMUASCLBHgYarbDLCqoLUpCm2ADnbNDGCro                         |
        |---------------+----------------------------------------------------------------------|
        | LP supply     | 30                                                                   |
        |---------------+----------------------------------------------------------------------|
        | Fee (bps)     | 30                                                                   |
        |---------------+----------------------------------------------------------------------|
        | Price A/B     | 1.123875                                                             |
        +---------------+----------------------------------------------------------------------+
        ```
    13) Wallet (SOL + tokens):

        Enter `7` to see your account balance, the output will similiar to this:
        ```
        Wallet: 6jE69LoQ7pgMu4DAXrFxuvY8NZkKMGnR6A7FuXfwVP4n | SOL: 499999998.374095
        Token accounts (ATAs):
        +------------------------------------------------------------------------------------------------------------------------------------+
        | Token                                          Mint                                           Amount                               |
        +====================================================================================================================================+
        | 3J37jDFjVkxRRWZihwJ1yYYzPzrKAnatAK1EZ7RcahKy   2vkPUFEER4azYpymYUemypk1GDM4LEZngTQkhRgYPunv   100 (raw: 100000000000, decimals: 9) |
        |                                                                                                                                    |
        | 5ZDVJj8VdZ9wjmm7fGN19FSQFv8saYZiwis4qZqnDifd   AQDAWpkivvh8p95unPiFiqP5Rj195oNWpGnSuoAW4dS4   100 (raw: 100000000000, decimals: 9) |
        +------------------------------------------------------------------------------------------------------------------------------------+
        ```
    15) Quit: entering `q` to quit the program

## Contrubution

Below is the table showing the contirbution of both teammebers:


|  | Wu, Chia-Chun | Chen, Yuanhan |
| -------- | -------- | -------- |
| AMM Dex     | Implemented Pool Initialization,<br>Added bash scrips for testing and running,<br>Implemented Token Swap,<br> Added pool counter to enable multiple pools creation even the tokens are same    | Architecture design,<br>Modularized the code,<br>Implemented Add / Remove liquidity, bps fee and LP token,<br>Added bash scrips for testing and running,    |
| CLI Client     | Pool Initialization function,<br>Added bash scrips for testing and running,<br>Token Swap function in accordance to the AMM Dex, <br> Pool(s) inspection implementation,<br>Beautify CLI interface and outputs| Architecture design,<br>Modularized the code,<br>Add / remove liquidity, bps fee and LP token in accordance to the AMM Dex,     |
| Trading Agent    | Researching arbitrage tading strategy,<br>Testing fetching pool information on the AMM Dex.|  Testing  |
|Documentation<br>and <br>Videos|Final Report<br>Video Demo|Final Report<br>Presentation|

## Lessons learned and concluding remarks:

This project make us understand how modern blockchain systems operate in practice. By implementing an on-chain AMM from scratch, we gained hands-on experience with fundamental blockchain concepts such as account-based state models, transaction execution, program ownership, and trustless state transitions. Beyond theoretical knowledge, building and testing the system on a live Solana cluster clarified how decentralized programs enforce correctness and security without centralized control.

Working within the Solana ecosystem exposed us to a high-performance blockchain architecture optimized for parallelism and low-latency execution. We learned how Solana’s account model, program-derived addresses (PDAs), and SPL token standard enable scalable decentralized applications. Interacting with Solana through both CLI tools and custom Rust clients reinforced the importance of precise account management, deterministic address derivation, and careful handling of on-chain resources.

The Anchor framework significantly streamlined the development of our smart contract logic. Through Anchor, we learned how declarative account constraints, automatic serialization, and CPI abstractions reduce boilerplate while improving safety and readability. At the same time, the project highlighted the need to understand Anchor’s underlying abstractions—such as PDA bumps and signer semantics—to avoid subtle bugs and ensure correct program behavior.

Although we were unable to fully complete the off-chain trading agent within the project timeline, both team members invested significant effort in researching its design and implementation. This exploration introduced us to practical algorithmic trading strategies, including arbitrage detection and band-based market-making, within a decentralized setting. From a Rust-focused perspective, this work reinforced key concepts such as asynchronous programming, state management, and modular system architecture. It also highlighted the importance of robust error handling, slippage control, and risk management when designing off-chain automation that interacts safely and deterministically with on-chain execution.

In conclusion, this project successfully bridged theoretical blockchain concepts with hands-on Rust systems programming. By developing a decentralized exchange entirely in Rust—spanning on-chain smart contracts, client-side tooling, and the foundations of an autonomous agent, we deepened our understanding of Rust’s safety guarantees and ecosystem tooling. The project emphasized Rust as a practical language for building reliable, high-performance systems, and provided a strong foundation for future work in systems programming and distributed applications.

## References
1. https://github.com/raydium-io/raydium-amm/tree/master
2. https://github.com/alphaengine9/Market-Maker-Framework/tree/main
3. https://www.anchor-lang.com/docs
