use crate::decoder::{
    loc::{eac::decode_eac, Location},
    state::DecoderState,
};

pub fn decode_imm_to_rm(state: &mut DecoderState) -> (Location, Location) {
    let first = state.get_byte(0);
    state.add_len(1);
    let w = first & 0b00000001;
    let eac = decode_eac(state);
    let src = if w == 0 {
        let data = state.get_byte(1);
        state.add_len(1);
        Location::Immediate8(data)
    } else {
        let second = state.get_byte(1);
        let third = state.get_byte(2);
        state.add_len(2);
        let data = (third as u16) << 8 | second as u16;
        Location::Immediate16(data)
    };
    (Location::Eac(eac), src)
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
}
