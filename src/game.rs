use engine::*;
use renderer;
use renderer::*;
use map;
use map::*;
use utils;

use glium;

const KEY_UP : usize = 72;
const KEY_DOWN : usize = 80;
const KEY_LEFT : usize = 75;
const KEY_RIGHT : usize = 77;

pub struct Game {
    renderer: Renderer,
    map: Map,
    position: Position,
    last_update: f64
}

#[derive(Copy, Clone)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
pub enum State {
    Playing,
    Won
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {

        let map = map::load(1);
        let position = map.start_position;

        return Game {
            renderer: renderer::new(engine),
            map: map,
            position,
            last_update: 0.0
        }
    }

    pub fn get_box_index(&self, pos: Position) -> i8 {
        return match self.map.boxes.iter().position(|&s| s.0 == pos.0 && s.1 == pos.1) {
            None => -1i8,
            Some(n) => n as i8
        }
    }
}

impl GameApplication for Game 
{
    fn update(&mut self, dt: f64, keys : [bool; 100]) {

        // Update the play state.
        if get_play_state(&self.map) == State::Won {
            // Load the next level.
            self.map = map::load(self.map.level + 1);
            self.position = self.map.start_position;
        }

        self.last_update = self.last_update + dt;
        if self.last_update < 0.18 
        {
            return;
        }

        // Get movement from keyboard.
        let movement = if keys[KEY_RIGHT] {
            Movement::Left
        }
        else if keys[KEY_LEFT] {
            Movement::Right
        }
        else if keys[KEY_UP] {
            Movement::Up
        }
        else if keys[KEY_DOWN] {
            Movement::Down
        }
        else {
            // No keyboard update.
            return;
        };

        // Reset the timer.
        self.last_update = 0.0;

        // Calculate the new position for the player.
        let new_player_pos = get_new_position(self.position, movement);
        if !can_move(&mut self.map, new_player_pos) {
            return;
        }

        // Are we pushing a box?
        let box_index = self.get_box_index(new_player_pos);
        if box_index != -1 {
            let new_box_pos = get_new_position(self.map.boxes[box_index as usize], movement);
            if self.map.tiles[new_box_pos.0 as usize][new_box_pos.1 as usize] == map::Tile::Wall {
                // Can't move box because of wall.
                return;
            }
            if self.get_box_index(new_box_pos) != -1 {
                // Another box is blocking the way.
                return;
            }

            // Update the box position.
            self.map.boxes[box_index as usize].0 = new_box_pos.0;
            self.map.boxes[box_index as usize].1 = new_box_pos.1;
        }

        //
        let x = utils::clamp(new_player_pos.0, 0, 19);
        let y = utils::clamp(new_player_pos.1, 0, 14);

        // Update the player position.
        self.position = (x, y);
    }

    fn draw(&mut self, target: &mut glium::Frame) 
    {
        // Draw the map.
        self.map.draw(target, &mut self.renderer);

        // Draw the player.
        self.renderer.draw_quad(target, 
            renderer::Rect {
                    x: self.position.0 as f32 * 32.0,
                    y: self.position.1 as f32 * 32.0,
                    w: 32.0,
                    h: 32.0
                }, 
            Color(1.0, 1.0, 0.0, 1.0));
    }
}

fn get_new_position(position: Position, movement: Movement) -> Position {
    return match movement {
        Movement::Right => (position.0 - 1, position.1),
        Movement::Left => (position.0 + 1, position.1),
        Movement::Up => (position.0, position.1 - 1),
        Movement::Down => (position.0, position.1 + 1)
    };
}

fn can_move(map: &Map, position: Position) -> bool {
    return map.tiles[position.0 as usize][position.1 as usize] != map::Tile::Wall;
}

fn get_play_state(map: &Map) -> State {
    // Have we won?
    for item in map.boxes.iter() {
        if map.tiles[item.0 as usize][item.1 as usize] != Tile::Placeholder {
            return State::Playing;
        }
    }
    return State::Won;
}