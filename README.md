# Crustabri_ipafair

Crustabri_ipafair provides an IPAFAIR-compatible library built upon the Crustabri solver.
It was ranked first for the three tracks of the [ICCMA'23 competition](https://iccma2023.github.io/).

Compiling the project with `cargo build --release` produces the library `libcrustabri_ipafair.so` in the `target/release` directory.
See the [rust-lang.org dedicated page](https://www.rust-lang.org/tools/install) if you need to install the Rust toolchain (which includes `cargo`).

## Requirements

Crustabri_ipafair relies on on `bindgen` to generate a C-compatible library, which itself relies on `Clang`.
See the requirements on the [bindgen documentation](https://rust-lang.github.io/rust-bindgen/) to install `Clang` if you issue compilation errors.

## Resources

The `resources` directory contains an archive with the material needed to replay the ICCMA'23 competition with the submitted solver and with alternative algorithms.
Some minor changes have been since the competition in order to make the project (library + Python scripts) compatible with more operating systems.

## License

Crustabri is developed at CRIL (Univ. Artois & CNRS).
It is made available under the terms of the GNU GPLv3 license.
