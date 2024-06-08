#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        loc::Location,
        mov::AX,
        op::{OpInstr, OpKind},
    };

    #[test]
    fn test_add_acc_16bit() {
        let mut bytes = vec![0b10000011, 0b11000000, 0b1100].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(AX),
                src: Location::Immediate16(12),
            })
        );
    }
}
