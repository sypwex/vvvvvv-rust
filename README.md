# VVVVVV #RIIR Project

## Build by yourself

Arch

```bash
rustup default stable
??? sudo pacman -S physfs
cargo build
```

Ubuntu 20.04

```bash
rustup default stable
sudo apt install clang-12 libphysfs-dev libsdl2-dev libsdl2-mixer-dev
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

## Build by crossbuild.Dockerfile

```bash
docker build -t vvvvvv-crossbuild -f crossbuild.Dockerfile .
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-unknown-linux-gnu
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-apple-darwin
docker run --rm -it -v `pwd`:/workdir vvvvvv-crossbuild cargo build --target=x86_64-pc-windows-gnu
```
