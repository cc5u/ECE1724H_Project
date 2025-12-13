POOL_PUBKEY=Bp9U9LdNP1Xqhrugjjo2SRi4kyqqwhHP1oNc8xw1cnk

cd cli_client
echo "=== Add Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 4000000000 \
    --amount-b 8000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts