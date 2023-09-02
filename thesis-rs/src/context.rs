pub struct Context {
    numbering_id_counter: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            numbering_id_counter: 0,
        }
    }

    pub fn next_numbering_id(&mut self) -> usize {
        self.numbering_id_counter += 1;
        self.numbering_id_counter
    }
}