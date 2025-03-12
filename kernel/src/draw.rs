use crate::{color::ColorCode, error::Result};

pub trait Draw {
    fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: ColorCode,
    ) -> Result<()>;
    fn draw_string(
        &mut self,
        x: usize,
        y: usize,
        s: &str,
        fore_color: ColorCode,
        back_color: ColorCode,
    ) -> Result<()>;
    fn draw_font(
        &mut self,
        x: usize,
        y: usize,
        c: char,
        fore_color: ColorCode,
        back_color: ColorCode,
    ) -> Result<()>;
    fn fill(&mut self, color: ColorCode) -> Result<()>;
    fn copy(&mut self, x: usize, y: usize, to_x: usize, to_y: usize) -> Result<()>;
    fn read(&self, x: usize, y: usize) -> Result<ColorCode>;
    fn write(&mut self, x: usize, y: usize, color: ColorCode) -> Result<()>;
}
