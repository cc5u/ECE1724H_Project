POOL_PUBKEY=7qeryxM274jkK7K6AnJkrZABv1DiJkBQ9WZKzyL9nngx

cd cli_client
echo "=== Swap Token (A -> B) ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    swap \
    --pool "$POOL_PUBKEY" \
    --amount-in 2000000000 \
    --minimum-out 9000000 \
    --is-a-to-b
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts