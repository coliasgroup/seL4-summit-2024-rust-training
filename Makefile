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
    examples/microkit/hello \
    examples/microkit/http-server

.PHONY: none
none:

.PHONY: clean
clean:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true

.PHONY: test
test:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true
