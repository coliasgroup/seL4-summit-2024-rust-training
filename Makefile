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

.PHONY: clean
clean: clean-each-example
	rm -rf target

.PHONY: test
test: test-each-example

.PHONY: clean-each-example test-each-example
clean-each-example test-each-example:
	$(foreach example,$(examples),$(MAKE) -C $(example) $(subst -each-example,,$@) &&) true
