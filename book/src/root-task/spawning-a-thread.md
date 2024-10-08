<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Spawning a Thread

In this chapter, we will explore IPC through an example that spawns a secondary thread.
This example starts with a more interesting program than the other examples we've worked with so far.
It spawns a secondary thread, and then interacts with it using a notification.

Navigate to and run the example:

```
cd workspaces/root-task/spawn-thread
make simulate
```

Explore {{#gh_link @-6 (workspaces/root-task/spawn-thread/src/)main.rs}} at will.
If you're used to using [seL4_libs](https://github.com/seL4/seL4_libs), you may notice that our Rust code here is much more low-level and verbose.
That is because we aren't using any higher-level seL4-related libraries such as 
[`<sel4utils/thread.h>`](https://github.com/seL4/seL4_libs/blob/master/libsel4utils/include/sel4utils/thread.h).
Our code is more like spawning a thread using `<sel4/sel4.h>` alone.

The exercises in this chapter are only concerned with the following two functions, which run concurrently:

{{#fragment_with_gh_link "rust,ignore" @-6 workspaces/root-task/spawn-thread/src/main.rs:75:79}}

{{#fragment_with_gh_link "rust,ignore" @-6 workspaces/root-task/spawn-thread/src/main.rs:81:85}}

`secondary_thread_main()` runs in the secondary thread, and `interact_with_secondary_thread()` runs in the secondary thread.

{{#step 6.A}}

In this step, we introduce IPC between `secondary_thread_main()` and `interact_with_secondary_thread` by changing

```rust
inter_thread_nfn: sel4::cap::Notification
```

to

```rust
inter_thread_ep: sel4::cap::Endpoint
```

Before, the two sides of the notification communicated via
{{#rustdoc_link root-task sel4/cap/type.Notification.html#method.signal `inter_thread_nfn.signal()`}}
and
{{#rustdoc_link root-task sel4/cap/type.Notification.html#method.wait `inter_thread_nfn.wait()`}}.

Now, communication over this IPC endpoint will happen by
{{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.send `inter_thread_ep.send()`}} and
{{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.recv `inter_thread_ep.recv()`}}
.
<!-- , as described in
{{#manual_link #4.2 (Endpoints)}}. -->

As described in {{#manual_link #4 (Message Passing (IPC))}} and {{#manual_link #4.2 (Endpoints)}}, a capability for an Endpoint object can be invoked with {{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.send `send()`}} and {{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.recv `recv()`}}.
`send()` and `recv()` block until the two sides of the object rendezvous, at which point a message is passed from the sender to the receiver.
This message may contain data and capabilities.

Each thread is associated with a special page of memory called an IPC buffer.
The sender populates its own IPC buffer with data and/or CPtrs, and then calls `send()` with a metadata value called the {{#rustdoc_link root-task sel4/struct.MessageInfo.html `MessageInfo`}}.
The kernel copies data from the sender's IPC buffer into that of the receiver, and/or capabilities from the sender's CSpace into that of the receiver.
The kernel uses the `MessageInfo` to determine how much data to copy between the two IPC buffers
(the {{#rustdoc_link root-task sel4/struct.MessageInfo.html#method.length `length`}} field)
and how many CPtrs to read from the IPC buffers for copying betwen CSpaces
(the
{{#rustdoc_link root-task sel4/struct.MessageInfo.html#method.caps_unwrapped `caps_unwrapped`}}
and
{{#rustdoc_link root-task sel4/struct.MessageInfo.html#method.extra_caps `extra_caps`}}
fields).
Finally, the kernel passes the `MessageInfo`, and control, to the receiver.

In Rust, the sender and receiver can interact with their own IPC buffer using
{{#rustdoc_link root-task sel4/fn.with_ipc_buffer.html `sel4::with_ipc_buffer`}}
and
{{#rustdoc_link root-task sel4/fn.with_ipc_buffer_mut.html `sel4::with_ipc_buffer_mut`}}.
Message data is held in the message registers (an array of machine words), which is a field of the IPC buffer (
    {{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_regs `msg_regs`}}
    and
    {{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_regs_mut `msg_regs_mut`}}
).
The `length` field of the `MessageInfo` specifies how many message registers will be copied into the receiver's IPC buffer.

Once can also view the message registers as an array of bytes using
    {{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes `msg_bytes`}}
    and
    {{#rustdoc_link root-task sel4/struct.IpcBuffer.html#method.msg_bytes_mut `msg_bytes_mut`}}.
In this case, one rounds the length of their message up to the nearest multiple of the machine word size when computing the message length for the `MessageInfo`.

The `MessageInfo` also includes a few bits of data called a {{#rustdoc_link root-task sel4/struct.MessageInfo.html#method.label `label`}} ({{#rustdoc_link root-task sel4/struct.MessageInfo.html#method.label_width `label_width`}} bits wide) that is not interpreted by the kernel.

In Rust, the {{#rustdoc_link root-task sel4/struct.MessageInfoBuilder.html `MessageInfoBuilder`}} type is a clear and concise way to construct a {{#rustdoc_link root-task sel4/struct.MessageInfo.html `MessageInfo`}}.

In Rust, to smooth out differences between the mixed-criticality and legacy kernel schedulers, {{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.recv `recv()`}} always takes a `reply_authority` argument.
Under legacy scheduler configurations, which is what this text uses, this argument can be of type `()`.

Taking all of this together, let's use our IPC endpoint to send an empty message:

{{#fragment_with_gh_link "rust,ignore" @6.A workspaces/root-task/spawn-thread/src/main.rs:75:79}}

{{#fragment_with_gh_link "rust,ignore" @6.A workspaces/root-task/spawn-thread/src/main.rs:81:87}}

{{#step 6.B (exercise)}}

**Exercise:** use
{{#rustdoc_link root-task sel4/fn.with_ipc_buffer.html `sel4::with_ipc_buffer`}}
and
{{#rustdoc_link root-task sel4/fn.with_ipc_buffer_mut.html `sel4::with_ipc_buffer_mut`}}
to send a message with some data.

{{#step 6.C (exercise)}}

See {{#manual_link #4.2.4 (Calling and Replying)}} for a description of the `seL4_Call` syscall ({{#rustdoc_link root-task sel4/cap/type.Endpoint.html#method.call `Endpoint::call()`}} in Rust).

**Exercise:** Change the

```rust
inter_thread_ep.send()
```

in `secondary_thread_main()` to

```rust
inter_thread_ep.call()
```

and modify `interact_with_secondary_thread()` to {{#rustdoc_link root-task sel4/fn.reply.html `sel4::reply`}} with a message.
