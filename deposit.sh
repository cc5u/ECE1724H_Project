POOL_PUBKEY=7qeryxM274jkK7K6AnJkrZABv1DiJkBQ9WZKzyL9nngx

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 2000000000 \
    --amount-b 8000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts