<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Hello, World!

Navigate to and run the hello world Microkit example:

```
cd workspaces/microkit/hello
make simulate
```

{{#fragment_with_gh_link "rust,ignore" @-9 (workspaces/microkit/)hello-world/pds/hello/src/main.rs:7:22}}

The Rust standard library is divided into three layers:
- [`core`](https://doc.rust-lang.org/core/): dependency-free foundation
- [`alloc`](https://doc.rust-lang.org/alloc/): implements heap-backed data structures, but requires a runtime that provides a heap allocator
- [`std`](https://doc.rust-lang.org/std/) includes `core` and `alloc`, and adds APIs that depend on OS services such as networking and filesystems

The high-level `std` doesn't support the low-level seL4 root task target.
[`#![no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html) declares that this crate does not depend on `std`, and prevents rustc from automatically importing it.

Our langauge runtime will handle the program's entrypoint differently than a typical Rust program.
[`#![no_main]`](https://doc.rust-lang.org/reference/crates-and-source-files.html#the-no_main-attribute) informs rustc of this fact.

The {{#rustdoc_link root-task sel4/index.html `sel4` crate}} binds the seL4 API.
It is generated from source (`.xml`, `.bf`, and `.h`) in `libsel4`.
We will cover the contents of this crate in future chapters.

The {{#rustdoc_link root-task sel4_root_task/index.html `sel4_root_task` crate}} implements a Rust language runtime for the root task environment.

The {{#rustdoc_link root-task sel4_root_task/attr.root_task.html `#[root_task]`}} attribute macro declares a function to be the root task's entrypoint.
The entrypoint function must have a signature of the form:

```rust,ignore
fn(&sel4::BootInfoPtr) -> T
where
    T: sel4_root_task::Termination
```

(Rustdoc for for {{#rustdoc_link root-task sel4/struct.BootInfoPtr.html `BootInfoPtr`}} and {{#rustdoc_link root-task sel4_root_task/trait.Termination.html `Termination`}})

The root task has no way to exit, so, to terminate cleanly, it must suspend its own thread.
`sel4::init_thread::suspend_self()` does exactly this.
