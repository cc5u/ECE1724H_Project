POOL_PUBKEY=Aq6jvkFwKrrBvM6v2eXhzP14TSre6CzCWzGcdGQeu1Gi

cd cli_client
echo "=== Add Liquidity ==="
./target/release/cli_client \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    add-liquidity \
    --pool "$POOL_PUBKEY" \
    --amount-a 30000000000 \
    --amount-b 25000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts