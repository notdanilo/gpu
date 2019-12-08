use crate::Resource;
use crate::data::Texture2D;
use crate::data::Renderbuffer;

enum FramebufferAttachment {
    Texture(Texture2D),
    Renderbuffer(Renderbuffer),
    None
}

pub struct Framebuffer {
    id : u32,
    dimension : (usize, usize),
    color: FramebufferAttachment,
    _depth: FramebufferAttachment,
    _stencil: FramebufferAttachment,
}

// Incomplete implementation
// 1. Lacks default renderbuffers for depth and stencil testing
// 2. Lacks depth and stencil implementation for textures
// 3. Lacks framebuffer completeness test
// 4. Lacks checking for returning Result::Err
// 5. Check attachment dimensions (does framebuffer completeness check takes that into account?)

impl Framebuffer {
    pub fn default() -> Self {
        let dimension = unsafe {
            let mut viewport : [i32; 4] = [0, 0, 0, 0];
            gl::GetIntegerv(gl::VIEWPORT, &mut viewport[0] as *mut i32);
            (viewport[2] as usize, viewport[3] as usize)
        };

        Self {
            id : 0,
            dimension,
            color : FramebufferAttachment::Renderbuffer(Renderbuffer::default()),
            _depth : FramebufferAttachment::Renderbuffer(Renderbuffer::default()),
            _stencil : FramebufferAttachment::Renderbuffer(Renderbuffer::default())
        }
    }

    pub fn new(color: Option<Texture2D>, depth: Option<Texture2D>, stencil: Option<Texture2D>) -> Result<Self, String> {
        let mut id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
        }

        let mut dimension = (0, 0);

        let color = match color {
            Some(texture) => {
                dimension = texture.get_dimension();
                unsafe {
                    gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.get_id(), 0);
                }

                FramebufferAttachment::Texture(texture)
            },
            None => FramebufferAttachment::None
        };
        let depth = match depth {
            Some(texture) => FramebufferAttachment::Texture(texture),
            None => FramebufferAttachment::None
        };
        let stencil = match stencil {
            Some(texture) => FramebufferAttachment::Texture(texture),
            None => FramebufferAttachment::None
        };

        Ok(Self {
            id,
            color,
            dimension,
            _depth : depth,
            _stencil : stencil
        })
    }

    pub fn get_dimension(&self) -> (usize, usize) { self.dimension }

    pub fn get_color(&self) -> Option<&Texture2D> {
        match &self.color {
            FramebufferAttachment::Texture(texture) => Some(&texture),
            _ => None
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.get_id());
        }
    }
}

impl Resource for Framebuffer {
    fn get_id(&self) -> u32 { self.id }
}
