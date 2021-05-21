cd ../host
RUST_LOG=debug cargo run -- --host 127.0.0.1:8000 \
       --role agent \
       --wallet-type indy \
       --wallet-config "{\"id\":\"aries-agent\", \"key\":\"agent-key\", \"poolFileName\":\"pool_file_name\"}" \
       --automation http \
       --automation-config http://localhost:5001
cd ../tools
