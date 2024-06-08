#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        loc::Location,
        mov::AL,
        op::{OpInstr, OpKind},
    };

    #[test]
    fn test_add_acc_8bit() {
        let mut bytes = vec![0b100, 0b1100].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);

        assert_eq!(
            asm[0],
            Instr::Op(OpInstr {
                kind: OpKind::Add,
                dest: Location::Reg(AL),
                src: Location::Immediate8(12),
            })
        );
    }
}
