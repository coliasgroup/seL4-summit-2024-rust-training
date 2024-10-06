<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Spawning a Thread

In this chapter, we will explore IPC through an example that spawns a secondary thread.
This example starts with a more interesting program than the others so far.
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

`interact_with_secondary_thread()` runs in the primary thread, whereas `secondary_thread_main()` runs in the secondary thread.

{{#step 6.A}}

Change `inter_thread_nfn: sel4::cap::Notification` to `inter_thread_ep: sel4::cap::Endpoint`.

Replace `inter_thread_nfn.signal()` and `inter_thread_nfn.wait()` with `inter_thread_ep.send()` and `inter_thread_ep.recv()`.

{{#step 6.B (exercise)}}

`TODO`

{{#step 6.C (exercise)}}

`TODO`
