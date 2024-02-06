pub mod glbackend;

use skia_safe::Surface;

pub trait SkiaBackend {
    fn create_surface(&self, width: u32, height: u32) -> Box<Surface>;
}