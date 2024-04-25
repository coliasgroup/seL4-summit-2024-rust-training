#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../common.mk

crate_name := hello
crate := $(build_dir)/$(crate_name).elf
crate_intermediate := $(build_dir)/$(crate_name).intermediate

$(crate): $(crate_intermediate)

.INTERMDIATE: $(crate_intermediate)
$(crate_intermediate):
	SEL4_PREFIX=$(sel4_prefix) \
		cargo build \
			-Z build-std=core,alloc,compiler_builtins \
			-Z build-std-features=compiler-builtins-mem \
			--target aarch64-sel4 \
			--target-dir $(build_dir)/target \
			--out-dir $(build_dir) \
			-p $(crate_name)

$(app): $(crate)
	cp $< $@
