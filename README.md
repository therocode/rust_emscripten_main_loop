# Emscripten Main Loop Helper Library

[![crates.io](https://img.shields.io/crates/v/emscripten_main_loop.svg)](https://crates.io/crates/emscripten_main_loop)

[Documentation](https://docs.rs/emscripten_main_loop)

## Purpose

Many interactive applications such as games will utilise the technique of having a main loop. This loop will represent one "tick" of the simulation and often also capture input and render output. However, the typical implementation of this technique (simple loop/while statement) breaks down when targeting [Emscripten](https://emscripten.org/) since naive looping will cause the browser tab to freeze up. This library provides a simple trait to use as a replacement for that loop statement and will make sure that the looping is happening in the way that Emscripten needs it to.

## Usage

Add `emscripten_main_loop` as a dependency to your `Cargo.toml`.

Implement the `emscripten_main_loop::MainLoop` trait for your application object that contains all the data that should be accessible to the main loop. The trait will require you to implement the `main_loop` function which is where you are meant to put your looping logic. `main_loop` will be called once per loop iteration and must return either `MainLoopEvent::Continue` or `MainLoopEvent::Terminate` as appropriate.

When the trait is implemented, you can invoke the looping by passing your data object to `emscripten_main_loop::run()`. See the [documentation](https://docs.rs/emscripten_main_loop) for further information, or check out [this example project](https://github.com/therocode/rust_sdl2_opengl_emscripten) as a reference, which uses this library.

## I don't know how to use Emscripten yet

For a comprehensive guide on how to use Emscripten to build a project using Rust+SDL2+OpenGL for the web, check out my [blog post](https://blog.therocode.net/2020/10/a-guide-to-rust-sdl2-emscripten).

## I think this library can be improved

Feel free to raise issues or pull requests as desired, contributions would be appreciated.
