# Vga-Terminal

This is the Rust binding for [SDL2-Vga-Terminal](https://github.com/Raffaello/sdl2-vga-terminal) project.


## Versioning

The crate version is matching the original library that is required to use with.

## Linking

It supports only the x64 arch of the library at the moment.

### Library search path

For convinience the sub-dir `./lib/**` is included in the search path.

### SDL2 dependency

It is not possible initialize the SDL2 from rust and using FFI for some C routine using the inited SDL2.

So this FFI, won't be really be possible as originally thought, It requires to create custom FFI interface in the C API.

Or using low level SDL2 Rust initialization instead of the "wrapped managed" context.


## Implementation

At the moment is considering only the binding for the dynamic library with the C API.

The bindings might be implemented for the C++ static libray too as well.

