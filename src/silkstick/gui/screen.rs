use crate::silkstick::graphicsys::ContentScreen;

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

    fn update(&mut self, _: Box<dyn crate::silkstick::graphicsys::GrahicsCanvas>) {
        
    }
}