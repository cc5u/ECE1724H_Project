TOKEN_A=5rcxiFmcBiGsneZpq2WwwbFzJvjk8nbov28pHBd5rqDq
TOKEN_B=5FtFH6YunD2wPeu9e6mrm4uKN3ihPN2BxU9xfrY5VVNh
TOKEN_C=GRzSUxgUz75aqFToabTSc9GB2QJjC2ZtDnKXsCDjTiBB

USER_KEYPAIR=~/id2.json
PAYER=${PAYER:-$HOME/.config/solana/id.json}
MINT_AUTH=${MINT_AUTH:-$PAYER}
# to create a new keypair, run: solana-keygen new --no-bip39-passphrase -o $USER_KEYPAIR
USER_PUBKEY=$(solana-keygen pubkey $USER_KEYPAIR)
echo "User2 wallet: $USER_PUBKEY"
echo "=== Create user ATA accounts and mint ==="
echo "===== Token A ====="
spl-token create-account $TOKEN_A --owner $USER_PUBKEY --fee-payer "$PAYER"
spl-token mint \
    --recipient-owner $USER_PUBKEY \
    $TOKEN_A 50 \
    --fee-payer "$PAYER" \
    --mint-authority "$MINT_AUTH"
echo "===== Token B ====="
spl-token create-account $TOKEN_B --owner $USER_PUBKEY --fee-payer "$PAYER"
spl-token mint \
    --recipient-owner $USER_PUBKEY \
    $TOKEN_B 40 \
    --fee-payer "$PAYER" \
    --mint-authority "$MINT_AUTH"
echo "===== Token C ====="
spl-token create-account $TOKEN_C --owner $USER_PUBKEY --fee-payer "$PAYER"
spl-token mint \
    --recipient-owner $USER_PUBKEY \
    $TOKEN_C 5 \
    --fee-payer "$PAYER" \
    --mint-authority "$MINT_AUTH"
echo ""
# show user2 all token accounts
spl-token accounts --owner "$USER_PUBKEY"