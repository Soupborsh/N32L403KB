# T90 rust code template.

There are 3 options to build this.
- in_ram
- app
- bare (default)

### in_ram

Easiest, and AFAIK safest way.

Enabling `in_ram` feature, makes firmware be linked to be launched from SRAM.

You can build and run like this:
```shell
cargo run --release -F in_ram
```
It will use `probe-rs`.

### app

Warning: working unconfirmed.

Used to create fw that official T90 bootloader accepts in update mode.
It can be made into .atk file.
In constart to `bare` it goes to 0x5000 offset in firmware.

To build as an `app` enable `app` feature:

```sh
cargo build --release -F app
```

TODO: Instructions on getting bin from elf, packing into .atk file, flashing.

### bare (WIP, does not work)

Warning: until probe-rs flash algo is broken, this soft bricks the MCU.

This writes to root of flash and overwrites the stock "update mode" bootloader.
```shell
cargo run --release
```

#### About build modes

Depending on feature `memory_ram.x` / `memory_app.x` / `memory_bare.x`
gets copied to `memory.x`.

`memory.x` should not be modified by hand.

See `build.rs` for more details.

### probe-rs

Support for N32L40x is WIP in [my probe-rs fork](https://github.com/Soupborsh/probe-rs).

## License

My code is 'GPL-3.0-only'. The `link_ram.x` file is 'MIT OR Apache-2.0'.
This file is from https://github.com/embassy-rs/teleprobe/blob/main/link_ram_cortex_m.x
