This crate is deprecated please use [shaderc-rs](https://github.com/google/shaderc-rs) instead.


BEVY NOTE: This crate is a temporary measure until native rust shader compilation like https://github.com/gfx-rs/naga lands.

# Targets requiring build-from-source

`glslang` will be built from source the first time. Compiled libraries are re-used afterwards.

[cmake](https://cmake.org/download/) is required to build from source.

The `glslang` submodule is assumed to be initialized.
Run `git submodule update --init` if you're checking out from git.

NOTE: There is an additional commit not from upstream that changes one file.
This change is what allows gnu toolchains to build.

## `i686-pc-windows-msvc`
- MSVC Windows host (either 32 or 64-bit)
- [VS C++ Build Tools](https://aka.ms/buildtools) (2017 or higher)
  - Select **Visual C++ build tools**
  - Make sure **Visual C++ tools for CMake** is ticked on the right
  - Restart the computer after installing build tools - will fail to build otherwise

## `i686-pc-windows-gnu`, `x86_64-pc-windows-gnu`
- MSYS2 or MinGW
- Install the appropriate toolchain (e.g. `pacman -S mingw-w64-x86_64-toolchain mingw-w64-i686-toolchain`)
- Add bin directories to your PATH (e.g. `C:\msys64\mingw64\bin`)
