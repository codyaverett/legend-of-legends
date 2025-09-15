use crate::engine::core::{Color, Transform};
use crate::engine::rendering::mock_assets::{BuildingAsset, BuildingStyle};
use glam::Vec2;

/// Component representing a building in the game world
#[derive(Debug, Clone)]
pub struct Building {
    pub asset: BuildingAsset,
    pub destructible: bool,
    pub health: Option<f32>,
}

impl Building {
    pub fn new(asset: BuildingAsset) -> Self {
        Self {
            asset,
            destructible: false,
            health: None,
        }
    }

    pub fn destructible(mut self, health: f32) -> Self {
        self.destructible = true;
        self.health = Some(health);
        self
    }
}

/// Manages building generation and placement
pub struct BuildingGenerator {
    seed: u64,
}

impl BuildingGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate a city block of buildings
    pub fn generate_city_block(&self, start_x: f32, end_x: f32, ground_y: f32, density: f32) -> Vec<Building> {
        use crate::engine::rendering::mock_assets::MockAssetGenerator;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut buildings = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut asset_gen = MockAssetGenerator::new(self.seed);
        
        let mut x = start_x;
        
        while x < end_x {
            if rng.gen::<f32>() < density {
                // Choose building style based on location
                let style = if rng.gen::<f32>() < 0.3 {
                    BuildingStyle::Skyscraper
                } else if rng.gen::<f32>() < 0.5 {
                    BuildingStyle::Office
                } else if rng.gen::<f32>() < 0.7 {
                    BuildingStyle::Industrial
                } else {
                    BuildingStyle::Residential
                };
                
                let building_asset = asset_gen.generate_building(x, ground_y, style);
                let building = Building::new(building_asset.clone());
                
                // Make some buildings destructible
                let building = if rng.gen::<f32>() < 0.2 {
                    building.destructible(100.0)
                } else {
                    building
                };
                
                x += building_asset.size.x + rng.gen_range(100.0..400.0); // Much wider spacing between buildings
                buildings.push(building);
            } else {
                x += rng.gen_range(200.0..600.0); // Large open areas
            }
        }
        
        buildings
    }

    /// Generate background skyline buildings (simplified, for parallax layers)
    pub fn generate_skyline(&self, width: f32, ground_y: f32, layer_depth: u32) -> Vec<BuildingAsset> {
        use crate::engine::rendering::mock_assets::MockAssetGenerator;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut skyline = Vec::new();
        let mut rng = StdRng::seed_from_u64(self.seed + layer_depth as u64);
        let mut asset_gen = MockAssetGenerator::new(self.seed + layer_depth as u64);
        
        let building_count = (width / 500.0) as usize;  // Fewer, more spaced out buildings
        
        for i in 0..building_count {
            let x = i as f32 * (width / building_count as f32);
            
            // Skyline buildings are always skyscrapers or office buildings
            let style = if rng.gen::<f32>() < 0.6 {
                BuildingStyle::Skyscraper
            } else {
                BuildingStyle::Office
            };
            
            let mut building = asset_gen.generate_building(x, ground_y, style);
            
            // Make background buildings darker based on depth
            let darkness_factor = 0.7 - (layer_depth as f32 * 0.15);
            building.base_color = Color::new(
                (building.base_color.r as f32 * darkness_factor) as u8,
                (building.base_color.g as f32 * darkness_factor) as u8,
                (building.base_color.b as f32 * darkness_factor) as u8,
                building.base_color.a,
            );
            
            skyline.push(building);
        }
        
        skyline
    }
}

/// Component for building health and destruction
#[derive(Debug, Clone)]
pub struct Destructible {
    pub max_health: f32,
    pub current_health: f32,
    pub debris_particles: usize,
}

impl Destructible {
    pub fn new(health: f32) -> Self {
        Self {
            max_health: health,
            current_health: health,
            debris_particles: 10,
        }
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.current_health = (self.current_health - damage).max(0.0);
        self.current_health <= 0.0
    }

    pub fn health_percentage(&self) -> f32 {
        self.current_health / self.max_health
    }
}