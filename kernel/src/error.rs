use crate::cpu::CpuModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Failed(&'static str),
    UnsupportedCpuModel(CpuModel),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Failed(s)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
