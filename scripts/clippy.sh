#!/bin/bash

export RUSTC_WRAPPER=$(dirname $0)/clippy-wrapper.sh
set -x

cargo check \
  --package drone-stm32-macros \
  --package drone-stm32-svd
xargo check --target "thumbv7m-none-eabi" --all "$@" \
  --exclude drone-stm32-macros \
  --exclude drone-stm32-svd
