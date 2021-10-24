#!/bin/bash
set -euo pipefail

cargo build --release --all-features

rm -rf .tmp
mkdir -p .tmp
cp target/x86_64-unknown-linux-musl/release/guntamatic .tmp
cp Dockerfile .tmp
pushd .tmp
docker build -t geropl/guntamatic -f Dockerfile .
popd
rm -rf .tmp