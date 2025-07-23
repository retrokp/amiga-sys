# amiga-sys tester

This is a simple tester program for amiga-sys.

## Build

Run:

    cargo +nightly build --release
    elf2hunk target/m68k-bare-metal/release/tester.elf target/m68k-bare-metal/release/tester
