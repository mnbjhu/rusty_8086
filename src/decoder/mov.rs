use core::panic;
use std::{fmt::Display, vec::IntoIter};

#[derive(Debug, PartialEq)]
pub struct MoveInstr {
    pub dest: Location,
    pub src: Location,
}

#[derive(Debug, PartialEq)]
pub enum Location {
    Reg(&'static str),
    Mem(u16),
    Immediate(u16),
}

const AX: &str = "ax";
const CX: &str = "cx";
const DX: &str = "dx";
const BX: &str = "bx";
const SP: &str = "sp";
const BP: &str = "bp";
const SI: &str = "si";
const DI: &str = "di";

const AL: &str = "al";
const CL: &str = "cl";
const DL: &str = "dl";
const BL: &str = "bl";
const AH: &str = "ah";
const CH: &str = "ch";
const DH: &str = "dh";
const BH: &str = "bh";

pub fn decode_mov(first: u8, bytes: &mut IntoIter<u8>) -> MoveInstr {
    let second = bytes.next().unwrap();
    let d = first & 0b00000010;
    let w = first & 0b00000001;
    let mod_ = second & 0b11000000;
    if mod_ != 0b11000000 {
        panic!("Not memory 'mov' not implemented");
    }
    let reg = (second & 0b000111000) >> 3;
    let rm = second & 0b000000111;

    let reg = decode_reg(w, reg);
    let rm = decode_reg(w, rm);
    if d == 0 {
        MoveInstr {
            dest: Location::Reg(rm),
            src: Location::Reg(reg),
        }
    } else {
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Reg(rm),
        }
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
        write!(f, "mov {}, {}", self.dest, self.src)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Reg(reg) => write!(f, "{}", reg),
            Location::Mem(addr) => write!(f, "[{}]", addr),
            Location::Immediate(val) => write!(f, "{:#x}", val),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        dis,
        instr::Instr,
        mov::{Location, MoveInstr, AH, AL, AX, BP, BX, CH, CL, CX, DI, DX, SI, SP},
    };

    #[test]
    fn basic_test() {
        let bytes = vec![0b10001001, 0b11011001];
        let mut bytes = bytes.into_iter();
        let instr = super::decode_mov(bytes.next().unwrap(), &mut bytes);
        assert_eq!(instr.dest, Location::Reg(CX));
        assert_eq!(instr.src, Location::Reg(BX));
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
        let asm = dis(&mut bytes);

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

    #[test]
    fn test_8bit_immediate_to_reg() {
        let mut bytes = vec![0b10110001, 0b1100, 0b10110101, 0b11110100].into_iter();
        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 2);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CL),
                src: Location::Immediate(12),
            })
        );
        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Immediate(244),
            })
        );
    }

    #[test]
    fn test_16_bit_immediate_to_reg() {
        let mut bytes = vec![
            0b10111001, 0b1100, 0b0, 0b10111001, 0b11110100, 0b11111111, 0b10111010, 0b1101100,
            0b1111, 0b10111010, 0b10010100, 0b11110000,
        ]
        .into_iter();

        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 4);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate(12),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate(65524),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate(3948),
            })
        );

        assert_eq!(
            asm[3],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate(61588),
            })
        );
    }
}
