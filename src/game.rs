use engine::*;

use glium;
use glium::{Surface};
use glium::index::PrimitiveType;

pub struct Game {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {

        // Build the vertex buffer for the quad.
        let vertex_buffer = {
            implement_vertex!(Vertex, position, color);

            glium::VertexBuffer::new(&engine.display,
                &[
                    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                    Vertex { position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0] },
                    Vertex { position: [ 0.5,  0.5], color: [1.0, 0.0, 0.0] },
                    Vertex { position: [-0.5,  0.5], color: [1.0, 1.0, 0.0] },
                ]
            ).unwrap()
        };

        // Build the index buffer.
        let index_buffer = glium::IndexBuffer::new(&engine.display, PrimitiveType::TrianglesList,
                                                &[0u16, 1, 2, 2, 3, 0]).unwrap();


        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            in vec3 color;
            out vec3 vColor;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                vColor = color;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec3 vColor;
            out vec4 color;
            void main() {
                color = vec4(vColor, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&engine.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        return Game {
            program,
            vertex_buffer,
            index_buffer
        }
    }
}

impl GameApplication for Game 
{
    fn update(&mut self, _dt: f64) {

    }

    fn draw(&mut self, target: &mut glium::Frame) 
    {
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default())
                .unwrap();
    }
}