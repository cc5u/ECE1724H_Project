POOL_PUBKEY=9is954T9eDx1yDKaP3gqo1VKrAg5gjRSFj9p9krZFXuy

cd cli_client
echo "=== Swap Token (A -> B) ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    swap \
    --pool "$POOL_PUBKEY" \
    --amount-in 1000000000 \
    --minimum-out 9000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts