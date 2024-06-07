use std::{fmt::Display, vec::IntoIter};

use crate::decoder::mov::{immediate::decode_mov_imm_to_reg, MoveInstr};

use super::{
    loc::Location,
    mov::{
        acc::{decode_acc_to_mem, decode_mem_to_acc},
        decode_rm_to_from_reg,
        immediate::decode_mov_imm_to_rm,
    },
    op::{decode_op_kind, OpInstr},
};

#[derive(Debug, PartialEq)]
pub enum Instr {
    Mov(MoveInstr),
    Op(OpInstr),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Mov(mov) => write!(f, "{}", mov),
            Instr::Op(op) => write!(f, "{}", op),
        }
    }
}

pub fn decode_instr(byte: u8, bytes: &mut IntoIter<u8>, count: i32) -> Instr {
    match byte {
        _ if 0b10001000 == byte & 0b11111100 => {
            let (dest, src) = decode_rm_to_from_reg(byte, bytes);
            Instr::Mov(MoveInstr { dest, src })
        }
        _ if 0b10110000 == byte & 0b11110000 => Instr::Mov(decode_mov_imm_to_reg(byte, bytes)),
        _ if 0b11000110 == byte & 0b11111110 => {
            let second = bytes.next().unwrap();
            let (dest, src) = decode_mov_imm_to_rm(byte, second, bytes);
            Instr::Mov(MoveInstr { dest, src })
        }
        _ if 0b10100000 == byte & 0b11111110 => Instr::Mov(decode_mem_to_acc(byte, bytes)),
        _ if 0b10100010 == byte & 0b11111110 => Instr::Mov(decode_acc_to_mem(byte, bytes)),
        _ if 0b00000000 == byte & 0b11000100 => {
            let (dest, src) = decode_rm_to_from_reg(byte, bytes);
            let kind = (byte & 0b00111000) >> 3;
            Instr::Op(OpInstr {
                kind: decode_op_kind(kind),
                dest,
                src,
            })
        }
        _ if 0b10000000 == byte & 0b11111100 => {
            let second = bytes.next().unwrap();
            if second & 0b11000000 == 0b11000000 {
                let w = byte & 0b00000001;
                let src = if w == 0 {
                    let second = bytes.next().unwrap();
                    Location::Immediate8(second)
                } else {
                    let second = bytes.next().unwrap();
                    let third = bytes.next().unwrap();
                    let second = (third as u16) << 8 | second as u16;
                    Location::Immediate16(second)
                };

                let dest = decode_rm_to_from_reg(byte, bytes);

                Instr::Op(OpInstr {
                    kind: decode_op_kind(second),
                    dest,
                    src,
                })
            } else {
                let (dest, src) = decode_mov_imm_to_rm(byte, second, bytes);
                Instr::Op(OpInstr {
                    kind: decode_op_kind(second),
                    dest,
                    src,
                })
            }
        }

        _ => panic!("Unknown instruction #{}: {:#10b} ", count, byte),
    }
}
