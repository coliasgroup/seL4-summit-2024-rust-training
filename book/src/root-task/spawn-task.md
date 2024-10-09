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

This example consists of two programs.
The {{#gh_link [`spawn-task`] @-7 workspaces/root-task/spawn-task/src/main.rs}} crate is the root task, and the {{#gh_link [`spawn-task-child`] @-7 workspaces/root-task/spawn-task/child/src/main.rs}} crate is the child task.

The child task does not spawn in any standard sort of environment, so is includes its own ad-hoc Rust language runtime in {{#gh_link @-7 (workspaces/root-task/spawn-task/)child/src/runtime.rs}}, complete with thread-local storage, a global heap allocator, and exception handling.
This runtime is built using a few Rust langauge runtime building block crates:
- {{#rustdoc_link root-task sel4_runtime_common/index.html `sel4-runtime-common`}}
- {{#rustdoc_link root-task sel4_initialize_tls/index.html `sel4-initialize-tls`}}
- {{#rustdoc_link root-task sel4_dlmalloc/index.html `sel4-dlmalloc`}}
- {{#rustdoc_link root-task sel4_stack/index.html `sel4-stack`}}
- {{#rustdoc_link root-task sel4_panicking/index.html `sel4-panicking`}}
- {{#rustdoc_link root-task sel4_panicking_env/index.html `sel4-panicking-env`}}

This minimal, ad-hoc language runtime is a neat, instructive piece of code.
If you are interested in learning more about building a new Rust language runtime out of the building blocks provided by the [rust-sel4](https://github.com/seL4/rust-sel4) project, let the instructor know.

Explore the {{#gh_link [root task] @-7 workspaces/root-task/spawn-task/src}} and {{#gh_link [child task] @-7 workspaces/root-task/spawn-task/child/src}} at will.
Let the instructor know if you would like to discuss any particular aspect of it.

[Step 4.B](./address-space.html#step-4b)

{{#step 7.A}}

`TODO`

{{#step 7.B}}

`TODO`

{{#step 7.C}}

`TODO`

{{#step 7.D (challenge)}}

`TODO`
