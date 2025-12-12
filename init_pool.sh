# Pass the Token A and B mint addresses to initilaize
TOKEN_A=3nam7KMB8jTriFbZqGqBQjW41MKm7HhXpLWqcSXxrMQw
TOKEN_B=62iez9SZLJxAPUC8xSUn2ZSt2cL9nEEg5avjY1TSJTDN
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

