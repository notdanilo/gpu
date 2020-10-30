use crate::data::Texture2D;
use crate::data::Renderbuffer;
use crate::{Context, GLContext};


type FramebufferResource = u32;

enum FramebufferAttachment {
    Texture(Texture2D),
    Renderbuffer(Renderbuffer),
    None
}

/// A Framebuffer representation with optional `color`, `depth` and `stencil` attachments.
pub struct Framebuffer {
    gl         : GLContext,
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
        let gl         = context.gl_context();
        Self { gl, resource, dimensions, color, _depth, _stencil }
    }

    pub(crate) fn resource(&self) -> FramebufferResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        let resource = self.resource();
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, resource);
        }
    }

    /// Creates a new `Framebuffer` with optional `color`, `depth` and `stencil`.
    pub fn new(context:&Context, color: Option<Texture2D>, depth:Option<Texture2D>, stencil:Option<Texture2D>) -> Result<Self, String> {
        let gl = context.gl_context();
        let resource = unsafe {
            let mut resource = 0;
            gl::CreateFramebuffers(1, &mut resource);
            gl::BindFramebuffer(gl::FRAMEBUFFER, resource);
            resource
        };
        let mut dimensions = (0, 0);

        let color = match color {
            Some(texture) => {
                dimensions = texture.dimensions();
                unsafe {
                    gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
                                             gl::TEXTURE_2D, texture.resource(), 0);
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

        Ok(Self {gl, resource, dimensions, color, _depth, _stencil})
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
        unsafe {
            gl::DeleteFramebuffers(1, &self.resource());
        }
    }
}