use glam::Vec2;
use crate::engine::core::{Color, Rect};

pub const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Ground,
    Platform,
    Wall,
    Destructible,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub solid: bool,
    pub color: Color,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let (solid, color) = match tile_type {
            TileType::Empty => (false, Color::new(0, 0, 0, 0)),
            TileType::Ground => (true, Color::new(80, 60, 40, 255)),
            TileType::Platform => (true, Color::new(120, 100, 80, 255)),
            TileType::Wall => (true, Color::new(100, 100, 100, 255)),
            TileType::Destructible => (true, Color::new(160, 140, 120, 255)),
        };
        
        Self {
            tile_type,
            solid,
            color,
        }
    }
}

pub struct Level {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub spawn_point: Vec2,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::new(TileType::Empty); width]; height];
        Self {
            tiles,
            width,
            height,
            spawn_point: Vec2::new(100.0, 100.0),
        }
    }
    
    pub fn from_string(level_str: &str) -> Self {
        let lines: Vec<&str> = level_str.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        
        let mut tiles = vec![vec![Tile::new(TileType::Empty); width]; height];
        let mut spawn_point = Vec2::new(100.0, 100.0);
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let tile_type = match ch {
                    '#' => TileType::Ground,
                    '=' => TileType::Platform,
                    '|' => TileType::Wall,
                    'D' => TileType::Destructible,
                    'S' => {
                        spawn_point = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                        TileType::Empty
                    }
                    _ => TileType::Empty,
                };
                tiles[y][x] = Tile::new(tile_type);
            }
        }
        
        Self {
            tiles,
            width,
            height,
            spawn_point,
        }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }
    
    pub fn get_tile_at_position(&self, pos: Vec2) -> Option<&Tile> {
        let x = (pos.x / TILE_SIZE) as usize;
        let y = (pos.y / TILE_SIZE) as usize;
        self.get_tile(x, y)
    }
    
    pub fn check_collision(&self, rect: Rect) -> bool {
        let start_x = ((rect.x / TILE_SIZE).floor() as usize).max(0);
        let end_x = (((rect.x + rect.width) / TILE_SIZE).ceil() as usize).min(self.width);
        let start_y = ((rect.y / TILE_SIZE).floor() as usize).max(0);
        let end_y = (((rect.y + rect.height) / TILE_SIZE).ceil() as usize).min(self.height);
        
        for y in start_y..end_y {
            for x in start_x..end_x {
                if let Some(tile) = self.get_tile(x, y) {
                    if tile.solid {
                        let tile_rect = Rect::new(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            TILE_SIZE,
                            TILE_SIZE,
                        );
                        if rect.intersects(&tile_rect) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    pub fn test_level_1() -> Self {
        Self::from_string(
            "\
            ..................................................\n\
            ..................................................\n\
            ..................................................\n\
            ..................................................\n\
            .....S............................................\n\
            ..................................................\n\
            ....................==............................\n\
            ..................................................\n\
            .............==...................................\n\
            ..................................................\n\
            ........==................==......................\n\
            ..................................................\n\
            .....==......................................==...\n\
            ..................................................\n\
            ..................................==..............\n\
            ##################################################\n\
            ##################################################\n\
            ##################################################\n\
            "
        )
    }
}