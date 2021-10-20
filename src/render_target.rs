use glow::{Context, HasContext};
use std::rc::Rc;

pub struct RenderTarget {
    context: Rc<Context>,
    size: (u32, u32),
}

impl RenderTarget {
    pub fn new(context: Rc<Context>, size: (u32, u32)) -> Self {
        Self { context, size }
    }

    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.size
    }

    pub fn get_virtual_context(&self, width: u32, height: u32) -> VirtualContext {
        let gl = &self.context;
        unsafe {
            let framebuffer = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));

            let texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32, height as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, None);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);

            // depth buffer goes in here

            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(texture), 0);

            gl.draw_buffers(&[glow::COLOR_ATTACHMENT0]);

            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            gl.viewport(0, 0, width as i32, height as i32);

            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                eprintln!("Something wrong");
            }

            VirtualContext {gl: gl.clone(), framebuffer, texture}
        }
    }
}

pub struct VirtualContext {
    gl: Rc<Context>,
    framebuffer: <Context as HasContext>::Framebuffer,
    texture: <Context as HasContext>::Texture,
}

impl VirtualContext {
    pub fn gl(&self) -> &Rc<Context> {
        &self.gl
    }

    pub fn pixel_value(&self) -> u32 {
        let mut buffer = [0; 4];
        let pack = glow::PixelPackData::Slice(&mut buffer);
        unsafe {
            self.gl.read_pixels(0, 0, 1, 1, glow::RGBA, glow::UNSIGNED_BYTE, pack);
        }
        let bytes = [buffer[0], buffer[1], buffer[2], 0];
        u32::from_le_bytes(bytes)
    }
}

impl Drop for VirtualContext {
    fn drop(&mut self) {
        let gl = &self.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, None);
            gl.delete_texture(self.texture);

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.delete_framebuffer(self.framebuffer);
        }
    }
}