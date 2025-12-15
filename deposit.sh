POOL_PUBKEY=4V3YyBD31X3U3sHKriTDDfxQtW7BWhFm4nkRvBEko2GE

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 30000000000 \
    --amount-b 25000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts