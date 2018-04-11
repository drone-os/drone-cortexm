#!/bin/bash

exec rustc --cfg procmacro2_semver_exempt $@
