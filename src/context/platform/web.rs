#![allow(missing_docs)]

use crate::context::ContextBuilder;

use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Web Contexts doesn't emit any error.
pub type ContextError = ();

// ===============
// === Context ===
// ===============

//TODO: pub gl : glow::Context is redefined both in platform/web.rs and platform/desktop.rs. They
// should have a base object for it.
//TODO: Context should be generalized for both desktop and web. What if we create a callback
// based render loop and remove the method `run`? How to make it far more generalized and
// consider a loop for when the Context doesn't need to swap buffers and is only used for computing?

pub struct ContextData {
    pub(crate) gl : glow::Context,
    canvas        : HtmlCanvasElement
}

pub struct WeakContext {
    pub(crate) data : Weak<RefCell<ContextData>>
}

impl WeakContext {
    pub fn upgrade(&self) -> Option<Context> {
        self.data.upgrade().map(|data| {
            Context { data }
        })
    }
}

pub struct Context {
    pub data : Rc<RefCell<ContextData>>
}

impl Context {
    pub fn weak(&self) -> WeakContext {
        WeakContext { data: Rc::downgrade(&self.data) }
    }

    pub fn new(_builder:&ContextBuilder) -> Self {
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

    pub fn from_canvas(canvas: HtmlCanvasElement) -> Self {
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("get_context failed")
            .expect("Couldn't get WebGL2 context")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .expect("Couldn't convert WebGl2RenderingContext");
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        let data = Rc::new(RefCell::new(ContextData {gl,canvas}));
        Self { data }
    }

    pub fn run(&mut self) -> bool {
        false
    }

    pub fn make_current(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn swap_buffers(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn get_proc_address(&self, _addr: &str) -> *const () {
        std::ptr::null()
    }

    pub fn inner_dimensions(&self) -> (usize, usize) {
        (self.data.borrow().canvas.width() as usize, self.data.borrow().canvas.height() as usize)
    }
}
