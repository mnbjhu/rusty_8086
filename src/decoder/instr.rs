use std::fmt::Display;

use crate::decoder::mov::MoveInstr;

pub enum Instr {
    Mov(MoveInstr),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Instr::Mov(instr) = self;
        write!(f, "{}", instr)
    }
}
