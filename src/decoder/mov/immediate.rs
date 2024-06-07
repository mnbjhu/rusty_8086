use std::vec::IntoIter;

use crate::decoder::mov::{decode_reg, Location, MoveInstr};

pub fn decode_mov_im(first: u8, bytes: &mut IntoIter<u8>) -> MoveInstr {
    let w = (first & 0b00001000) >> 3;
    let reg = first & 0b00000111;
    let reg = decode_reg(w, reg);
    if w == 0 {
        let second = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate(second.into()),
        }
    } else {
        let second = bytes.next().unwrap();
        let third = bytes.next().unwrap();
        let second = (third as u16) << 8 | second as u16;
        MoveInstr {
            dest: Location::Reg(reg),
            src: Location::Immediate(second),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        dis,
        instr::Instr,
        mov::{Location, MoveInstr, CH, CL, CX, DX},
    };

    #[test]
    fn test_8bit_immediate_to_reg() {
        let mut bytes = vec![0b10110001, 0b1100, 0b10110101, 0b11110100].into_iter();
        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 2);
        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CL),
                src: Location::Immediate(12),
            })
        );
        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CH),
                src: Location::Immediate(244),
            })
        );
    }

    #[test]
    fn test_16_bit_immediate_to_reg() {
        let mut bytes = vec![
            0b10111001, 0b1100, 0b0, 0b10111001, 0b11110100, 0b11111111, 0b10111010, 0b1101100,
            0b1111, 0b10111010, 0b10010100, 0b11110000,
        ]
        .into_iter();

        let asm = dis(&mut bytes);

        assert_eq!(asm.len(), 4);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate(12),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(CX),
                src: Location::Immediate(65524),
            })
        );

        assert_eq!(
            asm[2],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate(3948),
            })
        );

        assert_eq!(
            asm[3],
            Instr::Mov(MoveInstr {
                dest: Location::Reg(DX),
                src: Location::Immediate(61588),
            })
        );
    }
}
