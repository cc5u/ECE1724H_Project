POOL_PUBKEY=FjJHKRbQ1DJnJCHrMLpgYa4TbHbgbgvmdrfCRFeUNEgW

cd cli_client
echo "=== Remove Liquidity ==="
cargo run -- \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    remove-liquidity \
    --pool "$POOL_PUBKEY" \
    --lp-amount 2000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts