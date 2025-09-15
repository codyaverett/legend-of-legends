use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Margin {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct Layout;

impl Layout {
    pub fn stack_vertical(elements: &[(Vec2, Vec2)], start_pos: Vec2, spacing: f32) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let mut current_y = start_pos.y;

        for (_, size) in elements {
            positions.push(Vec2::new(start_pos.x, current_y));
            current_y += size.y + spacing;
        }

        positions
    }

    pub fn stack_horizontal(elements: &[(Vec2, Vec2)], start_pos: Vec2, spacing: f32) -> Vec<Vec2> {
        let mut positions = Vec::new();
        let mut current_x = start_pos.x;

        for (_, size) in elements {
            positions.push(Vec2::new(current_x, start_pos.y));
            current_x += size.x + spacing;
        }

        positions
    }

    pub fn center_in_rect(element_size: Vec2, container_pos: Vec2, container_size: Vec2) -> Vec2 {
        Vec2::new(
            container_pos.x + (container_size.x - element_size.x) / 2.0,
            container_pos.y + (container_size.y - element_size.y) / 2.0,
        )
    }

    pub fn apply_margin(position: Vec2, margin: Margin) -> Vec2 {
        Vec2::new(position.x + margin.left, position.y + margin.top)
    }
}