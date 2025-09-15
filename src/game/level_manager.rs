use crate::game::level::Level;
use crate::game::win_condition::{WinCondition, WinProgress};
use glam::Vec2;

pub struct LevelManager {
    levels: Vec<Level>,
    current_level_index: usize,
    level_time_elapsed: f32,
    items_collected: usize,
    boss_defeated: bool,
    level_complete: bool,
    all_levels_complete: bool,
}

impl LevelManager {
    pub fn new() -> Self {
        let levels = vec![
            Level::test_level_1(),
            Level::level_2_reach_goal(),
            Level::level_3_survive(),
            Level::level_4_collect(),
            Level::level_5_compound(),
        ];
        
        Self {
            levels,
            current_level_index: 0,
            level_time_elapsed: 0.0,
            items_collected: 0,
            boss_defeated: false,
            level_complete: false,
            all_levels_complete: false,
        }
    }
    
    pub fn get_current_level(&self) -> &Level {
        &self.levels[self.current_level_index]
    }
    
    pub fn get_current_level_mut(&mut self) -> &mut Level {
        &mut self.levels[self.current_level_index]
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if !self.level_complete {
            self.level_time_elapsed += delta_time;
        }
    }
    
    pub fn check_win_condition(
        &mut self,
        player_pos: Vec2,
        enemy_count: usize,
    ) -> (bool, WinProgress) {
        let level = &self.levels[self.current_level_index];
        let (complete, progress) = level.win_condition.check_completion(
            player_pos,
            enemy_count,
            self.level_time_elapsed,
            self.items_collected,
            self.boss_defeated,
        );
        
        if complete && !self.level_complete {
            self.level_complete = true;
        }
        
        (complete, progress)
    }
    
    pub fn collect_item(&mut self, index: usize) -> bool {
        let level = &mut self.levels[self.current_level_index];
        if index < level.collectibles.len() && !level.collectibles[index].1 {
            level.collectibles[index].1 = true;
            self.items_collected += 1;
            return true;
        }
        false
    }
    
    pub fn defeat_boss(&mut self) {
        self.boss_defeated = true;
    }
    
    pub fn next_level(&mut self) -> bool {
        if self.current_level_index < self.levels.len() - 1 {
            self.current_level_index += 1;
            self.reset_level_state();
            true
        } else {
            self.all_levels_complete = true;
            false
        }
    }
    
    pub fn restart_level(&mut self) {
        self.reset_level_state();
    }
    
    fn reset_level_state(&mut self) {
        self.level_time_elapsed = 0.0;
        self.items_collected = 0;
        self.boss_defeated = false;
        self.level_complete = false;
        
        // Reset collectibles
        let level = &mut self.levels[self.current_level_index];
        for collectible in &mut level.collectibles {
            collectible.1 = false;
        }
    }
    
    pub fn go_to_level(&mut self, index: usize) -> bool {
        if index < self.levels.len() {
            self.current_level_index = index;
            self.reset_level_state();
            true
        } else {
            false
        }
    }
    
    pub fn is_level_complete(&self) -> bool {
        self.level_complete
    }
    
    pub fn is_all_levels_complete(&self) -> bool {
        self.all_levels_complete
    }
    
    pub fn get_level_count(&self) -> usize {
        self.levels.len()
    }
    
    pub fn get_current_level_index(&self) -> usize {
        self.current_level_index
    }
    
    pub fn get_level_time(&self) -> f32 {
        self.level_time_elapsed
    }
    
    pub fn get_collectibles_status(&self) -> Vec<(Vec2, bool)> {
        self.levels[self.current_level_index].collectibles.clone()
    }
}