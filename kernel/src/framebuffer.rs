use crate::{
    addr::VirtualAddress,
    color::ColorCode,
    draw::Draw,
    error::{Error, Result},
    mutex::Mutex,
};

static mut FB: Mutex<Framebuffer> = Mutex::new(Framebuffer::new());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PixelFormat {
    Bgr = 0,
    Rgb = 1,
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::Bgr
    }
}

impl TryFrom<u32> for PixelFormat {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Self::Bgr),
            1 => Ok(Self::Rgb),
            _ => Err(Error::InvalidArgument),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferError {
    PositionOutOfRange { x: usize, y: usize },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FramebufferInfo {
    pub p_width: usize,
    pub p_height: usize,
    pub v_width: usize,
    pub v_height: usize,
    pub depth: usize,
    pub pixel_format: PixelFormat,
    pub buf_base: VirtualAddress,
    pub buf_size: usize,
}

struct Framebuffer {
    info: Option<FramebufferInfo>,
}

impl Framebuffer {
    const fn new() -> Self {
        Self { info: None }
    }

    fn info(&self) -> Result<FramebufferInfo> {
        self.info.ok_or(Error::NotInitialized)
    }

    fn init(&mut self, info: FramebufferInfo) {
        self.info = Some(info);
    }
}

impl Draw for Framebuffer {
    fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: ColorCode,
    ) -> Result<()> {
        for y in y..y + height {
            for x in x..x + width {
                self.write(x, y, color)?;
            }
        }

        Ok(())
    }

    fn draw_string(
        &mut self,
        _x: usize,
        _y: usize,
        _s: &str,
        _fore_color: ColorCode,
        _back_color: ColorCode,
    ) -> Result<()> {
        unimplemented!()
    }

    fn draw_font(
        &mut self,
        _x: usize,
        _y: usize,
        _c: char,
        _fore_color: ColorCode,
        _back_color: ColorCode,
    ) -> Result<()> {
        todo!()
    }

    fn fill(&mut self, color: ColorCode) -> Result<()> {
        let info = self.info()?;

        for y in 0..info.v_height {
            for x in 0..info.v_width {
                self.write(x, y, color)?;
            }
        }

        Ok(())
    }

    fn copy(&mut self, _x: usize, _y: usize, _to_x: usize, _to_y: usize) -> Result<()> {
        unimplemented!()
    }

    fn read(&self, _x: usize, _y: usize) -> Result<ColorCode> {
        unimplemented!()
    }

    fn write(&mut self, x: usize, y: usize, color: ColorCode) -> Result<()> {
        let info = self.info()?;
        let offset = (y * info.v_width + x) * 4;
        let buf_ptr: *mut u32 = info.buf_base.offset(offset).as_ptr_mut();

        if x >= info.v_width || y >= info.v_height {
            return Err(FramebufferError::PositionOutOfRange { x, y }.into());
        }

        unsafe { *buf_ptr = color.to_u32(info.pixel_format) }

        Ok(())
    }
}

pub fn init(info: FramebufferInfo) -> Result<()> {
    unsafe { FB.try_lock() }?.init(info);
    Ok(())
}

pub fn fill(color: ColorCode) -> Result<()> {
    unsafe { FB.try_lock() }?.fill(color)
}

pub fn draw_rect(x: usize, y: usize, width: usize, height: usize, color: ColorCode) -> Result<()> {
    unsafe { FB.try_lock() }?.draw_rect(x, y, width, height, color)
}
