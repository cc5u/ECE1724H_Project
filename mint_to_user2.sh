TOKEN_A=7eiUFfU4ZFS3BYBYFWgXFHYvPxxkEx4ERcAaQ6UP26tk
AMOUNT=1000

USER_KEYPAIR=~/id2.json
PAYER=${PAYER:-$HOME/.config/solana/id.json}
MINT_AUTH=${MINT_AUTH:-$PAYER}
# to create a new keypair, run: solana-keygen new --no-bip39-passphrase -o $USER_KEYPAIR
USER_PUBKEY=$(solana-keygen pubkey $USER_KEYPAIR)
echo "User2 wallet: $USER_PUBKEY"
spl-token create-account $TOKEN_A --owner $USER_PUBKEY --fee-payer "$PAYER"
spl-token mint \
    --recipient-owner $USER_PUBKEY \
    $TOKEN_A $AMOUNT \
    --fee-payer "$PAYER" \
    --mint-authority "$MINT_AUTH"
echo "Minted $AMOUNT TOKEN_A to $USER_PUBKEY"
# show user2 all token accounts
spl-token accounts --owner "$USER_PUBKEY"