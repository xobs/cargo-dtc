# Devicetree Compiler for Rust

Wrap the `dtc` source code in Rust to make it easier to build for non-Posix platforms.

The binary is named `cargo-dtc` to avoid conflicts with the original `dtc` binary.

## Licensing

This project wraps the GPL2-licensed `dtc` code. As such, this project is also licensed under GPL2.

## Development Process

Generated files (namely the `flex` and `bison` grammar files) were generated and the resulting file copied to `dtc`. Original files may be consulted from the original source.

Any non-Posix code was adapted to build under more strict compilers.

The `main()` was renamed to `dtc_main()` and is wrapped by Rust.

## Changes from `dtc`

The original version of dtc is available as a submodule in the `dtc-orig` directory. The code in this directory is unused, and it serves to provide a reference for the changes made to the original code.

The following changes were made:

* The directory structure was reorganized to be more idiomatic for Rust.
* Preprocessed versions of `dtc-lexer.lex.l` and `dtc-parser.tab.y` were copied to `dtc/src/` and are used to avoid needing to install `flex` and `bison`.
* To work around `__VA_ARGS__` in MSVC, the `CHECK_ENTRY()` macro inserts a `NULL` element at the start of the array. `checks.c` was modified to ignore the first element.
* The ternary operator in `treesource.c` was modified to be C99 compliant.
* The `main()` function was renamed to `dtc_main()` and is wrapped by Rust.
* `strcasecmp` is renamed to `_stricmp` inside `dtc.c` via a macro.
* On Windows, `getopt.c` was taken from <https://github.com/Chunde/getopt-for-windows> as a drop-in replacement for the one from glibc.
* On Windows,  `dirent.h` was taken from <https://github.com/tronkko/dirent>.
* On Windows, a fake `unistd.h` was added that renames `main` and chain-includes `dirent.h` to define `S_ISDIR` and `S_ISREG`.
* On other platforms, `build.rs` performs the rename of `main`.
* A fake `version_gen.h` gets generated indicating it comes from Rust.
