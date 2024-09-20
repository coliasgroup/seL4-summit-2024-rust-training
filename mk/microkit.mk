#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

root_dir := $(dir $(lastword $(MAKEFILE_LIST)))/..

build_dir := build

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)

$(build_dir):
	mkdir -p $(build_dir)

microkit_board := qemu_virt_aarch64
microkit_config := debug

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
		-machine virt,virtualization=on -cpu cortex-a53 -m size=2G \
		-serial mon:stdio \
		-nographic \
		-device loader,file=$(image),addr=0x70000000,cpu-num=0 \
		$(extra_qemu_args)

.PHONY: simulation-context
simulation-context: $(image)

.PHONY: simulate
simulate: simulation-context
	$(qemu_cmd)

.PHONY: test
test: test.py simulation-context
	PYTHONPATH=$(root_dir)/test-utils python3 $< $(qemu_cmd)

common_cargo_args := \
	--target-dir $(build_dir)/target \
	--artifact-dir $(build_dir)
