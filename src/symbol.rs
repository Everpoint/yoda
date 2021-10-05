mod circle;
pub use circle::*;

mod line;
pub use line::LineSymbol;

mod polygon;
pub use polygon::PolygonSymbol;
use glow::{Context, Program};

pub trait Symbol<G> {
    type Vertex;

    fn vertex_shader(&self) -> &str;
    fn fragment_shader(&self) -> &str;

    fn compile(&mut self, context: &Context);
    fn program(&self) -> &Program;
    fn convert(&self, geometry: &G) -> (Vec<Self::Vertex>, Option<Vec<u32>>);
}