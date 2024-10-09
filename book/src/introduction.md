<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Introduction

Support for Rust in seL4 userspace has been an official seL4 Foundation project since November 2023:

<https://github.com/seL4/rust-sel4>

The exports of this project covered in this training are:
- Rust bindings for the seL4 API
- A runtime for root tasks
- Rust bindings for the seL4 Microkit API
- A runtime for [seL4 Microkit](https://github.com/seL4/microkit) protection domains
- Custom rustc [target specifications](https://docs.rust-embedded.org/embedonomicon/custom-target.html) for seL4 userspace

This training is a self-paced, hands-on tutorial that will lead you through the usage of those exports.
There is more tutorial here than could be worked through during this training session, so feel to skip around.

**Important:**
The instructor is eager to work through exercises or discuss any related issues or topic on an individual basis with you.
Take advantage of the fact that this is an in-person interactive session!

[Part I](#the-root-task) covers the Rust bindings for the seL4 API and the runtime for root tasks.
Familiarity with the seL4 API isn't necessarily assumed or required, but this text doesn't introduce its elements in as much detail as the {{#manual_link [seL4 Manual]}}.
Please let the instructor know if you'd like an introduction or review for any seL4 API concepts or details.

[Part II](#sel4-microkit) covers the Rust bindings for the seL4 Microkit API and the runtime for microkit protection domains.
This part does assume that the reader is familiar with the basics of the Microkit framework and API.
