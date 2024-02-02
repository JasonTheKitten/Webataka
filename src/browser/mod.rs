use crate::silkstick::graphicsys;
use crate::silkstick::graphicsys::GraphicsHandle;
use crate::silkstick::graphicsys::GraphicsSystem;
use crate::silkstick::graphicsys::WindowSettings;
use crate::silkstick::graphicsys::skiags;

pub fn run() {
    let mut graphics_system = skiags::SkiaGraphicsSystem::new();
    let hooks = graphicsys::GraphicsSystemHooks {
        on_graphics_start: on_graphics_start,
        on_graphics_end: on_graphics_end,
        ..graphicsys::GraphicsSystemHooks::default()
    };
    graphics_system.start_and_block(hooks);
}

fn on_graphics_start(graphics_handle: &mut Box<&mut dyn GraphicsHandle>) {
    println!("Graphics start");
    let window_settings = WindowSettings {
        title: "Webataka".to_string(),
        ..WindowSettings::default()
    };

    graphics_handle.create_window(window_settings, |window| {
        window.show();
    });
}

fn on_graphics_end(_: &mut Box<&mut dyn GraphicsHandle>) {
    println!("Graphics end");
}