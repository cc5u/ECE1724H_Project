POOL_PUBKEY=Dtza6TRoGJsf4ThA2sKciQ9jJobLX7eg3HnHW8PxPpi1

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    remove-liquidity \
    --pool "$POOL_PUBKEY" \
    --lp-amount 50000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts