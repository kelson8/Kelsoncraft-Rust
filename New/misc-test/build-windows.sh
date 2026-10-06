#!/bin/bash

# Cross compile from Linux to Windows.
# https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows

# This requires x86_64-pc-windows-gnu to be installed
# rustup target add x86_64-pc-windows-gnu

cargo build --target x86_64-pc-windows-gnu
