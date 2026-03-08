# NextStd (`ns`)

**NextStd** is a modern, type-safe alternative to C's standard `<stdio.h>` and `<string.h>`.

It is designed to provide the ergonomics of high-level languages—like
automatic type detection, memory safety, and zero-allocation string
optimizations—while maintaining strict C compatibility. It achieves this
by using **Rust** as a robust, memory-safe backend while exposing a clean,
macro-driven **C API**.

View the [CHANGELOG.md](./CHANGELOG.md)

![version](https://img.shields.io/badge/version-0.1.6-blue)

> [!IMPORTANT]  
> NextStd is currently in active development. You cannot install it globally
to your system yet (e.g., `#include <ns.h>`). For now, clone the repository
and run the examples in the `examples/` directory.

## Features

- **Type-Safe Printing:** No more format specifiers (`%d`, `%s`, `%f`).
The `ns_print()` macro automatically detects types at compile-time using C11 `_Generic`.
- **Memory-Safe Strings:** The `ns_string` type uses Small String Optimization
(SSO). Strings under 24 bytes are stack-allocated (zero heap fragmentation)
, while larger strings safely scale to the heap.
- **Rust Backend:** The core I/O and memory logic is written in Rust,
mathematically preventing buffer overflows and segmentation faults.
- **Zero-Config Build:** A single Makefile handles building the Rust workspace,
compiling the C examples, and linking everything together.

## Current Support

- [x] Integer Printing (`int`)
- [x] Float/Double Printing (`float` / `double`)
- [x] Standard C String Printing (`char*`)
- [x] Type-Safe SSO Strings (`ns_string`)
- [x] Separate `print` and `println` functions
- [x] Cargo Workspace Architecture
- [x] User Input via `ns_read()`
- [ ] Printing Variables + Strings (Interpolation/Formatting)

## Quick Start

The quickest way to see NextStd in action is to
run the built-in examples.
See [BUILD.md](./BUILD.md) for prerequisite dependencies.

**List all available examples:**

```bash
make list
```

### Run a specific example

To run a specific example, for eg: 01_print_integer use the below command

```bash
make 01_print_integer
```

The example file name can be taken from the output of `make list`

To see more detailed usage instructions, check out [USAGE.md](./USAGE.md)
