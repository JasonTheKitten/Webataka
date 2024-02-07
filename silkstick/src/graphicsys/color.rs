/// Allows for the creation of colors and provides some predefined colors.
pub struct Color(u8, u8, u8, u8);

pub const BLACK: Color = Color(0, 0, 0, 255);
pub const WHITE: Color = Color(255, 255, 255, 255);

pub const RED: Color = Color(255, 0, 0, 255);
pub const GREEN: Color = Color(0, 255, 0, 255);
pub const BLUE: Color = Color(0, 0, 255, 255);

pub const CYAN: Color = Color(0, 255, 255, 255);
pub const MAGENTA: Color = Color(255, 0, 255, 255);
pub const YELLOW: Color = Color(255, 255, 0, 255);

pub const GRAY: Color = Color(128, 128, 128, 255);
pub const LIGHT_GRAY: Color = Color(192, 192, 192, 255);
pub const DARK_GRAY: Color = Color(64, 64, 64, 255);

pub const TRANSPARENT: Color = Color(0, 0, 0, 0);

impl Color {
    
    /// Create a new color from the given RGBA values.
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(r, g, b, a)
    }

    /// Create a new color from the given RGB values.
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b, 255)
    }

    /// Get the RGBA values of the color as a tuple.
    pub fn to_rgba8(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }
    
}