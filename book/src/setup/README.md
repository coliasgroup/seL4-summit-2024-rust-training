<!--
     Copyright 2024, Colias Group, LLC

     SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Foo

{{#fragment_with_gh_link "rust,ignore" examples/root-task/hello/src/main.rs:12:17}}

{{#fragment_with_gh_link "rust,ignore" (examples/root-task/)hello/src/main.rs:12:17}}

Foo {{#gh_link examples/root-task/hello/src/main.rs:12:17}} bar

Foo {{#gh_link examples/root-task/hello/src/main.rs:12}} bar

Foo {{#gh_link examples/root-task/hello/src/main.rs}} bar

Foo {{#gh_link (examples/root-task/hello/)src/main.rs}} bar

Foo {{#gh_link examples/root-task/hello/src/}} bar
