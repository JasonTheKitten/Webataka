pub mod glbackend;

use skia_safe::Surface;

/// This trait allows specifying a custom rendering backend for Skia.
pub trait SkiaBackend {
    /// Create a Skia surface backed by the custom rendering backend.
    /// It will allow drawing within the specified width and height.
    fn create_surface(&self, width: u32, height: u32) -> Box<Surface>;
}