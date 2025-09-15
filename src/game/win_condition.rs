use glam::Vec2;

#[derive(Debug, Clone)]
pub enum WinCondition {
    /// Reach a specific goal position
    ReachGoal { 
        position: Vec2,
        radius: f32,
    },
    
    /// Defeat all enemies in the level
    DefeatAllEnemies,
    
    /// Survive for a certain amount of time
    SurviveTime { 
        duration: f32,
    },
    
    /// Collect a certain number of items
    CollectItems { 
        required: usize,
    },
    
    /// Defeat a boss enemy
    DefeatBoss { 
        boss_id: Option<String>,
    },
    
    /// Multiple conditions that must all be met
    Compound { 
        conditions: Vec<WinCondition>,
    },
    
    /// Any one of multiple conditions
    Any { 
        conditions: Vec<WinCondition>,
    },
}

#[derive(Debug, Clone)]
pub struct WinConditionState {
    pub condition: WinCondition,
    pub is_complete: bool,
    pub progress: WinProgress,
}

#[derive(Debug, Clone)]
pub enum WinProgress {
    NotStarted,
    InProgress {
        current: f32,
        target: f32,
        description: String,
    },
    Complete,
}

impl WinCondition {
    pub fn get_description(&self) -> String {
        match self {
            WinCondition::ReachGoal { .. } => "Reach the goal".to_string(),
            WinCondition::DefeatAllEnemies => "Defeat all enemies".to_string(),
            WinCondition::SurviveTime { duration } => format!("Survive for {} seconds", duration),
            WinCondition::CollectItems { required } => format!("Collect {} items", required),
            WinCondition::DefeatBoss { .. } => "Defeat the boss".to_string(),
            WinCondition::Compound { conditions } => {
                format!("Complete {} objectives", conditions.len())
            },
            WinCondition::Any { conditions } => {
                format!("Complete any 1 of {} objectives", conditions.len())
            },
        }
    }
    
    pub fn check_completion(
        &self,
        player_pos: Vec2,
        enemy_count: usize,
        time_elapsed: f32,
        items_collected: usize,
        boss_defeated: bool,
    ) -> (bool, WinProgress) {
        match self {
            WinCondition::ReachGoal { position, radius } => {
                let distance = (player_pos - *position).length();
                let is_complete = distance <= *radius;
                let progress = if is_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: (*radius - distance).max(0.0),
                        target: *radius,
                        description: format!("Distance to goal: {:.0}m", distance / 40.0),
                    }
                };
                (is_complete, progress)
            },
            
            WinCondition::DefeatAllEnemies => {
                let is_complete = enemy_count == 0;
                let progress = if is_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: 0.0,
                        target: enemy_count as f32,
                        description: format!("{} enemies remaining", enemy_count),
                    }
                };
                (is_complete, progress)
            },
            
            WinCondition::SurviveTime { duration } => {
                let is_complete = time_elapsed >= *duration;
                let progress = if is_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: time_elapsed,
                        target: *duration,
                        description: format!("Time remaining: {:.0}s", duration - time_elapsed),
                    }
                };
                (is_complete, progress)
            },
            
            WinCondition::CollectItems { required } => {
                let is_complete = items_collected >= *required;
                let progress = if is_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: items_collected as f32,
                        target: *required as f32,
                        description: format!("{}/{} items collected", items_collected, required),
                    }
                };
                (is_complete, progress)
            },
            
            WinCondition::DefeatBoss { .. } => {
                let is_complete = boss_defeated;
                let progress = if is_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: if boss_defeated { 1.0 } else { 0.0 },
                        target: 1.0,
                        description: "Boss still alive".to_string(),
                    }
                };
                (is_complete, progress)
            },
            
            WinCondition::Compound { conditions } => {
                let mut all_complete = true;
                let mut completed_count = 0;
                
                for condition in conditions {
                    let (complete, _) = condition.check_completion(
                        player_pos,
                        enemy_count,
                        time_elapsed,
                        items_collected,
                        boss_defeated,
                    );
                    if complete {
                        completed_count += 1;
                    } else {
                        all_complete = false;
                    }
                }
                
                let progress = if all_complete {
                    WinProgress::Complete
                } else {
                    WinProgress::InProgress {
                        current: completed_count as f32,
                        target: conditions.len() as f32,
                        description: format!("{}/{} objectives complete", completed_count, conditions.len()),
                    }
                };
                (all_complete, progress)
            },
            
            WinCondition::Any { conditions } => {
                for condition in conditions {
                    let (complete, _) = condition.check_completion(
                        player_pos,
                        enemy_count,
                        time_elapsed,
                        items_collected,
                        boss_defeated,
                    );
                    if complete {
                        return (true, WinProgress::Complete);
                    }
                }
                
                let progress = WinProgress::InProgress {
                    current: 0.0,
                    target: 1.0,
                    description: "Complete any objective".to_string(),
                };
                (false, progress)
            },
        }
    }
}