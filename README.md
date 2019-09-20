# An example project for Rust embedded

[![Build Status](https://travis-ci.org/korken89/rust-embedded-example-project.svg?branch=master)](https://travis-ci.org/korken89/rust-embedded-example-project)

## Aim

This is an example/template for how to structure embedded Rust projects to the firmware can still be tested, i.e. `cargo test` still works.

## Layout

There are 2 folders:

1. **binary**: Here all code that has hard dependencies on the hardware is located. `cargo test` will not work here.
2. **firmware**: The code here is only an `#![no_std]` library which can be tested with `cargo test`

### Testing

To check the firmware crate, go to the firmware crate and run `cargo test`

### Running

To flash and run the complete binary project, go to the binary crate and run `cargo run`

## Documentation

The `cargo doc --open` command works in both crates

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

