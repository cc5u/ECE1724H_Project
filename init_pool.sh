# Pass the Token A and B mint addresses to initilaize
TOKEN_A=9tCwG6of3tAjpDcV7VpG5tjo9RgKGcwfbjWHcinGQuda
TOKEN_B=3aM2Jy4edDiVhD5i7K7V7ZjfXiVjm7zrnFDWvg46VT6d
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

