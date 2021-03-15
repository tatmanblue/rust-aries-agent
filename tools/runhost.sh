cd ../host
RUST_LOG=debug cargo run -- --host 127.0.0.1:8000 \
       --role agent \
       --wallet-type basic \
       --wallet-config "{\"fileName\":\"basicwallet.json\"}" \
       --automation http \
       --automation-config http://localhost:5001
cd ../tools
