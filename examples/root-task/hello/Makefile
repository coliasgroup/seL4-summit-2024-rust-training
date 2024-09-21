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

sel4_prefix := $(SEL4_INSTALL_DIR)

crate_name := hello
crate := $(build_dir)/$(crate_name).elf

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	cargo build \
		--target-dir $(build_dir)/target \
		--artifact-dir $(build_dir) \
		-p $(crate_name)

loader_artifacts_dir := $(SEL4_INSTALL_DIR)/bin
loader := $(loader_artifacts_dir)/sel4-kernel-loader
loader_cli := $(loader_artifacts_dir)/sel4-kernel-loader-add-payload

image := $(build_dir)/image.elf

$(image): $(crate) $(loader) $(loader_cli)
	$(loader_cli) \
		--loader $(loader) \
		--sel4-prefix $(sel4_prefix) \
		--app $(crate) \
		-o $@

qemu_cmd = \
	qemu-system-aarch64 \
		-machine virt,virtualization=on -cpu cortex-a57 -m size=1G \
		-serial mon:stdio \
		-nographic \
		-kernel $(image)

.PHONY: simulate
simulate: $(image)
	$(qemu_cmd)

.PHONY: test
test: test.py $(image)
	PYTHONPATH=../../../test-utils python3 $< $(qemu_cmd)
