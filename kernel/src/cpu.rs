use crate::{
    asm,
    error::{Error, Result},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuModel {
    CortexA53, // Raspberry Pi 3
    CortexA72, // Raspberry Pi 4
    Unknown(u32),
}

pub fn detect_cpu_model() -> Result<CpuModel> {
    let midr = asm::read_main_id_reg();
    let cpu_model = match (midr >> 4) & 0xfff {
        0xd03 => CpuModel::CortexA53,
        0xd08 => CpuModel::CortexA72,
        other => CpuModel::Unknown(other),
    };

    match cpu_model {
        CpuModel::CortexA53 => Ok(cpu_model),
        _ => Err(Error::UnsupportedCpuModel(cpu_model)),
    }
}

pub fn mmio_base() -> u32 {
    match detect_cpu_model().unwrap() {
        CpuModel::CortexA53 => 0x3f000000,
        CpuModel::CortexA72 => unreachable!(),
        CpuModel::Unknown(_) => unreachable!(),
    }
}
