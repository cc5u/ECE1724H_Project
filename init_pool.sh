# Pass the Token A and B mint addresses to initilaize
TOKEN_A=EDH6NjZNnyozWAQuqVZE3vEG5mqaV3uPE3LRGgzqdtcC
TOKEN_B=7Sy6a5ZWDJ9seTN3JC1FkmuJLMFTg9Q3tCEazxh5eY7E
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

