use crate::graphicsys::{ ContentScreen, GrahicsCanvas };

/// A GUIContentScreen is a special screen that allows for composing a GUI
/// using this GUI module.
pub struct GUIContentScreen {

}

impl GUIContentScreen {
    pub fn new() -> Self {
        GUIContentScreen {}
    }
}

impl ContentScreen for GUIContentScreen {
    fn needs_update(&self) -> bool {
        return true;
    }

    fn update(&mut self, canvas: &mut dyn GrahicsCanvas) {
        canvas.alter_paint(|paint| {
            paint.set_color(crate::graphicsys::color::Color::from_rgba8(255, 255, 0, 255));
        });
        canvas.draw_rect((10.0, 10.0), (100.0, 100.0));

        canvas.enter_section((200.0, 200.0), (100.0, 100.0), true, |canvas| {
            canvas.alter_paint(|paint| {
                paint.set_color(crate::graphicsys::color::CYAN);
            });
            canvas.draw_elipse((0.0, 0.0), (150.0, 150.0));
        });

        canvas.draw_elipse((50.0, 50.0), (150.0, 100.0));
    }
}