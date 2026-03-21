# Building **NextStd**

## Prerequisites

You will need the following tools installed to build `NextStd`:

1. **Rust & Cargo** (For compiling the backend):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **GCC or Clang** (For the C front-end and macro expansion)
3. **Make** (For build automation)

## System-Wide Installation

To install NextStd directly to your system's `/usr/local` directories (allowing
you to use `#include <nextstd/ns.h>` globally), run:

```bash
sudo make install
```

To remove the library from your system, run:

```bash
sudo make uninstall
```

## Manual Compilation (Local)

If you want to build the Rust backend libraries locally without running a C
example or installing system-wide, use:

```bash
make rust
```

This generates the static (`.a`) libraries in `target/release`, making them
ready for C to link against.

## Project Architecture

`NextStd` uses a `cargo` workspace to separate core logic into modular crates,
and a unified `include/` directory to expose the C-API cleanly. Data structures
are separated into their own modules to prevent core header bloat.

```bash
.
├── _typos.toml
├── BUILD.md
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── cliff.toml
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── crates
│   ├── ns_data
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── hashmap.rs
│   │       ├── lib.rs
│   │       └── vec.rs
│   ├── ns_error
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   ├── ns_io
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── input.rs
│   │       ├── lib.rs
│   │       └── print.rs
│   └── ns_string
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── examples
│   ├── 01_print_integer.c
│   ├── 02_print_float_double.c
│   ├── 03_print_string.c
│   ├── 04_user_input.c
│   ├── 05_sso_strings.c
│   ├── 06_errors.c
│   ├── 07_colors.c
│   ├── 08_string_error.c
│   ├── 09_vectors.c
│   ├── 10_hashmap.c
│   ├── 11_underline.c
│   ├── 12_string_length.c
│   ├── 13_read_ns_string.c
│   └── 14_string_interpolation.c
├── include
│   ├── data_structures
│   │   ├── ns_hashmap.h
│   │   └── ns_vec.h
│   ├── ns.h
│   ├── ns_color.h
│   ├── ns_data.h
│   ├── ns_error.h
│   ├── ns_print.h
│   ├── ns_read.h
│   └── ns_string.h
├── LICENSE
├── Makefile
├── README.md
├── ROADMAP.md
├── rumdl.toml
└── USAGE.md
```
