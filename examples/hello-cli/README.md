# amiga-sys hello cli example

This example shows how to print out to the Amiga command line interface (CLI).

## Build

Run:

    cargo +nightly build --release
    elf2hunk target/m68k-bare-metal/release/hello-cli.elf target/m68k-bare-metal/release/hello-cli
