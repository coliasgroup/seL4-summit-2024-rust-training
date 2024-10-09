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

The example system XML file specifies two protection domains, with a channel between them:

{{#fragment_with_gh_link "xml" @-9 (workspaces/microkit/ipc/)ipc.system:7:20}}

{{#fragment_with_gh_link "rust,ignore" @-9 (workspaces/microkit/ipc/)pds/client/src/main.rs:12:32}}

{{#fragment_with_gh_link "rust,ignore" @-9 (workspaces/microkit/ipc/)pds/server/src/main.rs:12:39}}

```
TODO
- Channel
```

{{#step 10.A (exercise)}}

**Exercise:**
In the client's `notified()` handler, make a protected procedure call to the server using `SERVER.pp_call()`.
Handle the call in the server's `protected()` handler.
Include data in the message using `sel4_microkit::with_msg_regs{,_mut}`.

**Exercise (optional):**
Send something more interesting over IPC using
{{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes `msg_bytes`}}
and
{{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes_mut `msg_bytes_mut`}}
.
For example, the [`zerocopy` crate](https://docs.rs/zerocopy/latest/zerocopy/) can be used to view certain types as bytes and vice versa, and the lightweight [`postcard` crate](https://docs.rs/postcard/latest/postcard/) can be used to serialize and deserialize a wider range of types using [`serde`](https://serde.rs/).
