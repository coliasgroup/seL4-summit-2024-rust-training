#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

examples := \
    cases/root-task/cases/hello \
    cases/root-task/cases/serial-device \
    cases/root-task/cases/spawn-task \
    cases/root-task/cases/spawn-thread \
    cases/microkit/cases/banscii \
    cases/microkit/cases/hello

.PHONY: none
none:

.PHONY: clean test
clean test:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true

clean: clean-top-level

.PHONY: clean-top-level
clean-top-level:
	rm -rf target
