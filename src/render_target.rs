use glium::{Display, Frame};

pub struct RenderTarget<'a> {
    display: &'a Display,
    frame: &'a mut Frame,
}

impl<'a> RenderTarget<'a> {
    pub fn new(display: &'a Display, frame: &'a mut Frame) -> Self {
        Self {display, frame}
    }

    pub fn display(&self) -> &Display {
        self.display
    }

    pub fn frame(&mut self) -> &mut Frame {
        self.frame
    }
}
