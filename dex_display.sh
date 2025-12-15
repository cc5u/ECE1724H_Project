cd cli_client
echo "=== AMM DEX ==="
./target/release/cli_client \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    showing-dex 
    
echo ""