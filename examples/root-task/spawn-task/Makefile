#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/root-task.mk

child_crate_name := spawn-task-child
child_crate := $(build_dir)/$(child_crate_name).elf

$(child_crate): $(child_crate).intermediate

.INTERMDIATE: $(child_crate).intermediate
$(child_crate).intermediate:
	$(common_cargo_env) \
		cargo build \
			$(common_cargo_flags) \
			--target aarch64-sel4 \
			-p $(child_crate_name)

crate_name := spawn-task
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate: $(child_crate)
	$(common_cargo_env) \
	CHILD_ELF=$(abspath $(child_crate)) \
		cargo build \
			$(common_cargo_flags) \
			--target aarch64-sel4 \
			-p $(crate_name)

$(app): $(crate)
	cp $< $@
