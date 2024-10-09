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

The {{#rustdoc_link microkit sel4_microkit/index.html `sel4_microkit` crate}} implements a Rust language runtime for Microkit protection domains, and implements the Microkit API.

The {{#rustdoc_link microkit sel4_microkit/attr.protection_domain.html `#[protection_domain]`}} attribute macro declares a function to be the protection domain's initialization function.
The entrypoint function must have a signature of the form:

```rust,ignore
fn() -> T
where
    T: sel4_microkit::Handler
```

An implementation of the {{#rustdoc_link microkit sel4_microkit/trait.Handler.html `Handler`}} trait is used as the event handler for the protection domain's main loop.
The
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.notified `notified`}},
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.protected `protected`}},
and
{{#rustdoc_link microkit sel4_microkit/trait.Handler.html#method.fault `fault`}}