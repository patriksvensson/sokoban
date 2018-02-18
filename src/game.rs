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
        // TODO: Add coordinates and size of quad.
        self.renderer.draw_quad(target);
    }
}