use silkstick::graphicsys::{ GraphicsHandle, GraphicsSystem, WindowSettings, GraphicsSystemHooks };
use silkstick::graphicsys::skiags::SkiaGraphicsSystem;
use silkstick::graphicsys::skiags::backend::glbackend::GLBackend;
use silkstick::gui::screen::GUIContentScreen;

pub fn main() {
    let mut graphics_system = SkiaGraphicsSystem::new(Box::new(GLBackend::new()));
    let hooks = GraphicsSystemHooks {
        on_graphics_start: on_graphics_start,
        on_graphics_end: on_graphics_end,
        ..GraphicsSystemHooks::default()
    };
    graphics_system.start_and_block(hooks);
}

fn on_graphics_start(graphics_handle: &mut Box<&mut dyn GraphicsHandle>) {
    println!("Graphics start");
    let window_settings = WindowSettings {
        title: "Webataka".to_string(),
        screen: Box::new(GUIContentScreen::new()),
        ..WindowSettings::default()
    };

    graphics_handle.create_window(window_settings, |window| {
        window.show();
    });
}

fn on_graphics_end(_: &mut Box<&mut dyn GraphicsHandle>) {
    println!("Graphics end");
}