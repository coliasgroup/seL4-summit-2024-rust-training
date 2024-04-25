#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/root-task.mk

crate_name := hello
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	$(common_cargo_env) \
		cargo build \
			$(common_cargo_flags) \
			--target aarch64-sel4 \
			-p $(crate_name)

$(app): $(crate)
	cp $< $@
