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

As was the case in [Chapter 6 (Spawning a Thread)](spawning-a-thread.html)

[Step 4.B](./address-space.html#step-4b)

, we will explore IPC through an example that spawns a secondary thread.
This example starts with a more interesting program than the other examples we've worked with so far.
It spawns a secondary thread, and then interacts with it using a notification.


{{#step 7.A}}

`TODO`

{{#step 7.B}}

`TODO`

{{#step 7.C}}

`TODO`

{{#step 7.D (challenge)}}

`TODO`
