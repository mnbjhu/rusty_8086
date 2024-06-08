use std::vec::IntoIter;

use super::instr::Instr;

pub fn decode_jump(byte: u8, bytes: &mut IntoIter<u8>) -> Option<Instr> {
    match byte {
        _ if 0b01110100 == byte => {
            let to = bytes.next().unwrap();
            Some(Instr::Je(to))
        }
        _ if 0b01110101 == byte => {
            let to = bytes.next().unwrap();
            Some(Instr::Jne(to))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{decode, instr::Instr};

    #[test]
    fn test_jump_eq_zero() {
        let mut bytes = vec![0b01110100, 0b00000111].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);
        assert_eq!(asm[0], Instr::Je(7));
    }

    #[test]
    fn test_jump_not_eq_zero() {
        let mut bytes = vec![0b01110101, 0b00000111].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 1);
        assert_eq!(asm[0], Instr::Jne(7));
    }
}
