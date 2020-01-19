//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

pub use wasm_bindgen_test::wasm_bindgen_test_configure as web_configure;
web_configure!(run_in_browser);

//TODO:src/context.rs tests to tests/context.rs?

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use gpu::ContextBuilder;
    use gpu::ContextDisplay;

    #[wasm_bindgen_test]
    fn create_context() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        //context.make_current().expect("Couldn't make current");
    }
}
