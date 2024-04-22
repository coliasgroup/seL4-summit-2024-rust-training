#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

ifeq ($(IN_NIX_SHELL),)
	run_in_nix_shell := nix-shell -A shell --run
else
	run_in_nix_shell := $(SHELL) -c
endif

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf out

.PHONY: shell
shell:
	nix-shell -A shell

.PHONY: build-preprocessors
build-preprocessors:
	$(run_in_nix_shell) "cargo build --manifest-path preprocessors/Cargo.toml"

.PHONY: build
build:
	FOO_LOCAL_ROOT=$(abspath ..) \
	FOO_REMOTE_PREFIX=https://github.com/coliasgroup/rust-sel4-tutorial/blob/$$(git rev-parse HEAD) \
	PATH=preprocessors/target/debug:$$PATH \
		$(run_in_nix_shell) "mdbook build"

.PHONY: check
check:
	$(run_in_nix_shell) "linkchecker out"

.PHONY: open
open:
	open out/index.html
