use crate::{cpu::CpuModel, framebuffer::FramebufferError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Failed(&'static str),
    UnsupportedCpuModel(CpuModel),
    NotInitialized,
    InvalidArgument,
    FramebufferError(FramebufferError),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Failed(s)
    }
}

impl From<FramebufferError> for Error {
    fn from(err: FramebufferError) -> Self {
        Self::FramebufferError(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
