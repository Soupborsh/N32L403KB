# T90 rust code template.

This can be built as an `app` or as `bare` firmware.

By default it builds as an `bare` firmware which goes to top of FLASH and overwrites the stock bootloader(update mode won't work).

To build as an `app` enable `app` feature:
```sh
cargo build --release -F app
```

It copies `memory_app.x` to `memory.x`. If `app` feature is disabled it copies `memory_bare.x` to `memory.x`.
`memory.x` should not be modified by hand.

TODO: Instructions on getting bin from elf, packing into .atk file.

## License

My code is 'GPL-3.0-only'. The `link_ram.x` file is 'MIT OR Apache-2.0'.
This file is from https://github.com/embassy-rs/teleprobe/blob/main/link_ram_cortex_m.x
