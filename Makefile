#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

examples := \
	examples/root-task/hello \
	examples/microkit/hello

.PHONY: none
none:

.PHONY: clean
clean:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true

.PHONY: test
test:
	$(foreach example,$(examples),$(MAKE) -C $(example) $@ &&) true
