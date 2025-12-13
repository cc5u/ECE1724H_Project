TOKEN_A=FaSWg8SrEwmYXa9zf87QRmDaHNJ1NjKCmbrmVgrVEbkL
TOKEN_B=Co3AJVrTRmQWPfoTjHqmDo1aWEXHRXnoLZcZvyYiTU4S

cd cli_client
echo "=== Create user ATA accounts ==="
echo "Token A:"
spl-token create-account $TOKEN_A
spl-token mint $TOKEN_A 100
echo "Token B:"
spl-token create-account $TOKEN_B
spl-token mint $TOKEN_B 80
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts