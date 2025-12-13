POOL_PUBKEY=FjJHKRbQ1DJnJCHrMLpgYa4TbHbgbgvmdrfCRFeUNEgW

cd cli_client
echo "=== Pool Inspection ==="
cargo run -p cli_client --  \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    inspect-pool \
    --pool "$POOL_PUBKEY" \
    
echo ""