use crate::{Color, Polygon, PolygonRef};
use crate::symbol::Symbol;
use lyon::tessellation::{VertexBuffers, FillTessellator, FillOptions, FillRule, FillVertexConstructor, FillVertex, StrokeTessellator, StrokeOptions};
use lyon::lyon_tessellation::{BuffersBuilder};
use lyon::math::point;
use glow::Program;
use lyon::tessellation::path::Path;
use crate::symbol::line::{LineVertex, VertexCtor};

pub struct PolygonSymbol {
    pub fill_color: Color,
    pub stroke_width: f32,
    pub stroke_color: Color,
    pub program: Option<Program>
}

const VERTEX_SHADER: &str = r#"
layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in uint id;

uniform mat4 transformation;
uniform uint mode;
uniform vec2 screen_size;

out vec4 frag_color;

void main() {
    gl_Position = vec4(position.xyz, 1.0) * transformation;
    frag_color = color;
    if (mode == 1u) {
        frag_color = vec4((float(id) + 1.0) / 255.0, 0.0, 0.0, 1.0);
    }
}
"#;

const FRAGMENT_SHADER: &str = r#"
precision mediump float;

in vec4 frag_color;
out vec4 FragColor;

void main() {
    FragColor = frag_color;
}
"#;

impl FillVertexConstructor<LineVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> LineVertex {
        let point = vertex.position();
        LineVertex {
            position: [point.x, point.y, 0.0],
            color: self.color,
            id: self.id,
        }
    }
}

impl Symbol<Polygon> for PolygonSymbol {
    type Vertex = LineVertex;

    fn vertex_shader(&self) -> &str {
        VERTEX_SHADER
    }

    fn fragment_shader(&self) -> &str {
        FRAGMENT_SHADER
    }

    fn set_program(&mut self, program: Program) {
        self.program = Some(program)
    }

    fn program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    fn convert(&self, geometry: &Polygon, id: u32) -> (Vec<Self::Vertex>, Option<Vec<u32>>) {

        let path = build_geometry(geometry);
        let mut buffers: VertexBuffers<LineVertex, u32> = VertexBuffers::new();

        let mut fill_vertex_builder = BuffersBuilder::new(&mut buffers, VertexCtor {color: self.fill_color, id});
        let mut fill_tessellator = FillTessellator::new();

        fill_tessellator.tessellate_path(
            &path,
            &FillOptions::default().with_fill_rule(FillRule::EvenOdd),
            &mut fill_vertex_builder
        ).unwrap();

        let mut stroke_vertex_builder = BuffersBuilder::new(&mut buffers, VertexCtor {color: self.stroke_color, id});
        let mut stroke_tessellator = StrokeTessellator::new();

        stroke_tessellator.tessellate_path(
            &path,
        &StrokeOptions::default().with_line_width(self.stroke_width),
            &mut stroke_vertex_builder
        ).unwrap();

        let VertexBuffers {vertices, indices} = buffers;
        (vertices, Some(indices))
    }
}

fn build_geometry(geometry: &PolygonRef) -> Path {

    let mut path_builder = Path::builder();

    for contour in geometry {
        path_builder.begin(point(contour[0][0], contour[0][1]));
        for p in contour.iter().skip(1) {
            path_builder.line_to(point(p[0], p[1]));
        }
        path_builder.end(true);
    }

    path_builder.build()
}
