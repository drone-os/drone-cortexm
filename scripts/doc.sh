#!/bin/bash

source $(dirname $0)/_env.sh
set -x

cargo doc --package drone-cortex-m-macros
cargo doc --target $BUILD_TARGET --all --exclude drone-cortex-m-macros
