#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)

microkit_board := qemu_virt_aarch64
microkit_config := debug

crate_name := microkit-hello
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	cargo build \
		--target-dir $(build_dir)/target \
		--artifact-dir $(build_dir) \
		-p $(crate_name)

system_description := hello.system

image := $(build_dir)/image.img

$(image): $(system_description) $(crate)
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
		-device loader,file=$(image),addr=0x70000000,cpu-num=0

.PHONY: simulate
simulate: $(image)
	$(qemu_cmd)

.PHONY: test
test: test.py $(image)
	PYTHONPATH=../../../test-utils python3 $< $(qemu_cmd)
