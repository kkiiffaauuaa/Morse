#!/bin/bash

cd "$(dirname "$0")" || exit 1
XDG_DATA_DIRS="../Resources" ./morse "$@"
