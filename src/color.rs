extern crate image;

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum ColorType {
    Gray(u8),
    RGB(u8)
}

impl ColorType {
    pub fn from_bitdepth(bitdepth: u8) -> Result<ColorType, &'static str> {
        match bitdepth {
            0  => Ok(ColorType::RGB(8)),
            1  => Ok(ColorType::Gray(1)),
            2  => Ok(ColorType::Gray(2)),
            4  => Ok(ColorType::Gray(4)),
            8  => Ok(ColorType::Gray(8)),
            12 => Ok(ColorType::RGB(4)),
            24 => Ok(ColorType::RGB(8)),
            _  => Err("Unsupported bitdepth")
        }
    }

    pub fn to_image_colortype(&self) -> image::ColorType {
        match self {
            &ColorType::Gray(n) => image::Gray(n),
            &ColorType::RGB(n) => image::RGB(n)
        }
    }

    pub fn bits_per_pixel(&self) -> u32 {
        match self {
            &ColorType::Gray(n) => n as u32,
            &ColorType::RGB(n) => 3 * n as u32,
        }
    }
}
