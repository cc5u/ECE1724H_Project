POOL_PUBKEY=5UiF4g1JjbbCrrDi5KE6DJUp71LRxwGkhFU9qYHHLCqn

cd cli_client
echo "=== Remove Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    remove-liquidity \
    --pool "$POOL_PUBKEY" \
    --lp-amount 1000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts