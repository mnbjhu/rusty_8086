use std::fmt::Display;

use super::eac_mode::{decode_eac_mode, EffectiveAddressMode};

#[derive(Debug, PartialEq)]
pub enum EffectiveAddress {
    Mode(EffectiveAddressMode),
    Byte(EffectiveAddressMode, i8),
    Word(EffectiveAddressMode, i16),
}

impl Display for EffectiveAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddress::Mode(mode) => write!(f, "{}", mode),
            EffectiveAddress::Byte(mode, offset) => {
                if *offset < 0 {
                    write!(f, "{} - {}", mode, offset.abs())
                } else {
                    write!(f, "{} + {}", mode, offset)
                }
            }
            EffectiveAddress::Word(mode, offset) => {
                if *offset < 0 {
                    write!(f, "{} - {}", mode, offset.abs())
                } else {
                    write!(f, "{} + {}", mode, offset)
                }
            }
        }
    }
}

pub fn decode_eac(first: u8, bytes: &mut std::vec::IntoIter<u8>) -> EffectiveAddress {
    let mode = decode_eac_mode(first & 0b111);
    match first >> 6 {
        0b00 => EffectiveAddress::Mode(mode),
        0b01 => {
            let offset = bytes.next().unwrap() as i8;
            EffectiveAddress::Byte(mode, offset)
        }
        0b10 => {
            let low = bytes.next().unwrap() as u16;
            let high = bytes.next().unwrap() as u16;
            let offset = (high << 8 | low) as i16;
            EffectiveAddress::Word(mode, offset)
        }
        _ => panic!("Expected 2 bits, got: {:#b}", first >> 6),
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        loc::{eac::EffectiveAddress, eac_mode::EffectiveAddressMode, Location},
        mov::{MoveInstr, AH, AL, BP, BX, CH, CL, CX, DX},
    };

    #[test]
    fn test_source_addr_calulation() {
        let mut bytes = vec![
            0b10001010, 0b0, 0b10001011, 0b11011, 0b10001011, 0b1010110, 0b0,
        ]
        .into_iter();

        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 3);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BxSi)),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BpDi)),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Eac(EffectiveAddress::Byte(EffectiveAddressMode::Bp, 0)),
            })
        );
    }

    #[test]
    fn test_source_addr_calulation_with_8bit_offset() {
        let mut bytes = vec![0b10001010, 0b1100000, 0b100].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AH),
                src: Location::Eac(EffectiveAddress::Byte(EffectiveAddressMode::BxSi, 4)),
            })
        );
    }

    #[test]
    fn test_source_addr_calulation_with_16bit_offset() {
        let mut bytes = vec![0b10001010, 0b10000000, 0b10000111, 0b10011].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Eac(EffectiveAddress::Word(EffectiveAddressMode::BxSi, 4999)),
            })
        );
    }

    #[test]
    fn test_dest_add_calculation() {
        let mut bytes = vec![
            0b10001001, 0b1001, 0b10001000, 0b1010, 0b10001000, 0b1101110, 0b0,
        ]
        .into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 3);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BxDi,)),
                src: Location::Reg(CX),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BpSi,)),
                src: Location::Reg(CL),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Eac(EffectiveAddress::Byte(EffectiveAddressMode::Bp, 0)),
                src: Location::Reg(CH),
            })
        );
    }

    #[test]
    fn test_direct_access() {
        let mut bytes = vec![
            0b10001011, 0b101110, 0b101, 0b0, 0b10001011, 0b11110, 0b10000010, 0b1101,
        ]
        .into_iter();

        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 2);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BP),
                src: Location::Mem(5),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Mem(3458),
            })
        );
    }

    #[test]
    fn test_eac_display_none() {
        let eac = EffectiveAddress::Mode(EffectiveAddressMode::Bx);
        assert_eq!(eac.to_string(), "bx");
    }

    #[test]
    fn test_eac_display_byte() {
        let eac = EffectiveAddress::Byte(EffectiveAddressMode::Bp, 5);
        assert_eq!(eac.to_string(), "bp + 5");
    }

    #[test]
    fn test_eac_display_word() {
        let eac = EffectiveAddress::Word(EffectiveAddressMode::Bp, 5);
        assert_eq!(eac.to_string(), "bp + 5");
    }
}
