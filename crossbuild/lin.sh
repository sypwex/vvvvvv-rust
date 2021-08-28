#!/bin/bash
export CROSS_TRIPLE="x86_64-unknown-linux-gnu"
# exec crossbuild cc -I/usr/local/src/physfs/src -L/usr/local/lib/$CROSS_TRIPLE "$@"
exec gcc -I/usr/local/src/physfs/src -L/usr/local/lib/$CROSS_TRIPLE "$@"
