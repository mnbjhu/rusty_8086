use std::fmt::Display;

use super::loc::Location;

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
        instr::Instr,
        loc::Location,
        mov::{AX, BX},
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
}
