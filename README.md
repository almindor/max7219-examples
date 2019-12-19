# `max7219 examples with hifive1 board`

> Generated from a template for building Rust applications for HiFive1 boards

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.36 or a newer toolchain. e.g. `rustup default stable`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the RISC-V target. Run:

``` console
$ rustup target add riscv32imac-unknown-none-elf
```

- [RISC-V toolchain for SiFive boards](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz)

- Programmer software
  * HiFive1 Rev B: [Segger JLink software & documentation pack for Linux](https://www.segger.com/downloads/jlink/)
  * HiFive1: [OpenOCD from SiFive](https://static.dev.sifive.com/dev-tools/riscv-openocd-0.10.0-2019.02.0-x86_64-linux-ubuntu14.tar.gz) 

## Using the examples

1. If you have an old HiFive1 board, edit `Cargo.toml`:
replace `board-hifive1-revb` with `board-hifive1`.

2. Run the programmer software.
  * HiFive1 Rev B:
```sh
/path/to/JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333
```
  * HiFive1:
```sh
/path/to/openocd -f board/sifive-hifive1.cfg
```

3. Build a given example (main is just a placeholder)

``` console
$ cargo build --example display_spi
```

4. Run the example given that everything is connected correctly on the hw

``` console
$ cargo run --example display_spi
```

## License
Copyright 2017-2019 [Aleš Katona][ales]

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.

[ales]: https://github.com/almindor
