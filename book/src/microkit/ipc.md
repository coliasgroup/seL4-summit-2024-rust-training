<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# IPC

This chapter covers making and handling protected procedure calls in protection domains written in Rust.
Navigate to and run the example:

```
cd workspaces/microkit/ipc
make simulate
```

The example system description specifies two protection domains, with a channel between them:

{{#fragment_with_gh_link "xml" @-10 (workspaces/microkit/ipc/)ipc.system:7:20}}

{{#fragment_with_gh_link "rust,ignore" @-10 (workspaces/microkit/ipc/)pds/client/src/main.rs:12:32}}

{{#fragment_with_gh_link "rust,ignore" @-10 (workspaces/microkit/ipc/)pds/server/src/main.rs:12:39}}

The
{{#rustdoc_link microkit sel4_microkit/struct.Channel.html `Channel`}} type is the Rust equivalent of the
[microkit_channel](https://github.com/seL4/microkit/blob/b8cf3094ba08b37198b1943ec832c3a1168f4409/libmicrokit/include/microkit.h#L14C22-L14C38)
type alias in `libmicrokit`.
Note how the functionality corresponding to `libmicrokit`'s `microkit_notify`, `microkit_irq_ack`, and `microkit_ppcall` is implemented in methods for `Channel`.

The
{{#rustdoc_link microkit sel4_microkit/struct.MessageInfo.html `MessageInfo`}} type is the Rust equivalent of the
[microkit_msginfo](https://github.com/seL4/microkit/blob/b8cf3094ba08b37198b1943ec832c3a1168f4409/libmicrokit/include/microkit.h#L16C28-L16C44)
type alias in `libmicrokit`.
Just as `microkit_msginfo` is an alias for `seL4_MessageInfo_t`, {{#rustdoc_link microkit sel4_microkit/struct.MessageInfo.html `sel4_microkit::MessageInfo`}} is just a thin wrapper around {{#rustdoc_link microkit sel4/struct.MessageInfo.html `MessageInfo`}}.

`libmicrokit` has `microkit_mr_set()` and `microkit_mr_get()` for interacting with the IPC buffer.
In the `sel4_microkit` crate, we have
{{#rustdoc_link microkit sel4_microkit/fn.get_mr.html `get_mr()`}}
and
{{#rustdoc_link microkit sel4_microkit/fn.set_mr.html `set_mr()`}}
,
but we also have 
{{#rustdoc_link microkit sel4_microkit/fn.with_msg_regs.html `with_msg_regs()`}},
{{#rustdoc_link microkit sel4_microkit/fn.with_msg_regs_mut.html `with_msg_regs_mut()`}},
{{#rustdoc_link microkit sel4_microkit/fn.with_msg_bytes.html `with_msg_bytes()`}}, and
{{#rustdoc_link microkit sel4_microkit/fn.with_msg_bytes_mut.html `with_msg_bytes_mut()`}},
which use
{{#rustdoc_link microkit sel4/fn.with_ipc_buffer.html `sel4::with_ipc_buffer()`}} and
{{#rustdoc_link microkit sel4/fn.with_ipc_buffer_mut.html `sel4::with_ipc_buffer_mut()`}} under the hood.

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
