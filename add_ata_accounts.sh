TOKEN_A=EDH6NjZNnyozWAQuqVZE3vEG5mqaV3uPE3LRGgzqdtcC
TOKEN_B=7Sy6a5ZWDJ9seTN3JC1FkmuJLMFTg9Q3tCEazxh5eY7E

cd cli_client
echo "=== Create user ATA accounts ==="
echo "Token A:"
spl-token create-account $TOKEN_A
spl-token mint $TOKEN_A 10000000000
echo "Token B:"
spl-token create-account $TOKEN_B
spl-token mint $TOKEN_B 10000000000
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts