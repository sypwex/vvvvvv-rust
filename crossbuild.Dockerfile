# build linux dependencies: SDL, SDL mixer, physfs
FROM multiarch/crossbuild AS linux-dependencies
RUN git clone https://github.com/libsdl-org/SDL /usr/local/src/SDL; \
    cd /usr/local/src/SDL; ./configure; make; make install; \
    git clone https://github.com/libsdl-org/SDL_mixer /usr/local/src/SDL_mixer; \
    cd /usr/local/src/SDL_mixer; ./autogen.sh; ./configure; make; make install; \
    git clone https://github.com/icculus/physfs /usr/local/src/physfs; \
    cd /usr/local/src/physfs; cmake .; make; make install

# build physfs for windows
FROM mmozeiko/mingw-w64 AS physfs-w64
RUN git clone https://github.com/icculus/physfs /usr/local/src/physfs; \
    cd /usr/local/src/physfs; \
    cmake -DCMAKE_SYSTEM_NAME=Windows \
          -DCMAKE_INSTALL_PREFIX=${MINGW} \
          -DCMAKE_FIND_ROOT_PATH=${MINGW} \
          -DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER \
          -DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY \
          -DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY \
          -DCMAKE_C_COMPILER=x86_64-w64-mingw32-gcc \
          -DCMAKE_CXX_COMPILER=x86_64-w64-mingw32-g++ \
          -DCMAKE_RC_COMPILER=x86_64-w64-mingw32-windres \
          .; \
    make; make install

# build physfs for mac
FROM multiarch/crossbuild AS physfs-darwin
ENV CROSS_TRIPLE="x86_64-apple-darwin"
RUN git clone https://github.com/icculus/physfs /usr/local/src/physfs; \
    cd /usr/local/src/physfs; \
    crossbuild cmake -mmacosx-version-min=10.7 .; \
    make; make install

################################################################################
################################## main ########################################
################################################################################

FROM multiarch/crossbuild
RUN apt update; apt upgrade -y; apt install -y clang-3.9

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
    ~/.cargo/bin/rustup install stable; \
    ~/.cargo/bin/rustup default stable; \
    ~/.cargo/bin/rustup target install x86_64-apple-darwin; \
    ~/.cargo/bin/rustup target install x86_64-pc-windows-gnu
ENV PATH=/root/.cargo/bin:$PATH

RUN git clone https://github.com/icculus/physfs /usr/local/src/physfs

# copy linux dependencies
COPY --from=linux-dependencies /usr/local/lib /usr/local/lib/x86_64-unknown-linux-gnu

# copy mac dependencies
COPY --from=physfs-darwin /usr/local/lib /usr/local/lib/x86_64-apple-darwin
COPY ./crossbuild/libSDL/libSDL2-2.0.0.dylib /usr/local/lib/x86_64-apple-darwin/libSDL2-2.0.0.dylib
COPY ./crossbuild/libSDL/libSDL2_mixer-2.0.0.dylib /usr/local/lib/x86_64-apple-darwin/libSDL2_mixer-2.0.0.dylib
RUN ln -s /usr/local/lib/x86_64-apple-darwin/libSDL2-2.0.0.dylib /usr/local/lib/x86_64-apple-darwin/libSDL2.dylib; \
    ln -s /usr/local/lib/x86_64-apple-darwin/libSDL2_mixer-2.0.0.dylib /usr/local/lib/x86_64-apple-darwin/libSDL2_mixer.dylib

# copy windows dependencies
COPY --from=physfs-w64 ./mingw/lib /usr/local/lib/x86_64-pc-windows-gnu
COPY --from=physfs-w64 /mingw/bin/libphysfs.dll /tmp/
RUN cd /tmp/; \
    wget https://www.libsdl.org/release/SDL2-devel-2.0.16-mingw.tar.gz; \
    tar -xvf SDL2-devel-2.0.16-mingw.tar.gz; \
    cp SDL2-2.0.16/x86_64-w64-mingw32/lib/libSDL2.* /usr/local/lib/x86_64-pc-windows-gnu/; \
    wget https://libsdl.org/projects/SDL_mixer/release/SDL2_mixer-devel-2.0.4-mingw.tar.gz; \
    tar -xvf SDL2_mixer-devel-2.0.4-mingw.tar.gz; \
    cp SDL2_mixer-2.0.4/x86_64-w64-mingw32/lib/libSDL2_mixer.* /usr/local/lib/x86_64-pc-windows-gnu/
