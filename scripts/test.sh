#!/bin/bash

export RUSTC_WRAPPER=$(dirname $0)/rustc-wrapper.sh
set -x

cargo test \
  --package drone-stm32-macros \
  --package drone-stm32-svd
cargo drone test \
  --package drone-stm32
