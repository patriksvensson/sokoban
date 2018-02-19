use engine::*;
use renderer;
use renderer::*;

use glium;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {
        return Game {
            renderer: renderer::new(engine)
        }
    }
}

impl GameApplication for Game 
{
    fn update(&mut self, _dt: f64) {
        // TODO: Perform update.
    }

    fn draw(&mut self, target: &mut glium::Frame) 
    {
        self.renderer.draw_quad(target, renderer::Rect {
            x: 32.0,
            y: 32.0,
            w: 32.0,
            h: 32.0
        });

        self.renderer.draw_quad(target, renderer::Rect {
            x: 128.0,
            y: 128.0,
            w: 64.0,
            h: 64.0
        });
    }
}