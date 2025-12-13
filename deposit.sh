POOL_PUBKEY=9is954T9eDx1yDKaP3gqo1VKrAg5gjRSFj9p9krZFXuy

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 10000000000 \
    --amount-b 5000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts