<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# The Root Task

After initializing itself, the seL4 kernel passes control to a userspace program called the root task, whose image is provided to the kernel at boot-time.
The root task's {{#manual_link [capability space] #3}} contains capabilities for all memory and boot-time objects.
That is to say, the root task spawns with the maximum priveledge that a userspace program can have.
The kernel provides the root task with a map of its initial capability space in the form of the {{#manual_link [BootInfo frame] #9.2}}.
You can read more about the root task's environment in {{#manual_link #9.1}}.

Part I of this training will focus on writing from-scratch root tasks in Rust.
Some of the code in Part I will be quite low-level, interacting with the language runtime, the linker, and some of the finer details related to bootstrapping an seL4-based system.
The only seL4-related crates we will be using are the language runtime and bindings for the seL4 API.
Unlike situations where higher-level seL4-related libraries (such as [seL4_libs](https://github.com/seL4/seL4_libs)) are used, we will be allocate objects and manage virtual address spaces from scratch.

At boot time, the seL4 kernel and root task are loaded into memory by a kind of bootloader stub referred to as a seL4 kernel loader.
For Part I, we will use the [kernel loader from the rust-sel4 project](https://github.com/seL4/rust-sel4/tree/v1.0.0/crates/sel4-kernel-loader).
We won't cover how it works or how to use it explicitly in this text.
It is built at {{#gh_link docker/Dockerfile:94:108}}, and used at {{#gh_link mk/root-task.mk:31:36}}.
