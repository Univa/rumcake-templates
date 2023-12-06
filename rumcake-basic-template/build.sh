#!/usr/bin/env bash

# Note: `cargo build` isn't actually needed to generate a binary, but
# `cargo objcopy` doesn't show errors if they happen during the build process,
# so we use `cargo build` anyways to see them. Should be fine as long as
# incremental compilation is enabled.

# Generate bin files
cargo build --release &&
	cargo objcopy --release -- -O binary firmware.bin
