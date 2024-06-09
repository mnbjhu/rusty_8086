use crate::decoder::{loc::Location, state::DecoderState};

use super::rm_to_reg::decode_rm;

pub fn decode_imm_to_rm(state: &mut DecoderState) -> (Location, Location) {
    state.add_len(2);
    let dest = decode_rm(state);
    let src = decode_imm(state);
    (dest, src)
}

fn decode_imm(state: &mut DecoderState) -> Location {
    let w = state.get_byte(0) & 0b00000001;
    let len = state.get_instr_len();
    let src = if w == 0 {
        let data = state.get_byte(len);
        state.add_len(1);
        Location::Immediate8(data)
    } else {
        let low = state.get_byte(len);
        let high = state.get_byte(len + 1);
        state.add_len(2);

        let data = (high as u16) << 8 | low as u16;
        Location::Immediate16(data)
    };
    src
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        loc::Location,
        mov::{MoveInstr, BL, BX},
    };

    #[test]
    fn test_decode_8bit_imm_to_rm() {
        let asm = decode(vec![0b10110011, 0b1100100]);

        assert_eq!(asm.len(), 1);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                src: Location::Immediate8(0b1100100),
                dest: Location::Reg(BL)
            })
        );
    }

    #[test]
    fn test_decode_16bit_imm_to_rm() {
        let asm = decode(vec![0b10111011, 0b1100100, 0b0]);

        assert_eq!(asm.len(), 1);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                src: Location::Immediate16(0b1100100),
                dest: Location::Reg(BX)
            })
        );
    }

    #[test]
    fn test_decode_imm_to_mem_byte() {
        let asm = decode(vec![0b11000110, 0b110, 0b11, 0b0, 0b100]);

        assert_eq!(asm.len(), 1);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                src: Location::Immediate8(4),
                dest: Location::Mem(3)
            })
        );
    }

    #[test]
    fn test_decode_imm_to_mem_word() {
        let asm = decode(vec![0b11000111, 0b110, 0b11, 0b0, 0b100, 0b0]);

        assert_eq!(asm.len(), 1);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                src: Location::Immediate16(4),
                dest: Location::Mem(3)
            })
        );
    }
}
