use std::fmt::Display;

use self::eac::EffectiveAddress;

pub mod eac;
pub mod eac_mode;

#[derive(Debug, PartialEq)]
pub enum Location {
    Reg(&'static str),
    Mem(u16),
    Immediate16(u16),
    Immediate8(u8),
    Eac(EffectiveAddress),
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Reg(reg) => write!(f, "{}", reg),
            Location::Mem(addr) => write!(f, "[{}]", addr),
            Location::Immediate16(val) => write!(f, "{}", val),
            Location::Immediate8(val) => write!(f, "{}", val),
            Location::Eac(eac) => write!(f, "[{}]", eac),
        }
    }
}
