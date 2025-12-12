# Pass the Token A and B mint addresses to initilaize
TOKEN_A=F2A24Ua1mKhphMYT8AirsyP3xE8rbWPMtBhmKCSdVtok
TOKEN_B=DRiRcYbNhcrbWwm1x18p26bPepm5b3osiMR5Nof6fKpd
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

