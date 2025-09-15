pub mod core;
pub mod physics;
pub mod platform;
pub mod rendering;
pub mod ui;

use anyhow::Result;

pub struct Engine {
    pub platform: platform::Platform,
    pub renderer: rendering::Renderer,
    pub world: hecs::World,
    pub delta_time: f32,
    pub running: bool,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let platform = platform::Platform::new(title, width, height)?;
        let renderer = rendering::Renderer::new(&platform)?;

        Ok(Self {
            platform,
            renderer,
            world: hecs::World::new(),
            delta_time: 0.0,
            running: true,
        })
    }

    pub fn run<F>(&mut self, mut update: F) -> Result<()>
    where
        F: FnMut(&mut Engine, f32),
    {
        let mut last_time = std::time::Instant::now();

        while self.running {
            let current_time = std::time::Instant::now();
            self.delta_time = current_time.duration_since(last_time).as_secs_f32();
            last_time = current_time;

            self.platform.handle_events(&mut self.running)?;

            update(self, self.delta_time);

            self.renderer.present();
        }

        Ok(())
    }
}
