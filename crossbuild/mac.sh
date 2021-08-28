#!/bin/bash
export CROSS_TRIPLE="x86_64-apple-darwin"
exec crossbuild cc -mmacosx-version-min=10.7 -I/usr/local/src/physfs/src -L/usr/local/lib/$CROSS_TRIPLE "$@"
