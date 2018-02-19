extern crate glium;

use std::time::{Instant, Duration};
use self::glium::{glutin};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub trait GameApplication {
    fn update(&mut self, dt: f64);
    fn draw(&mut self, target: &mut glium::Frame);
}

pub struct Engine {
    pub display: glium::Display,
    pub window_width: u32,
    pub window_height: u32,
    events: glutin::EventsLoop,
    last_frame_time : Instant,
}

impl Engine {
    pub fn new() -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title("Sokoban");

        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        return Engine {
            display,
            window_width: 640,
            window_height: 480,
            events: events_loop,
            last_frame_time: Instant::now()
        }
    }

    pub fn run(&mut self, app: &mut GameApplication) {
        let mut closed = false;
        while !closed {

            // Calculate the number of seconds since last frame.
            let current_time = Instant::now();
            let dt = duration_to_secs(current_time - self.last_frame_time);

            // Update
            app.update(dt);

            // Draw
            let mut target = self.display.draw();
            app.draw(&mut target);
            target.finish().unwrap();

            // Process events.
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

fn duration_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0
}