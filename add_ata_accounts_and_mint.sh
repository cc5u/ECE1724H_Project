TOKEN_A=2DWFb9G76q9d5EQi7kbTeYjrErhUXwAM84apfkehNGcv
TOKEN_B=ErjhnhfF7f1P5fK2gvFphS4M6o9TswYc278WLNBaZ9u8
TOKEN_C=Fwh88EjNwTkZVYii8mr4WngyBDybCiFyWbXk1ciZ2Dmk

cd cli_client
echo "=== Create user ATA accounts ==="
echo "===== Token A ====="
spl-token create-account $TOKEN_A
spl-token mint $TOKEN_A 100
echo "===== Token B ====="
spl-token create-account $TOKEN_B
spl-token mint $TOKEN_B 75
echo "===== Token C ====="
spl-token create-account $TOKEN_C
spl-token mint $TOKEN_C 50
echo ""
echo "=== User ATA Accounts ==="
spl-token accounts