<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Hello, World!

Run the hello world example:

```
cd workspaces/root-task/hello
make simulate
```

Press `ctrl-a x` to exit QEMU.

Here is its source:

{{#fragment_with_gh_link "rust,ignore" @-1 (workspaces/root-task/)hello/src/main.rs:7}}

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

The root task has no way to exit, so, to terminate cleanly, it must suspend its own thread.
`sel4::init_thread::suspend_self()` does exactly this.

{{#step 1.A (exercise)}}

**Exercise:** Cause a panic.

{{#step 1.B (exercise)}}

**Exercise:** Catch the panic using {{#rustdoc_link root-task sel4_root_task/panicking/fn.catch_unwind.html `sel4_root_task::panicking::catch_unwind()`}}.

{{#step 1.C (exercise)}}

You can set a custom panic hook with {{#rustdoc_link root-task sel4_root_task/panicking/type.PanicHook.html `sel4_root_task::panicking::PanicHook`}}.
The default hook just prints the panic's `ExternalPanicInfo`.

**Exercise:** Set a custom panic hook.

{{#step 1.D (exercise)}}

**Exercise:** Cause a stack overflow.

{{#step 1.E (exercise)}}

The `#[root_task]` attribute macro accepts a named `stack_size` parameter, which can be any expression of type `usize` and whose value is interpreted as the root task's initial thread's stack size, in bytes.
For example:

```rust,ignore
#[root_task(stack_size = 13 * 37)]
```

The default stack size is {{#rustdoc_link root-task sel4_root_task/constant.DEFAULT_STACK_SIZE.html `sel4_root_task::DEFAULT_STACK_SIZE`}}.

**Exercise:** Adjust the root task's initial thread's stack size to prevent the stack overflow you just caused.

{{#step 1.F (exercise)}}

By default, the `sel4_root_task` runtime does not include a heap.
Any attempt to use the `alloc` crate will result in a link-time failure.

The `#[root_task]` attribute macro accepts a `heap_size` parameter, which can be any expression of type `usize` and whose value is interpreted as the root task's heap size, in bytes.
Note that `heap_size` must come after `stack_size` in the case where both are present.
For example:

```rust,ignore
#[root_task(heap_size = 0xf00d)]
```

or

```rust,ignore
#[root_task(stack_size = 13 * 37, heap_size = 0xf00d)]
```

**Exercise:** Add a heap and use it.

{{#step 1.G}}

The `sel4_logging` crate builds on top of the [log](https://docs.rs/log/latest/log/) crate to add utilities for initializing simple loggers in minimal environments, such as a seL4 root task.
This step demonstrates one way to initialize a logger using this crate:

{{#fragment_with_gh_link "rust,ignore" @1.G (workspaces/root-task/)hello/src/main.rs:17:20}}

{{#fragment_with_gh_link "rust,ignore" @1.G (workspaces/root-task/)hello/src/main.rs:41}}
