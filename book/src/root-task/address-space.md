<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Modifying the Address Space

This chapter will focus on using the {{#manual_link [seL4 virtual address space (VSpace) APIs] #7}} to manipulate the root task's own address space.
Start by navigating to and running this chapter's example, which, so far, is empty.

```
cd workspaces/root-task/address-space
make simulate
```

In seL4, each thread is associated with a virtual address space (VSpace).
A VSpace is comprised of a tree of translation tables mapping virtual addresses into physical frames of memory.
VSpaces are represented by their root translation structures.
While the high-level virtual address space API concepts are architecture-independent, the specific kernel object types for a given architecture mirror that architecture's paging structures.
That said, all architectures share two object types:
- The top-level paging structure, called a VSpace, which is used to represent a virtual address space.
- Frames of physical memory. The set of possible frames sizes is architecture-dependant. See {{#manual_link #7.1.3 (Page)}}.

The root task is provided with capabilites for the objects comprising its own virtual address space.
The locations of these capabilities in the root task's CSpace is provided in the `BootInfo` struct.
Our goal for this chapter will be to create a frame object and experiment with mapping it into the root task's own address space.


