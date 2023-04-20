# Crustabri_ipafair

Crustabri_ipafair provided a version of Crustabri for dynamic Argumentation Frameworks thanks to the IPAFAIR API.

## Compiling

Compiling Crustabri_ipafair requires a recent version of the Rust toolchain; see the [get started page](https://www.rust-lang.org/learn/get-started) on rust-lang.org.
Once installed, a call to `cargo build --release` should be sufficient to download the dependencies and to build the binaries in the `target/release` directory.
Note that compiling on the machine on which Crustabri_ipafair will be launched (or at least on an identical system) may prevent some compatibility issues.

At this time, **Crustabri_ipafair requires the presence of the `crustabri` and `ipafair-sys` crates** in the parent folder of this file.
This will change when the crates will be available from `crates.io`.

The `cargo` tool will download and compile the dependencies, including the CaDiCaL SAT solver.
This means that the machine on which Crustabri_ipafair is built requires an internet connection and a recent C/C++ compiler.
In case a network connection is not available on the machine, `cargo vendor` should be the solution.
A call to this command should download the dependencies, put them in the `vendor` directory, and give the content of a `.cargo/config.toml` file.
Putting both the `vendor` directory and the populated `.cargo/config.toml` file in the root folder of Crustabri should be sufficient to prevent the use of the internet connection.
This procedure is required **for all three crates**.

Compiling the project produces the library `libcrustabri_ipafair.so` in the `target/release` directory.

## License

Crustabri is developed at CRIL (Univ. Artois & CNRS).
It is made available under the terms of the GNU GPLv3 license.
