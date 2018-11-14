#!/bin/bash

source $(dirname $0)/_env.sh
export RUSTC_WRAPPER=$(dirname $0)/_clippy_wrapper.sh
set -x

cargo check \
  --package drone-stm32-macros \
  --package drone-stm32-svd
xargo check --target $BUILD_TARGET --all "$@" \
  --exclude drone-stm32-macros \
  --exclude drone-stm32-svd
