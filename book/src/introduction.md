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

This training is a hands-on tutorial that will lead you through the usage of those exports.
- [Part I](#the-root-task) covers the Rust bindings for the seL4 API and the runtime for root tasks
- [Part II](#sel4-microkit) covers the Rust bindings for the seL4 Microkit API and the runtime for microkit protection domains

This text assumes some familiarity with seL4 API, but the instructor will be happy to introduce or review any relevant seL4 API concepts.
