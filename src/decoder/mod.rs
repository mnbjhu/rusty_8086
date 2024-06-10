use crate::decoder::{
    instr::{decode_instr, Instr},
    state::Decoder,
};

use self::state::DecoderState;

pub mod common;
pub mod instr;
pub mod jump;
pub mod loc;
pub mod mov;
pub mod op;
pub mod state;

pub fn decode(bytes: Vec<u8>) -> Vec<Instr> {
    let mut found = vec![];
    let mut state = DecoderState::new(bytes);
    while state.next() {
        found.push(decode_instr(&mut state));
    }
    found
}
