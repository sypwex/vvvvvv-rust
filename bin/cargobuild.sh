#!/bin/bash

target_dir=./target/release

echo "============================= cargo build"
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=x86_64-apple-darwin
cargo build --target=x86_64-pc-windows-gnu
