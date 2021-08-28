In order to use macos crossbuild with crossbuild.Dockerfile, put `libSDL2-2.0.0.dylib` and `libSDL2_mixer-2.0.0.dylib` here.
Those file cold be obtained after homebrew SDL installation, like:

```bash
brew install sdl2 sdl2_mixer
cp /usr/local/Cellar/sdl2/2.0.14_1/lib/libSDL2-2.0.0.dylib \
   /usr/local/Cellar/sdl2_mixer/2.0.4_1/lib/libSDL2_mixer-2.0.0.dylib \
   crossbuild/lidSDL/
```

Exact library locations can be found via:

```bash
brew ls sdl2
brew ls sdl2_mixer
```
