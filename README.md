# NextStd (`ns`)

**NextStd** is a modern, type-safe alternative to C's standard library
(`<stdio.h>`, `<string.h>`, and standard data structures).

It is designed to provide the ergonomics of high-level languages—like automatic
type detection, memory safety, automatic heap scaling, and zero-allocation
string optimizations—while maintaining strict C compatibility. It achieves this
by using **Rust** as a robust, memory-safe backend while exposing a clean,
macro-driven **C API**.

View the [CHANGELOG.md](./CHANGELOG.md)

![version](https://img.shields.io/badge/version-0.2.0-blue)

## Features

- **Type-Safe Printing:** No more format specifiers (`%d`, `%s`, `%f`). The
  `ns_print()` macro automatically detects types at compile-time using C11
  `_Generic`.
- **Memory-Safe Strings:** The `ns_string` type uses Small String Optimization
  (SSO). Strings under 24 bytes are stack-allocated (zero heap fragmentation),
  while larger strings safely scale to the heap.
- **Safe Data Structures:** Modern, bounds-checked dynamic arrays (`ns_vec`)
  that automatically manage heap allocations and prevent out-of-bounds reads
  without sacrificing C macro execution speed.
- **Rust Backend:** The core I/O and memory logic is written in Rust,
  mathematically preventing buffer overflows, memory leaks, and segmentation
  faults.
- **Zero-Config Build:** A single Makefile handles building the Rust workspace,
  compiling the C examples, and deploying the library system-wide.

## Current Support

- [x] Integer Printing (`int`)
- [x] Float/Double Printing (`float` / `double`)
- [x] Standard C String Printing (`char*`)
- [x] Type-Safe SSO Strings (`ns_string`)
- [x] Separate `print` and `println` functions
- [x] Cargo Workspace Architecture
- [x] User Input via `ns_read()`
- [x] Custom Error types (`ns_error_t`)
- [x] Safe Dynamic Arrays (`ns_vec`)
- [ ] Printing Variables + Strings (Interpolation/Formatting)

## Installation (System-Wide)

NextStd can be installed directly to your system's `/usr/local` directories,
allowing you to use it in any C project globally. See [BUILD.md](./BUILD.md) for
prerequisite dependencies.

```bash
git clone [https://github.com/NextStd/NextStd.git](https://github.com/NextStd/NextStd.git)
cd NextStd
sudo make install
```

Once installed, you can include NextStd in any external C file:

```c
#include <nextstd/ns.h>
#include <nextstd/ns_color.h>
#include <nextstd/ns_data.h>
```

And compile your standalone projects by linking the compiled Rust archives:

```bash
gcc main.c -lns_data -lns_io -lns_string -lns_error -lpthread -ldl -lm -o my_app
```

*(To remove the library from your system, simply run `sudo make uninstall` from
the repository).*

## Quick Start (Local Examples)

If you just want to test the library without installing it globally, you can run
the built-in examples directly from the cloned repository.

**List all available examples:**

```bash
make list
```

**Run a specific example:**

To run a specific example (e.g., `01_print_integer`), use the command below (do
not include the `.c` extension):

```bash
make 01_print_integer
```

To see more detailed usage instructions, check out [USAGE.md](./USAGE.md).
