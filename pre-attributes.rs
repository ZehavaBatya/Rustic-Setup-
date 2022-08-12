use wasm_binsgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run the program, then place all logic in the following or in modules
#[wasm_bindgen]
pub fn run() {
    set_panic_hook();
    // Insert Logic-based program 
    printIn!("What is up?");
}

/// Make error messages better when using WebAssembly
fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}