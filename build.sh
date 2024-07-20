#!/bin/bash
rustc netvm/main.rs -C lto -C panic=abort -C opt-level=2 --out-dir=./build
strip ./build/main
du -h ./build/main
