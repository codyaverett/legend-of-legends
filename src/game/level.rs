use crate::engine::core::{Color, Rect};
use crate::engine::rendering::mock_assets::{BuildingAsset, MockAssetGenerator, StreetProp};
use crate::game::buildings::Building;
use glam::Vec2;

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
    pub buildings: Vec<Building>,
    pub background_buildings: Vec<Vec<BuildingAsset>>, // Multiple layers for parallax
    pub street_props: Vec<StreetProp>,
    pub sky_gradient: Vec<(Rect, Color)>,
    pub clouds: Vec<(Vec2, f32, Color)>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::new(TileType::Empty); width]; height];
        let mut asset_gen = MockAssetGenerator::new(42);
        
        Self {
            tiles,
            width,
            height,
            spawn_point: Vec2::new(100.0, 100.0),
            buildings: Vec::new(),
            background_buildings: Vec::new(),
            street_props: Vec::new(),
            sky_gradient: asset_gen.generate_sky_gradient(width as f32 * TILE_SIZE, height as f32 * TILE_SIZE),
            clouds: Vec::new(),
        }
    }

    pub fn from_string(level_str: &str) -> Self {
        let lines: Vec<&str> = level_str.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut tiles = vec![vec![Tile::new(TileType::Empty); width]; height];
        let mut spawn_point = Vec2::new(100.0, 100.0);
        let mut building_positions = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let tile_type = match ch {
                    '#' => TileType::Ground,
                    '=' => TileType::Platform,
                    '|' => TileType::Wall,
                    'D' => TileType::Destructible,
                    'B' => {
                        building_positions.push((x, y));
                        TileType::Empty
                    }
                    'S' => {
                        spawn_point = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                        TileType::Empty
                    }
                    _ => TileType::Empty,
                };
                tiles[y][x] = Tile::new(tile_type);
            }
        }

        // Generate procedural assets (sky and clouds only for now)
        let mut asset_gen = MockAssetGenerator::new(42);
        
        let level_width_pixels = width as f32 * TILE_SIZE;
        // Ground is at the bottom of the level (last 3 rows)
        let ground_y = (height - 3) as f32 * TILE_SIZE;
        
        // No buildings for now
        let background_buildings = Vec::new();
        let buildings = Vec::new();
        let street_props = Vec::new();
        
        // Generate clouds spread across the wide sky
        let mut clouds = Vec::new();
        for i in 0..20 {  // Clouds across the level
            let cloud_x = level_width_pixels * 0.05 + i as f32 * (level_width_pixels * 0.9 / 20.0);
            let cloud_y = 50.0 + (i % 3) as f32 * 80.0;
            clouds.extend(asset_gen.generate_cloud(cloud_x, cloud_y));
        }

        Self {
            tiles,
            width,
            height,
            spawn_point,
            buildings,
            background_buildings,
            street_props,
            sky_gradient: asset_gen.generate_sky_gradient(level_width_pixels, height as f32 * TILE_SIZE),
            clouds,
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
        // Create a much wider level - assuming ~40 pixels per meter, 2000m = 80,000 pixels / 32 tile size = 2500 tiles
        let width_in_tiles = 2500;
        let mut level_str = String::new();
        
        // Add empty space above ground (15 rows)
        for _ in 0..15 {
            level_str.push_str(&".".repeat(width_in_tiles));
            level_str.push('\n');
        }
        
        // Add spawn point row (centered)
        let spawn_row = ".".repeat(width_in_tiles / 2 - 1) + "S" + &".".repeat(width_in_tiles / 2);
        level_str.push_str(&spawn_row);
        level_str.push('\n');
        
        // Add some platforms at various heights
        for i in 0..4 {
            let mut row = ".".repeat(width_in_tiles);
            if i == 1 {
                // Add some scattered platforms
                for j in 0..10 {
                    let platform_pos = 200 + j * 200;
                    if platform_pos + 5 < width_in_tiles {
                        row.replace_range(platform_pos..platform_pos+5, "=====");
                    }
                }
            }
            level_str.push_str(&row);
            level_str.push('\n');
        }
        
        // Add ground at the very bottom (3 rows of solid ground)
        for _ in 0..3 {
            level_str.push_str(&"#".repeat(width_in_tiles));
            level_str.push('\n');
        }
        
        Self::from_string(&level_str)
    }
}
