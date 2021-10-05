use glow::Context;

pub struct RenderTarget<'a> {
    context: &'a Context,
    size: (u32, u32),
}

impl<'a> RenderTarget<'a> {
    pub fn new(context: &'a Context, size: (u32, u32)) -> Self {
        Self { context, size }
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.size
    }
}
