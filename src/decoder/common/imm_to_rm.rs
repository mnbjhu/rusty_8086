use std::vec::IntoIter;

use crate::decoder::loc::{eac::decode_eac, Location};

pub fn decode_imm_to_rm(first: u8, second: u8, bytes: &mut IntoIter<u8>) -> (Location, Location) {
    let w = first & 0b00000001;
    let eac = decode_eac(second, bytes);
    let src = if w == 0 {
        let second = bytes.next().unwrap();
        Location::Immediate8(second)
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        Location::Immediate16(second)
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
        let mut bytes = vec![0b10110011, 0b1100100].into_iter();

        let asm = decode(&mut bytes);

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
        let mut bytes = vec![0b10111011, 0b1100100, 0b0].into_iter();

        let asm = decode(&mut bytes);

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
