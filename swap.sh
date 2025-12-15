POOL_PUBKEY=3LjkmU5ovuFhfsN5zMsgkPVGcdHbkKSMmazGjx3yLoom

cd cli_client
echo "=== Swap Token (A -> B) ==="
echo ""
echo "=== User ATA Accounts Balance (before)==="
spl-token accounts
./target/release/cli_client \
    --cluster localnet \
    --keypair ~/.config/solana/id.json \
    swap \
    --pool "$POOL_PUBKEY" \
    --amount-in 1000000000 \
    --minimum-out 9000000
echo ""
echo "=== User ATA Accounts Balance (after)==="
spl-token accounts