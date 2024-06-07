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

#[cfg(test)]
mod test {
    use crate::decoder::{
        dis,
        instr::Instr,
        mov::{
            eac::{EffectiveAddress, EffectiveAddressMode},
            Location, MoveInstr, AH, AL, BX, CH, CL, CX, DX,
        },
    };

    #[test]
    fn test_source_addr_calulation() {
        let mut bytes = vec![
            0b10001010, 0b0, 0b10001011, 0b11011, 0b10001011, 0b1010110, 0b0,
        ]
        .into_iter();

        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 3);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Eac(EffectiveAddress::NoOffset(EffectiveAddressMode::BxSi)),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Eac(EffectiveAddress::NoOffset(EffectiveAddressMode::BpDi)),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Eac(EffectiveAddress::ByteOffset(EffectiveAddressMode::Bp, 0)),
            })
        );
    }

    #[test]
    fn test_source_addr_calulation_with_8bit_offset() {
        let mut bytes = vec![0b10001010, 0b1100000, 0b100].into_iter();
        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AH),
                src: Location::Eac(EffectiveAddress::ByteOffset(EffectiveAddressMode::BxSi, 4)),
            })
        );
    }

    #[test]
    fn test_source_addr_calulation_with_16bit_offset() {
        let mut bytes = vec![0b10001010, 0b10000000, 0b10000111, 0b10011].into_iter();
        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Eac(EffectiveAddress::WordOffset(
                    EffectiveAddressMode::BxSi,
                    4999
                )),
            })
        );
    }

    #[test]
    fn test_dest_add_calculation() {
        let mut bytes = vec![
            0b10001001, 0b1001, 0b10001000, 0b1010, 0b10001000, 0b1101110, 0b0,
        ]
        .into_iter();
        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 3);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::NoOffset(EffectiveAddressMode::BxDi,)),
                src: Location::Reg(CX),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::NoOffset(EffectiveAddressMode::BpSi,)),
                src: Location::Reg(CL),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::ByteOffset(EffectiveAddressMode::Bp, 0)),
                src: Location::Reg(CH),
            })
        );
    }
}
