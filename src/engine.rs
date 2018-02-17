extern crate glium;

use self::glium::{glutin, Surface};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub trait GameApplication {
    fn draw(&mut self, target: &mut glium::Frame);
}

pub struct Engine {
    pub display: glium::Display,
    events: glutin::EventsLoop
}
impl Engine {
    pub fn new() -> Self {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title("Sokoban");

        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        return Engine {
            display,
            events: events_loop
        }
    }

    pub fn run(&mut self, app: &mut GameApplication) {
        let mut closed = false;
        while !closed {
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            app.draw(&mut target);
            target.finish().unwrap();

            self.events.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => closed = true,
                        _ => ()
                    },
                    _ => (),
                }
            });
        }
    }
}
