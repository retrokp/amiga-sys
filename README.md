
# amiga-sys

Unofficial unsafe bindings for the Amiga (m68k) system libraries.

Tested on rustc 1.99.0-nightly (87e5904f5 2026-07-20) and cargo 1.99.0-nightly
(3efb1f477 2026-07-17).

Note: if you want to build high performance games or demos, have a look at
[amiga-rust](https://github.com/grahambates/amiga-rust).

## WARNING: this is an *experimental* crate built on top of *experimental* Rust m68k toolchain

It is likely that Rust programs developed for Amiga will crash or simply refuse to
compile, because many pieces are still missing:

 - the LLVM compiler has only experimental support for m68k: it may produce invalid code or
   crash during compilation because of [many LLVM bugs for m68k](https://github.com/llvm/llvm-project/issues?q=state%3Aopen%20label%3A%22backend%3Am68k%22)
 - there's [many Rust bugs for m68k](https://github.com/rust-lang/rust/issues?q=is%3Aissue%20label%3AO-motorola68k)
 - this crate isn't well tested: it may have bugs and corrupt memory

If you still want to try this out, then read on..

## Features

 - direct unsafe bindings to the Amiga system libraries (based on
   [Amiga NDK 3.2R4](https://www.hyperion-entertainment.com/index.php/downloads?view=details&file=126))
 - includes almost all definitions from NDK for Kickstart 1.0 (V30) to 3.2 (V47)
 - the bindings are unsafe: it is very easy to write a program which crashes
 - no dependency to the Amiga Native Development Kit (NDK): no dependency to the NDK headers
   or amiga.lib (note: these bindings have been generated from some of the NDK headers)
 - supports `no_std` (no dependency to `std` or `alloc`)
 - only cross-compiling for Amiga (no building on Amiga)
 - extra feature: a lazy developer who doesn't respond quickly to issues or pull requests

## Not supported

 - no start up code to handle launching programs from Workbench
 - no functions with variadic arguments: there's always a similar function available
   without variadic arguments (the replacement function's name usually ends with Args, List or A)
 - no amiga.lib functions: BeginIO(), CreatePort(), CreateTask(), NewList(), TimeDelay(), etc.
   These are implemented in [amiga-support](https://github.com/retrokp/amiga-support).
 - no alib_stdio: functions duplicating libc functionality: printf(), fgetc(), etc.
 - no debug.lib or ddebug.lib functions: KGetChar(), KPrintF(), DGetChar(), DPrintF(), etc.
 - no direct access to hardware
 - no third-party libraries or devices
 - no support for AmigaOS 4.0 or other derivatives, PowerPC or other non-m68k Amiga versions

## Special cases

 - callbacks called by the Amiga libraries have their arguments stored in CPU registers and
   should return a value in the CPU register d0. It's up to the callback function
   to use assembly code to access the register values. For an example, see the tester
   example `test_rawdofmt()`.
 - `FileInfoBlock` and `InfoData` should be aligned to 4 bytes, but they don't have
    alignment modifiers, because it would change the size of `AnchorPath` and `AChain`.
    Proper alignment can be achieved by wrapping `FileInfoBlock` and `InfoData`:
    `#[repr(C, align(4))] pub struct AlignedFileInfoBlock { data: FileInfoBlock }`

## Generating the bindings

The crate includes pregenerated bindings. If someone wants to regenerate them, follow these steps.
These steps have been tested with nightly-2026-07-20.

 - Copy [Amiga NDK 3.2R4](https://www.hyperion-entertainment.com/index.php/downloads?view=details&file=126)
   to the `amiga-sys` parent folder, so that it is next to the `amiga-sys` folder
 - Install [bindgen](https://crates.io/crates/bindgen) with its
   [requirements](https://rust-lang.github.io/rust-bindgen/requirements.html).
 - `cd amiga-sys`
 - Run `generate.sh`

## Building the examples

These steps have been tested on Ubuntu 24.04 LTS.

Prerequisites:

 - Rust nightly: `rustup install nightly` and `rustup component add rust-src`.
 - the `m68k-linux-gnu-ld` linker for [m68k-unknown-none-elf](https://doc.rust-lang.org/rustc/platform-support/m68k-unknown-none-elf.html#requirements) (qemu-user-static isn't needed)
 - [elf2hunk](https://github.com/BartmanAbyss/elf2hunk)

Building:

    cd examples/hello-cli
    cargo +nightly build --target m68k-unknown-none-elf --release
    elf2hunk target/m68k-unknown-none-elf/release/hello-cli target/m68k-unknown-none-elf/release/hello-cli.exe
    # Amiga executable: target/m68k-unknown-none-elf/release/hello-cli.exe

The release build is usually more successful than the debug build.

The examples have a `.cargo/config.toml` file to select the linker.

To build the `tester` example, follow the steps above by replacing `hello-cli` with `tester`.

## Typical build errors

 - `could not compile` .. `(signal: 5, SIGTRAP: trace/breakpoint trap)`
   - solution: [fix the compiler](https://github.com/rust-lang/rust/issues/139311)

## Amiga string handling

Often function string parameters are null-terminated. Sometimes they are not
null-terminated and the length is given as an extra parameter. Structs mostly use null-terminated
strings (`STRPTR`). Amiga uses the
[ECMA-94](http://amigadev.elowar.com/read/ADCD_2.1/Devices_Manual_guide/node0083.html) encoding,
which is the same as [ISO-8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1#History) and Latin1.

## Related

 - [amidos](https://github.com/retrokp/amidos): safe API to write CLI programs for Amiga m68k
 - [amiga-rust](https://github.com/grahambates/amiga-rust): direct access to hardware
 - [amiga-debug Visual Studio Code Extension](https://github.com/BartmanAbyss/vscode-amiga-debug/tree/master):
   C/C++ and build tools for Amiga

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-0BSD">0BSD license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
