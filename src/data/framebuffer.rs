use crate::data::Texture2D;
use crate::data::Renderbuffer;
use crate::{Context, WeakContext};
use glow::HasContext;

type FramebufferResource = <glow::Context as HasContext>::Framebuffer;

enum FramebufferAttachment {
    Texture(Texture2D),
    Renderbuffer(Renderbuffer),
    None
}

/// A Framebuffer representation with optional `color`, `depth` and `stencil` attachments.
pub struct Framebuffer {
    context    : WeakContext,
    resource   : FramebufferResource,
    dimensions : (usize, usize),
    color      : FramebufferAttachment,
    _depth     : FramebufferAttachment,
    _stencil   : FramebufferAttachment
}

//FIXME: Incomplete implementation
// 1. Lacks default renderbuffers for depth and stencil testing
// 2. Lacks depth and stencil implementation for textures
// 3. Lacks framebuffer completeness test
// 4. Lacks checking for returning Result::Err
// 5. Check attachment dimensions (does framebuffer completeness check takes that into account?)

impl Framebuffer {
    /// The default `Framebuffer` created during the `Context` creation.
    pub fn default(context:&Context) -> Self {
        let dimensions = context.inner_dimensions();
        let resource   = Default::default();
        let color      = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _depth     = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _stencil   = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let context = context.weak();
        Self {context,resource,dimensions,color,_depth,_stencil}
    }

    pub(crate) fn resource(&self) -> FramebufferResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        self.context.upgrade().map(|context| {
            let gl       = &context.data.borrow().gl;
            let resource = self.resource();
            let resource = if resource == Default::default() { None } else { Some(resource) };
            unsafe {
                gl.bind_framebuffer(glow::FRAMEBUFFER, resource);
            }
        });
    }

    /// Creates a new `Framebuffer` with optional `color`, `depth` and `stencil`.
    pub fn new
    (context:&Context, color: Option<Texture2D>,
     depth:Option<Texture2D>, stencil:Option<Texture2D>) -> Result<Self,
        String> {
        let gl       = &context.data.borrow().gl;
        let resource = unsafe {
            let resource = gl.create_framebuffer().expect("Couldn't create Framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(resource));
            resource
        };
        let mut dimensions = (0, 0);

        let color = match color {
            Some(texture) => {
                dimensions = texture.dimensions();
                unsafe {
                    gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0,
                                              glow::TEXTURE_2D, Some(texture.resource()), 0);
                }
                FramebufferAttachment::Texture(texture)
            },
            None => FramebufferAttachment::None
        };
        let _depth = match depth {
            Some(texture) => FramebufferAttachment::Texture(texture),
            None => FramebufferAttachment::None
        };
        let _stencil = match stencil {
            Some(texture) => FramebufferAttachment::Texture(texture),
            None => FramebufferAttachment::None
        };

        let context = context.weak();
        Ok(Self {context,resource,dimensions,color,_depth,_stencil})
    }

    /// Gets the `Framebuffer`'s dimension.
    pub fn dimensions(&self) -> (usize, usize) { self.dimensions }

    /// Returns the `Texture2D` used as the `ColorBuffer` if any.
    pub fn color(&self) -> Option<&Texture2D> {
        match &self.color {
            FramebufferAttachment::Texture(texture) => Some(&texture),
            _ => None
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.context.upgrade().map(|context| {
            unsafe {
                context.data.borrow().gl.delete_framebuffer(self.resource());
            }
        });
    }
}