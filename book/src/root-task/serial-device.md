<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Using a Serial Device

In this chapter, we will build a root task that interacts with a serial device.
Start by navigating to and running this chapter's example, which, so far, doesn't do anything interesting.

```
cd workspaces/root-task/serial-device
make simulate
```

The module at {{#gh_link @-5 (workspaces/root-task/serial-device/src/)device.rs}} implements a higher-level interface over the serial device's MMIO registers, whose physical base address is:

{{#fragment_with_gh_link "rust,ignore" @-5 workspaces/root-task/serial-device/src/main.rs:16:16}}

Our first goal will be to map the serial device's MMIO registers into the root task's address space.

After that, we will set up the root task's access to the serial device's IRQ, whose value is:

{{#fragment_with_gh_link "rust,ignore" @-5 workspaces/root-task/serial-device/src/main.rs:18:18}}

Finally, we will implement a simple loop that echos serial input to serial output.

{{#step 5.A}}

First, add some familiar snippets that we will use for allocating CSlots and kernel objects:

{{#fragment_with_gh_link "rust,ignore" @5.A workspaces/root-task/serial-device/src/main.rs:34:43}}

{{#fragment_with_gh_link "rust,ignore" @5.A workspaces/root-task/serial-device/src/main.rs:22:27}}

{{#step 5.B (exercies)}}

`largest_kernel_ut` will be useful for allocating kernel objects whose backing physical addresses don't matter to us, but we must allocate the frame which contains the serial device's MMIO registers at a particular physicall address (`SERIAL_DEVICE_MMIO_PADDR`).
Furthermore, the seL4 API distinguishes between _general purpose untyped_ _device untyped_. General purpose untyped is backed by normal memory, and can be used to create any type of object. Device untyped is not backed by normal memory, and can only be used to create frames.
See the last two paragraphs of {{#manual_link #2.4 (Kernel Memory Allocation)}} for more information.
So, we must allocate the serial device MMIO frame from the initial device untyped that contains `SERIAL_DEVICE_MMIO_PADDR`.

**Exercice:** Identify this initial untyped in the `bootinfo`. We will need a corresponding `sel4::cap::Untyped` along with the untyped's physical address (or `sel4::UntypedDesc`, which contains the physical address) for the next step.


