# ns_io (NextStd I/O)

**ns** is a modern, type-safe alternative for C's standard `<stdio.h>`.

It is designed to provide the ergonomics of high-level languages (like
automatic type detection and safety) while maintaining C compatibility.
It achieves this by using **Rust** as a robust,
memory-safe backend while exposing a clean **C API**.

View the [CHANGELOG.md](./CHANGELOG.md)

> [!IMPORTANT]
> Currently you cannot add `ns` to your system i.e it cannot be imported as
below :
> `#include <ns.h>`
> For now just run the examples or add your own in the `examples/` directory.

## Features

- **Type-Safe Printing:** No more format specifiers (`%d`, `%s`).
The `ns_print()` macro automatically detects types using C11 `_Generic`.
- **Rust Backend:** The core logic is written in Rust, ensuring memory safety
and preventing buffer overflows in the implementation.
- **Zero-Config Build:** A single Makefile handles compiling Rust, linking C,
and running binaries.

### Current Support

- [x] Integer Printing (`int`)
- [x] Float/Double Printing (`float` / `double`)
- [x] String Printing
- [x] Type safe strings (`ns_string`)
- [x] Separate `print` and `println` functions for better printing
- [x] Create `cargo` workspaces for a more structured monorepo
- [ ] Printing Variables + String
- [x] User Input

---

## Prerequisites

You need the following tools installed to build **ns_io**:

1. **Rust & Cargo** (For the backend)

```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

1. **GCC** (For the C frontend)
2. **Make** (For build automation)

---

## Quick Start

### 1. Build the Library

You can build the Rust backend independently:

```bash
make rust
```

This generates the static and dynamic libraries in `target/release/`.

### 2. Run Examples

The project includes a set of examples in the `examples/` directory.
The `Makefile` is designed to compile and run them in one step.

**List all available examples:**

```bash
make list

```

**Run a specific example:**
Do not include the `.c` extension. Just use the filename.

```bash
make 01_print_integer

```

*Output:*

```text
42 -100 0 
42
-100
0
12345 12345
```

---

## Usage in Your Code

Include the header and link against the library.

```c
#include "ns.h"

int main() {
    int x = 42;
    
    // Automatically detects integer type
    ns_print(x); 
    
    return 0;
}

```

---

## Project Structure

```bash
nextstd
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
│   └── ns_math
├── examples
│   ├── 01_print_integer.c
│   ├── 02_print_float_double.c
│   ├── 03_print_string.c
│   └── 04_user_input.c
├── LICENSE
├── Makefile
├── ns.h
└── README.md
```
