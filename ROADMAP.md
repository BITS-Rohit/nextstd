# NextStd Roadmap

**NextStd** is evolving from a memory-safe wrapper for `<stdio.h>` and
`<string.h>` into a comprehensive, modern systems toolkit for C developers.

The goal of this roadmap is to eliminate the need for complex POSIX APIs,
dangerous `system()` calls, and verbose external networking libraries by
offloading the heavy lifting to our safe Rust backend.

## Phase 0: Completed Core Features

The foundational architecture and core modules are fully implemented and ready
for production use.

* [x] **Type-Safe Printing (`ns_io`):** `ns_print` and `ns_println` functions
      with `_Generic` routing for `int`, `float`, `double`, and `char*`.
* [x] **Safe User Input:** `ns_read()` macro for dynamically capturing terminal
      input without buffer overflows.
* [x] **Memory-Safe Strings (`ns_string`):** Small String Optimization (SSO),
      dynamic scaling, safe concatenation, and length tracking.
* [x] **Crash-Proof Control Flow (`ns_error`):** Python-style `NS_TRY` and
      `NS_EXCEPT` macros, null-pointer protection, and `ns_error_t` enums.
* [x] **Safe Data Structures (`ns_data`):** Bounds-checked dynamic arrays
      (`ns_vec`) and Key-Value Maps (`ns_map`).
* [x] **Build & Architecture:** Cargo workspace integration, Makefile
      automation, and system-wide installation logic.

---

## Phase 1: Core Enhancements (Backlog)

These features will finalize the core `ns_io` and `ns_string` modules.

* [ ] **String Interpolation & Formatting:** Introduce Python/Rust-style string
      formatting to eliminate the need for `sprintf`.
  * *Concept:* `ns_println("Value: {val}");`

## Phase 2: Process Execution (`ns_cmd` & `ns_process`)

Replacing standard C's `fork()`, `exec()`, and the highly insecure `system()`
calls with safe, memory-managed alternatives.

* [ ] **The Better `system()` (`ns_cmd`):** A high-level execution macro that
      prevents shell injection and captures output safely without POSIX pipes.
  * *Architecture:* Introduces an `ns_cmd_output` struct containing separated
    `ns_string stdout` and `ns_string stderr` fields. This allows developers to
    parse clean data from `stdout` while independently handling error logs from
    `stderr`.
* [ ] **Advanced Process Management (`ns_process`):** A wrapper around Rust's
      `std::process::Command` using a safe builder pattern.
  * *Features:* Ability to construct commands safely (`ns_process_add_arg`),
    spawn processes in the background, and manage process handles (e.g.,
    launching external media players like `mpv`).

## Phase 3: Safe File System (`ns_file`)

Fixing the resource leaks and buffer overflow risks associated with standard C's
`FILE*` API.

* [ ] **One-Shot File I/O:** High-level functions like `ns_file_read_to_string`
      that automatically open a file, calculate its size, dynamically allocate
      an `ns_string`, and safely close the handle.
* [ ] **Safe Streaming:** Opaque `ns_file_t` structs managed entirely by the
      Rust backend, allowing C to safely read large files in chunks without ever
      touching raw pointers or leaking descriptors.

## Phase 4: Networking (`ns_http`)

Bringing modern, memory-safe HTTPS requests to C without the massive boilerplate
of `libcurl`.

* [ ] **Simple GET Requests:** Wrapper around Rust's `reqwest` to easily fetch
      HTML or JSON APIs directly into dynamically sized `ns_string` structures.
* [ ] **Safe Error Propagation:** Catch dropped connections, 404s, or Cloudflare
      blocks via `NS_TRY` blocks instead of crashing the program.

---

## The Ultimate Stress Test: CLI Media Scraper

As these modules come online, they will be stress-tested by building a complex
interactive CLI application.

**Concept:** A terminal-based scraper for media sites.

1. Use `ns_io` to capture search queries safely.
2. Use `ns_http` to fetch and parse the streaming API data.
3. Use `ns_cmd` / `ns_process` to seamlessly pass the scraped `.m3u8` video
   links to an external player like `mpv` for direct terminal playback.
