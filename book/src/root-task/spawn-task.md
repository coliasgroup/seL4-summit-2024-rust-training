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

Right now, all the child task does is send a test message over an endpoint back to the root task.
The challenge in this chapter, [step 7.E](#step-7e-challenge), is to extend the root task so that it sets up the child task to be able to interact with the serial device, and to extend the child task to implement the same echo loop as in [./serial-device.html#step-5h].
Steps [7.A](#step-7a), [7.B](#step-7b), [7.C](#step-7c), and [7.D](#step-7d), which are not exercises, make some incremental extensions towards those goals to help you get started.

{{#step 7.A}}

This step extends the `ObjectAllocator` type in {{#gh_link @7.B workspaces/root-task/spawn-task/src/object_allocator.rs}} with the `recklessly_allocate_at()` method.
This method allocates an object according to the `blueprint` parameter at the given physical address `paddr`.
Instead of just allocating the object from the largest kernel untyped like the `allocate()` method does, this method searches through the bootinfo to find the initial untyped capability whose corresponding untyped object contains `paddr`, allocates dummy objects from this untyped object until its watermark reaches `paddr`, and then allocates the desired object.
`recklessly_allocate_at()`'s procedure is similar to that which we followed in [step 5.C](#step-5c).

This implementation is "reckless" because it modifies the state of the untyped capability it allocates from (allocating from it and changing its watermark) without keeping track of having done so.
So, subsequent calls for `paddr`s contained in the same initial untyped would fail or, worse, misbehave.
However, we expect to only need to call it once, so we are okay with this caveat.

In [step 7.E](#step-7e-challenge), you be able to use this method to allocate the serial device MMIO register frame.

{{#step 7.B}}

This step extends the `create_child_vspace()` function in {{#gh_link @7.A workspaces/root-task/spawn-task/src/child_vspace.rs}} to take an `extra_frames` parameter.
`create_child_vspace()` now maps these extra frames into the child task's address space, after the end of the program image, and after the IPC buffer frame.

In [step 7.E](#step-7e-challenge), you be able to use this parameter to pass in the serial device MMIO register frame to mapped into the child task's address space.

{{#step 7.C}}

This step simply copies the `Device` type from [chapter 5](./serial-device.html) into the child task.

In [step 7.E](#step-7e-challenge), you be able to use this type to interact with the serial device's MMIO registers, just like we as part of [step 5.E](./serial-device.html#step-5e-exercise).

{{#step 7.D}}

This step just adds the `SERIAL_DEVICE_MMIO_PADDR` and `SERIAL_DEVICE_IRQ` constants from [chapter 5](./serial-device.html) to the root task.

{{#step 7.E (challenge)}}

**Exercise:**
Extend the root task so that it sets up the child task to be able to interact with the serial device, and extend the child task to implement the same echo loop as in [./serial-device.html#step-5h-exercise].

<div class="step-hint">
    <details>
        <summary>
            Hint for the root task (click to expand)
        </summary>

Try following this sequence of sub-steps:
- Allocate `serial_device_frame_cap: sel4::cap::Granule` using `object_allocator.recklessly_allocate_at()`.
- Map `serial_device_frame_cap` into the child task's address space using `create_child_vspace()`'s `extra_frames` parameter.
- Similarly to how we did so in steps [5.F](./serial-device.html#step-5f-exercise) and [5.G](./serial-device.html#step-5g-exercise), obtain `irq_handler_cap: sel4::cap::IrqHandler` for `SERIAL_DEVICE_IRQ` (`object_allocator.allocate_slot()` might come in handy), allocate `irq_nfn_cap: sel4::cap::Notification`, and associate `irq_nfn_cap` with `SERIAL_DEVICE_IRQ` using `irq_handler_cap`.
- Copy `irq_handler_cap` and `irq_nfn_cap` into the child task's CSpace in a similar way to how `child_tcb` and `inter_task_ep` are copied.

    </details>
</div>

<p></p>

<div class="step-hint">
    <details>
        <summary>
            Hint for the child task (click to expand)
        </summary>

Try following this sequence of sub-steps:
- Declare constants `IRQ_HANDLER: sel4::cap::IrqHandler` and `IRQ_NFN: sel4::cap::Notification` after `OWN_TCB` and `INTRA_TASK_EP`.
- Obtain the virtual address of the serial device MMIO frame with `addr_of_page_beyond_image(1)` (recall how `create_child_vspace()`'s `extra_frames` parameter works).
- Initialize the serial device with `Device::new()` and `Device::init()` (as we did for part of [step 5.E](./serial-device.html#step-5e-exercise)), and use the serial device just like we did in [step 5.H](./serial-device.html#step-5h-exercise).

    </details>
</div>
