pub mod input;
pub mod window;

use anyhow::Result;
use sdl2::EventPump;

pub use input::*;
pub use window::*;

pub struct Platform {
    pub sdl_context: sdl2::Sdl,
    pub window: Window,
    pub event_pump: EventPump,
    pub input: InputState,
}

impl Platform {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
        let window = Window::new(&sdl_context, title, width, height)?;
        let event_pump = sdl_context.event_pump().map_err(|e| anyhow::anyhow!(e))?;
        let input = InputState::new();

        Ok(Self {
            sdl_context,
            window,
            event_pump,
            input,
        })
    }

    pub fn handle_events(&mut self, running: &mut bool) -> Result<()> {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        self.input.update();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => *running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => *running = false,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    self.input.handle_key_down(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    self.input.handle_key_up(key);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    self.input.handle_mouse_down(x, y, mouse_btn);
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    self.input.handle_mouse_up(x, y, mouse_btn);
                }
                Event::MouseMotion { x, y, .. } => {
                    self.input.handle_mouse_motion(x, y);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
