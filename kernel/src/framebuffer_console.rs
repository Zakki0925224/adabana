use crate::{
    color::ColorCode,
    error::{Error, Result},
    font::{FONT, TAB_DISP_STR},
    framebuffer::{self, FramebufferInfo},
    mutex::Mutex,
};
use core::fmt::{self, Write};

static mut FB_CONSOLE: FramebufferConsole = FramebufferConsole::new();

struct FramebufferConsole {
    cursor_x: usize,
    cursor_y: usize,
    fb_width: Option<usize>,
    fb_height: Option<usize>,
    fore_color: ColorCode,
    back_color: ColorCode,
}

impl FramebufferConsole {
    const fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            fb_width: None,
            fb_height: None,
            fore_color: ColorCode::default(),
            back_color: ColorCode::default(),
        }
    }

    fn fb_wh(&self) -> Result<(usize, usize)> {
        Ok((
            self.fb_width.ok_or(Error::NotInitialized)?,
            self.fb_height.ok_or(Error::NotInitialized)?,
        ))
    }

    fn init(&mut self, fore_color: ColorCode, back_color: ColorCode) -> Result<()> {
        self.fore_color = fore_color;
        self.back_color = back_color;

        let FramebufferInfo {
            p_width: _,
            p_height: _,
            v_width,
            v_height,
            ..
        } = framebuffer::get_info()?;
        self.fb_width = Some(v_width as usize);
        self.fb_height = Some(v_height as usize);

        Ok(())
    }

    fn swap_color(&mut self) {
        if !(self.cursor_x == 0 && self.cursor_y == 0) {
            return;
        }

        let tmp = self.back_color;
        self.back_color = self.fore_color;
        self.fore_color = tmp;
    }

    fn inc_cursor(&mut self) -> Result<()> {
        let (fb_width, fb_height) = self.fb_wh()?;
        let (font_width, font_height) = FONT.get_wh();

        self.cursor_x += 1;

        if (self.cursor_x + 1) * font_width >= fb_width {
            self.cursor_x = 0;
            self.cursor_y += 1;
            if (self.cursor_y + 1) * font_height >= fb_height {
                self.cursor_y = 0;
            }
        }

        self.swap_color();
        Ok(())
    }

    fn inc_cursor_new_line(&mut self) -> Result<()> {
        let (_, fb_height) = self.fb_wh()?;
        let (_, font_height) = FONT.get_wh();

        self.cursor_x = 0;
        self.cursor_y += 1;

        if (self.cursor_y + 1) * font_height >= fb_height {
            self.cursor_y = 0;
        }

        self.swap_color();
        Ok(())
    }

    fn tab(&mut self) -> Result<()> {
        for c in TAB_DISP_STR.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<()> {
        match c {
            '\n' => return self.inc_cursor_new_line(),
            '\t' => return self.tab(),
            _ => (),
        }

        let (font_width, font_height) = FONT.get_wh();
        framebuffer::draw_font(
            self.cursor_x * font_width,
            self.cursor_y * font_height,
            c,
            self.fore_color,
            self.back_color,
        )?;

        self.inc_cursor()?;
        Ok(())
    }
}

impl fmt::Write for FramebufferConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if let Err(_) = self.write_char(c) {
                return Err(fmt::Error);
            }
        }
        Ok(())
    }
}

pub fn init(fore_color: ColorCode, back_color: ColorCode) -> Result<()> {
    unsafe { FB_CONSOLE.init(fore_color, back_color) }
}

pub fn write_fmt(args: fmt::Arguments) -> Result<()> {
    let _ = unsafe { FB_CONSOLE.write_fmt(args) };
    Ok(())
}
