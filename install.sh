#!/bin/bash
cargo build --release
sudo mkdir -p /usr/local/bin/
sudo cp target/release/md5solver /usr/local/bin/md5solver