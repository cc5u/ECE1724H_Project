POOL_PUBKEY=bqh9y7TGtCWMfJ4Y1eAFd22rvdwZbdTNja2jY4vXVj4

cd cli_client
echo "=== Remove Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    remove-liquidity \
    --pool "$POOL_PUBKEY" \
    --lp-amount 50000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts