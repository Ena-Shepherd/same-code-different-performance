#!/usr/bin/env bash

set -eo pipefail

for i in {1..50}; do
  NOP_COUNT="${i}" cargo run --release $@ 2> /dev/null
done
