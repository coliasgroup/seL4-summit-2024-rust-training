#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include $(dir $(lastword $(MAKEFILE_LIST)))/base.mk

microkit_board := qemu_virt_aarch64
microkit_config := debug
microkit_sdk_config_dir := $(MICROKIT_SDK)/board/$(microkit_board)/$(microkit_config)

sel4_include_dirs := $(microkit_sdk_config_dir)/include

system_description := $(build_dir)/this.system

$(system_description): | $(build_dir)

image := $(build_dir)/image.img

$(image): $(system_description)
	$(MICROKIT_SDK)/bin/microkit \
		$< \
		--search-path $(build_dir) \
		--board $(microkit_board) \
		--config $(microkit_config) \
		--report $(build_dir)/report.txt \
		--output $@

qemu_cmd = \
	qemu-system-aarch64 \
		-machine virt -cpu cortex-a53 -m size=2G \
		-serial mon:stdio \
		-nographic \
		-device loader,file=$(image),addr=0x70000000,cpu-num=0 \
		$(extra_qemu_args)

.PHONY: run-context
run-context: $(image)

.PHONY: run
run: run-context
	$(qemu_cmd)

.PHONY: test
test: test.py run-context
	PYTHONPATH=$(root_dir)/testing python3 $< $(qemu_cmd)

common_cargo_env := \
	SEL4_INCLUDE_DIRS=$(sel4_include_dirs)

common_cargo_flags := \
	-Z build-std=core,alloc,compiler_builtins \
	-Z build-std-features=compiler-builtins-mem \
	--target-dir $(build_dir)/target \
	--out-dir $(build_dir)
