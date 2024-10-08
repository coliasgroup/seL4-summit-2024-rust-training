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
It is paramterized by a capability type {{#rustdoc_link root-task sel4/trait.CapType.html `T: sel4::CapType`}}, representing the type of capability in that slot.
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

{{#step 3.C}}

In this step, we will allocate a {{#manual_link [Notification] #5}} object.

We already know how to refer to a capability in the current thread's CSpace: a CPtr.
However, some capability invocations allow us to refer to a capability slot in any CSpace whose root CNode is present in the current thread's CSpace.
In these cases, we must provide:
- A CPtr, interpreted in the context of the current thread's CSpace, which points to the target CSpace's root CNode
- A CPtr, interpreted in the context of the target CSpace, which points to the target capability slot
- A depth, which is the number of bits of the second CPtr to interpret. This allows for the second CPtr to point to a CNode. Why this is necessary is outside the scope of this training, but you can read about it in {{#manual_link #2.4 (CSpace Addressing)}}.

Consider, for example, the {{#manual_link [`seL4_CNode_Mint`] #10.3.1.4}} capability invocation.
`dest_*` and `src_*` are both capability slots addressed in this way.

This more flexible method of capability slot addressing is encapsulated in the {{#rustdoc_link root-task sel4/struct.AbsoluteCPtr.html `sel4::AbsoluteCPtr`}} type.

The {{#manual_link [`seL4_Untyped_Retype`] #10.3.9.1}} method is used for allocating new objects from an untyped object.
The `_service` parameter is the address of the untyped object as a normal CPtr.
`root`, `node_index`, and `node_depth` address, in the more flexible way outlined above, the destination CNode into which capabilities for the new objects will be inserted.
`node_offset` and `num_objects` specify a range of slots within the selected CNode for the new capabilites (and, simultaneously, the number of new objects that will be created).

`type` and `size_bits` specify the shape of the new object(s)
Note that `size_bits` is relevant for only certain object types (see {{#manual_link #2.4.2 (Summary of Object Sizes)}} for more information).
This shape information is encapsulated in the {{#rustdoc_link root-task sel4/enum.ObjectBlueprint.html `sel4::ObjectBlueprint`}} type.

Multiple kernel objects can be allocated from a single unytped object.
For each untyped object, the kernel maintains a watermark which tracks how much of the untyped object has been used up for object allocation.
`seL4_Untyped_Retype` aligns the watermark to the desired object type's size, and then advances it according to the object type size and number of objects.
This process is detailed in the fourth paragraph of {{#manual_link #2.4.1 (Reusing Memory)}}.

Let us now work towards calling {{#rustdoc_link root-task sel4/cap/type.Untyped.html#method.untyped_retype `sel4::cap::Untyped::untyped_retype()`}} on our previously acquired `largest_kernel_ut`.
We wish to allocate one notification object and insert a capability for it into a free slot in the current thread's own CSpace.
More precisely, we need a `sel4::AbsoluteCPtr` for the current thread's own CSpace's root CNode, and an index into that CNode for a free slot.

The CPtr for the initial thread's own CSpace root is a constant:

{{#fragment_with_gh_link "rust,ignore" @3.C workspaces/root-task/kernel-objects/src/main.rs:27:27}}

`bootinfo` can tell us about a range of empty slots in this CSpace.
We can leverage the fact that Rust's `Range<T>` type is an iterator for certain `T` to allocate slots in an ergonomic way:

{{#fragment_with_gh_link "rust,ignore" @3.C workspaces/root-task/kernel-objects/src/main.rs:29:33}}

The {{#rustdoc_link root-task sel4/cap/type.CNode.html#method.relative_self `sel4::cap::CNode::relative_self()`}}  method elaborates a `sel4::cap::Cnode` into a `sel4::AbsoluteCPtr`.
Interestingly, there are two ways to do this, but the current implementation is just to use a depth of zero.

Now we can invoke our untyped capability to allocate a notification object:

{{#fragment_with_gh_link "rust,ignore" @3.C workspaces/root-task/kernel-objects/src/main.rs:36:43}}

Now that we know that `notification_slot` contains a notification capability, we can cast it and get a {{#rustdoc_link root-task sel4/cap/type.Notification.html `sel4::cap::Notification`}}:

{{#fragment_with_gh_link "rust,ignore" @3.C workspaces/root-task/kernel-objects/src/main.rs:45:47}}

{{#step 3.D (exercise)}}

**Exercise:**: Use {{#rustdoc_link root-task sel4/cap/type.Notification.html#method.signal `sel4::cap::Notification::signal()`}} and {{#rustdoc_link root-task sel4/cap/type.Notification.html#method.wait `sel4::cap::Notification::wait()`}} to signal and then wait on the notification.

{{#step 3.E (exercise)}}

As described in {{#manual_link #5 (Notifications)}}, a notification capability can contain a word-sized mask called a badge.
When that capability is used to signal the notification, the notification's word-sized state is bit-wise `or`ed with the capability's badge.
A wait call on the notification returns and clears the notification's state, provided that a signal call has occurred since the last wait call.

{{#rustdoc_link root-task sel4/struct.AbsoluteCPtr.html#method.mint `sel4::AbsoluteCPtr::mint()`}} mints a new capability from an existing capability, updatings its access rights and badge.

**Exercise:**: Allocate a new empty slot in the current CNode.

A slot in the root task's CSpace (i.e. a value of type {{#rustdoc_link root-task sel4/init_thread/struct.Slot.html `sel4::init_thread::Slot`}}) can be turned into an `sel4::AbsoluteCPtr` using {{#rustdoc_link root-task sel4/cap/type.CNode.html#method.relative `sel4::CNode::relative()`}}:

{{#fragment_with_gh_link "rust,ignore" @3.E workspaces/root-task/kernel-objects/src/main.rs:63:63}}

**Exercise:**: Mint a capability based on the capability in `notification_slot` into your newly allocated slot. Use {{#rustdoc_link root-task sel4/struct.CapRights.html#method.all `sel4::CapRights::all()`}} for the `rights` parameter, and specify a non-zero badge value.

**Exercise:**: Signal the notification using your newly minted badged capability. Using the return value of `sel4::Notification::wait()`, compare the badge value it returns with the badge you used to mint the capability.

{{#step 3.F (exercise)}}

**Exercise:**: `sel4::CapRights::all()` is overly-permissive. Use the overly-restrictive `sel4::CapRights::none()` instead and watch the program fail.

{{#step 3.G (exercise)}}

**Exercise:**: Now use the minimum rights necessary for the program to run.
