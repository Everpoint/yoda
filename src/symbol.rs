use crate::{Color, Point, Point3};
use glium::{Program, Display};

pub struct CircleSymbol {
    pub color: Color,
    pub size: f32,
    pub program: Option<Program>,
}

impl CircleSymbol {
    const VERTEX_SHADER: &'static str = r#"
        #version 330

        layout (location = 0) in vec3 position;
        layout (location = 1) in vec4 color;

        uniform mat4 transformation;

        out vec4 frag_color;

        void main() {
            gl_Position = vec4(position.xy, 0.0, 1.0) * transformation;
            frag_color = color;
        }
    "#;

    const FRAGMENT_SHADER: &'static str = r#"
        #version 330

        in vec4 frag_color;
        out vec4 FragColor;

        void main() {
            FragColor = frag_color;
        }
    "#;

    pub fn compile(&mut self, display: &Display) {
        self.program = Some(glium::Program::from_source(display, Self::VERTEX_SHADER, Self::FRAGMENT_SHADER, None).unwrap());
    }

    pub fn program(&self) -> &Program {
        self.program.as_ref().unwrap()
    }

    pub fn convert(&self, point: &Point) -> Vec<CirclePointVertex>{
        let mut result = vec![];
        const segments: usize = 32;
        for i in 0..segments {
            result.push(CirclePointVertex {position: [point[0], point[1], 0.0], color: self.color});

            let from = (i as f32) / (segments as f32);

            let angle = std::f32::consts::PI * from * 2.0;

            let dx = self.size / 2.0 * angle.cos();
            let dy = self.size / 2.0 * angle.sin();

            result.push(CirclePointVertex {position: [point[0] + dx, point[1] + dy, 0.0], color: self.color});

            let to = (i as f32 + 1.0) / (segments as f32);
            let angle = std::f32::consts::PI * to * 2.0;

            let dx = self.size / 2.0 * angle.cos();
            let dy = self.size / 2.0 * angle.sin();

            result.push(CirclePointVertex {position: [point[0] + dx, point[1] + dy, 0.0], color: self.color});
        }

        result
    }
}

#[derive(Copy, Clone)]
pub struct CirclePointVertex {
    position: Point3,
    color: Color,
}

implement_vertex!(CirclePointVertex, position, color);
