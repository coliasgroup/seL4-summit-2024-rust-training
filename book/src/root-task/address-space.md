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
  In the {{#rustdoc_link root-task sel4/index.html `sel4` crate}}, the corresponding {{#rustdoc_link root-task sel4/trait.CapType.html `sel4::CapType`}} is {{#rustdoc_link root-task sel4/cap_type/struct.VSpace.html `sel4::cap_type::VSpace`}} (note that, for some architectures, this is a type alias).
- Frames of physical memory. The set of possible frames sizes is architecture-dependant.
  See {{#manual_link #7.1.3 (Page)}}.
  In the {{#rustdoc_link root-task sel4/index.html `sel4` crate}}, {{#rustdoc_link root-task sel4/cap_type/struct.VSpace.html `sel4::cap_type::Granule`}} is an alias for the {{#rustdoc_link root-task sel4/trait.CapType.html `sel4::CapType`}} corresponding to the smallest frame object type.

The root task is provided with capabilites for the objects comprising its own virtual address space.
The locations of these capabilities in the root task's CSpace is provided in the `BootInfo` struct.
Our goal for this chapter will be to create a frame object and experiment with mapping it into the root task's own address space.

{{#step 4.A (exercise)}}

**Exercise:** Using code snippets from [Chapter 3 (Allocating Kernel Objects)](kernel-objects.html), create a frame object. We seek a value `frame: sel4::cap::Granule` (which can also be written as `frame: sel4::Cap<sel4::cap_type::Granule>`).

Note that, after importing the {{#rustdoc_link root-task sel4/trait.CapTypeForObjectOfFixedSize.html `sel4::CapTypeForObjectOfFixedSize`}} trait, you can express the appropriate {{#rustdoc_link root-task sel4/enum.ObjectBlueprint.html `sel4::ObjectBlueprint`}} as `sel4::cap_type::Granule::object_blueprint()`.

{{#step 4.B}}

In order to avoid interacting with intermediate translation structures, let's reserve an aligned granule-sized region in the root task's program image that we can use for our experiment:

{{#fragment_with_gh_link "rust,ignore" @4.B workspaces/root-task/address-space/src/main.rs:15:20}}

{{#fragment_with_gh_link "rust,ignore" @4.B workspaces/root-task/address-space/src/main.rs:46:49}}

This reservation covers exactly one page in the root task's virtual address space.

{{#step 4.C}}

Let's unmap the page in the root task image that initially backs our reserved region. This will give us a hole in the root task VSpace, which we can use for our experiment.

This function determines the slot in the root task's CSpace of the page capability corresponding to the page in the root task's VSpace which contains the given address:

{{#fragment_with_gh_link "rust,ignore" @4.C workspaces/root-task/address-space/src/main.rs:73:84}}

`__executable_start` is defined by the linker.
We can use it, along with `addr`, to identify the offset into {{#rustdoc_link root-task sel4/struct.BootInfo.html#method.user_image_frames `bootinfo.user_image_frames()`}} of the capability we are after.

Use the `get_user_image_frame_slot()` function to find the capability corresponding to the frame backing our reserved region, and unmap that frame, leaving a hole in the root task's virtual address space.
Try to access that region, and observe a page fault:

{{#fragment_with_gh_link "rust,ignore" @4.C workspaces/root-task/address-space/src/main.rs:51:54}}

{{#step 4.D (exercise)}}

**Exercise:** Map `new_frame` at `page_a_addr` using {{#rustdoc_link root-task sel4/struct.Cap.html#method.frame_map `new_frame.frame_map()`}}, which corresponds to {{#manual_link [seL4_ARM_Page_Map] #10.7.6.6}}.

Here are some hints for the parameters of `.frame_map()`:
- `vspace`: A capability for the root task's VSpace can be expressed using `sel4::init_thread::slot::VSPACE.cap()`.
- `rights`: `sel4::CapRights::read_write()` are the most permissive rights, and will work for our purposes.
- `attrs`: {{#rustdoc_link root-task sel4/struct.Cap.html#method.frame_map `sel4::VmAttributes::default()`}} will work here.
