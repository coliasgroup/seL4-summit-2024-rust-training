#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/microkit.mk

crate = $(build_dir)/$(1).elf

define build_crate

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	$$(common_cargo_env) \
		cargo build \
			$$(common_cargo_flags) \
			--target aarch64-sel4-microkit \
			--release \
			-p $(1)

endef

crate_names := \
	microkit-hello

crates := $(foreach crate_name,$(crate_names),$(call crate,$(crate_name)))

$(eval $(foreach crate_name,$(crate_names),$(call build_crate,$(crate_name))))

$(system_description): hello.system
	cp $< $@

$(image): $(crates)
