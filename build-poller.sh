#!/bin/bash

cd poller
TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
(cd target/x86_64-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/)