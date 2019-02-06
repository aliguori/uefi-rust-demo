#!/bin/sh

RUST_TARGET_PATH=`pwd` cargo xbuild --target=x86_64-none-efi
