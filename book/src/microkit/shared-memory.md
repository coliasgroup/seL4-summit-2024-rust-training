<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Shared Memory

This chapter covers interacting with shared memory from protection domains written in Rust.
Navigate to and run the example:

```
cd workspaces/microkit/shared-memory
make simulate
```

The example system description specifies two protection domains which share two memory regions:

{{#fragment_with_gh_link "xml" @-11 (workspaces/microkit/shared-memory/)shared-memory.system:7:28}}

The Microkit tool will inject memory region virtual addresses into protection domain images according to the `setvar_vaddr` attribute values.
For example, the virtual address of the mapping of `region_a` into the `client` protection domain will be injected into the `microkit-shared-memory-client.elf` image at the location specified by then `region_a_vaddr` symbol.

In the case of Rust, declaring a symbol that the Microkit tool can patch requires a bit more intentionality than in the C case.
The {{#rustdoc_link microkit sel4_microkit/macro.var.html `sel4_microkit::var!`}} macro is provided to declare such symbols.

The `var!` macro's {{#rustdoc_link microkit src/sel4_microkit_base/symbols.rs.html#55-67 implementation}} is just a few lines of code.
We want to express this symbol as a global variable that does not change at runtime, but which cannot be assumed to have the value we assign it at compile time, and which must not be optimized away.
The near-trivial
{{#rustdoc_link microkit sel4_immutable_cell/struct.ImmutableCell.html `sel4_immutable_cell::ImmutableCell`}} type encapsulates this pattern.
The `#[no_mangle]` attribute instructs the compiler to use the name of the variable as the name of the symbol.
This is the default in C, but not Rust.
We direct the compiler to put this symbol in the `.data` section with `#[link_section = ".data"]` to ensure that space is allocated for it in the ELF file itself, not just the program image it describes.

So far, the example protection domains just store pointers to the shared memory regions in their handler state:

{{#fragment_with_gh_link "rust,ignore" @-11 (workspaces/microkit/shared-memory/)pds/client/src/main.rs:14:36}}

{{#fragment_with_gh_link "rust,ignore" @-11 (workspaces/microkit/shared-memory/)pds/server/src/main.rs:14:36}}

{{#step 10.A (exercise)}}

**Exercise:**
In the client's `notified()` handler, make a protected procedure call to the server using `SERVER.pp_call()`.
Handle the call in the server's `protected()` handler.
Include data in the message using `sel4_microkit::with_msg_regs{,_mut}`.

**Exercise (optional):**
Send something more interesting over IPC using
{{#rustdoc_link microkit sel4/struct.IpcBuffer.html#method.msg_bytes `msg_bytes`}}
and
{{#rustdoc_link microkit sel4/struct.IpcBuffer.html#method.msg_bytes_mut `msg_bytes_mut`}}
.
For example, the [`zerocopy` crate](https://docs.rs/zerocopy/latest/zerocopy/) can be used to view certain types as bytes and vice versa, and the lightweight [`postcard` crate](https://docs.rs/postcard/latest/postcard/) can be used to serialize and deserialize a wider range of types using [`serde`](https://serde.rs/).
