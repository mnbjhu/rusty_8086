use std::{fmt::Display, vec::IntoIter};

use crate::decoder::{
    mov::{decode_mov, MoveInstr},
    op::decode_op,
};

use super::op::OpInstr;

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
    if let Some(instr) = decode_mov(byte, bytes) {
        return instr;
    }
    if let Some(instr) = decode_op(byte, bytes) {
        return instr;
    }
    panic!("Unknown instruction #{}: {:#10b} ", count, byte)
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        loc::Location,
        mov::{AX, BX},
        op::OpKind,
    };

    use super::*;

    #[test]
    fn test_mov_display() {
        let mov = Instr::Mov(MoveInstr {
            dest: Location::Reg(AX),
            src: Location::Reg(BX),
        });

        assert_eq!(mov.to_string(), "mov ax, bx");
    }

    #[test]
    fn test_op_display() {
        let op = Instr::Op(OpInstr {
            kind: OpKind::Add,
            dest: Location::Reg(AX),
            src: Location::Reg(BX),
        });

        assert_eq!(op.to_string(), "add ax, bx");
    }
}
