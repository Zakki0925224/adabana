use crate::framebuffer::PixelFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorCode {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_u32(value: u32, pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::Bgr => Self::new(value as u8, (value >> 8) as u8, (value >> 16) as u8),
            PixelFormat::Rgb => Self::new((value >> 16) as u8, (value >> 8) as u8, value as u8),
        }
    }

    pub fn to_u32(&self, pixel_format: PixelFormat) -> u32 {
        match pixel_format {
            PixelFormat::Bgr => self.r as u32 | (self.g as u32) << 8 | (self.b as u32) << 16,
            PixelFormat::Rgb => (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32,
        }
    }
}
