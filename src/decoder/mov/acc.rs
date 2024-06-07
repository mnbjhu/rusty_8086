use crate::decoder::mov::{Location, MoveInstr};

pub fn decode_mem_to_acc(first: u8, bytes: &mut std::vec::IntoIter<u8>) -> MoveInstr {
    let w = first & 0b00000001;
    if w == 0 {
        let low = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Reg("al"),
            src: Location::Mem(low as u16),
        }
    } else {
        let low = bytes.next().unwrap();
        let high = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Reg("ax"),
            src: Location::Mem((high as u16) << 8 | low as u16),
        }
    }
}

pub fn decode_acc_to_mem(first: u8, bytes: &mut std::vec::IntoIter<u8>) -> MoveInstr {
    let w = first & 0b00000001;
    if w == 0 {
        let low = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Mem(low as u16),
            src: Location::Reg("al"),
        }
    } else {
        let low = bytes.next().unwrap();
        let high = bytes.next().unwrap();
        MoveInstr {
            dest: Location::Mem((high as u16) << 8 | low as u16),
            src: Location::Reg("ax"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decoder::{
        decode,
        instr::Instr,
        mov::{Location, MoveInstr},
    };

    #[test]
    fn test_memory_to_acc() {
        let mut bytes = vec![0b10100001, 0b11111011, 0b1001, 0b10100001, 0b10000, 0b0].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 2);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Reg("ax"),
                src: Location::Mem(2555),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Reg("ax"),
                src: Location::Mem(16),
            })
        );
    }

    #[test]
    fn test_acc_to_memory() {
        let mut bytes = vec![0b10100011, 0b11111010, 0b1001, 0b10100011, 0b1111, 0b0].into_iter();
        let asm = decode(&mut bytes);

        assert_eq!(asm.len(), 2);

        assert_eq!(
            asm[0],
            Instr::Mov(MoveInstr {
                dest: Location::Mem(2554),
                src: Location::Reg("ax"),
            })
        );

        assert_eq!(
            asm[1],
            Instr::Mov(MoveInstr {
                dest: Location::Mem(15),
                src: Location::Reg("ax"),
            })
        );
    }
}
