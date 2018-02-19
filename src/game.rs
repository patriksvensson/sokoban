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
        for x in 0..20 {
            for y in 0..15 {
                self.renderer.draw_quad(target, renderer::Rect {
                    x: x as f32 * 32.0,
                    y: y as f32 * 32.0,
                    w: 32.0,
                    h: 32.0
                });
            }
        }
    }
}