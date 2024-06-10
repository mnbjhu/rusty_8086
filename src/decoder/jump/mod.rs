use crate::decoder::state::Decoder;

use super::instr::Instr;

pub fn decode_jump<T: Decoder>(state: &mut T) -> Option<Instr> {
    let byte = state.get_byte(0);
    match byte {
        _ if 0b01110100 == byte => {
            let to = state.get_byte(1);
            state.add_len(2);
            Some(Instr::Je(to as i8))
        }
        _ if 0b01110101 == byte => {
            let to = state.get_byte(1);
            state.add_len(2);
            Some(Instr::Jne(to as i8))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{decode, instr::Instr};

    #[test]
    fn test_jump_eq_zero() {
        let asm = decode(vec![0b01110100, 0b00000111]);

        assert_eq!(asm.len(), 1);
        assert_eq!(asm[0], Instr::Je(7));
    }

    #[test]
    fn test_jump_not_eq_zero() {
        let asm = decode(vec![0b01110101, 0b00000111]);

        assert_eq!(asm.len(), 1);
        assert_eq!(asm[0], Instr::Jne(7));
    }
}
