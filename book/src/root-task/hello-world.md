<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Hello, World!

Some text here.

{{#step 1.A}}

Foo bar.

<details>
    <summary>What is a capability? (click to expand)</summary>
    <p>
    Whenever you open a file on UNIX-like operating systems, the kernel gives you a file-descriptor. A    unique token that
    is used to refer to that file from now on. Say if you want to read or write or close the file, you    have to use the
    file-descriptor. You can think of capabilities as similar to file-descriptors except that it is    for every kind of
    object in seL4. For example if you wanted a thread to have access to a certain page of memory, it    must have the
    capability to that page. If you want multiple threads to communicate (as we'll see later), each    thread must
    also have capabilities to the communication objects (such as Endpoints and Notifications in seL4).
</details>

{{#step 1.B}}

Foo baz.
