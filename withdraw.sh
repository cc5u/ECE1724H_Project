POOL_PUBKEY=Aq6jvkFwKrrBvM6v2eXhzP14TSre6CzCWzGcdGQeu1Gi
cd cli_client
echo "=== Remove Liquidity ==="
echo ""
echo "=== User ATA Accounts Balance (before)==="
spl-token accounts
./target/release/cli_client \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    remove-liquidity \
    --pool "$POOL_PUBKEY" \
    --lp-amount 2000000000
echo ""
echo "=== User ATA Accounts Balance (after)==="
spl-token accounts