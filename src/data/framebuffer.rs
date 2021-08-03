use crate::data::Image2D;
use crate::data::Renderbuffer;
use crate::{Context, GLContext};


type FramebufferResource = u32;

enum FramebufferAttachment {
    Image(Image2D),
    Renderbuffer(Renderbuffer),
    None
}

/// A Framebuffer representation with optional `color`, `depth` and `stencil` attachments.
pub struct Framebuffer {
    _gl: GLContext,
    resource   : FramebufferResource,
    dimensions : (usize, usize),
    color      : FramebufferAttachment,
    _depth     : FramebufferAttachment,
    _stencil   : FramebufferAttachment
}

//FIXME: Incomplete implementation
// 1. Lacks default renderbuffers for depth and stencil testing
// 2. Lacks depth and stencil implementation for image
// 3. Lacks framebuffer completeness test
// 4. Lacks checking for returning Result::Err
// 5. Check attachment dimensions (does framebuffer completeness check takes that into account?)

impl Framebuffer {
    // TODO: Make this function private and only allow to get the default Framebuffer instance
    // through context.framebuffer()
    /// The default `Framebuffer` created during the `Context` creation.
    pub fn default(context:&Context) -> Self {
        let dimensions = context.resolution();
        let resource   = Default::default();
        let color      = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _depth     = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let _stencil   = FramebufferAttachment::Renderbuffer(Renderbuffer::default(context));
        let gl         = context.gl_context();
        Self { _gl: gl, resource, dimensions, color, _depth, _stencil }
    }

    pub(crate) fn resource(&self) -> FramebufferResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        // let resource = self.resource();
        // unsafe {
        //     gl::BindFramebuffer(gl::FRAMEBUFFER, resource);
        // }
        unimplemented!()
    }

    /// Creates a new `Framebuffer` with optional `color`, `depth` and `stencil`.
    pub fn new(context:&Context, color: Option<Image2D>, depth:Option<Image2D>, stencil:Option<Image2D>) -> Result<Self, String> {
        // let gl = context.gl_context();
        // let resource = unsafe {
        //     let mut resource = 0;
        //     gl::CreateFramebuffers(1, &mut resource);
        //     gl::BindFramebuffer(gl::FRAMEBUFFER, resource);
        //     resource
        // };
        // let mut dimensions = (0, 0);
        //
        // let color = match color {
        //     Some(image) => {
        //         dimensions = image.dimensions();
        //         unsafe {
        //             gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0,
        //                                      gl::TEXTURE_2D, image.internal(), 0);
        //         }
        //         FramebufferAttachment::Image(image)
        //     }
        //     None => FramebufferAttachment::None
        // };
        // let _depth = match depth {
        //     Some(texture) => FramebufferAttachment::Image(texture),
        //     None => FramebufferAttachment::None
        // };
        // let _stencil = match stencil {
        //     Some(texture) => FramebufferAttachment::Image(texture),
        //     None => FramebufferAttachment::None
        // };
        //
        // Ok(Self { _gl: gl, resource, dimensions, color, _depth, _stencil})
        unimplemented!()
    }

    /// Gets the `Framebuffer`'s dimension.
    pub fn dimensions(&self) -> (usize, usize) { self.dimensions }

    /// Returns the `Image2D` used as the `ColorBuffer` if any.
    pub fn color(&self) -> Option<&Image2D> {
        match &self.color {
            FramebufferAttachment::Image(texture) => Some(&texture),
            _ => None
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteFramebuffers(1, &self.resource());
        // }
        unimplemented!()
    }
}