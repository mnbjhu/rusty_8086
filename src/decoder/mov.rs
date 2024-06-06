use core::panic;
use std::fmt::Display;

pub struct MoveInstr {
    pub dest: &'static str,
    pub src: &'static str,
}

const AX: &str = "ax";
const CX: &str = "cx";
const DX: &str = "dx";
const BX: &str = "bx";
const SB: &str = "sb";
const SP: &str = "sp";
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

pub fn decode_mov(first: u8, second: u8) -> MoveInstr {
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
        MoveInstr { dest: rm, src: reg }
    } else {
        MoveInstr { dest: reg, src: rm }
    }
}

pub fn decode_reg(w: u8, reg: u8) -> &'static str {
    match (w, reg) {
        (1, 0b000) => AX,
        (1, 0b001) => CX,
        (1, 0b010) => DX,
        (1, 0b011) => BX,
        (1, 0b100) => SB,
        (1, 0b101) => SP,
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
        _ => panic!("unexpected reg {}", reg),
    }
}

impl Display for MoveInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mov {}, {}", self.dest, self.src)
    }
}
