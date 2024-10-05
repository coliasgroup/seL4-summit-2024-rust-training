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

Take a look at {{#gh_link @-6 (workspaces/root-task/spawn-thread/src/)main.rs}}.
If you're used to using [seL4_libs](https://github.com/seL4/seL4_libs), you may notice that our Rust code here is much more low-level and verbose.
That is because we aren't using any higher-level seL4-related libraries such as 
[`<sel4utils/thread.h>`](https://github.com/seL4/seL4_libs/blob/master/libsel4utils/include/sel4utils/thread.h).
Our code is more like spawning a thread using `<sel4/sel4.h>` alone.

{{#step 6.A (exercise)}}

`TODO`

{{#step 6.B (exercise)}}

`TODO`

{{#step 6.C (exercise)}}

`TODO`
