#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

PROD ?= 0

build_dir := build

code_dir := ../code
repo := coliasgroup/sel4-rust-tutorial
manual_url := https://sel4.systems/Info/Docs/seL4-manual-13.0.0.pdf

.PHONY: none
none:

.PHONY: clean
clean: clean-preprocessor clean-book

preprocessor_manifest_path_args := --manifest-path preprocessor/Cargo.toml

.PHONY: build-preprocessor
build-preprocessor:
	cargo build $(preprocessor_manifest_path_args)

.PHONY: clean-preprocessor
clean-preprocessor:
	cargo clean $(preprocessor_manifest_path_args)

.PHONY: build-book-mdbook
build-book-mdbook:
	MDBOOK_GH_LINK_LOCAL_ROOT=$(abspath $(code_dir)) \
	MDBOOK_GH_LINK_REPO=$(repo) \
	MDBOOK_GH_LINK_REV=$$(git rev-parse HEAD) \
	MDBOOK_MANUAL_URL=$(manual_url) \
	PATH=preprocessor/target/debug:$$PATH \
		mdbook build

src_rustdoc_dir := $(code_dir)/build/rustdoc
dst_rustdoc_dir := $(build_dir)/rustdoc

.PHONY: build-book
build-book: build-book-mdbook
	set -eu -o pipefail; \
	src=$(src_rustdoc_dir); \
	dst=$(dst_rustdoc_dir); \
	[ ! -e "$$dst" ]; \
	if [ "$(PROD)" = "1" ]; then \
		cp -r $$src $$dst; \
	else \
		ln -s ../$$src $$dst; \
	fi

.PHONY: clean-book
clean-book:
	rm -rf $(build_dir)

.PHONY: build
build: build-book

.PHONY: check
check:
	linkchecker $(build_dir)/index.html --no-follow-url '.*/rustdoc/.*'

.PHONY: open
open:
	open $(build_dir)/index.html
