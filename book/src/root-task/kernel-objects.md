<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Allocating Kernel Objects

Start by navigating to and running this chapter's example, which, so far, is empty.

```
cd workspaces/root-task/kernel-objects
make simulate
```

Userspace operates the seL4 kernel by creating, managing, and accessing kernel objects via references called {{#manual_link [_capabilites_] #2.1}}.
See {{#manual_link #2.3 (Kernel Objects)}} for an outline of the core types of kernel objects.

In the context of seL4, a capability is a granular, unforgeable token of authority that references a kernel object and carries access rights that limit what the user of a cabability can do with the referenced kernel object.
In general, a system call in seL4 amounts to refering to a cabability and an action on the kernel object the cabability points to.
This is called _invoking a capability_.

<!-- As elaborated in {{#manual_link #2.3 (Kernel Objects)}}, the core kernel object types are:
- CNode: a table containing capabilites, used to construct so-called capability spaces, which are assigned to threads. When making a syscall, a thread refers to capabilities in its capability space to in turn refer to kernel objects.
- Thread control block: the state associated with a thread's execution context. Used to control a thread.
- Scheduling context (only relevent to certain kernel configurations, not covered here): represents access to a CPU, used for scheduling.
- Endpoint: synchronous IPC endpoint, for message-passing between threads.
- Reply object (only relevent to certain kernel configurations, not covered here): tracks scheduling information accross IPC endpoint calls.
- Notification object: asynchronous signalling mechanism. -->

Just as each thread is associated with a virtual address space which the hardware uses to translate virtual memory addresses into locations in physical memory, each seL4 thread is also assocated with a {{#manual_link [capability space] #3}} (_CSpace_) which the kernel uses to translate so-called _capability pointers_ (_CPointers_ or _CPtrs_) into locations (slots) in the kernel's capability tables called _CNodes_.

The {{#rustdoc_link root-task sel4/struct.CPtr.html `sel4::CPtr`}} type is a wrapper around a machine word.
The {{#rustdoc_link root-task sel4/struct.Cap.html `sel4::Cap<T>`}} type wraps a `CPtr` associated with the current thread's CSpace, and thus points to a particular capability slot within the kernel.
It is paramterized by a capability type `T`, representing the type of capability in that slot.
It is up to the crate user to ensure that `Cap`'s are valid and well-typed in the current thread.

The {{#rustdoc_link root-task sel4/cap/index.html `sel4::cap`}} module contains aliases of the form `sel4::Cap<sel4::cap_type::*>`.

Depending on the `Cap`'s type `T`, a `Cap` has a number of methods available, which correspond to invocations for that capability type.
For example, {{#rustdoc_link root-task sel4/struct.Cap.html#method.tcb_resume `sel4::Cap::tcb_resume()`}} is available when `T` is {{#rustdoc_link root-task sel4/cap_type/struct.Tcb.html `sel4::cap_type::Tcb`}}.

{{#step 3.A}}

As elaborated in {{#manual_link #2.4 (Kernel Memory Allocation)}}, userspace is responsible for managing the memory associated with kernel objects.
_Untyped memory_ is the name of the object type for free memory.
At boot time, the root task is granted untyped memory capabilities for all of the system's memory, except for the memory used for the kernel and the root task's own resources.

Print the untyped memory described by the {{#manual_link [BootInfo frame] #9.2}}:

{{#fragment_with_gh_link "rust,ignore" @3.A workspaces/root-task/kernel-objects/src/main.rs:14:22}}

Sample output:

```
untyped:
    paddr: 0x00000000000000, size bits: 27, is device: true
    paddr: 0x00000008001000, size bits: 12, is device: true
    paddr: 0x00000008002000, size bits: 13, is device: true
    paddr: 0x00000008004000, size bits: 14, is device: true
    paddr: 0x00000008008000, size bits: 15, is device: true
    paddr: 0x00000008011000, size bits: 12, is device: true
    paddr: 0x00000008012000, size bits: 13, is device: true
    ...
```

{{#rustdoc_link root-task sel4/struct.UntypedDesc.html#method.size_bits `sel4::UntypedDesc::size_bits()`}} returns the log base 2 of the size of the region of memory described by the descriptor. 

{{#rustdoc_link root-task sel4/struct.UntypedDesc.html#method.is_device `sel4::UntypedDesc::is_device()`}} returns whether the region is device memory or kernel memory.
Device memory can only be used for creating frames, whereas kernel memory has no such restrictions.

{{#step 3.B}}

Add a function that finds the largest untyped region passed to the root task:

{{#fragment_with_gh_link "rust,ignore" @3.B workspaces/root-task/kernel-objects/src/main.rs:32:42}}

The expression `bootinfo.untyped().index(ut_ix).cap()` indexes into `bootinfo.untyped(): SlotRegion<Untyped>` to retrieve a `Slot<Untyped>`, which can be turned into a `Cap<Untyped> (= cap::Untyped)`.
