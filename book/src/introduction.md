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
- A runtime for [seL4 Microkit](https://github.com/seL4/microkit) protection domains
- Custom rustc [target specifications](https://docs.rust-embedded.org/embedonomicon/custom-target.html) for seL4 userspace
