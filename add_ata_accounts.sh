TOKEN_A=F2A24Ua1mKhphMYT8AirsyP3xE8rbWPMtBhmKCSdVtok
TOKEN_B=DRiRcYbNhcrbWwm1x18p26bPepm5b3osiMR5Nof6fKpd

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