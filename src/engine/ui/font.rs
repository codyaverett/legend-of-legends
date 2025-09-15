use glam::Vec2;
use crate::engine::core::Color;
use std::collections::HashMap;

pub struct BitmapFont {
    char_width: f32,
    char_height: f32,
    char_data: HashMap<char, Vec<Vec<bool>>>,
}

impl BitmapFont {
    pub fn new_5x7() -> Self {
        let mut char_data = HashMap::new();
        
        // Define 5x7 bitmap font for common characters
        char_data.insert('0', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('1', vec![
            vec![false, false, true, false, false],
            vec![false, true, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('2', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![false, false, false, false, true],
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
            vec![true, true, true, true, true],
        ]);
        
        char_data.insert('3', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![false, false, false, false, true],
            vec![false, false, true, true, false],
            vec![false, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('4', vec![
            vec![false, false, false, true, false],
            vec![false, false, true, true, false],
            vec![false, true, false, true, false],
            vec![true, false, false, true, false],
            vec![true, true, true, true, true],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
        ]);
        
        char_data.insert('5', vec![
            vec![true, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, true, true, true, false],
            vec![false, false, false, false, true],
            vec![false, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('6', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, false],
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('7', vec![
            vec![true, true, true, true, true],
            vec![false, false, false, false, true],
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
            vec![false, true, false, false, false],
            vec![false, true, false, false, false],
        ]);
        
        char_data.insert('8', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('9', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
            vec![false, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        // Letters
        char_data.insert('A', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
        ]);
        
        char_data.insert('B', vec![
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, false],
        ]);
        
        char_data.insert('C', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('D', vec![
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, false],
        ]);
        
        char_data.insert('E', vec![
            vec![true, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, true, true, true, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, true, true, true, true],
        ]);
        
        char_data.insert('F', vec![
            vec![true, true, true, true, true],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, true, true, true, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
        ]);
        
        char_data.insert('G', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, false],
            vec![true, false, true, true, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('I', vec![
            vec![false, true, true, true, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('L', vec![
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, true, true, true, true],
        ]);
        
        char_data.insert('M', vec![
            vec![true, false, false, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
        ]);
        
        char_data.insert('N', vec![
            vec![true, false, false, false, true],
            vec![true, true, false, false, true],
            vec![true, false, true, false, true],
            vec![true, false, false, true, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
        ]);
        
        char_data.insert('P', vec![
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
        ]);
        
        char_data.insert('S', vec![
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('T', vec![
            vec![true, true, true, true, true],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
        ]);
        
        char_data.insert('V', vec![
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, false, true, false],
            vec![false, false, true, false, false],
        ]);
        
        // Lowercase letters (subset)
        char_data.insert('a', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, true],
            vec![false, true, true, true, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
        ]);
        
        char_data.insert('e', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, true, true, true, true],
            vec![true, false, false, false, false],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('g', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
            vec![false, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('i', vec![
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('l', vec![
            vec![false, true, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('m', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, true, false, true, false],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
            vec![true, false, true, false, true],
        ]);
        
        char_data.insert('n', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
        ]);
        
        char_data.insert('o', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        char_data.insert('r', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, false, true, true, false],
            vec![true, true, false, false, true],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
        ]);
        
        char_data.insert('s', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, true],
            vec![true, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, true],
            vec![true, true, true, true, false],
        ]);
        
        char_data.insert('t', vec![
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![true, true, true, true, true],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, false],
        ]);
        
        char_data.insert('u', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
        ]);
        
        char_data.insert('v', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, false, true, false],
            vec![false, false, true, false, false],
        ]);
        
        char_data.insert('y', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
            vec![false, false, false, false, true],
            vec![false, true, true, true, false],
        ]);
        
        // Special characters
        char_data.insert(':', vec![
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
        ]);
        
        char_data.insert('.', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
        ]);
        
        char_data.insert(',', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
        ]);
        
        char_data.insert('(', vec![
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
            vec![false, true, false, false, false],
            vec![false, true, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, true, false],
        ]);
        
        char_data.insert(')', vec![
            vec![false, true, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
        ]);
        
        char_data.insert('/', vec![
            vec![false, false, false, false, true],
            vec![false, false, false, true, false],
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, false, false],
            vec![false, true, false, false, false],
            vec![true, false, false, false, false],
        ]);
        
        char_data.insert('-', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]);
        
        char_data.insert(' ', vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]);
        
        Self {
            char_width: 5.0,
            char_height: 7.0,
            char_data,
        }
    }
    
    pub fn get_char_bitmap(&self, ch: char) -> Option<&Vec<Vec<bool>>> {
        self.char_data.get(&ch)
    }
    
    pub fn get_char_size(&self) -> Vec2 {
        Vec2::new(self.char_width, self.char_height)
    }
    
    pub fn measure_text(&self, text: &str, scale: f32) -> Vec2 {
        let char_count = text.len() as f32;
        Vec2::new(
            char_count * (self.char_width + 1.0) * scale,
            self.char_height * scale,
        )
    }
}