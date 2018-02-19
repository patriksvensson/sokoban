#[macro_use]
extern crate glium;

mod engine;
mod game;
mod map;
mod renderer;
mod utils;

fn main() {
    let mut engine = engine::Engine::new();
    let game = &mut game::Game::new(&mut engine);
    
    engine.run(game);
}