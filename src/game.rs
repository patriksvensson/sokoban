use engine::*;
use glium;
use glium::{Surface};

pub struct Game {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {
        implement_vertex!(Vertex, position);

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&engine.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&engine.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        return Game {
            program,
            vertex_buffer,
            indices
        }
    }
}

impl GameApplication for Game 
{
    fn draw(&mut self, target: &mut glium::Frame) 
    {
        target.draw(
            &self.vertex_buffer,
            &self.indices,
            &self.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default())
                .unwrap();
    }
}