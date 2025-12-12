# 1. Start a local validator + deploy the program
### 1.1 Start solana-test-validator
```
solana-test-validator --reset
```

### 1.2 Point Solana to localnet
```
solana config set --url http://127.0.0.1:8899
solana config get
```

### 1.3 Making sure there are SOL in the wallet
```
solana airdrop 5
solana balance
```

### 1.4 Deploy your Anchor program to localnet
```
cd amm_dex
anchor deploy

```
---
# 2. Create two test token mints (Token A, Token B)
Create Token A
```
spl-token create-token
```
=> Save the printed mint address, e.g. HkW5thmVBmaoLyjaLjhbNxAeWTGPhe2vNnP3guruAk3m
`Token A:  <TOKEN_A_MINT>`

Create Token B
```
spl-token create-token
```
=> Save the printed mint address, e.g. 3ubQ1YYDAa7xr8m2sav1GZnTk3ZjbzNNVUT6zoxqFA3P
`Token B:  <TOKEN_B_MINT>`

# 3. Run CLI to initialize the pool
```
cargo run -p cli_client -- \
  --cluster localnet \
  --keypair ~/.config/solana/id.json \
  init-pool \
  --token-a-mint <TOKEN_A_MINT> \
  --token-b-mint <TOKEN_B_MINT> \
  --fee-bps 30
```
=> The command line will print:
`Initialized pool <POOL_PDA> with tx <TX_SIGNATURE>`

Initialized pool FTgUPnuekSXE3mWaJaSADAzFM4Kxszwu7ihsfJLN8EoQ tx: jtb6m5QVdCmCvDQivcjntKqzUc9qC3aVkRC29DeTkZy9pKSuhYZq5ExqYQk4PuwUFYLemizvufutUGVTVFzN4TR
Pool PDA          : FTgUPnuekSXE3mWaJaSADAzFM4Kxszwu7ihsfJLN8EoQ
Pool authority PDA: AimFHMNysCG1bXjjEbrVgiUQ5Y9pkafqJhrnZ6weP6PS
Token A mint      : HkW5thmVBmaoLyjaLjhbNxAeWTGPhe2vNnP3guruAk3m
Token B mint      : 3ubQ1YYDAa7xr8m2sav1GZnTk3ZjbzNNVUT6zoxqFA3P
Token A vault     : 3ERRLiNo4qyk4mJ2qdfXZaqMSQeJ5394V9qn4wj4ryuc
Token B vault     : 3m6rTuBsjfjtw7rvzsFW6PF6Jfkk7TDbhqEwRmg1BCz5
LP mint           : CtR7KJ3fDjFnEtwsHaG35AYNqT1mrWwM6YPpw6de6ArX


# 4. Verify on-chain that the pool exists
To confirm the transaction:
```
solana confirm <TX_SIGNATURE>
```

To inspect the pool:
```
solana account <POOL_PDA>
```

To check vaults and LP mint
```
solana account <TOKEN_A_VAULT_PUBKEY>
solana account <TOKEN_B_VAULT_PUBKEY>
spl-token account-info <LP_MINT_PUBKEY>
```

# 5. Create ATA account for Token A, B and LP mint

```
spl-token create-account <MINT_PUBKEY>
```
=> save the printed ATA address, e.g. 76qM3sv9QHUiiWzcj5Z9owcgKZ2gJzERU1xSKg15S7bD
`Creating account <ATA_KEY>  Signature: <SIGNATURE>`

# 6. Run CLI to add liquidity to the pool

add token to user ATA account, run for both token
```
spl-token mint <MINT_PUBKEY> 100000000000 <ATA_KEY>
```

add liquidity

```
cargo run -- --cluster localnet --keypair ~/.config/solana/id.json add-liquidity --pool <POOL_PUBKEY> --amount-a 1000000000 --amount-b 1000000000
```
=> the command line output will be like

Added liquidity to pool BpzXu9kAxcKVSG7WXjbMEJAkgwAiDKqFZLWKvTuK97eR tx: 2kMYjp56kEk9rHy8rvCFSm5e69zcf9pVhoSanhZ8bVw8S44qMuiSoqhayBcFzpkKc2nGbRP7bUfUkxzPfs4xqrkF
User token A ATA: BjgAMsdGhAdFU5QLbwBizWfGffdJdXwYTBqB9FckQDeM
User token B ATA: 3RCsrA8q6N4imSx1YMoENXWawEofJkmARhtqQ7nqM2hb
User LP ATA: 76qM3sv9QHUiiWzcj5Z9owcgKZ2gJzERU1xSKg15S7bD