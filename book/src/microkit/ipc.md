<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# IPC

{{#step 10.A (exercise)}}

**Exercise:**

**Exercise (optional):**
Send something more interesting over IPC using
{{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes `msg_bytes`}}
and
{{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes_mut `msg_bytes_mut`}}
.
For example, the [`zerocopy` crate](https://docs.rs/zerocopy/latest/zerocopy/) can be used to view certain types as bytes, and the lightweight [`postcard` crate](https://docs.rs/postcard/latest/postcard/) can be used to serialize a wider range of types using [`serde`](https://serde.rs/).
