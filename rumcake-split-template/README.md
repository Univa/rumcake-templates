# rumcake-template

<!--toc:start-->
  - [About](#about)
    - [memory.x](#memoryx)
  - [Toolchain requirements](#toolchain-requirements)
    - [Target](#target)
  - [Building and Flashing](#building-and-flashing)
    - [With a probe (e.g. ST-LINK or J-LINK)](#with-a-probe-eg-st-link-or-j-link)
    - [Without a probe](#without-a-probe)
<!--toc:end-->

## About

This template builds a firmware for a split keyboard that has:

- Bluetooth host communication (using the left half)
- nRF52840 MCU on both halves (tested with nice!nano v2)

### memory.x

Note that `memory.x` contains values that correspond to an nRF52840 that has softdevice S140 6.1.1.
If your MCU has a different softdevice on it, you may need to change these values.

See the [nrf-softdevice repo](https://github.com/embassy-rs/nrf-softdevice) for more details.

## Toolchain requirements

This template assumes that you have `rustup` and `cargo` available in your `$PATH`, and that you have the latest `nightly` Rust toolchain.

### Target

Since this keyboard uses an nRF52840, we need to add the `thumbv7em-none-eabihf` target to our rust toolchain.

```bash
rustup target add thumbv7em-none-eabihf
```

## Building and Flashing

The following instructions are specific to this MCU (nRF52840).

Note that `.cargo/config.toml` specifies `thumbv7em-none-eabihf` as our build target.

### With a probe (e.g. ST-LINK or J-LINK)

If you have a probe, make sure you [install probe-run](https://probe.rs/docs/getting-started/installation/).

To build and flash the left half, call `cargo run` with the required features (specified in the `Cargo.toml`).

```bash
cargo run --bin left --release --features "rumcake/split-central rumcake/bluetooth rumcake/usb"
```

Similarly with the right side:

```bash
cargo run --bin right --release --features "rumcake/split-peripheral"
```

### Without a probe

No instructions yet. Converting the binary to a UF2 and flashing that to the MCU doesn't seem to work.
