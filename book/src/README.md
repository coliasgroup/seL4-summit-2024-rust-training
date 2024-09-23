<!--
     Copyright 2024, Colias Group, LLC

     SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Introduction

{{#fragment_with_gh_link "rust,ignore" workspaces/root-task/hello/src/main.rs:12:19}}

{{#fragment_with_gh_link "rust,ignore" @1.A (workspaces/root-task/)hello/src/main.rs:12:19}}

Foo {{#gh_link @-1.A workspaces/root-task/hello/src/main.rs:12:17}} bar

Foo {{#gh_link workspaces/root-task/hello/src/main.rs:12}} bar

Foo {{#gh_link workspaces/root-task/hello/src/main.rs}} bar

Foo {{#gh_link (workspaces/root-task/hello/)src/main.rs}} bar

Foo {{#gh_link workspaces/root-task/hello/src/}} bar

Foo {{#gh_link [foo] workspaces/root-task/hello/src/}} bar

Foo {{#manual_link [manual]}} bar

Foo {{#manual_link [foo] #4.100}} bar

Foo {{#manual_link #4.1 (bar)}} bar

Foo {{#manual_link #4 (bar)}} bar

Foo {{#rustdoc_link microkit sel4_microkit/macro.debug_println.html `debug_println!()`}} bar

[`sel4_immediate_sync_once_cell::ImmediateSyncOnceCell`](https://sel4.github.io/rust-sel4/views/aarch64-root-task/aarch64-sel4/doc/sel4_immediate_sync_once_cell/struct.ImmediateSyncOnceCell.html)
