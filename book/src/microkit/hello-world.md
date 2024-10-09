<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Hello, World!

Navigate to and run the hello world Microkit example:

```
cd workspaces/microkit/hello-world
make simulate
```

Here is its source:

{{#fragment_with_gh_link "rust,ignore" @-9 (workspaces/microkit/)hello-world/pds/hello/src/main.rs:7:22}}

The {{#rustdoc_link microkit sel4_microkit/index.html `sel4_microkit` crate}} implements a Rust language runtime for Microkit protection domains, and implements the Microkit API.
It is written in pure Rust, and does not link against [`libmicrokit`](https://github.com/seL4/microkit/blob/main/libmicrokit).

### The Event Handler

The {{#rustdoc_link microkit sel4_microkit/attr.protection_domain.html `#[protection_domain]`}} attribute macro declares a function to be the protection domain's initialization function.
The entrypoint function must have a signature of the form:

```rust,ignore
fn() -> T
where
    T: sel4_microkit::Handler
```

An implementation of the {{#rustdoc_link microkit sel4_microkit/trait.Handler.html `Handler`}} trait is used by the runtime as the event handler for the protection domain's main loop.
The
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.notified `notified`}},
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.protected `protected`}},
and
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.fault `fault`}}
methods correspond to their equivalents in [`<microkit.h>`](https://github.com/seL4/microkit/blob/main/libmicrokit/include/microkit.h).

The default implementations of these methods just panic.
Our event handler, which we've called `HandlerImpl`, is the simplest possible `Handler` implementation.

### Language Runtime

As detailed in its rustdoc, the {{#rustdoc_link microkit sel4_microkit/attr.protection_domain.html `#[protection_domain]`}} attribute macro takes the same parameters as {{#rustdoc_link root-task sel4_root_task/attr.root_task.html `#[root_task]`}}.
Furthermore, the {{#rustdoc_link microkit sel4_microkit/index.html `sel4_microkit`}} crate supports all of the same Rust language runtime elements that we explored in [Chapter 2](../root-task/hello-world.html), including
{{#rustdoc_link microkit sel4_microkit/panicking/fn.catch_unwind.html `sel4_microkit::panicking::catch_unwind()`}}
and
{{#rustdoc_link microkit sel4_microkit/panicking/fn.set_hook.html `sel4_microkit::panicking::set_hook()`}}
.
