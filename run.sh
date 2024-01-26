#!/usr/bin/env bash

set -eo pipefail

for i in {1..100}; do
  NOP_COUNT="${i}" cargo run --release 2> /dev/null
done
