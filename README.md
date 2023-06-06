# AgIsoStack-rs

[![crates](https://img.shields.io/crates/v/AgIsoStack-rs.svg)](https://crates.io/crates/AgIsoStack-rs)
[![Documentation](https://img.shields.io/docsrs/AgIsoStack-rs.svg)](https://docs.rs/AgIsoStack-rs)
[![Crate License](https://img.shields.io/crates/l/AgIsoStack-rs.svg)](https://crates.io/crates/AgIsoStack-rs)
[![Dependency Status](https://deps.rs/repo/github/Thom-de-Jong/AgIsoStack-rs/status.svg)](https://deps.rs/repo/github/Thom-de-Jong/AgIsoStack-rs)

Rust bindings for the [AgIsoStack++](https://github.com/Open-Agriculture/AgIsoStack-plus-plus) library.

## Installation

### Windows

- Install and use the Windows GNU toolchain.
  - `rustup toolchain install stable-x86_64-pc-windows-gnu`
  - `rustup default stable-gnu`
- Use `AgIsoStack-rs` as a dependency

### Linux

- Install and use the Linux GNU toolchain.
  - `rustup toolchain install stable-x86_64-unknown-linux-gnu`
  - `rustup default stable-gnu`
- Use `AgIsoStack-rs` as a dependency

## To-Do list
- [ ] Support for Windows and Linux (Linux not tested)
- [ ] Wrap all data structures (NAME, CANFrame, etc.)
- [ ] Wrap the network manager
- [ ] Wrap the TP and ETP managers
- [ ] Add test cases for all systems
- [ ] ... More ...

## License / Terms of Usage

The source code of this project is licensed under the MIT/Apache-2.0 license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the MIT/Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/Thom-de-Jong/AgIsoStack-rs/issues) to contact me.
