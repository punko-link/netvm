#!/bin/bash

cargo build
cp -fr ./target/x86_64-unknown-none/debug/netvm ./bin

du -h ./bin/netvm
