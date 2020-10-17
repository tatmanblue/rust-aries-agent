cd ..\host
cargo run -- --host 127.0.0.1:8000 \
       --role agent \
       --wallet-type basic \
       --wallet-config "{\"fileName\":\"basicwallet.json\"}" \
       --talk-back http \
       --talk-back-config http://localhost:5001
cd ..\tools