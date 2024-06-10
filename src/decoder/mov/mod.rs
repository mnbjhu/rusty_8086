use core::panic;
use std::fmt::Display;

use crate::decoder::{
    common::{
        imm_to_reg::decode_imm_to_reg, imm_to_rm::decode_imm_to_rm,
        rm_to_reg::decode_rm_to_from_reg,
    },
    instr::Instr,
    mov::acc::{decode_acc_to_mem, decode_mem_to_acc},
    state::Decoder,
};

use super::loc::Location;

pub mod acc;
pub mod immediate;

#[derive(Debug, PartialEq)]
pub struct MoveInstr {
    pub dest: Location,
    pub src: Location,
}

pub const AX: &str = "ax";
pub const CX: &str = "cx";
pub const DX: &str = "dx";
pub const BX: &str = "bx";
pub const SP: &str = "sp";
pub const BP: &str = "bp";
pub const SI: &str = "si";
pub const DI: &str = "di";

pub const AL: &str = "al";
pub const CL: &str = "cl";
pub const DL: &str = "dl";
pub const BL: &str = "bl";
pub const AH: &str = "ah";
pub const CH: &str = "ch";
pub const DH: &str = "dh";
pub const BH: &str = "bh";

pub fn decode_mov<T: Decoder>(state: &mut T) -> Option<Instr> {
    let byte = state.get_byte(0);
    match byte {
        // Register/Memory to/from Register
        _ if 0b10001000 == byte & 0b11111100 => {
            let (dest, src) = decode_rm_to_from_reg(state);
            Some(Instr::Mov(MoveInstr { dest, src }))
        }
        // Immediate to Register/Memory
        _ if 0b10110000 == byte & 0b11110000 => {
            let (dest, src) = decode_imm_to_reg(state);
            Some(Instr::Mov(MoveInstr { dest, src }))
        }
        // Immediate to Register
        _ if 0b11000110 == byte & 0b11111110 => {
            let (dest, src) = decode_imm_to_rm(state);
            Some(Instr::Mov(MoveInstr { dest, src }))
        }
        // Memory to Accumulator
        _ if 0b10100000 == byte & 0b11111110 => Some(Instr::Mov(decode_mem_to_acc(state))),
        // Accumulator to Memory
        _ if 0b10100010 == byte & 0b11111110 => Some(Instr::Mov(decode_acc_to_mem(state))),
        _ => None,
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
        _ => panic!("unexpected reg: {}, w: {}", reg, w),
    }
}

impl Display for MoveInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.dest, &self.src) {
            (Location::Eac(_), Location::Immediate8(_)) => {
                write!(f, "mov {}, byte {}", self.dest, self.src)
            }
            (Location::Eac(_), Location::Immediate16(_)) => {
                write!(f, "mov {}, word {}", self.dest, self.src)
            }
            _ => write!(f, "mov {}, {}", self.dest, self.src),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        decoder::{
            common::rm_to_reg::decode_rm_to_from_reg,
            decode,
            instr::Instr,
            loc::{eac::EffectiveAddress, eac_mode::EffectiveAddressMode},
            mov::{Location, MoveInstr, AH, AL, AX, BP, BX, CH, CL, CX, DI, DX, SI, SP},
            state::DecoderState,
        },
        sim::SimState,
    };

    #[test]
    fn basic_test() {
        let mut state = DecoderState::new(vec![0b10001001, 0b11011001]);
        let (dest, src) = decode_rm_to_from_reg(&mut state);
        assert_eq!(dest, Location::Reg(CX));
        assert_eq!(src, Location::Reg(BX));
    }

    #[test]
    fn extended_test() {
        let asm = decode(vec![
            0b10001001, 0b11011001, 0b10001000, 0b11100101, 0b10001001, 0b11011010, 0b10001001,
            0b11011110, 0b10001001, 0b11111011, 0b10001000, 0b11001000, 0b10001000, 0b11101101,
            0b10001001, 0b11000011, 0b10001001, 0b11110011, 0b10001001, 0b11111100, 0b10001001,
            0b11000101,
        ]);

        println!("{:?}", asm);
        assert_eq!(asm.len(), 11);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Reg(AH)
            })
        );
        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[3],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(SI),
                src: Location::Reg(BX)
            })
        );
        assert_eq!(
            asm[4],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(DI)
            })
        );
        assert_eq!(
            asm[5],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(AL),
                src: Location::Reg(CL)
            })
        );
        assert_eq!(
            asm[6],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Reg(CH)
            })
        );
        assert_eq!(
            asm[7],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(AX)
            })
        );
        assert_eq!(
            asm[8],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BX),
                src: Location::Reg(SI)
            })
        );
        assert_eq!(
            asm[9],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(SP),
                src: Location::Reg(DI)
            })
        );
        assert_eq!(
            asm[10],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(BP),
                src: Location::Reg(AX)
            })
        );
    }

    #[test]
    fn test_display_mov_basic() {
        let mov = Instr::Mov(MoveInstr {
            dest: Location::Reg(AX),
            src: Location::Reg(BX),
        });

        assert_eq!(mov.to_string(), "mov ax, bx");
    }

    #[test]
    fn test_display_mov_mem_word() {
        let mov = Instr::Mov(MoveInstr {
            dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BxSi)),
            src: Location::Immediate16(123),
        });

        assert_eq!(mov.to_string(), "mov [bx + si], word 123");
    }

    #[test]
    fn test_display_mov_mem_byte() {
        let mov = Instr::Mov(MoveInstr {
            dest: Location::Eac(EffectiveAddress::Mode(EffectiveAddressMode::BxSi)),
            src: Location::Immediate8(123),
        });

        assert_eq!(mov.to_string(), "mov [bx + si], byte 123");
    }

    #[test]
    fn test_mem_mov() {
        let mut state = SimState::new(vec![
            0b11000111, 0b110, 0b11101000, 0b11, 0b1, 0b0, 0b11000111, 0b110, 0b11101010, 0b11,
            0b10, 0b0, 0b11000111, 0b110, 0b11101100, 0b11, 0b11, 0b0, 0b11000111, 0b110,
            0b11101110, 0b11, 0b100, 0b0, 0b10111011, 0b11101000, 0b11, 0b11000111, 0b1000111,
            0b100, 0b1010, 0b0, 0b10001011, 0b11110, 0b11101000, 0b11, 0b10001011, 0b1110,
            0b11101010, 0b11, 0b10001011, 0b10110, 0b11101100, 0b11, 0b10001011, 0b101110,
            0b11101110, 0b11,
        ]);

        state.run();

        assert_eq!(state.get_register_16("bx"), 1);
        assert_eq!(state.get_register_16("cx"), 2);
        assert_eq!(state.get_register_16("dx"), 10);
        assert_eq!(state.get_register_16("bp"), 1);
        assert_eq!(state.get_register_16("ip"), 1);
    }
}
