use crate::silkstick::graphicsys;
use glfw::{self, fail_on_errors, Context};

pub struct SkiaWindow {
    glfw_window: glfw::PWindow,
}

impl SkiaWindow {
    pub fn new(graphicsys: &mut SkiaGraphicsSystem, settings: graphicsys::WindowSettings) -> Self {
        glfw::WindowHint::Decorated(settings.decorated);
        let (glfw_window, _events) = graphicsys.glfw.create_window(
            settings.size.0,
            settings.size.1,
            &settings.title,
            glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
        SkiaWindow {
            glfw_window: glfw_window
        }
    }
}

impl graphicsys::Window for SkiaWindow {
    fn show(&mut self) {
        self.glfw_window.show();
    }
}

pub struct SkiaGraphicsSystem {
    glfw: glfw::Glfw,
    windows: Vec<Box<SkiaWindow>>,
}

impl SkiaGraphicsSystem {
    pub fn new() -> Self {
        let glfw = glfw::init(fail_on_errors!()).unwrap();
        SkiaGraphicsSystem {
            glfw: glfw,
            windows: Vec::new(),
        }
    }
}

impl graphicsys::GraphicsSystem for SkiaGraphicsSystem {
    fn start_and_block(&mut self, hooks: graphicsys::GraphicsSystemHooks) {
        (hooks.on_graphics_start)(&mut Box::new(self as &mut dyn graphicsys::GraphicsHandle));
        while !self.windows.is_empty() {
            (hooks.before_graphics_update)(&mut Box::new(self as &mut dyn graphicsys::GraphicsHandle));
            self.glfw.poll_events();
            for window in self.windows.iter_mut() {
                window.glfw_window.swap_buffers();
            }
            self.windows.retain(|window| !(window.as_ref() as &SkiaWindow).glfw_window.should_close());
        }
        (hooks.on_graphics_end)(&mut Box::new(self as &mut dyn graphicsys::GraphicsHandle));
    }
}

impl graphicsys::GraphicsHandle for SkiaGraphicsSystem {
    fn create_window(&mut self, settings: graphicsys::WindowSettings, on_window_created: fn(&mut Box<&mut dyn graphicsys::Window>)) {
        let mut window = SkiaWindow::new(self, settings);
        window.glfw_window.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        on_window_created(&mut Box::new(&mut window as &mut dyn graphicsys::Window));
        self.windows.push(Box::new(window));
    }
}