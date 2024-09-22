#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

work_root := ..

id := sel4-rust-tutorial
image_tag := $(id)
container_name := $(id)

mount_arg := --mount type=bind,src=$(abspath $(work_root)),dst=/work

uid := $(shell id -u)
gid := $(shell id -g)

.PHONY: none
none:

.PHONY: build
build:
	docker build \
		--build-arg UID=$(uid) --build-arg GID=$(gid) \
		-t $(image_tag) .

.PHONY: run
run: build
	docker run -d --name $(container_name) \
		$(mount_arg) \
		$(image_tag) \
		sleep inf

.PHONY: exec
exec:
	docker exec -it $(container_name) bash

.PHONY: rm-container
rm-container:
	for id in $$(docker ps -aq -f "name=^$(container_name)$$"); do \
		docker rm -f $$id; \
	done

.PHONY: check-examples
check-examples: build
	docker run --rm \
		$(mount_arg) \
		-i $$([ -t 0 ] && echo -t) \
		$(image_tag) \
		make test
