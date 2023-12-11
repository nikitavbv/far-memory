while true; do
    cargo run --release -- --evaluation --manager-endpoint 192.168.254.30:14000 --storage-endpoint 192.168.254.30:14001
    sleep 10
done
