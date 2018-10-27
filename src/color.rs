extern crate image;

/// Rather than using the image crate's ColorType, a more restricted ColorType
/// is defined containing only grayscale and RGB types. This simplifies match
/// statements and error handling.
#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum ColorType {
    Gray(u8),
    RGB(u8)
}

impl ColorType {
    /// Create a ColorType from a number of bits per pixel.
    pub fn from_bitdepth(bitdepth: u8) -> Result<ColorType, &'static str> {
        match bitdepth {
            1  => Ok(ColorType::Gray(1)),
            2  => Ok(ColorType::Gray(2)),
            4  => Ok(ColorType::Gray(4)),
            8  => Ok(ColorType::Gray(8)),
            24 => Ok(ColorType::RGB(8)),
            _  => Err("Unsupported bitdepth")
        }
    }

    /// Convert to the image crate's ColorType enum.
    pub fn to_image_colortype(&self) -> image::ColorType {
        match self {
            &ColorType::Gray(n) => image::Gray(n),
            &ColorType::RGB(n)  => image::RGB(n)
        }
    }

    pub fn bits_per_pixel(&self) -> u32 {
        match self {
            &ColorType::Gray(n) => n as u32,
            &ColorType::RGB(n)  => n as u32 * 3,
        }
    }

    pub fn bytes_per_pixel(&self) -> f32 {
        self.bits_per_pixel() as f32 / 8.0
    }
}
