#!/bin/bash

cargo build  --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res