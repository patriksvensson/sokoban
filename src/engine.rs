extern crate glium;

use std::time::{Instant};
use self::glium::{glutin};

use utils;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2]
}

pub trait GameApplication {
    fn update(&mut self, dt: f64, keys : [bool; 100]);
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
        let mut keys : [bool; 100] = [false; 100];

        loop {
            // Calculate the number of seconds since last frame.
            let dt = utils::duration_to_secs(Instant::now() - self.last_frame_time);

            // Process events.
            self.events.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => return,
                        glutin::WindowEvent::KeyboardInput {device_id: _, input} =>{
                            match input.state {
                                glutin::ElementState::Pressed =>
                                    keys[input.scancode as usize] = true,
                                glutin::ElementState::Released =>
                                    keys[input.scancode as usize] = false,
                            }
                        },
                        _ => ()
                    },
                    _ => (),
                }
            });

            // Update
            app.update(dt, keys);
            self.last_frame_time = Instant::now();

            // Draw
            let mut target = self.display.draw();
            app.draw(&mut target);
            target.finish().unwrap();
        }
    }
}