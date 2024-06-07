use core::panic;
use std::{fmt::Display, vec::IntoIter};

use super::loc::{eac::decode_eac, Location};

pub mod acc;
pub mod immediate;

#[derive(Debug, PartialEq)]
pub struct MoveInstr {
    pub dest: Location,
    pub src: Location,
}

pub const AX: &str = "ax";
pub const CX: &str = "cx";
pub const DX: &str = "dx";
pub const BX: &str = "bx";
pub const SP: &str = "sp";
pub const BP: &str = "bp";
pub const SI: &str = "si";
pub const DI: &str = "di";

pub const AL: &str = "al";
pub const CL: &str = "cl";
pub const DL: &str = "dl";
pub const BL: &str = "bl";
pub const AH: &str = "ah";
pub const CH: &str = "ch";
pub const DH: &str = "dh";
pub const BH: &str = "bh";

pub fn decode_rm_to_from_reg(first: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let second = bytes.next().unwrap();
    let d = first & 0b00000010;
    let w = first & 0b00000001;
    let reg = (second & 0b000111000) >> 3;
    let reg = decode_reg(w, reg);
    let mod_ = second & 0b11000000;
    let rm = if mod_ != 0b11000000 {
        if (second & 0b00000111) == 0b110 && mod_ == 0 {
            if w == 0 {
                Location::Mem(bytes.next().unwrap() as u16)
            } else {
                let low = bytes.next().unwrap();
                let high = bytes.next().unwrap();
                Location::Mem((high as u16) << 8 | low as u16)
            }
        } else {
            Location::Eac(decode_eac(second, bytes))
        }
    } else {
        Location::Reg(decode_reg(w, second & 0b000000111))
    };

    if d == 0 {
        (rm, Location::Reg(reg))
    } else {
        (Location::Reg(reg), rm)
    }
}

pub fn decode_rm_to_from_reg_dest(first: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let second = bytes.next().unwrap();
    let d = first & 0b00000010;
    let w = first & 0b00000001;
    let reg = (second & 0b000111000) >> 3;
    let reg = decode_reg(w, reg);
    let mod_ = second & 0b11000000;
    let rm = if mod_ != 0b11000000 {
        if (second & 0b00000111) == 0b110 && mod_ == 0 {
            if w == 0 {
                Location::Mem(bytes.next().unwrap() as u16)
            } else {
                let low = bytes.next().unwrap();
                let high = bytes.next().unwrap();
                Location::Mem((high as u16) << 8 | low as u16)
            }
        } else {
            Location::Eac(decode_eac(second, bytes))
        }
    } else {
        Location::Reg(decode_reg(w, second & 0b000000111))
    };

    if d == 0 {
        (rm, Location::Reg(reg))
    } else {
        (Location::Reg(reg), rm)
    }
}

pub fn decode_reg(w: u8, reg: u8) -> &'static str {
    match (w, reg) {
        (1, 0b000) => AX,
        (1, 0b001) => CX,
        (1, 0b010) => DX,
        (1, 0b011) => BX,
        (1, 0b100) => SP,
        (1, 0b101) => BP,
        (1, 0b110) => SI,
        (1, 0b111) => DI,
        (0, 0b000) => AL,
        (0, 0b001) => CL,
        (0, 0b010) => DL,
        (0, 0b011) => BL,
        (0, 0b100) => AH,
        (0, 0b101) => CH,
        (0, 0b110) => DH,
        (0, 0b111) => BH,
        _ => panic!("unexpected reg: {}, w: {}", reg, w),
    }
}

impl Display for MoveInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.dest, &self.src) {
            (Location::Eac(_), Location::Immediate8(_)) => {
                write!(f, "mov {}, byte {}", self.dest, self.src)
            }
            (Location::Eac(_), Location::Immediate16(_)) => {
                write!(f, "mov {}, word {}", self.dest, self.src)
            }
            _ => write!(f, "mov {}, {}", self.dest, self.src),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        mov::{Location, MoveInstr, AH, AL, AX, BP, BX, CH, CL, CX, DI, DX, SI, SP},
    };

    #[test]
    fn basic_test() {
        let bytes = vec![0b10001001, 0b11011001];
        let mut bytes = bytes.into_iter();
        let (dest, src) = super::decode_rm_to_from_reg(bytes.next().unwrap(), &mut bytes);
        assert_eq!(dest, Location::Reg(CX));
        assert_eq!(src, Location::Reg(BX));
    }

    #[test]
    fn extended_test() {
        let mut bytes = vec![
            0b10001001, 0b11011001, 0b10001000, 0b11100101, 0b10001001, 0b11011010, 0b10001001,
            0b11011110, 0b10001001, 0b11111011, 0b10001000, 0b11001000, 0b10001000, 0b11101101,
            0b10001001, 0b11000011, 0b10001001, 0b11110011, 0b10001001, 0b11111100, 0b10001001,
            0b11000101,
        ]
        .into_iter();
        let asm = decode(&mut bytes);

        println!("{:?}", asm);
        assert_eq!(asm.len(), 11);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Reg(AH)
            })
        );
        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[3],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(SI),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[4],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(DI)
            })
        );
        assert_eq!(
            asm[5],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Reg(CL)
            })
        );
        assert_eq!(
            asm[6],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Reg(CH)
            })
        );
        assert_eq!(
            asm[7],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(AX)
            })
        );
        assert_eq!(
            asm[8],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(SI)
            })
        );
        assert_eq!(
            asm[9],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(SP),
                src: Location::Reg(DI)
            })
        );
        assert_eq!(
            asm[10],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BP),
                src: Location::Reg(AX)
            })
        );
    }
}
