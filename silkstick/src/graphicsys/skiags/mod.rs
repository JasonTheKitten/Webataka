pub mod backend;
mod canvas;

use crate::graphicsys::*;
use crate::graphicsys::skiags::backend::SkiaBackend;
use glfw::{self, fail_on_errors, Context};
use skia_safe::{Canvas, Surface};
use canvas::*;

pub struct SkiaWindow {
    glfw_window: glfw::PWindow,
    screen: Box<dyn ContentScreen>,
    surface: Box<Surface>,
}

impl SkiaWindow {
    pub fn new(graphicsys: &mut SkiaGraphicsSystem, settings: WindowSettings) -> Self {
        graphicsys.glfw.window_hint(glfw::WindowHint::Decorated(settings.decorated));
        graphicsys.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        graphicsys.glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut glfw_window, _events) = graphicsys.glfw.create_window(
            settings.size.0,
            settings.size.1,
            &settings.title,
            glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
        glfw_window.set_pos(settings.position.0 as i32, settings.position.1 as i32);
        glfw_window.make_current();
        
        SkiaWindow {
            glfw_window: glfw_window,
            screen: settings.screen,
            surface: graphicsys.backend.create_surface(settings.size.0, settings.size.1),
        }
    }
}

impl Window for SkiaWindow {
    fn show(&mut self) {
        self.glfw_window.show();
    }
}

/// A graphics system that uses Skia for rendering and GLFW for window management.
pub struct SkiaGraphicsSystem {
    glfw: glfw::Glfw,
    windows: Vec<Box<SkiaWindow>>,
    backend: Box<dyn SkiaBackend>,
}

impl SkiaGraphicsSystem {
    pub fn new(backend: Box<dyn SkiaBackend>) -> Self {
        let glfw = glfw::init(fail_on_errors!()).unwrap();
        SkiaGraphicsSystem {
            glfw: glfw,
            windows: Vec::new(),
            backend: backend,
        }
    }
}

impl GraphicsSystem for SkiaGraphicsSystem {
    fn start_and_block(&mut self, hooks: GraphicsSystemHooks) {
        (hooks.on_graphics_start)(&mut Box::new(self as &mut dyn GraphicsHandle));
        while !self.windows.is_empty() {
            (hooks.before_graphics_update)(&mut Box::new(self as &mut dyn GraphicsHandle));
            self.glfw.poll_events();
            for window in self.windows.iter_mut() {
                update_window(window);
                window.glfw_window.swap_buffers();
            }
            self.windows.retain(|window| !(window.as_ref() as &SkiaWindow).glfw_window.should_close());
        }
        (hooks.on_graphics_end)(&mut Box::new(self as &mut dyn GraphicsHandle));
    }
}

impl GraphicsHandle for SkiaGraphicsSystem {
    fn create_window(&mut self, settings: WindowSettings, on_window_created: fn(&mut Box<&mut dyn Window>)) {
        let mut window = SkiaWindow::new(self, settings);
        window.glfw_window.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        on_window_created(&mut Box::new(&mut window as &mut dyn Window));
        self.windows.push(Box::new(window));
    }

    // TODO: Create buffer method
}

fn update_window(window: &mut Box<SkiaWindow>) {
    window.glfw_window.make_current();
    if window.screen.needs_update() {
        let canvas = window.surface.canvas();
        update_screen(&mut window.screen, &canvas);
        canvas.direct_context()
            .expect("This window does not have a bound direct context, somehow!")
            .flush(None);
    }
}

fn update_screen<'a>(screen: &mut Box<dyn ContentScreen>, canvas: &Canvas) {
    screen.update(&mut create_canvas(&canvas));
}