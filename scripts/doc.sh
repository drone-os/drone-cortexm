#!/bin/bash

source $(dirname $0)/_env.sh
export RUSTC_WRAPPER=$(dirname $0)/_rustc_wrapper.sh
set -x

cargo doc \
  --package drone-cortex-m-macros
xargo doc --target $BUILD_TARGET --all "$@" \
  --exclude drone-cortex-m-macros
