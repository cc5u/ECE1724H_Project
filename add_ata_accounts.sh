TOKEN_A=91vzzThegp7xxwzrq4e4YWp18SoKaJVuLU8S89xy3k1X
TOKEN_B=At2AjF4uozHFdp9cM6DpYkWUSQedmDGvXQBJ8U8vfNpo

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