#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../common.mk

common_env := \
	SEL4_INCLUDE_DIRS=$(sel4_include_dirs)

common_options := \
	-Z build-std=core,alloc,compiler_builtins \
	-Z build-std-features=compiler-builtins-mem \
	--target aarch64-sel4-microkit \
	--release \
	--target-dir $(build_dir)/target \
	--out-dir $(build_dir)

target_for_crate = $(build_dir)/$(1).elf
intermediate_target_for_crate = $(build_dir)/$(1).intermediate

define build_crate

$(target_for_crate): $(intermediate_target_for_crate)

.INTERMDIATE: $(intermediate_target_for_crate)
$(intermediate_target_for_crate):
	$$(common_env) \
		cargo build \
			$$(common_options) \
			-p $(1)

endef

crates := \
	microkit-hello

built_crates := $(foreach crate,$(crates),$(call target_for_crate,$(crate)))

$(eval $(foreach crate,$(crates),$(call build_crate,$(crate))))

$(system_description): hello.system
	cp $< $@

$(image): $(built_crates)
