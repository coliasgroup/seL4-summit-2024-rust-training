#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/microkit.mk

crate_name := microkit-hello
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	cargo build \
		$(common_cargo_args) \
		-p $(crate_name)

$(system_description): hello.system
	cp $< $@

$(image): $(crate)
