use glam::Vec2;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::collections::HashSet;

pub struct InputState {
    keys_down: HashSet<Keycode>,
    keys_pressed: HashSet<Keycode>,
    keys_released: HashSet<Keycode>,

    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_released: HashSet<MouseButton>,

    mouse_position: Vec2,
    mouse_delta: Vec2,
    last_mouse_position: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_buttons_down: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),

            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            last_mouse_position: Vec2::ZERO,
        }
    }

    pub fn update(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();

        self.mouse_delta = self.mouse_position - self.last_mouse_position;
        self.last_mouse_position = self.mouse_position;
    }

    pub fn handle_key_down(&mut self, key: Keycode) {
        if !self.keys_down.contains(&key) {
            self.keys_pressed.insert(key);
        }
        self.keys_down.insert(key);
    }

    pub fn handle_key_up(&mut self, key: Keycode) {
        self.keys_down.remove(&key);
        self.keys_released.insert(key);
    }

    pub fn handle_mouse_down(&mut self, x: i32, y: i32, button: MouseButton) {
        self.mouse_position = Vec2::new(x as f32, y as f32);
        if !self.mouse_buttons_down.contains(&button) {
            self.mouse_buttons_pressed.insert(button);
        }
        self.mouse_buttons_down.insert(button);
    }

    pub fn handle_mouse_up(&mut self, x: i32, y: i32, button: MouseButton) {
        self.mouse_position = Vec2::new(x as f32, y as f32);
        self.mouse_buttons_down.remove(&button);
        self.mouse_buttons_released.insert(button);
    }

    pub fn handle_mouse_motion(&mut self, x: i32, y: i32) {
        self.mouse_position = Vec2::new(x as f32, y as f32);
    }

    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }
}
