#!/bin/bash

source $(dirname $0)/_env.sh
set -x

cargo clippy --package drone-cortex-m-macros
cargo clippy --target $BUILD_TARGET --all --exclude drone-cortex-m-macros
