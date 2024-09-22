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
	cargo build \
		$(common_cargo_args) \
		--release \
		-p $(1)

endef

crate_names := \
	banscii-artist \
	banscii-assistant \
	banscii-serial-driver

crates := $(foreach crate_name,$(crate_names),$(call crate,$(crate_name)))

$(eval $(foreach crate_name,$(crate_names),$(call build_crate,$(crate_name))))

$(system_description): banscii.system
	cp $< $@

$(image): $(crates)
