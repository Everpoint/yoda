use glow::Context;
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
}
