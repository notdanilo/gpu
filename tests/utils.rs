#[cfg(target_arch = "wasm32")]
mod platform {
    pub use wasm_bindgen_test::wasm_bindgen_test_configure as web_configure;
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
    web_configure!(run_in_browser);
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use test as test;
}

pub use platform::test;