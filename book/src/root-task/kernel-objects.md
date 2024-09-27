<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Allocating Kernel Objects

Some text here.

```
cd workspaces/root-task/kernel-objects
make simulate
```

More text here.

{{#step 3.A}}

Print the untyped memory described by `bootinfo`:

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

{{#step 3.B}}
