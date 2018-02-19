use engine::*;
use renderer;
use renderer::*;
use map;
use map::*;

use glium;

pub struct Game {
    renderer: Renderer,
    map: Map,
    position: PlayerPosition
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {

        let map = map::load("./data/1.level");
        let position = map.start_position;

        return Game {
            renderer: renderer::new(engine),
            map: map::load("./data/1.level"),
            position
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
        // Draw the map.
        self.map.draw(target, &mut self.renderer);

        // Draw the player.PlayerPosition
        self.renderer.draw_quad(target, 
            renderer::Rect {
                    x: self.position.0 as f32 * 32.0,
                    y: self.position.1 as f32 * 32.0,
                    w: 32.0,
                    h: 32.0
                }, 
            Color(1.0, 1.0, 0.0, 1.0))
    }
}