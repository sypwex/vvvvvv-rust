#!/bin/bash

target_dir=./target/release

echo "============================= copy binaries"
cp -v target/x86_64-unknown-linux-gnu/debug/vvvvvv-rust $target_dir/linux/
cp -v target/x86_64-apple-darwin/debug/vvvvvv-rust $target_dir/mac/
cp -v target/x86_64-pc-windows-gnu/debug/vvvvvv-rust.exe $target_dir/win64/

echo "============================= copy dependencies"
cp -v /usr/local/lib/x86_64-unknown-linux-gnu/{libphysfs,libSDL2,libSDL2_mixer}.so $target_dir/linux/
cp -v /usr/local/lib/x86_64-apple-darwin/{libphysfs,libSDL2,libSDL2_mixer}.dylib $target_dir/mac/
cp -v /tmp/SDL2-2.0.16/x86_64-w64-mingw32/bin/SDL2.dll /tmp/SDL2_mixer-2.0.4/x86_64-w64-mingw32/bin/SDL2_mixer.dll $target_dir/win64/
cp -v /tmp/libphysfs.dll $target_dir/win64/
