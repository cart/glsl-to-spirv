This crate is deprecated please use [shaderc-rs](https://github.com/google/shaderc-rs) instead.


BEVY NOTE: This crate is a temporary measure until native rust shader compilation like https://github.com/gfx-rs/naga lands.

---

## Additional Dependencies for `i686-pc-windows-msvc`

Assuming an MSVC Windows host (either 32 or 64-bit):
- git
- [cmake](https://cmake.org/download/)
- [VS C++ Build Tools](https://aka.ms/buildtools) (2017 or higher)
  - Select **Visual C++ build tools**
  - Make sure **Visual C++ tools for CMake** is ticked on the right
  - Restart the computer after installing build tools - will fail to build otherwise
- `glslang` submodule is assumed to be initialized
  - Run `git submodule update --init` if you're checking out from git

`glslang` will be built from source the first time. Compiled libraries are re-used afterwards.