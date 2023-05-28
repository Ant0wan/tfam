#!/usr/bin/env bash
cargo build --release
sudo rm -f /usr/local/bin/tfam
sudo install target/release/tfam /usr/local/bin

# cargo watch  -q -c -w src/ -x 'run -q'
