#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

examples := \
    root-task/cases/hello \
    root-task/cases/serial-device \
    root-task/cases/spawn-task \
    root-task/cases/spawn-thread \
    microkit/cases/banscii \
    microkit/cases/hello

.PHONY: none
none:

.PHONY: clean-each-example test-each-example
clean-each-example test-each-example:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true

.PHONY: clean
clean: clean-each-example
	rm -rf target

.PHONY: test
test: test-each-example
