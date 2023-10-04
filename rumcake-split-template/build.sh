#!/usr/bin/env bash

# Generate bin files
cargo objcopy --bin left --features "rumcake/split-central rumcake/bluetooth rumcake/usb" --release -- -O binary left.bin
cargo objcopy --bin right --features "rumcake/split-peripheral" --release -- -O binary right.bin
