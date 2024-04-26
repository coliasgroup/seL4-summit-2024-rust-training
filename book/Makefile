#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

repo := coliasgroup/rust-sel4-tutorial
manual_url := https://sel4.systems/Info/Docs/seL4-manual-12.1.0.pdf

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
	MDBOOK_GH_LINK_LOCAL_ROOT=$(abspath ..) \
	MDBOOK_GH_LINK_REPO=$(repo) \
	MDBOOK_GH_LINK_REV=$$(git rev-parse HEAD) \
	MDBOOK_MANUAL_URL=$(manual_url) \
	PATH=preprocessor/target/debug:$$PATH \
		mdbook build

.PHONY: check
check:
	linkchecker $(build_dir)

.PHONY: open
open:
	open $(build_dir)/index.html
