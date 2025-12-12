POOL_PUBKEY=bqh9y7TGtCWMfJ4Y1eAFd22rvdwZbdTNja2jY4vXVj4

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 2000000000 \
    --amount-b 1000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts