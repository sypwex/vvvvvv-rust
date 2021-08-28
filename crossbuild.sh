#!/bin/bash

target_dir=target/release
mkdir -p $target_dir/{linux,mac,win64}

echo "============================= build base docker image"
docker build -t vvvvvv-crossbuild -f crossbuild.Dockerfile .

# echo "============================= build"
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild bash -c "./bin/cargobuild.sh && ./bin/copy.sh"

# TODO: include version in archive name somehow
zip -r vvvvvv.zip $target_dir
