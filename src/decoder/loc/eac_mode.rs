use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum EffectiveAddressMode {
    BxSi,
    BxDi,
    BpSi,
    BpDi,
    Si,
    Di,
    Bp,
    Bx,
}

impl Display for EffectiveAddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddressMode::BxSi => write!(f, "bx + si"),
            EffectiveAddressMode::BxDi => write!(f, "bx + di"),
            EffectiveAddressMode::BpSi => write!(f, "bp + si"),
            EffectiveAddressMode::BpDi => write!(f, "bp + di"),
            EffectiveAddressMode::Si => write!(f, "si"),
            EffectiveAddressMode::Di => write!(f, "di"),
            EffectiveAddressMode::Bp => write!(f, "bp"),
            EffectiveAddressMode::Bx => write!(f, "bx"),
            // EffectiveAddressMode::Direct => unreachable!(),
        }
    }
}

pub fn decode_eac_mode(byte: u8) -> EffectiveAddressMode {
    match byte {
        0b000 => EffectiveAddressMode::BxSi,
        0b001 => EffectiveAddressMode::BxDi,
        0b010 => EffectiveAddressMode::BpSi,
        0b011 => EffectiveAddressMode::BpDi,
        0b100 => EffectiveAddressMode::Si,
        0b101 => EffectiveAddressMode::Di,
        0b110 => EffectiveAddressMode::Bp,
        0b111 => EffectiveAddressMode::Bx,
        _ => panic!("Expected 3 bits, got: {:#b}", byte),
    }
}
