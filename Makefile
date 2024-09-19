#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

examples := \
    examples/root-task/hello \
    examples/root-task/serial-device \
    examples/root-task/spawn-task \
    examples/root-task/spawn-thread \
    examples/microkit/banscii \
    examples/microkit/hello

.PHONY: none
none:

.PHONY: clean test
clean test:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true

clean: clean-top-level

.PHONY: clean-top-level
clean-top-level:
	rm -rf target
