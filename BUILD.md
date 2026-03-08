# Building **NextStd**

## Prerequisites

You will need the following tools installed to build `NextStd` :

1. **Rust & Cargo** (For compiling the backend) :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. **GCC** (For the C front-end and macro expansion)
2. **Make** (For build automation)

## Manual Compilation

If you want to build the Rust backend libraries without running a C example,
use :

```bash
make rust
```

This generates the static (`.a`) and dynamic (`.so`) libraries in the `target/release`,
making then ready for C to link against

## Project Architecture

`NextStd` uses a `cargo` workspace to separate core logic into modular crates,
and a unified `include/` directory to expose the C-API cleanly.

```bash
.
├── BUILD.md
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── changelog.sh
├── cliff.toml
├── crates
│   ├── ns_data
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
│   └── 05_sso_strings.c
├── include
│   ├── ns.h
│   ├── ns_print.h
│   ├── ns_read.h
│   └── ns_string.h
├── LICENSE
├── Makefile
└── README.md
```
