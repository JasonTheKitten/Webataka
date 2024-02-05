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

pub trait GrahicsCanvas {
    
}

pub trait ContentScreen {
    fn needs_update(&self) -> bool;
    fn update(&mut self, canvas: Box<dyn GrahicsCanvas>);
}

struct DefaultContentScreen {}

impl ContentScreen for DefaultContentScreen {
    fn needs_update(&self) -> bool {
        true
    }

    fn update(&mut self, _:Box<dyn GrahicsCanvas>) {
        // Do nothing
    }
}

pub struct WindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub position: (u32, u32),
    pub decorated: bool,
    pub screen: Box<dyn ContentScreen>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            title: "Application".to_string(),
            size: (800, 600),
            position: (100, 100),
            decorated: true,
            screen: Box::new(DefaultContentScreen{}),
        }
    }
}

pub trait Window {
    fn show(&mut self);
}