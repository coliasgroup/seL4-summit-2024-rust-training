#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

rev_url_prefix := https://github.com/coliasgroup/rust-sel4-tutorial/blob

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: build-preprocessors
build-preprocessors:
	cargo build --manifest-path preprocessors/Cargo.toml

.PHONY: build
build:
	MDBOOK_FRAGMENT_WITH_CONTEXT_LOCAL_ROOT=$(abspath ..) \
	MDBOOK_FRAGMENT_WITH_CONTEXT_REMOTE_PREFIX=$(rev_url_prefix)/$$(git rev-parse HEAD) \
	PATH=preprocessors/target/debug:$$PATH \
		mdbook build

.PHONY: check
check:
	linkchecker $(build_dir)

.PHONY: open
open:
	open $(build_dir)/index.html
