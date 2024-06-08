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

#[cfg(test)]
mod test {
    use crate::decoder::loc::{eac::EffectiveAddress, eac_mode::EffectiveAddressMode, Location};

    #[test]
    fn test_location_display() {
        let reg = Location::Reg("ax");
        let mem = Location::Mem(1234);
        let imm16 = Location::Immediate16(1234);
        let imm8 = Location::Immediate8(12);
        let eac = Location::Eac(EffectiveAddress::Byte(EffectiveAddressMode::BxSi, 12));

        assert_eq!(reg.to_string(), "ax");
        assert_eq!(mem.to_string(), "[1234]");
        assert_eq!(imm16.to_string(), "1234");
        assert_eq!(imm8.to_string(), "12");
        assert_eq!(eac.to_string(), "[bx + si + 12]");
    }
}
