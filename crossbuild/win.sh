#!/bin/bash
export CROSS_TRIPLE="x86_64-w64-mingw32"
exec crossbuild cc -I/usr/local/src/physfs/src -L/usr/local/lib/x86_64-pc-windows-gnu "$@"
