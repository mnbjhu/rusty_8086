use std::{fmt::Display, vec::IntoIter};

use crate::decoder::{
    common::rm_to_reg::{decode_rm_to_from_reg, decode_rm_to_reg},
    instr::Instr,
    mov::{AL, AX},
};

use super::loc::Location;

pub mod acc;

#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Cmp,
}

#[derive(Debug, PartialEq)]
pub struct OpInstr {
    pub kind: OpKind,
    pub dest: Location,
    pub src: Location,
}

pub fn decode_op(byte: u8, bytes: &mut IntoIter<u8>) -> Option<Instr> {
    match byte {
        _ if 0b00000000 == byte & 0b11000100 => {
            let (dest, src) = decode_rm_to_from_reg(byte, bytes);
            let kind = (byte & 0b00111000) >> 3;
            Some(Instr::Op(OpInstr {
                kind: decode_op_kind(kind),
                dest,
                src,
            }))
        }
        _ if 0b10000000 == byte & 0b11000100 => {
            let second = bytes.next().unwrap();
            let op = (second & 0b00111000) >> 3;
            let (dest, src) = decode_rm_to_reg(byte, second, bytes);
            Some(Instr::Op(OpInstr {
                kind: decode_op_kind(op),
                dest,
                src,
            }))
        }
        _ if 0b00111100 == byte & 0b11111110 => {
            let w = 0b00000001 & byte;
            if w == 0 {
                let dest = Location::Reg(AL);
                let src = Location::Immediate8(bytes.next().unwrap());
                Some(Instr::Op(OpInstr {
                    kind: OpKind::Add,
                    dest,
                    src,
                }))
            } else {
                let dest = Location::Reg(AX);
                let low = bytes.next().unwrap();
                let high = bytes.next().unwrap();
                let src = Location::Immediate16((high as u16) << 8 | low as u16);
                Some(Instr::Op(OpInstr {
                    kind: OpKind::Add,
                    dest,
                    src,
                }))
            }
        }
        _ => None,
    }
}

impl Display for OpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpKind::Add => write!(f, "add"),
            OpKind::Sub => write!(f, "sub"),
            OpKind::Cmp => write!(f, "cmp"),
        }
    }
}

pub fn decode_op_kind(op_part: u8) -> OpKind {
    match op_part {
        0b000 => OpKind::Add,
        0b101 => OpKind::Sub,
        0b111 => OpKind::Cmp,
        _ => panic!("Unknown op kind: {:#10b}", op_part),
    }
}

impl Display for OpInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}, {}", self.kind, self.dest, self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::{decode_instr, Instr},
        loc::Location,
        mov::{AX, BX, CX},
        op::{OpInstr, OpKind},
    };

    #[test]
    fn test_op_rm_and_reg_to_either() {
        let mut bytes =
            vec![0b1, 0b11011000, 0b101001, 0b11011000, 0b111001, 0b11011000].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 3);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(AX),
                src: Location::Reg(BX),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Op(OpInstr {
                kind: OpKind::Sub,
                dest: Location::Reg(AX),
                src: Location::Reg(BX),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Op(OpInstr {
                kind: OpKind::Cmp,
                dest: Location::Reg(AX),
                src: Location::Reg(BX),
            })
        );
    }

    #[test]
    fn test_op_imm_with_rm() {
        let mut bytes = vec![0b10000011, 0b11000001, 0b1100].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(CX),
                src: Location::Immediate8(12),
            })
        );
    }
}
