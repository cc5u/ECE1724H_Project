# Pass the Token A and B mint addresses to initilaize
TOKEN_A=91vzzThegp7xxwzrq4e4YWp18SoKaJVuLU8S89xy3k1X
TOKEN_B=At2AjF4uozHFdp9cM6DpYkWUSQedmDGvXQBJ8U8vfNpo
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

