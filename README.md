# rumcake-templates

<!--toc:start-->
- [Template updates](#template-updates)
- [Available templates](#available-templates)
  - [`rumcake-basic-template`](#rumcake-basic-template)
  - [`rumcake-split-template`](#rumcake-split-template)
  <!--toc:end-->

A collection of templates to start building your rumcake firmware.

To get started, [install `cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation),
then generate one of the templates:

```bash
cargo generate --git https://github.com/Univa/rumcake-templates
```

# Template updates

`rumcake` is being developed rapidly, and may introduce a lot of breaking changes
to its public API. If you want to update `rumcake`, please refer to the templates
to see how to use it with the most recent version.

# Available templates

## `rumcake-basic-template`

Template for a basic keyboard with underglow and a duplex matrix, with the following layout:

![image](https://github.com/Univa/rumcake-templates/assets/41708691/f9b832d5-6e7c-4959-88db-42b89bcd983c)

- STM32F303CBx
- USB host communication
- WS2812 Underglow
- Duplex Matrix
- Via

## `rumcake-split-template`

![image](https://github.com/Univa/rumcake-templates/assets/41708691/e2bc9e90-3b6a-459f-970c-278bcc45ef7c)

Template for a split keyboard with a left and right half, no dongle. No OLED display support yet.

- nRF52840 on both MCUs
- Bluetooth host communication
- Bluetooth used to communicate between both halves
