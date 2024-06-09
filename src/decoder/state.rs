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

    pub fn has_more(&self) -> bool {
        self.offset < self.src.len()
    }

    pub fn get_byte(&self, offset: usize) -> u8 {
        let byte = self
            .src
            .get(self.offset + offset)
            .expect("Out of instr bounds");
        *byte
    }

    pub fn add_len(&mut self, len: usize) {
        self.instr_len += len;
    }

    pub fn next(&mut self) -> bool {
        self.offset += self.instr_len;
        self.instr_len = 0;
        self.has_more()
    }

    pub fn get_instr_len(&self) -> usize {
        self.instr_len
    }
}
