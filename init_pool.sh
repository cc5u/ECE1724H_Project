# Pass the Token A and B mint addresses to initilaize
TOKEN_A=ZPJ9tfbHEtw7jnnrdyHB82EKYWQ1wZsDa6nFtp89dow
TOKEN_B=85byDtdrrZhdoPNRHDq2hbuHC1WBfEMjcemvht2Rf4xf
cd cli_client
echo "=== Initializing Pool ==="
cargo run -p cli_client -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    init-pool \
    --token-a-mint "$TOKEN_A" \
    --token-b-mint "$TOKEN_B" \
    --fee-bps 30

