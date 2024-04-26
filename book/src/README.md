<!--
     Copyright 2024, Colias Group, LLC

     SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Introduction

{{#fragment_with_gh_link "rust,ignore" examples/root-task/hello/src/main.rs:12:17}}

{{#fragment_with_gh_link "rust,ignore" (examples/root-task/)hello/src/main.rs:12:17}}

Foo {{#gh_link examples/root-task/hello/src/main.rs:12:17}} bar

Foo {{#gh_link examples/root-task/hello/src/main.rs:12}} bar

Foo {{#gh_link examples/root-task/hello/src/main.rs}} bar

Foo {{#gh_link (examples/root-task/hello/)src/main.rs}} bar

Foo {{#gh_link examples/root-task/hello/src/}} bar

Foo {{#gh_link [foo] examples/root-task/hello/src/}} bar

Foo {{#manual_link [manual]}} bar

Foo {{#manual_link [foo] #4.1}} bar

Foo {{#manual_link #4.1 (bar)}} bar

Foo {{#manual_link #4 (bar)}} bar
