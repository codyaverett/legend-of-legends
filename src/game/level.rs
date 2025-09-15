use crate::engine::core::{Color, Rect};
use crate::engine::rendering::mock_assets::{BuildingAsset, MockAssetGenerator, StreetProp};
use crate::game::buildings::Building;
use crate::game::win_condition::WinCondition;
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
    
    // Level metadata and objectives
    pub name: String,
    pub description: String,
    pub win_condition: WinCondition,
    pub goal_position: Option<Vec2>,
    pub time_limit: Option<f32>,
    pub collectibles: Vec<(Vec2, bool)>,
    pub boss_spawn: Option<Vec2>,
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
            name: "Unnamed Level".to_string(),
            description: "No description".to_string(),
            win_condition: WinCondition::DefeatAllEnemies,
            goal_position: None,
            time_limit: None,
            collectibles: Vec::new(),
            boss_spawn: None,
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
            name: "Custom Level".to_string(),
            description: "Complete the objective".to_string(),
            win_condition: WinCondition::DefeatAllEnemies,
            goal_position: None,
            time_limit: None,
            collectibles: Vec::new(),
            boss_spawn: None,
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
        
        let mut level = Self::from_string(&level_str);
        level.name = "Level 1: First Contact".to_string();
        level.description = "Defeat all enemy forces".to_string();
        level.win_condition = WinCondition::DefeatAllEnemies;
        level
    }
    
    pub fn level_2_reach_goal() -> Self {
        let width_in_tiles = 500; // Shorter level
        let mut level_str = String::new();
        
        // Build level with spawn on left, goal on right
        for y in 0..20 {
            let mut row = String::new();
            for x in 0..width_in_tiles {
                if y == 15 && x == 10 {
                    row.push('S'); // Spawn point
                } else if y >= 17 {
                    row.push('#'); // Ground
                } else if y == 16 && x % 50 == 0 && x > 0 && x < width_in_tiles - 20 {
                    // Platforms every 50 tiles
                    for _ in 0..10 {
                        row.push('=');
                    }
                } else {
                    row.push('.');
                }
            }
            level_str.push_str(&row);
            level_str.push('\n');
        }
        
        let mut level = Self::from_string(&level_str);
        level.name = "Level 2: The Journey".to_string();
        level.description = "Reach the extraction point".to_string();
        level.goal_position = Some(Vec2::new((width_in_tiles - 20) as f32 * TILE_SIZE, 15.0 * TILE_SIZE));
        level.win_condition = WinCondition::ReachGoal {
            position: level.goal_position.unwrap(),
            radius: 50.0,
        };
        level
    }
    
    pub fn level_3_survive() -> Self {
        let width_in_tiles = 300; // Small arena
        let mut level_str = String::new();
        
        // Build arena-style level
        for y in 0..20 {
            let mut row = String::new();
            for x in 0..width_in_tiles {
                if y == 15 && x == width_in_tiles / 2 {
                    row.push('S'); // Spawn in center
                } else if y >= 17 {
                    row.push('#'); // Ground
                } else if y == 14 && (x == 50 || x == width_in_tiles - 50) {
                    // Side platforms
                    for _ in 0..20 {
                        row.push('=');
                    }
                } else {
                    row.push('.');
                }
            }
            level_str.push_str(&row);
            level_str.push('\n');
        }
        
        let mut level = Self::from_string(&level_str);
        level.name = "Level 3: Last Stand".to_string();
        level.description = "Survive the enemy assault for 2 minutes".to_string();
        level.win_condition = WinCondition::SurviveTime { duration: 120.0 };
        level
    }
    
    pub fn level_4_collect() -> Self {
        let width_in_tiles = 800;
        let mut level = Self::test_level_1(); // Base it on level 1 layout
        level.name = "Level 4: Scavenger Hunt".to_string();
        level.description = "Collect 5 power cores".to_string();
        
        // Add collectibles at various positions
        level.collectibles = vec![
            (Vec2::new(200.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
            (Vec2::new(400.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
            (Vec2::new(600.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
            (Vec2::new(300.0 * TILE_SIZE, 10.0 * TILE_SIZE), false), // On platform
            (Vec2::new(500.0 * TILE_SIZE, 10.0 * TILE_SIZE), false), // On platform
        ];
        
        level.win_condition = WinCondition::CollectItems { required: 5 };
        level
    }
    
    pub fn level_5_compound() -> Self {
        let mut level = Self::level_2_reach_goal();
        level.name = "Level 5: Multi-Objective".to_string();
        level.description = "Complete all objectives".to_string();
        
        // Add collectibles
        level.collectibles = vec![
            (Vec2::new(100.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
            (Vec2::new(200.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
            (Vec2::new(300.0 * TILE_SIZE, 14.0 * TILE_SIZE), false),
        ];
        
        // Compound win condition
        level.win_condition = WinCondition::Compound {
            conditions: vec![
                WinCondition::CollectItems { required: 3 },
                WinCondition::ReachGoal {
                    position: level.goal_position.unwrap(),
                    radius: 50.0,
                },
                WinCondition::DefeatAllEnemies,
            ],
        };
        level
    }
}
