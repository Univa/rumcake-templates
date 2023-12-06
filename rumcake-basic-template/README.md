# rumcake-template

<!--toc:start-->
  - [About](#about)
  - [Toolchain requirements](#toolchain-requirements)
    - [Target](#target)
  - [Building and Flashing](#building-and-flashing)
    - [With a probe (e.g. ST-LINK or J-LINK)](#with-a-probe-eg-st-link-or-j-link)
    - [Without a probe](#without-a-probe)
<!--toc:end-->

## About

This template builds a firmware for a keyboard that has:

- USB host communication
- Via support
- WS2812 Underglow (using a bitbang driver)
- PCB with a duplex matrix
- STM32F303CBx MCU

## Toolchain requirements

This template assumes that you have `rustup` and `cargo` available in your `$PATH`, and that you have the latest `nightly` Rust toolchain.

### Target

Since this keyboard uses an STM32F303CBx, we need to add the `thumbv6m-none-eabi` target to our rust toolchain.

```bash
rustup target add thumbv6m-none-eabi
```

## Building and Flashing

The following instructions are specific to this MCU (STM32F303CBx).

Note that `.cargo/config.toml` specifies `thumbv6m-none-eabi` as our build target.

### With a probe (e.g. ST-LINK or J-LINK)

If you have a probe, make sure you [install probe-run](https://probe.rs/docs/getting-started/installation/).

Then, you can simply call `cargo run --release` to build and flash the firmware onto your keyboard (and debug!)

### Without a probe

If you don't have access to a probe, first install `cargo-binutils`:

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Then, you can build your firmware:

```bash
cargo objcopy --release -- -O binary firmware.bin
```

Then, flash it to your keyboard (make sure it is in DFU mode already):

```bash
dfu-util -d 0483:df11 -a 0 -s 0x80000000:leave -D keyboard.bin
```
