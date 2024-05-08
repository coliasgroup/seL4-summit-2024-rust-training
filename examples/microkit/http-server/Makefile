#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

include ../../../mk/microkit.mk

target := aarch64-sel4-microkit

target_cc := aarch64-none-elf-gcc

cc_cargo_env := \
	CC_$(subst -,_,$(target))=$(target_cc)

crate = $(build_dir)/$(1).elf

define build_crate

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate:
	$(common_cargo_env) \
	$(cc_cargo_env) \
		cargo build \
			$(common_cargo_args) \
			--target $(target) \
			--release \
			-p $(1)

endef

crate_names := \
	microkit-http-server-example-server \
	microkit-http-server-example-pl031-driver \
	microkit-http-server-example-sp804-driver \
	microkit-http-server-example-virtio-net-driver \
	microkit-http-server-example-virtio-blk-driver

crates := $(foreach crate_name,$(crate_names),$(call crate,$(crate_name)))

$(eval $(foreach crate_name,$(crate_names),$(call build_crate,$(crate_name))))

$(system_description): http-server.system
	cp $< $@

$(image): $(crates)

disk_img := $(build_dir)/disk.img
compressed_disk_img := resources/disk.img.gz

$(disk_img): $(compressed_disk_img)
	gunzip < $< > $@

extra_qemu_args := \
	-device virtio-net-device,netdev=netdev0 \
	-netdev user,id=netdev0,hostfwd=tcp::8080-:80,hostfwd=tcp::8443-:443 \
	-device virtio-blk-device,drive=blkdev0 \
	-blockdev node-name=blkdev0,read-only=on,driver=file,filename=$(disk_img)

simulation-context: $(disk_img)
