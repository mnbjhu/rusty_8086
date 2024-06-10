pub struct DecoderState {
    pub src: Vec<u8>,
    pub offset: usize,
    pub instr_len: usize,
}

impl DecoderState {
    pub fn new(src: Vec<u8>) -> Self {
        Self {
            src,
            offset: 0,
            instr_len: 0,
        }
    }
}

impl Decoder for DecoderState {
    fn has_more(&self) -> bool {
        self.offset < self.src.len()
    }

    fn get_byte(&self, offset: usize) -> u8 {
        let byte = self
            .src
            .get(self.offset + offset)
            .expect("Out of instr bounds");
        *byte
    }

    fn add_len(&mut self, len: usize) {
        self.instr_len += len;
    }

    fn next(&mut self) -> bool {
        self.offset += self.instr_len;
        self.instr_len = 0;
        self.has_more()
    }

    fn get_instr_len(&self) -> usize {
        self.instr_len
    }

    fn advance(&mut self) {
        self.offset += self.instr_len;
        self.instr_len = 0;
    }
}

pub trait Decoder {
    fn has_more(&self) -> bool;
    fn get_byte(&self, offset: usize) -> u8;
    fn add_len(&mut self, len: usize);
    fn next(&mut self) -> bool;
    fn get_instr_len(&self) -> usize;
    fn advance(&mut self);
}
