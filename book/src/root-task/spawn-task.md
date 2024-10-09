<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Spawning a Task (Challenge)

This final chapter of Part I contains a more open-ended and challenging exercise.
We start with an example that spawns an entire new process, which, in the context of low-level seL4 userspace, is often called a _task_:

```
cd workspaces/root-task/spawn-task
make simulate
```

Similarly to what we saw in [Chapter 6 (Spawning a Thread)](spawn-thread.html), the code in this example is more low-level and complex compared to what you have seen in code that leverages [`<sel4utils/process.h>`](https://github.com/seL4/seL4_libs/blob/master/libsel4utils/include/sel4utils/process.h).
Again, our code here is more like spawning a task using `<sel4/sel4.h>` alone.

[Step 4.B](./address-space.html#step-4b)

{{#step 7.A}}

`TODO`

{{#step 7.B}}

`TODO`

{{#step 7.C}}

`TODO`

{{#step 7.D (challenge)}}

`TODO`
