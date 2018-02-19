use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use glium;
use renderer;
use renderer::*;

pub struct Map {
    pub level: u8,
    pub tiles: [[Tile; 15]; 20],
    pub start_position: Position,
    pub boxes: Vec<Position>
}

pub type Position = (u8, u8);

impl Map {
    pub fn draw(&self, target: &mut glium::Frame, renderer: &mut Renderer) {
        // Draw all tiles.
        for x in 0..20 {
            for y in 0..15 {
                // Get the tile color.
                let color = match self.tiles[x][y] {
                    Tile::Nothing => Color(0.0, 0.0, 0.0, 0.0),
                    Tile::Floor => Color(0.9, 0.9, 0.9, 1.0),
                    Tile::Wall => Color(0.2, 0.2, 0.2, 1.0),
                    Tile::Placeholder => Color(0.5, 0.5, 0.5, 1.0)
                };

                // Draw the tile.
                renderer.draw_quad(target, renderer::Rect {
                    x: x as f32 * 32.0,
                    y: y as f32 * 32.0,
                    w: 32.0,
                    h: 32.0
                }, color);
            }
        }

        // Draw all boxes.
        for item in self.boxes.iter() {
            // Get the box color?
            let color = if self.tiles[item.0 as usize][item.1 as usize] == Tile::Placeholder {
                Color(0.0, 1.0, 0.0, 1.0)
            } else {
                Color(1.0, 0.0, 0.0, 1.0)
            };

            renderer.draw_quad(target, renderer::Rect {
                x: item.0 as f32 * 32.0,
                y: item.1 as f32 * 32.0,
                w: 32.0,
                h: 32.0
            }, color);
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Nothing,
    Wall,
    Floor,
    Placeholder
}

pub fn load(level: u8) -> Map {
    // Create the level filename.
    let filename = format!("data/{}.level", level);
    if !Path::new(&filename).exists() {
        // Level does not exist. Start over.
        return load(1);
    }

    // Read the level file.
    let mut file = File::open(filename).expect("Level file not found.");
    let mut contents = String::new();
        file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    // Declare the array holding the level information.
    let mut tiles = [[Tile::Nothing; 15]; 20];
    let mut start_position : Position = (0,0);
    let mut boxes : Vec<Position> = Vec::new();

    // Process the map.
    let mut x = 0;
    let mut y = 0;
    for c in contents.chars() {

        if c == '\n' {
            x = 0;
            y = y + 1;
            continue;
        }

        // Make sure we don't go outside the array.
        if y >= 15 {
            break;
        }

        if c == '#' {
            tiles[x][y] = Tile::Wall;
        } else if c == '.' {
            tiles[x][y] = Tile::Floor;
        } else if c == 'o' {
            tiles[x][y] = Tile::Placeholder;
        } else if c == '$' {
            tiles[x][y] = Tile::Floor;
            boxes.push((x as u8, y as u8));
        } else if c == '@' {
            // Start position.
            tiles[x][y] = Tile::Floor;
            start_position = (x as u8, y as u8);
        }

        x = x + 1;
    }

    return Map {
        level,
        tiles,
        start_position,
        boxes
    };
}