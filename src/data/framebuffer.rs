use crate::data::Texture2D;
use crate::data::Renderbuffer;
use crate::Context;
use glow::HasContext;

type FramebufferResource = <glow::Context as HasContext>::Framebuffer;

enum FramebufferAttachment<'context> {
    Texture(Texture2D<'context>),
    Renderbuffer(Renderbuffer<'context>),
    None
}

/// A Framebuffer representation with optional `color`, `depth` and `stencil` attachments.
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
    /// The default `Framebuffer` created during the `Context` creation.
    pub fn default(context:&'context Context) -> Self {
        let dimensions = context.inner_dimensions();
        let resource   = Default::default();
        let color      = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _depth     = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _stencil   = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        Self {context,resource,dimensions,color,_depth,_stencil}
    }

    pub(crate) fn resource(&self) -> FramebufferResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        let gl       = &self.context.gl;
        let resource = self.resource();
        let resource = if resource == Default::default() { None } else { Some(resource) };
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, resource);
        }
    }

    /// Creates a new `Framebuffer` with optional `color`, `depth` and `stencil`.
    pub fn new
    (context:&'context Context, color: Option<Texture2D<'context>>,
     depth:Option<Texture2D<'context>>, stencil:Option<Texture2D<'context>>) -> Result<Self,
        String> {
        let gl       = &context.gl;
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

impl<'context> Drop for Framebuffer<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_framebuffer(self.resource());
        }
    }
}