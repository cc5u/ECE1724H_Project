TOKEN_A=ZPJ9tfbHEtw7jnnrdyHB82EKYWQ1wZsDa6nFtp89dow
TOKEN_B=85byDtdrrZhdoPNRHDq2hbuHC1WBfEMjcemvht2Rf4xf

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