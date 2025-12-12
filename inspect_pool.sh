POOL_PUBKEY=7qeryxM274jkK7K6AnJkrZABv1DiJkBQ9WZKzyL9nngx

cd cli_client
echo "=== Pool Inspection ==="
cargo run -p cli_client --  \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    inspect-pool \
    --pool "$POOL_PUBKEY" \
    
echo ""