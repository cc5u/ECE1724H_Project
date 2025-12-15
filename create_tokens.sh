echo "=== Creating Tokens A, Token B, and Token C ==="
TOKEN_A=$(spl-token create-token 2>&1 | awk '/Address:/ {print $2}')
echo "TOKEN_A=$TOKEN_A"
TOKEN_B=$(spl-token create-token 2>&1 | awk '/Address:/ {print $2}')
echo "TOKEN_B=$TOKEN_B"
TOKEN_C=$(spl-token create-token 2>&1 | awk '/Address:/ {print $2}')
echo "TOKEN_C=$TOKEN_C"
echo " "