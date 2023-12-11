pub struct Context {
    numbering_id_counter: usize,
    sections: Vec<SectionContext>,
    references_counter: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            numbering_id_counter: 0,
            sections: Vec::new(),
            references_counter: 0,
        }
    }

    pub fn next_numbering_id(&mut self) -> usize {
        self.numbering_id_counter += 1;
        self.numbering_id_counter
    }

    pub fn last_section_index(&self) -> usize {
        self.sections.len()
    }

    pub fn next_section_index(&mut self) -> usize {
        self.sections.push(SectionContext::new());
        self.last_section_index()
    }

    fn section(&mut self, section_index: usize) -> &mut SectionContext {
        // quick fix for images in conference abstract
        if self.sections.is_empty() {
            self.sections.push(SectionContext::new());
        }

        // quick fix for images in conference abstract
        let section_index = if section_index == 0 {
            0
        } else {
            section_index - 1
        };

        self.sections.get_mut(section_index).unwrap()
    }

    pub fn next_subsection_index(&mut self, section_index: usize) -> usize {
        self.section(section_index).next_subsection_index()
    }

    pub fn next_image_index(&mut self, section_index: usize) -> usize {
        self.section(section_index).next_image_index()
    }

    pub fn next_reference_id(&mut self) -> usize {
        self.references_counter += 1;
        self.references_counter
    }
}

struct SectionContext {
    subsection_counter: usize,
    image_counter: usize,
}

impl SectionContext {
    pub fn new() -> Self {
        Self {
            subsection_counter: 0,
            image_counter: 0,
        }
    }

    pub fn next_subsection_index(&mut self) -> usize {
        self.subsection_counter += 1;
        self.subsection_counter
    }

    pub fn next_image_index(&mut self) -> usize {
        self.image_counter += 1;
        self.image_counter
    }
}
