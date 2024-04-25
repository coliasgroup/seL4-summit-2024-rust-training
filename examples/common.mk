#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

root_dir := $(dir $(lastword $(MAKEFILE_LIST)))/..

relative_example_dir := $(shell realpath -m --relative-to $(root_dir) .)

BUILD ?= $(root_dir)

build_dir := $(BUILD)/$(relative_example_dir)/build

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(build_dir)
