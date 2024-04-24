#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

repo := coliasgroup/rust-sel4-tutorial

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: build-preprocessor
build-preprocessor:
	cargo build --manifest-path preprocessor/Cargo.toml

.PHONY: build
build:
	MDBOOK_GH_LINKS_LOCAL_ROOT=$(abspath ..) \
	MDBOOK_GH_LINKS_REPO=$(repo) \
	MDBOOK_GH_LINKS_REV=$$(git rev-parse HEAD) \
	PATH=preprocessor/target/debug:$$PATH \
		mdbook build

.PHONY: check
check:
	linkchecker $(build_dir)

.PHONY: open
open:
	open $(build_dir)/index.html
