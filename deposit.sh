POOL_PUBKEY=5kZZq93Xx3iNw6sawP9qLyYof1hi5gsQsfMiyKTFKqeZ

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 1000000000 \
    --amount-b 1000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts