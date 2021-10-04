use glium::{Program, Display};

mod circle;
pub use circle::*;

mod line;
pub use line::LineSymbol;

mod polygon;
pub use polygon::PolygonSymbol;

pub trait Symbol<G> {
    type Vertex: glium::Vertex;

    fn compile(&mut self, display: &Display);
    fn program(&self) -> &Program;
    fn convert(&self, geometry: &G) -> (Vec<Self::Vertex>, Option<Vec<u32>>);
}
