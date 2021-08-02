#![allow(missing_docs)]

use crate::context::ContextBuilder;

use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::{GLContext, HasContext, HasGLContext};

/// Web Contexts doesn't emit any error.
pub type ContextError = ();

// ===============
// === Context ===
// ===============

//TODO: Context should be generalized for both desktop and web. What if we create a callback
// based render loop and remove the method `run`? How to make it far more generalized and
// consider a loop for when the Context doesn't need to swap buffers and is only used for computing?

pub struct Context {
    gl     : GLContext,
    canvas : HtmlCanvasElement
}

impl Context {
    pub fn from_canvas(canvas: HtmlCanvasElement) -> Self {
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("get_context failed")
            .expect("Couldn't get WebGL2 context")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Couldn't convert WebGl2RenderingContext");
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        let gl = GLContext::from_glow_context(gl);
        Self { gl, canvas }
    }
}

impl HasGLContext for Context {
    fn gl_context(&self) -> GLContext {
        self.gl.clone()
    }
}

impl HasContext for Context {
    fn new(_builder:&ContextBuilder) -> Self {
        let document = web_sys::window()
            .expect("Couldn't get window")
            .document()
            .expect("Couldn't get document");
        let canvas = document
            .create_element("canvas")
            .expect("Couldn't create canvas")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Couldn't convert HtmlCanvasElement");
        let body = document
            .body()
            .expect("Couldn't get body");
        body.append_with_node_1(&canvas).expect("Couldn't append canvas");
        Self::from_canvas(canvas)
    }

    fn run(&mut self) -> bool {
        false
    }

    fn make_current(&self) -> Result<(), ()> {
        Ok(())
    }

    fn swap_buffers(&self) -> Result<(), ()> {
        Ok(())
    }

    fn get_proc_address(&self, _addr: &str) -> *const () {
        std::ptr::null()
    }

    fn resolution(&self) -> (usize, usize) {
        (self.canvas.width() as usize, self.canvas.height() as usize)
    }
}
