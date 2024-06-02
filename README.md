# VVVVVV #RIIR Project

This project is derivative work based on [original source code](https://github.com/TerryCavanagh/VVVVVV) of the desktop version of the [VVVVVV-game](https://en.wikipedia.org/wiki/VVVVVV). The main purpose were to learn plain Rust development, so directory structure and API mimics the original work. But the code is not a direct port, it's a rewrite from scratch with some changes like respecting borrow checker rules.

According to [original source code License](https://github.com/TerryCavanagh/VVVVVV/blob/master/LICENSE.md) the assets (`data.zip`) are not included in this repository, but you can get ones from [Make and Play edition](https://thelettervsixtim.es/makeandplay/).

## Features

Currently main features of the game are implemented, but various glitches are still included, so feel free to fork and contribute.

- [x] Title screen, menu, game start
- [x] Load levels from `data.zip`
- [x] Load/Save game progress
- [x] Gamepad support
- [x] Sound/music support
- [x] First location

WIP

- [ ] Second and other locations
- [ ] Map glitches
- [ ] Sprite glitches
- [ ] …

## Build

Arch Linux

```bash
rustup default stable
??? sudo pacman -S physfs
cargo build
```

Ubuntu 23.10

```bash
rustup default stable
sudo apt install clang libphysfs-dev libsdl2-dev libsdl2-mixer-dev
cargo build
```

MacOS 10.15.7

```bash
rustup default stable
brew install sdl2 sdl2_mixer physfs
cargo build
```

Windows

```
…
```

### Build by crossbuild.Dockerfile

```bash
docker build -t vvvvvv-crossbuild -f crossbuild.Dockerfile .
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-unknown-linux-gnu
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-apple-darwin
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-pc-windows-gnu
```

## Run

```bash
cargo run --release
```

`data.zip` currently should be in the same directory as the executable, so one can use next little helper script:

```bash
mkdir -p ./target/{debug,release} && cp data.zip ./target/debug/ && cp data.zip ./target/release/
```
