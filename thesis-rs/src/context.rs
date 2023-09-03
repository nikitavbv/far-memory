pub struct Context {
    numbering_id_counter: usize,
    section_counter: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            numbering_id_counter: 0,
            section_counter: 0,
        }
    }

    pub fn next_numbering_id(&mut self) -> usize {
        self.numbering_id_counter += 1;
        self.numbering_id_counter
    }

    pub fn next_section_index(&mut self) -> usize {
        self.section_counter += 1;
        self.section_counter
    }
}

pub struct SectionContext {
    subsection_counter: usize,
}

impl SectionContext {
    pub fn new() -> Self {
        Self {
            subsection_counter: 0,
        }
    }

    pub fn next_subsection_index(&mut self) -> usize {
        self.subsection_counter += 1;
        self.subsection_counter
    }
}