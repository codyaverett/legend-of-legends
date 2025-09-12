use anyhow::Result;
use sdl2::video::GLContext;

pub struct Window {
    sdl_window: sdl2::video::Window,
    _gl_context: GLContext,
}

impl Window {
    pub fn new(sdl: &sdl2::Sdl, title: &str, width: u32, height: u32) -> Result<Self> {
        let video_subsystem = sdl.video().map_err(|e| anyhow::anyhow!(e))?;
        
        let sdl_window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .resizable()
            .build()?;
            
        let gl_context = sdl_window.gl_create_context().map_err(|e| anyhow::anyhow!(e))?;
        
        Ok(Self {
            sdl_window,
            _gl_context: gl_context,
        })
    }
    
    pub fn canvas(&self) -> Result<sdl2::render::Canvas<sdl2::video::Window>> {
        self.sdl_window
            .clone()
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| anyhow::anyhow!(e))
    }
    
    pub fn size(&self) -> (u32, u32) {
        self.sdl_window.size()
    }
}