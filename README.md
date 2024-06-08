# VVVVVV Rust fork

This project is the game engine of the desktop version of the [VVVVVV-game](https://en.wikipedia.org/wiki/VVVVVV) and is derivative work based on [original source code](https://github.com/TerryCavanagh/VVVVVV). The main purpose is to learn plain Rust game engine development, so directory structure and API mimics the original work. But this code is not a direct port, it's a rewrite from scratch with some changes like respecting Rust borrow checker rules.

Since this repository is just a game engine, in order to start the game one need assets, and since VVVVVV is the commercial product + according to [original source code License](https://github.com/TerryCavanagh/VVVVVV/blob/master/LICENSE.md) the assets (`data.zip`) can not be included in this repository, so to say redistributed. So You can get your own copy by buying the original game, preferably PC-version since assets available directly in installation folder (e.g. from [Steam](https://store.steampowered.com/app/70300/VVVVVV/)). The other option is to load assets from [Make and Play edition](https://thelettervsixtim.es/makeandplay/). Please note that assets are intented for personal use only and should not be redistributed without Terry notice.

## Build

Assuming Rust is installed and available in PATH.

Use stable Rust toolchain, e.g. `rustup default stable`.

Ubuntu 23.10

```bash
sudo apt install clang libphysfs-dev libsdl2-dev libsdl2-mixer-dev
cargo build
```

Apple x86/Intel (tested on MacOS 10.15.7)

```bash
brew install sdl2 sdl2_mixer physfs
brew link sdl2
cargo build
```

Apple ARM/M1 (tested on MacOS 14.5)

Since brew installs x86 version, use one vcpkg provides.

```bash
rustup default stable
vcpkg install sdl2 sdl2-mixer physfs ???
cargo build
```

Windows (TODO)

```
vcpkg install sdl2 sdl2-mixer physfs ???
cargo build
```

## Run

```bash
cargo run --release
```

`data.zip` currently should be in the same directory as the executable, so one can use next little helper script:

```bash
mkdir -p ./target/{debug,release} && cp data.zip ./target/debug/ && cp data.zip ./target/release/
```

## Notes

You can search original code by function signature, like

`SoundTrack::SoundTrack(const char* fileName)`
