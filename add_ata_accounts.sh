TOKEN_A=9tCwG6of3tAjpDcV7VpG5tjo9RgKGcwfbjWHcinGQuda
TOKEN_B=3aM2Jy4edDiVhD5i7K7V7ZjfXiVjm7zrnFDWvg46VT6d

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