cd amm_dex
anchor keys sync
anchor build
anchor deploy
echo "=== Creating Tokens A and Token B ==="
TOKEN_A=$(spl-token create-token 2>&1 | awk '/Address:/ {print $2}')
echo "TOKEN_A: $TOKEN_A"
TOKEN_B=$(spl-token create-token 2>&1 | awk '/Address:/ {print $2}')
echo "TOKEN_B: $TOKEN_B"
echo " "