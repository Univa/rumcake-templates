#!/usr/bin/env bash

# Note: `cargo build` isn't actually needed to generate a binary, but
# `cargo objcopy` doesn't show errors if they happen during the build process,
# so we use `cargo build` anyways to see them. Should be fine as long as
# incremental compilation is enabled.

# Generate bin files
cargo build --bin left --features "rumcake/split-central rumcake/bluetooth rumcake/usb" --release &&
	cargo objcopy --quiet --bin left --features "rumcake/split-central rumcake/bluetooth rumcake/usb" --release -- -O binary left.bin

cargo build --quiet --bin right --features "rumcake/split-peripheral" --release &&
	cargo objcopy --quiet --bin right --features "rumcake/split-peripheral" --release -- -O binary right.bin
