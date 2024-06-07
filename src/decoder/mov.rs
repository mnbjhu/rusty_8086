use core::panic;
use std::{fmt::Display, vec::IntoIter};

#[derive(Debug, PartialEq)]
pub struct MoveInstr {
    pub dest: &'static str,
    pub src: &'static str,
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
        _ => panic!("unexpected reg {}", reg),
    }
}

impl Display for MoveInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mov {}, {}", self.dest, self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        dis,
        instr::Instr,
        mov::{MoveInstr, AH, AL, AX, BP, BX, CH, CL, CX, DI, DX, SI, SP},
    };

    #[test]
    fn basic_test() {
        let bytes = vec![0b10001001, 0b11011001];
        let mut bytes = bytes.into_iter();
        let instr = super::decode_mov(bytes.next().unwrap(), &mut bytes);
        assert_eq!(instr.dest, CX);
        assert_eq!(instr.src, BX);
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

        assert_eq!(asm[0], Instr::Mov(MoveInstr { dest: CX, src: BX }));
        assert_eq!(asm[1], Instr::Mov(MoveInstr { dest: CH, src: AH }));
        assert_eq!(asm[2], Instr::Mov(MoveInstr { dest: DX, src: BX }));
        assert_eq!(asm[3], Instr::Mov(MoveInstr { dest: SI, src: BX }));
        assert_eq!(asm[4], Instr::Mov(MoveInstr { dest: BX, src: DI }));
        assert_eq!(asm[5], Instr::Mov(MoveInstr { dest: AL, src: CL }));
        assert_eq!(asm[6], Instr::Mov(MoveInstr { dest: CH, src: CH }));
        assert_eq!(asm[7], Instr::Mov(MoveInstr { dest: BX, src: AX }));
        assert_eq!(asm[8], Instr::Mov(MoveInstr { dest: BX, src: SI }));
        assert_eq!(asm[9], Instr::Mov(MoveInstr { dest: SP, src: DI }));
        assert_eq!(asm[10], Instr::Mov(MoveInstr { dest: BP, src: AX }));
    }
}
