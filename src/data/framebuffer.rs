//use crate::data::Texture2D;
use crate::data::Renderbuffer;
use crate::Context;
use glow::HasContext;

type FramebufferResource = <glow::Context as HasContext>::Framebuffer;

enum FramebufferAttachment<'context> {
//    Texture(Texture2D),
    Renderbuffer(Renderbuffer<'context>),
    None
}

pub struct Framebuffer<'context> {
    context    : &'context Context,
    resource   : FramebufferResource,
    dimensions : (usize, usize),
    color      : FramebufferAttachment<'context>,
    _depth     : FramebufferAttachment<'context>,
    _stencil   : FramebufferAttachment<'context>
}

//FIXME: Incomplete implementation
// 1. Lacks default renderbuffers for depth and stencil testing
// 2. Lacks depth and stencil implementation for textures
// 3. Lacks framebuffer completeness test
// 4. Lacks checking for returning Result::Err
// 5. Check attachment dimensions (does framebuffer completeness check takes that into account?)

impl<'context> Framebuffer<'context> {
    pub fn default(context:&'context Context) -> Self {
        let gl         = &context.gl;
        let dimensions = context.inner_dimensions();
        let resource   = Default::default();
        let color      = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _depth     = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _stencil   = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        Self {context,resource,dimensions,color,_depth,_stencil}
    }

//    pub fn new(color: Option<Texture2D>, depth: Option<Texture2D>, stencil: Option<Texture2D>) -> Result<Self, String> {
//        let mut id = 0;
//        unsafe {
//            gl::GenFramebuffers(1, &mut id);
//            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
//        }
//
//        let mut dimension = (0, 0);
//
//        let color = match color {
//            Some(texture) => {
//                dimension = texture.get_dimension();
//                unsafe {
//                    gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.get_id(), 0);
//                }
//
//                FramebufferAttachment::Texture(texture)
//            },
//            None => FramebufferAttachment::None
//        };
//        let depth = match depth {
//            Some(texture) => FramebufferAttachment::Texture(texture),
//            None => FramebufferAttachment::None
//        };
//        let stencil = match stencil {
//            Some(texture) => FramebufferAttachment::Texture(texture),
//            None => FramebufferAttachment::None
//        };
//
//        Ok(Self {
//            id,
//            color,
//            dimension,
//            _depth : depth,
//            _stencil : stencil
//        })
//    }

    pub fn resource(&self) -> FramebufferResource { self.resource }

    pub fn dimensions(&self) -> (usize, usize) { self.dimensions }

//    pub fn get_color(&self) -> Option<&Texture2D> {
//        match &self.color {get_dimension
//            FramebufferAttachment::Texture(texture) => Some(&texture),
//            _ => None
//        }
//    }
}

impl<'context> Drop for Framebuffer<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_framebuffer(self.resource());
        }
    }
}