#!/usr/bin/env bash
#
# Copyright 2024, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

# This repo's examples mirrors those found in the rust-sel4 repo.

set -eu

external_rust_seL4_dir=../../rust-sel4
external_examples_dir=$external_rust_seL4_dir/crates/examples

cp $external_rust_seL4_dir/rust-toolchain.toml .

rm support/targets/*.json
cp $external_rust_seL4_dir/support/targets/aarch64-sel4{,-microkit}.json support/targets

find examples -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.system' \) -delete

find examples -type d -empty -delete

examples=" \
    root-task/hello \
    root-task/serial-device \
    root-task/spawn-task \
    root-task/spawn-thread \
    microkit/hello \
    microkit/banscii \
"

for example in $examples; do
    for file in $(find $external_examples_dir/$example -type f \( -name '*.rs' -o -name '*.toml' -o -name '*.system' \) -printf "%P\n"); do
        src=$external_examples_dir/$example/$file
        dst=examples/$example/$file
        mkdir -p $(dirname $dst)
        cp $src $dst
    done
done

frontmatter_expr='6,10d'

git_expr='s,path = "\(../\)*../../../\([^"]*\)",git = "https://github.com/seL4/rust-sel4",g' \

find examples -type f -name Cargo.toml -exec sed -i -e "$frontmatter_expr" -e "$git_expr" {} +

cargo update -w -p sel4
