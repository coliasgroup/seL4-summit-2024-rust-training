<!--
    Copyright 2024, Colias Group, LLC

    SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Setup

This training session is presented as a linear series of patches to [{{#gh_repo_url}}]({{#gh_repo_url}}), starting at commit `{{#rev_of_step_0 12}}`, and ending with commit `{{#rev_of_last_step 12}}`.
Each patch, or step, as we shall call them, is an instructive modification to a code example.
You are encouraged to attempt those marked as exercises in this text on your own.
Note that while most step patches include tests, the reader is not expected to write tests for exercises themselves.

Clone the repository and checkout the starting commit:

```
git clone {{#gh_repo_url}}
cd seL4-summit-2024-rust-training-code
git checkout {{#rev_of_step_0 12}}
```

Observe the steps ahead:

```
git log {{#rev_of_step_0 12}}..{{#rev_of_last_step 12}}
```

### Docker

We will work inside of a Docker container built with {{#gh_link docker/Dockerfile}}.
This Dockerfile installs some build-time dependencies, and then builds seL4, [seL4 Microkit](https://github.com/seL4/microkit), and some additional programs and tools.

Build the image:

```
make -C docker build
```

Run a container in the background:

```
make -C docker run
```

Start a shell inside the container:

```
make -C docker exec
```

The rest of this text assumes that you are in a shell inside the container.

### Two Workspaces

To simplify our Cargo invocations, this repository's Rust code has been divided between two distinct workspaces: {{#gh_link workspaces/root-task}} and {{#gh_link workspaces/microkit}}.
Both are subject to the top-level {{#gh_link .cargo/config.toml}}, whereas {{#gh_link workspaces/root-task/.cargo/config.toml}} and {{#gh_link workspaces/microkit/.cargo/config.toml}} each apply to only one.
These workspace-specific `config.toml` files demonstrate all that is required to configure the crates in the [rust-sel4](https://github.com/seL4/rust-sel4) project:

- An environment variable pointing at `libsel4` (which includes the kernel configuration).
  See {{#rustdoc_link root-task sel4/index.html#building the relevant crate docs}} for information on this environment variable.
- A reference to a custom rustc [target specification](https://docs.rust-embedded.org/embedonomicon/custom-target.html) (the location of the target specification is given at {{#gh_link .cargo/config.toml:8}}, which refers to {{#gh_link target-specs}}).

### VSCode + Dev Containers

If you want help from [rust-analyzer](https://rust-analyzer.github.io/), the Rust Language Server, you can use [VSCode with the Dev Containers extension](https://code.visualstudio.com/docs/devcontainers/tutorial).
This will enable you to run VSCode inside of the container you just built, which contains the seL4 build artifacts that rust-analyzer will require to analyzer your code.

This repository provides a separate Dev Container configuration for each workspace:
- {{#gh_link .devcontainer/root-task/devcontainer.json}}
- {{#gh_link .devcontainer/microkit/devcontainer.json}}

To work in a particular workspace, open this repository in VSCode, run `> Dev Containers: Reopen in Container`, and select the corresponding configuration.
You should now be able to use the rust-analyzer VSCode extension normally for the code in the selected workspace.
