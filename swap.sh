POOL_PUBKEY=5UiF4g1JjbbCrrDi5KE6DJUp71LRxwGkhFU9qYHHLCqn

cd cli_client
echo "=== Swap Token (A -> B) ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    swap \
    --pool "$POOL_PUBKEY" \
    --amount-in 1000000000 \
    --minimum-out 9000000 \
    --is-a-to-b
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts