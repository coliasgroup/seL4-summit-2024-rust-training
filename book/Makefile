#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

PROD ?= 0

build_dir := build

repo := coliasgroup/sel4-rust-tutorial
manual_url := https://sel4.systems/Info/Docs/seL4-manual-13.0.0.pdf

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)

.PHONY: build-preprocessor
build-preprocessor:
	cargo build --manifest-path preprocessor/Cargo.toml

.PHONY: build-book
build-book:
	MDBOOK_GH_LINK_LOCAL_ROOT=$(abspath ..) \
	MDBOOK_GH_LINK_REPO=$(repo) \
	MDBOOK_GH_LINK_REV=$$(git rev-parse HEAD) \
	MDBOOK_MANUAL_URL=$(manual_url) \
	PATH=preprocessor/target/debug:$$PATH \
		mdbook build

.PHONY: build
build: build-book
	set -eu -o pipefail; \
	src=../build/rustdoc; \
	dst=$(build_dir)/rustdoc; \
	[ ! -e "$$dst" ]; \
	if [ "$(PROD)" = "1" ]; then \
		cp -r $$src $$dst; \
	else \
		ln -s ../$$src $$dst; \
	fi

.PHONY: check
check:
	linkchecker $(build_dir)

.PHONY: open
open:
	open $(build_dir)/index.html
