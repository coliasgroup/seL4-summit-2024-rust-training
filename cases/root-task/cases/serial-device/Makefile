#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../../mk/root-task.mk

crate_name := serial-device
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	cargo build \
		$(common_cargo_args) \
		-p $(crate_name)

$(app): $(crate)
	cp $< $@
