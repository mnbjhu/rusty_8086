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
        let asm = decode(vec![0b10110001, 0b1100, 0b10110101, 0b11110100]);

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
        let asm = decode(vec![
            0b10111001, 0b1100, 0b0, 0b10111001, 0b11110100, 0b11111111, 0b10111010, 0b1101100,
            0b1111, 0b10111010, 0b10010100, 0b11110000,
        ]);

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
