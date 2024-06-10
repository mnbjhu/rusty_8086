use std::fmt::Display;

use crate::decoder::{
    jump::decode_jump,
    mov::{decode_mov, MoveInstr},
    op::decode_op,
    state::Decoder,
};

use super::op::OpInstr;

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

pub fn decode_instr<T: Decoder>(state: &mut T) -> Instr {
    let instr = decode_mov(state)
        .or_else(|| decode_op(state))
        .or_else(|| decode_jump(state));
    if let Some(instr) = instr {
        instr
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
        state::DecoderState,
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

    #[test]
    fn test_je_display() {
        let je = Instr::Je(0x12);

        assert_eq!(je.to_string(), "je 18");
    }

    #[test]
    fn test_jne_display() {
        let jne = Instr::Jne(0x12);

        assert_eq!(jne.to_string(), "jne 18");
    }

    #[test]
    #[should_panic(expected = "Unknown instruction: 0b11111111")]
    fn test_invalid_input() {
        let mut state = DecoderState::new(vec![0b11111111]);
        decode_instr(&mut state);
    }
}
