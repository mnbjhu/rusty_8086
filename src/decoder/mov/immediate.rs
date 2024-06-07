use std::vec::IntoIter;

use crate::decoder::{
    loc::{eac::decode_eac, Location},
    mov::{decode_reg, MoveInstr},
};

pub fn decode_mov_imm_to_reg(first: u8, bytes: &mut IntoIter<u8>) -> MoveInstr {
    let w = (first & 0b00001000) >> 3;
    let reg = first & 0b00000111;
    let reg = decode_reg(w, reg);
    if w == 0 {
        let second = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate8(second),
        }
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate16(second),
        }
    }
}

pub fn decode_mov_imm_to_rm(
    first: u8,
    second: u8,
    bytes: &mut IntoIter<u8>,
) -> (Location, Location) {
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
        mov::{MoveInstr, CH, CL, CX, DX},
    };

    #[test]
    fn test_8bit_immediate_to_reg() {
        let mut bytes = vec![0b10110001, 0b1100, 0b10110101, 0b11110100].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 2);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CL),
                src: Location::Immediate8(12),
            })
        );
        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Immediate8(244),
            })
        );
    }

    #[test]
    fn test_16bit_immediate_to_reg() {
        let mut bytes = vec![
            0b10111001, 0b1100, 0b0, 0b10111001, 0b11110100, 0b11111111, 0b10111010, 0b1101100,
            0b1111, 0b10111010, 0b10010100, 0b11110000,
        ]
        .into_iter();

        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 4);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate16(12),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate16(65524),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate16(3948),
            })
        );

        assert_eq!(
            asm[3],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate16(61588),
            })
        );
    }
}
