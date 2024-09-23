#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

code_dir := code

.PHONY: none
none:

.PHONY: ci
ci:
	$(MAKE) -C $(code_dir) 
