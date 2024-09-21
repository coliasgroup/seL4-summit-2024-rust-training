#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

build_dir := build

examples := \
    examples/root-task/hello \
    examples/root-task/serial-device \
    examples/root-task/spawn-task \
    examples/root-task/spawn-thread \
    examples/microkit/banscii \
    examples/microkit/hello

.PHONY: none
none:

$(build_dir):
	mkdir -p $(build_dir)

.PHONY: clean
clean: clean-here clean-each-example

.PHONY: clean-here
clean-here:
	rm -rf $(build_dir)

.PHONY: test
test: test-each-example

.PHONY: clean-each-example test-each-example
clean-each-example test-each-example:
	$(foreach example,$(examples),$(MAKE) -C $(example) $(subst -each-example,,$@) &&) true

rustdoc_dir := $(build_dir)/rustdoc
exported_rustdoc_dir := $(build_dir)/exported-rustdoc

.PHONY: rustdoc
rustdoc: 
	cd examples/root-task && cargo doc \
		--target-dir $(abspath $(rustdoc_dir)/root-task)
	cd examples/microkit && cargo doc \
		--target-dir $(abspath $(rustdoc_dir)/microkit)

.PHONY: exported-rustdoc
exported-rustdoc: rustdoc | $(build_dir)
	rm -rf $(exported_rustdoc_dir)
	rsync -r $(rustdoc_dir)/ $(exported_rustdoc_dir)/ \
        --info=progress2 --info=name0 \
		--exclude '/*/debug' \
		--exclude '/*/*/debug' \
		--exclude '/*/.*.json' \
		--exclude '/*/CACHEDIR.TAG'
