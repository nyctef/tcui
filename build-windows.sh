#!/usr/bin/env bash
cargo rustc --target x86_64-pc-windows-gnu "$@" -- -C linker=x86_64-w64-mingw32-gcc