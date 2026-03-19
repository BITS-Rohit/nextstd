# Contributing to NextStd

First off, thank you for considering contributing to NextStd! It's people like
you that make memory-safe C a reality.

NextStd is built on a dual-language architecture: a lightweight, macro-driven
**C11** front-end and a highly secure, memory-safe **Rust** back-end. Whether
you are a C veteran or a Rustacean, there is a place for you here.

## 🧠 Architecture Overview

Before you write any code, it helps to understand how NextStd works:

1. **The C Front-End (`include/`):** Contains the C headers and macros (like
   `ns_vec_get` or `NS_TRY`). We use C11 `_Generic` macros for type routing to
   keep the API clean.
2. **The Rust Back-End (`crates/`):** Contains the core logic. This is where
   memory is allocated, bounds are checked, and I/O is safely executed.
3. **The FFI Boundary:** C macros call the `extern "C"` functions exposed by the
   Rust backend. If an error occurs, Rust catches it and returns an `ns_error_t`
   code instead of crashing.

## 🛠️ Development Environment Setup

To start developing, you need `gcc` (or `clang`), `make`, and `cargo`.

1. Fork and clone the repository.
2. Build the Rust back-end locally:

   ```bash
   make rust
   ```

3. Test your changes by writing a C file in the `examples/` directory and
   running it:

   ```bash
   make <example_name_without_extension>
   ```

   *(Note: Do not run `sudo make install` on your development branch, or you
   will overwrite your system's stable NextStd installation!)*

## 📝 Coding Guidelines

### C Code (Headers & Macros)

- **C11 Standard:** We strictly target C11 to ensure `_Generic` macro support.
- **No Format Strings:** Never introduce standard `printf` or `scanf` into the
  library. Always route through the safe Rust backend.
- **Hot-Path Macros:** For performance-critical data structures (like Vectors),
  try to handle bounds checking via C macros to avoid unnecessary FFI overhead.

### Rust Code (The Backend)

- **Minimize `unsafe`:** The whole point of this library is safety. Keep your
  `unsafe` FFI blocks as small and isolated as possible.
- **Error Handling:** Never `panic!` or `unwrap()` in the Rust backend. If a C
  pointer is null or an index is out of bounds, return the appropriate `NsError`
  enum variant.
- **Adding Errors:** If you create a new error state, add it to the `NsError`
  enum in `ns_error` and update the `ns_error_message` C-string translator.

## 🚀 Submitting a Pull Request

1. **Create a branch:** `git checkout -b feat/my-new-module` or
   `git switch -c feat/my-new-module`
2. **Make your changes:** Keep commits focused and descriptive.
3. **Update Documentation:** If you add a new macro or module, update the
   `mdbook` files in the `docs` Repository.
4. **Open a PR:** Describe what you changed, why you changed it, and how to test
   it.

We review all PRs and are happy to help you work through FFI bugs if you get
stuck!
