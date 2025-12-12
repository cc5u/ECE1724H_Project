TOKEN_A=3nam7KMB8jTriFbZqGqBQjW41MKm7HhXpLWqcSXxrMQw
TOKEN_B=62iez9SZLJxAPUC8xSUn2ZSt2cL9nEEg5avjY1TSJTDN

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