pub mod skiags;

pub struct GraphicsSystemHooks {
    pub on_graphics_start: fn(&mut Box<&mut dyn GraphicsHandle>),
    pub before_graphics_update: fn(&mut Box<&mut dyn GraphicsHandle>),
    pub on_graphics_end: fn(&mut Box<&mut dyn GraphicsHandle>),
}

fn default_on_graphics_start(_: &mut Box<&mut dyn GraphicsHandle>) {}
fn default_before_graphics_update(_: &mut Box<&mut dyn GraphicsHandle>) {}
fn default_on_graphics_end(_: &mut Box<&mut dyn GraphicsHandle>) {}

impl Default for GraphicsSystemHooks {
    fn default() -> Self {
        GraphicsSystemHooks {
            on_graphics_start: default_on_graphics_start,
            before_graphics_update: default_before_graphics_update,
            on_graphics_end: default_on_graphics_end,
        }
    }
}

pub trait GraphicsSystem {
    fn start_and_block(&mut self, hooks: GraphicsSystemHooks);  
}

pub trait GraphicsHandle {
    fn create_window(&mut self, settings: WindowSettings, on_window_created: fn(&mut Box<&mut dyn Window>));
}

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub position: (u32, u32),
    pub decorated: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: "Application".to_string(),
            size: (800, 600),
            position: (100, 100),
            decorated: true,
        }
    }
}

pub trait Window {
    fn show(&mut self);
}