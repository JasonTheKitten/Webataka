use self::color::Color;

pub mod skiags;
pub mod color;

/// A set of hooks that are triggered at certain points in the graphics system's lifecycle.
pub struct GraphicsSystemHooks {
    /// This is called when the graphics system is started.
    pub on_graphics_start: fn(&mut Box<&mut dyn GraphicsHandle>),

    /// This is called before the graphics system processes updates for each window per frame.
    pub before_graphics_update: fn(&mut Box<&mut dyn GraphicsHandle>),

    /// This is called when the graphics system is stopped - typically when all windows are closed.
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

/// A graphics system provides a way to manage graphical outputs
/// and to draw to them.
pub trait GraphicsSystem {
    /// Start the graphics system, which will block the current thread until the graphics system ends.
    fn start_and_block(&mut self, hooks: GraphicsSystemHooks);  
}

/// A handle to the graphics system that allows creating windows and buffers.
pub trait GraphicsHandle {
    /// Create a new window with the specified settings.
    /// Actions can be performed on the window in the on_window_created callback.
    fn create_window(&mut self, settings: WindowSettings, on_window_created: fn(&mut Box<&mut dyn Window>));
}

/// A canvas that can be drawn to.
pub trait GrahicsCanvas<'a> {
    /// Alter the paint of the canvas (for example, to change the color).
    fn alter_paint(&mut self, alter_fn: fn(&mut dyn Paint) -> ());

    /// Run the given callback with a subsection of the canvas with the specified position and size.
    /// If clip is true, anything drawn outside of the section will be clipped (not shown).
    /// Note that the subsection starts with default paint settings, as opposed to inheriting the parent's paint settings.
    fn enter_section(&mut self, pos: (f32, f32), size: (f32, f32), clip: bool, section_func: fn(&mut dyn GrahicsCanvas));

    /// Draw text at the specified position.
    fn draw_text(&mut self, pos: (f32, f32), text: &str);

    /// Draw a rectangle at the specified position and with the specified size.
    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32));

    /// Draw an elipse at the specified position and with the specified size.
    fn draw_elipse(&mut self, pos: (f32, f32), size: (f32, f32));

    /// Draw a line starting at the specified position and moving by the specified amount.
    fn draw_line(&mut self, start: (f32, f32), mov: (f32, f32));
}

/// Paint allows for choosing what the elements drawn on the canvas will look like.
pub trait Paint {
    /// Set the color that shapes using this paint will be.
    fn set_color(&mut self, color: Color);
}

/// A ContentScreen represents dynamic content that can be drawn and that
/// may respond to events.
pub trait ContentScreen {
    /// Indicates if the content has been updated since the last time it was drawn.
    fn needs_update(&self) -> bool;
    /// Do any work necessary to draw a new frame with this content.
    fn update<'a>(&mut self, canvas: &mut dyn GrahicsCanvas<'a>);
}

struct DefaultContentScreen {}

impl ContentScreen for DefaultContentScreen {
    fn needs_update(&self) -> bool {
        true
    }

    fn update<'a>(&mut self, _:&mut dyn GrahicsCanvas<'a>) {
        // Do nothing
    }
}

/// Properties helping to define how a window appears and acts.
pub struct WindowSettings {
    /// The title of the window.
    pub title: String,
    /// The size of the window.
    pub size: (u32, u32),
    /// The position of the window.
    pub position: (u32, u32),
    /// Whether the window should have a border and title bar (decorated) or not (undecorated) when supported.
    pub decorated: bool,
    /// The content to display in the window.
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

/// A handle for manipulating a window.
pub trait Window {
    /// Make the window visible to the user.
    fn show(&mut self);
}