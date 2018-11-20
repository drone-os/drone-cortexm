#!/bin/bash

source $(dirname $0)/_env.sh
export RUSTC_WRAPPER=$(dirname $0)/_rustc_wrapper.sh
export RUST_TARGET_PATH=$(pwd)
export CROSS_COMPILE=arm-none-eabi
set -x

cargo test \
  --package drone-stm32-macros
xargo test --target $TEST_TARGET \
  --package drone-stm32
