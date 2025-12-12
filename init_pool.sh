# Pass the Token A and B mint addresses to initilaize
TOKEN_A=3rMeVm1gaVmnzxU1rLWTgX6nn5z4GvZJybLcxKWN82Sg
TOKEN_B=99nkHKhzxNLne7Yr9jhxkzmjav4ZDxNNZtzvT18VCNWo
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

