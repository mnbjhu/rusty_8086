use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_eac_mode() {
        assert_eq!(decode_eac_mode(0b000), EffectiveAddressMode::BxSi);
        assert_eq!(decode_eac_mode(0b001), EffectiveAddressMode::BxDi);
        assert_eq!(decode_eac_mode(0b010), EffectiveAddressMode::BpSi);
        assert_eq!(decode_eac_mode(0b011), EffectiveAddressMode::BpDi);
        assert_eq!(decode_eac_mode(0b100), EffectiveAddressMode::Si);
        assert_eq!(decode_eac_mode(0b101), EffectiveAddressMode::Di);
        assert_eq!(decode_eac_mode(0b110), EffectiveAddressMode::Bp);
        assert_eq!(decode_eac_mode(0b111), EffectiveAddressMode::Bx);
    }

    #[test]
    fn test_display_eac_mode() {
        assert_eq!(EffectiveAddressMode::BxSi.to_string(), "bx + si");
        assert_eq!(EffectiveAddressMode::BxDi.to_string(), "bx + di");
        assert_eq!(EffectiveAddressMode::BpSi.to_string(), "bp + si");
        assert_eq!(EffectiveAddressMode::BpDi.to_string(), "bp + di");
        assert_eq!(EffectiveAddressMode::Si.to_string(), "si");
        assert_eq!(EffectiveAddressMode::Di.to_string(), "di");
        assert_eq!(EffectiveAddressMode::Bp.to_string(), "bp");
        assert_eq!(EffectiveAddressMode::Bx.to_string(), "bx");
    }
}
