TOKEN_A=3rMeVm1gaVmnzxU1rLWTgX6nn5z4GvZJybLcxKWN82Sg
TOKEN_B=99nkHKhzxNLne7Yr9jhxkzmjav4ZDxNNZtzvT18VCNWo

cd cli_client
echo "=== Create user ATA accounts ==="
echo "Token A:"
spl-token create-account $TOKEN_A
spl-token mint $TOKEN_A 100
echo "Token B:"
spl-token create-account $TOKEN_B
spl-token mint $TOKEN_B 50
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts