# Pass the Token A and B mint addresses to initilaize
TOKEN_A=FaSWg8SrEwmYXa9zf87QRmDaHNJ1NjKCmbrmVgrVEbkL
TOKEN_B=Co3AJVrTRmQWPfoTjHqmDo1aWEXHRXnoLZcZvyYiTU4S
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

