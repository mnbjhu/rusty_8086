use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum EffectiveAddress {
    NoOffset(EffectiveAddressMode),
    ByteOffset(EffectiveAddressMode, i8),
    WordOffset(EffectiveAddressMode, i16),
}

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
    Direct,
}

impl Display for EffectiveAddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddressMode::BxSi => write!(f, "bx+si"),
            EffectiveAddressMode::BxDi => write!(f, "bx+di"),
            EffectiveAddressMode::BpSi => write!(f, "bp+si"),
            EffectiveAddressMode::BpDi => write!(f, "bp+di"),
            EffectiveAddressMode::Si => write!(f, "si"),
            EffectiveAddressMode::Di => write!(f, "di"),
            EffectiveAddressMode::Bp => write!(f, "bp"),
            EffectiveAddressMode::Bx => write!(f, "bx"),
            EffectiveAddressMode::Direct => unreachable!(),
        }
    }
}

impl Display for EffectiveAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddress::NoOffset(mode) => write!(f, "{}", mode),
            EffectiveAddress::ByteOffset(mode, offset) => write!(f, "{}+{}", mode, offset),
            EffectiveAddress::WordOffset(mode, offset) => write!(f, "{}+{}", mode, offset),
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

pub fn decode_eac(first: u8, bytes: &mut std::vec::IntoIter<u8>) -> EffectiveAddress {
    let mode = decode_eac_mode(first & 0b111);
    match first >> 6 {
        0b00 => EffectiveAddress::NoOffset(mode),
        0b01 => {
            let offset = bytes.next().unwrap() as i8;
            EffectiveAddress::ByteOffset(mode, offset)
        }
        0b10 => {
            let low = bytes.next().unwrap() as u16;
            let high = bytes.next().unwrap() as u16;
            let offset = (high << 8 | low) as i16;
            EffectiveAddress::WordOffset(mode, offset)
        }
        _ => panic!("Expected 2 bits, got: {:#b}", first >> 6),
    }
}
