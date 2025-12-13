cd cli_client
echo "=== AMM DEX ==="
cargo run -p cli_client --  \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    showing-dex 
    
echo ""