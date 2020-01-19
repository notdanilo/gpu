use crate::context::ContextBuilder;
use crate::context::ContextDisplay;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;



// ===============
// === Context ===
// ===============

//TODO: pub gl : glow::Context is redefined both in platform/web.rs and platform/desktop.rs. They
// should have a base object for it.
//TODO: Context should be generalized for both desktop and web. What if we create a callback
// based render loop and remove the method `run`? How to make it far more generalized and
// consider a loop for when the Context doesn't need to swap buffers and is only used for computing?

pub struct Context {
    pub gl      : glow::Context
}

impl Context {
    pub fn new(builder:&ContextBuilder) -> Self {
        let document = web_sys::window()
            .expect("Couldn't get window")
            .document()
            .expect("Couldn't get document");
        let canvas = document
            .create_element("canvas")
            .expect("Couldn't create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Couldn't convert HtmlCanvasElement");
        let body = document
            .body()
            .expect("Couldn't get body");
        body.append_with_node_1(&canvas).expect("Couldn't append canvas");
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("Couldn't get WebGL2 context")
            .expect("Wtf is this second expect")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Couldn't convert WebGl2RenderingContext");
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        use glow::HasContext;
        unsafe {
            gl.clear_color(0.0, 0.0, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
        Self {gl}
    }

    pub fn run(&mut self) -> bool {
        false
    }

    pub fn make_current(&self) -> Result<(), String> {
        Err("".into())
    }

    pub fn swap_buffers(&self) -> Result<(), String> {
        Err("".into())
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        std::ptr::null()
    }

    pub fn inner_dimensions(&self) -> (usize, usize) {
        (0, 0)
    }
}
