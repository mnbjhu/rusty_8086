use std::fmt::Display;

use crate::decoder::{
    common::rm_to_reg::{decode_rm_to_from_reg, decode_rm_to_reg},
    instr::Instr,
    mov::{AL, AX},
};

use super::{loc::Location, state::DecoderState};

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

pub fn decode_op(state: &mut DecoderState) -> Option<Instr> {
    let byte = state.get_byte(0);
    match byte {
        // Register/Memory with Register to Either
        _ if 0b00000000 == byte & 0b11000100 => {
            let (dest, src) = decode_rm_to_from_reg(state);
            let kind = (byte & 0b00111000) >> 3;
            Some(Instr::Op(OpInstr {
                kind: decode_op_kind(kind),
                dest,
                src,
            }))
        }
        // Immediate to Register/Memory
        _ if 0b10000000 == byte & 0b11000100 => {
            let second = state.get_byte(1);
            let op = (second & 0b00111000) >> 3;
            let (dest, src) = decode_rm_to_reg(state);
            Some(Instr::Op(OpInstr {
                kind: decode_op_kind(op),
                dest,
                src,
            }))
        }
        // Immediate to Accumulator
        _ if 0b00000100 == byte & 0b11000110 => decode_imm_to_acc(state),
        _ => None,
    }
}

fn decode_imm_to_acc(state: &mut DecoderState) -> Option<Instr> {
    let byte = state.get_byte(0);
    state.add_len(1);
    let w = 0b00000001 & byte;
    if w == 0 {
        let dest = Location::Reg(AL);
        let data = state.get_byte(1);
        state.add_len(1);
        let src = Location::Immediate8(data);
        Some(Instr::Op(OpInstr {
            kind: OpKind::Add,
            dest,
            src,
        }))
    } else {
        let dest = Location::Reg(AX);
        let low = state.get_byte(1);
        let high = state.get_byte(2);
        state.add_len(2);
        let src = Location::Immediate16((high as u16) << 8 | low as u16);
        Some(Instr::Op(OpInstr {
            kind: OpKind::Add,
            dest,
            src,
        }))
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
        match (&self.dest, &self.src) {
            (Location::Eac(_), Location::Immediate8(_)) => {
                write!(f, "{} {}, byte {}", self.kind, self.dest, self.src)
            }
            (Location::Eac(_), Location::Immediate16(_)) => {
                write!(f, "{} {}, word {}", self.kind, self.dest, self.src)
            }
            _ => write!(f, "{} {}, {}", self.kind, self.dest, self.src),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        loc::{eac::EffectiveAddress, eac_mode::EffectiveAddressMode, Location},
        mov::{AX, BX, CX, SI},
        op::{OpInstr, OpKind},
    };

    #[test]
    fn test_op_rm_and_reg_to_either() {
        let asm = decode(vec![
            0b1, 0b11011000, 0b101001, 0b11011000, 0b111001, 0b11011000,
        ]);

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
        let asm = decode(vec![0b10000011, 0b11000001, 0b1100]);

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

    #[test]
    fn test_add_si_imm() {
        let asm = decode(vec![0b10000011, 0b11000110, 0b10]);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(SI),
                src: Location::Immediate8(2),
            })
        );
    }

    #[test]
    fn test_add_mem_to_reg() {
        let asm = decode(vec![0b11, 0b110110, 0b1010, 0b0]);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(SI),
                src: Location::Mem(10),
            })
        );
    }

    #[test]
    fn test_add_imm_to_mem() {
        let asm = decode(vec![0b10000001, 0b110, 0b1010, 0b0, 0b11101000, 0b11]);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Mem(10),
                src: Location::Immediate16(1000),
            })
        );
    }

    #[test]
    fn test_display_op_kind() {
        assert_eq!(format!("{}", OpKind::Add), "add");
        assert_eq!(format!("{}", OpKind::Sub), "sub");
        assert_eq!(format!("{}", OpKind::Cmp), "cmp");
    }

    #[test]
    fn test_display_op_instr() {
        let instr = OpInstr {
            kind: OpKind::Add,
            dest: Location::Reg(AX),
            src: Location::Reg(BX),
        };

        assert_eq!(format!("{}", instr), "add ax, bx");

        let instr = OpInstr {
            kind: OpKind::Add,
            dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::Bx)),
            src: Location::Immediate8(12),
        };

        assert_eq!(format!("{}", instr), "add [bx], byte 12");

        let instr = OpInstr {
            kind: OpKind::Add,
            dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::Bx)),
            src: Location::Immediate16(12),
        };

        assert_eq!(format!("{}", instr), "add [bx], word 12");
    }
}
