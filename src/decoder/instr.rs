use std::fmt::Display;

use crate::decoder::{
    jump::decode_jump,
    mov::{decode_mov, MoveInstr},
    op::decode_op,
};

use super::{op::OpInstr, state::DecoderState};

#[derive(Debug, PartialEq)]
pub enum Instr {
    Mov(MoveInstr),
    Op(OpInstr),
    Je(u8),
    Jne(u8),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Mov(mov) => write!(f, "{}", mov),
            Instr::Op(op) => write!(f, "{}", op),
            Instr::Je(offset) => write!(f, "je {}", offset),
            Instr::Jne(offset) => write!(f, "jne {}", offset),
        }
    }
}

pub fn decode_instr(state: &mut DecoderState) -> Instr {
    let instr = decode_mov(state)
        .or_else(|| decode_op(state))
        .or_else(|| decode_jump(state));
    if let Some(instr) = instr {
        return instr;
    } else {
        panic!("Unknown instruction: {:#10b} ", state.get_byte(0))
    }
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
