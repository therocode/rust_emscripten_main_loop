/// This enum is returned by each iteration of the [main loop](MainLoop::main_loop) and indicates if the looping should continue or terminate.
pub enum MainLoopEvent {
    Continue,
    Terminate,
}

#[cfg(target_os = "emscripten")]
use super::emscripten::*;

/// A trait that abstracts a main loop mechanism that can be used on native targets as well as the wasm32-unknown-emscripten target.
/// The purpose of this is to get around the browser limitations that prevent you
/// from just naively looping, which would cause the browser tab to be frozen/killed.
/// Use this by implementing the trait on the data object that contains all data that should be available to the main loop
pub trait MainLoop {
    /// Will be run once per loop iteration. Functionally equivalent to the body of a loop statement.
    /// Use the return value to indicate if the loop should continue or terminate.
    fn main_loop(&mut self) -> MainLoopEvent;
}

/// Use this function to initiate the looping.
/// Note that Emscripten only simulates looping - in reality the looping is done through scheduling. Due to this, to prevent code beyond the loop
/// scheduling to be run, it has to terminate the code flow. This means that in your web builds, the code after this run function will never be executed, even if the looping is terminated.
/// For more information, consult the [Emscripten docs](https://emscripten.org/docs/api_reference/emscripten.h.html#c.emscripten_set_main_loop)
///
/// # Arguments
///
/// * `looper` - The data object that will be looped upon. Due to lifetime reasons, the looper object is consumed so that it can be kept alive during the looping.
pub fn run<T: 'static + MainLoop>(mut looper: T) {
    #[cfg(not(target_os = "emscripten"))]
    let quit_cell = std::cell::RefCell::new(false);
    #[cfg(not(target_os = "emscripten"))]
    let should_quit = || *quit_cell.borrow();
    let quit = || {
        #[cfg(not(target_os = "emscripten"))]
        {
            *quit_cell.borrow_mut() = true;
        }
    };

    #[cfg(target_os = "emscripten")]
    let loop_closure;
    #[cfg(not(target_os = "emscripten"))]
    let mut loop_closure;

    loop_closure = move || match looper.main_loop() {
        MainLoopEvent::Continue => MainLoopEvent::Continue,
        MainLoopEvent::Terminate => {
            quit();
            MainLoopEvent::Terminate
        }
    };

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(loop_closure);

    #[cfg(not(target_os = "emscripten"))]
    loop {
        loop_closure();
        if should_quit() {
            break;
        }
    }
}
