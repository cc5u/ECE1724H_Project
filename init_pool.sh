# Pass the Token A, B, and C mint addresses to initilaize
TOKEN_A=BFqoVjMr3wSutvcGqA5ehPV3TsyuA7ZRrJ99p5g7JtUU
TOKEN_B=2azP3w8HvMxEwQ7KyvtGVPY9t4MPViYjJtoE9frVD8Md
TOKEN_C=4qbLMZaxqYS4Y2FJX53cuGCJV9VNgJ8yYcw98GogyWTo
cd cli_client
echo "=== Initializing Pool ==="
./target/release/cli_client \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_C" \
    --fee-bps 30

