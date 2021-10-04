use glium::{Program, Display};

mod circle;
pub use circle::*;

pub trait Symbol<G> {
    type Vertex: glium::Vertex;

    fn compile(&mut self, display: &Display);
    fn program(&self) -> &Program;
    fn convert(&self, geometry: &G) -> Vec<Self::Vertex>;
}
